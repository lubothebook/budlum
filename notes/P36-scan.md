# P36 scan: Network rest (read-only audit, Tier R3)

Base commit: 9d46064. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md (line 55: egress privacy unwired), docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt (gossip_dedup record_valid, record_invalid, peers_below_threshold, peer_score, get_peer_score, window_utilization; mobile update_battery, update_nat, set_relay_address, is_sufficient_for_bud), .github/idle-code-baseline.txt (gossip_dedup constants; mobile ConnectionType, NatType, bud_available, default_policy, from_power_mode).

## COVERAGE

Read in full (every line, tests included):
- src/network/gossip_dedup.rs 1-705
- src/network/mobile.rs 1-768
- src/network/mod.rs 1-12
- src/network/peer_manager.rs 1-1438 (two reads: 1-1000, 1000-1438)
- src/network/privacy.rs 1-455
- src/network/proto_conversions.rs 1-2610 (two reads: 1-1500, 1500-2610)
- src/network/protocol.rs 1-340
- src/network/sync_codec.rs 1-121

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md sections 5 and 6 (lines 110-165); notes/P29-scan.md first 30 lines.

Outside the part (rg first, then at most 80 lines): src/network/node.rs 788-800 (gossipsub message id), 1060-1125 (mobile profile), 2070-2175 (gossip receive path), 2560-2600 and 2730-2745 (vote and blob gates), 2915-2966 (/sync request path), 1945-1960 and 2026-2040 (metrics, disconnect); rg only on node.rs for send_request, request_response::Config, dedup callers; src/ai/registry.rs 186-235 (register_model); src/execution/executor.rs 1420-1470 (AiModelRegister arm); src/core/transaction.rs 1795-1830 (spec signing preimage); src/core/address.rs from_hex and Display; src/main.rs 1240-1275 (mobile profile wiring); proto/budlum/network/protocol.proto 280-302 (ProtoAiModelRegister); .github baselines, STATUS.md and docs/audit/FINDINGS.md (rg only).

Not read: the rest of src/network/node.rs (huge file), the prost generated code (build.rs output), libp2p gossipsub and request_response sources (so the gossipsub duplicate cache time and request_response max_concurrent_streams default are taken from library knowledge, not read here), src/tests, fuzz targets. No cargo, no test run: every claim comes from reading source. No fuzz target or negative test for proto_conversions was looked at beyond the in-file tests.

Extra-check summary:
- Untrusted-peer input bounds: protobuf transport has a 10 MiB gossip cap (node.rs:851, protocol.rs:5) and a 1 MiB /sync request cap (sync_codec.rs:9), but item counts, string lengths and repeated fields are bounded only by those byte caps; counts are checked after full decode (P36-F06). Fixed-size hash fields are length-checked everywhere in proto_conversions.rs.
- Decode panics: none in production code of the 8 files. All unwrap, expect and panic! hits are inside #[cfg(test)] (proto_conversions.rs 2371-2607, gossip_dedup.rs 574 and 629, mobile.rs 508 and 546, sync_codec.rs 109). The only slice in production, mobile.rs:462 `[..8]`, is on a 64-char hex string.
- DoS: P36-F03, F04, F06, F09.
- Peer scoring bypass: P36-F02, F05, F08.
- Dedup cache poisoning: P36-F05, F07.
- Privacy leaks: P36-F11 (error text), F10 (classifier bypass; module is a known unwired seam).
- Unwired parts: privacy.rs known (STATUS.md:55); mobile NAT and relay state is never fed (known dead-pub baseline); MessageError::VersionMismatch is never built (P36-F13).
- Z3: one blanket #[allow] on generated code (P36-F14). Z7: no unsafe in the 8 files. Z9: no key, seed or token material in the 8 files. Z8: no float in consensus paths here (f64 appears only in network scoring and rate tokens).

## Findings

### P36-F01
File: src/network/proto_conversions.rs:202-228 (encode), 1071-1073 (decode); proto/budlum/network/protocol.proto:280-302; src/core/transaction.rs:1803-1809
Severity: High
Axis: correctness
Confidence: Likely (missing verification step: run a wire round trip of an AiModelRegister with execution_program_hash set and check Transaction::verify on the decoded side)
Evidence:
```
src/network/proto_conversions.rs:1071:                    require_execution_proof: false,
src/network/proto_conversions.rs:1072:                    execution_program_hash: None,
src/network/proto_conversions.rs:1073:                    execution_class: 0,
src/core/transaction.rs:1803:    put_u8(out, u8::from(spec.require_execution_proof));
src/core/transaction.rs:1804:    put_u8(out, spec.execution_class);
src/core/transaction.rs:1805:    match spec.execution_program_hash {
```
The encode arm (proto_conversions.rs:202-228) writes none of the three fields and ProtoAiModelRegister has no field for them. The round trip test (proto_conversions.rs:2235-2253) uses only the default values (false, None, 0), so it cannot see the loss.
Description: three AiModelSpec fields are part of the signed transaction preimage but are neither encoded nor decoded; the decoder invents defaults. A spec that carries any non-default value is a different transaction on the receiving node.
Impact: registry.rs:197-225 accepts a spec with a consistent execution_dims and execution_program_hash while require_execution_proof is false (only require=true is refused, registry.rs:191). Such a registration is valid on the node that got it by RPC, but the gossip copy and the copy inside a Block or Blocks message decode to a different preimage, so the hash or signature check fails on peers. The producing node builds blocks its peers reject; sync of that range stalls for every other node. When the verifier goes live (FULL_AI_STARK_VERIFICATION_LIVE, src/ai/mod.rs:25) every proof-required model hits the same wall.
Trigger: precondition any account that can pay the registration fee; entry src/execution/executor.rs:1420 via RPC; input an AiModelRegister whose spec sets execution_program_hash and execution_dims (consistent), included in a block by a producer; expected wrong result: peers decode a spec with program hash None and reject the tx or the block.
Known: none

### P36-F02
File: src/network/peer_manager.rs:73-84, 85-101, 399-427; call sites src/network/node.rs:2098, 2567, 2604, 2672, 2735, 2789
Severity: High
Axis: correctness
Confidence: Likely (missing verification step: a test that calls check_rate_limit then check_vote_rate_limit in the gossip order with a real clock gap of seconds between messages)
Evidence:
```
peer_manager.rs:92:        let elapsed = now.duration_since(self.rate_last_refill).as_secs_f64();
peer_manager.rs:94:        self.rate_last_refill = now;
peer_manager.rs:75:        let elapsed = now.duration_since(self.rate_last_refill).as_secs_f64();
peer_manager.rs:79:        self.vote_tokens = (self.vote_tokens + elapsed * 2.0).min(20.0);
peer_manager.rs:81:        self.blob_tokens = (self.blob_tokens + elapsed * 0.5).min(10.0);
```
The vote and blob buckets are refilled only inside refill_tokens, and refill_tokens measures the time since rate_last_refill. The shared timestamp is reset by consume_token_with_rate (line 94) on every inbound message, and node.rs:2098 calls check_rate_limit for every message before the Prevote, Precommit and QcBlobResponse arms call check_vote_rate_limit or check_blob_rate_limit. So elapsed in refill_tokens is the time between two lines of the same handler (microseconds to milliseconds), and each bucket gains about 0.01 tokens per message.
Description: vote_tokens starts at 10.0 and blob_tokens at 5.0 (peer_manager.rs:56-57) and effectively never refill for a connected peer. After 10 votes (Prevote and Precommit together) relayed by one peer, every further vote from that peer is dropped with a -1 score each (line 408) and, for blobs, after 5 with -5 each (line 423).
Impact: propagation_source is the forwarding mesh peer (node.rs:2083), so each mesh peer relays all validators' votes; with more than 5 validators one checkpoint exhausts the budget. Gossiped finality votes then stop being processed network-wide, within a minute of node start. The penalty path never calls ban_peer (lines 408, 423), so the scores sink to MIN_SCORE without a ban and the node keeps the useless peers. Also refill_tokens uses the constant MSG_REFILL_RATE (line 77) instead of the configured msg_refill_rate, so the configured mainnet rate of 2 per second is ignored on that path.
Trigger: no attacker needed; honest traffic. Entry node.rs:2567.
Known: none

### P36-F03
File: src/network/node.rs:2923-2937 (outside part), src/network/sync_codec.rs:52-56
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
node.rs:2923:  let parsed = NetworkMessage::from_bytes_validated(&request);
node.rs:2932:      !pm.is_banned(&peer)
node.rs:2934:          && pm.check_rate_limit(&peer)
sync_codec.rs:52:  // `/sync` requests are buffered before the handshake, ban and
sync_codec.rs:53:  // rate-limit checks, so the ceiling is the only thing between a
```
Description: every /sync request, including from a banned, unhandshaked or rate-limited peer, is fully protobuf-decoded and converted (up to 1 MiB, with Block and Transaction conversion, hex address parsing and bincode payload parsing) before the ban, handshake and rate-limit check runs. The decode result is needed only to classify handshake messages.
Impact: a banned or rate-limited peer keeps costing the node one full decode per request for as long as the connection lives; the ban does not stop the CPU cost. Classifying by the payload tag would avoid the decode.
Trigger: precondition an open connection; entry node.rs:2915; input repeated 1 MiB requests with many minimal transactions; expected wrong result: CPU spent after the ban.
Known: none

### P36-F04
File: src/network/sync_codec.rs:20-34, 56, 67; request_response config at src/network/node.rs:963-964
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: confirm request_response::Config::default max_concurrent_streams in the pinned libp2p version; it was not read here)
Evidence:
```
sync_codec.rs:24:    let mut buf = Vec::new();
sync_codec.rs:25:    let mut limited = io.take(limit as u64 + 1);
sync_codec.rs:26:    limited.read_to_end(&mut buf).await?;
node.rs:963:   let req_cfg = request_response::Config::default()
node.rs:964:       .with_request_timeout(Duration::from_secs(30));
```
Description: each inbound stream may grow a buffer to 1 MiB (read_to_end doubles capacity, so up to 2 MiB resident) and each response to 10 MiB, held until the 30 s timeout. The only configuration is the timeout; the concurrent stream limit is the library default (100 per connection as far as known).
Impact: one connection can pin about 100 to 200 MiB by trickling bytes on 100 streams; subnet and peer limits (4 per /24, max_peers) bound the connection count but not the per-connection total. A node with 50 connections is exposed to several GiB. Mobile nodes (peer budget 4 to 20) have far less headroom per MiB.
Trigger: precondition a connection that passes admission; entry sync_codec.rs:56; input slow 1 MiB writes on many parallel streams; expected wrong result: memory pressure or OOM.
Known: none

### P36-F05
File: src/network/gossip_dedup.rs:167-192, 249-254; call site src/network/node.rs:2114-2137
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: the gossipsub duplicate cache time, default 60 s, was not read in the pinned libp2p)
Evidence:
```
gossip_dedup.rs:173:  let score = self.peer_scores.entry(*peer).or_default();
gossip_dedup.rs:174:  score.score += SCORE_DUPLICATE;
gossip_dedup.rs:251:  Some(s) => s.score <= MIN_PEER_SCORE || s.duplicate_count >= MAX_DUPLICATE_COUNT,
node.rs:2114:  match dedup.check_and_record(&message.data, &peer_id) {
node.rs:2137:      pm.ban_peer(&peer_id);
```
Description: the duplicate penalty and the ban go to propagation_source, the forwarding peer, not the originator. The score only falls: record_valid and record_invalid have no caller (dead-pub baseline), so the +1 per valid message half of the design does not exist and the score never recovers or decays. Twenty duplicates (each -0.5) reach the -10 threshold.
Impact: gossipsub's own message id is SHA-256 of the payload (node.rs:795-798), so within its cache window it suppresses repeats; a payload republished by anyone after the cache window expires reaches the app dedup (window 10 000) as a Duplicate and charges every honest relay that forwards it. An attacker who replays one old valid message about every minute makes a victim ban honest mesh peers one by one (about 20 minutes each, ban length 1 hour) at the cost of one message per minute. Long-lived honest peers also drift to the threshold from ordinary retransmissions because nothing credits them.
Trigger: precondition a gossip publisher with a cached old message; entry node.rs:2114; input the same bytes repeated at intervals longer than the gossipsub duplicate cache; expected wrong result: the relays are banned instead of the sender.
Known: none (the dead record_valid is in the baseline; this consequence is not)

### P36-F06
File: src/network/proto_conversions.rs:1698-1701, 1978-1984, 1991-1996, 989-997; src/network/protocol.rs:209-217
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: measure resident memory of decoding a 10 MiB Headers message; the amplification factor is an estimate from struct sizes)
Evidence:
```
proto_conversions.rs:1979:   let mut headers = Vec::new();
proto_conversions.rs:1980:   for header in h.headers {
proto_conversions.rs:1981:       headers.push(BlockHeader::try_from(header)?);
protocol.rs:213:     if headers.len() > MAX_HEADERS_PER_REQUEST as usize {
```
Description: the item count limit (2000 headers, 9 blocks, locator 64) is applied after the whole message is decoded and converted. A header with all fields empty and timestamp "0" is a few bytes on the wire, so a 10 MiB Headers message can hold on the order of a million headers; prost builds all of them, then the conversion builds the same number again before validate_header_batch rejects the batch. Blocks and BlocksByHeight are the same with minimal ProtoBlock entries; StateUpdate entries (line 989) and the owners and signatures of multisig authorization have no count limit either.
Impact: one message of at most 10 MiB (gossip) causes a transient allocation of several hundred MiB, and a rate-limited peer still gets 2 to 20 such messages per second. The transient peak, not the steady state, is the risk on mobile and small VPS nodes.
Trigger: precondition any peer that passes the rate limit; entry NetworkMessage::from_bytes (protocol.rs:156); input a Headers payload of repeated minimal ProtoBlockHeader; expected wrong result: large allocation before the 2000 limit is enforced.
Known: none

### P36-F07
File: src/network/gossip_dedup.rs:378-383; src/network/proto_conversions.rs:840-843, 1010, 1020, 1030, 1340, 1350, 1360; src/core/address.rs:10 (outside part)
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
gossip_dedup.rs:379:   hasher.update(b"BDLM_GOSSIP_MSG_V1");
gossip_dedup.rs:381:   hasher.update(data);
proto_conversions.rs:842:  .parse::<u128>()
proto_conversions.rs:1010: bincode::deserialize(&payload.data)
address.rs:10:  let s = s.strip_prefix("0x").unwrap_or(s);
```
Description: the dedup key and the gossipsub id are the hash of the raw bytes, but decoding accepts many encodings of one value: unknown protobuf fields (prost skips them), field order, overlong varints, a timestamp string with a leading plus sign or zeros (u128 parse), an address with or without the 0x prefix or in upper case, and bincode payloads with trailing bytes (bincode::deserialize allows them).
Impact: one signed transaction or block can be re-wrapped into any number of distinct byte strings that all decode to the same object and all pass the dedup window and the gossipsub id cache. Each variant is relayed and decoded again by every node, charged to the rate limit of the peer that sends it, not to the author. Cost to the attacker is the per-peer rate limit; the amplification is network-wide bandwidth and decode CPU, and the replay defence in the module header ("deliberate replay attacks") does not hold.
Trigger: precondition any publisher; entry gossip_dedup.rs:167; input the same message with an appended unknown field; expected wrong result: classified New each time.
Known: none

### P36-F08
File: src/network/peer_manager.rs:314-322, 380-397, 510-515, 357-366
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: a test that fills 10 000 fresh negative records then calls set_handshaked for a new peer)
Evidence:
```
peer_manager.rs:315:  if !self.peers.contains_key(peer_id) && self.peers.len() >= self.max_tracked_peers {
peer_manager.rs:318:      return None;
peer_manager.rs:511:  let Some(score) = self.get_or_create(peer_id) else {
peer_manager.rs:512:      return;
```
Description: when 10 000 records are unprunable (running ban, connected, or negative with last_seen younger than one hour), a peer with no record gets None from get_or_create. check_rate_limit then returns false, so all its messages are dropped, and set_handshaked silently returns, so a new honest peer can never become handshaked. Fresh negative records are cheap to mint: an unhandshaked peer's first non-handshake message earns report_invalid_tx (-5, node.rs:2152-2160) and a record that survives one hour.
Impact: about 2.8 distinct PeerIds per second sustained (connect, send one message, disconnect) keep the table full and the node cannot admit any new peer; it can only keep the peers it already has. This is the eclipse precondition the ceiling comment says it prevents; the TTL (one hour, line 22) shifts the cost but does not remove it. Nothing is logged for the refused handshake except the generic rate-limit warning.
Trigger: precondition a few /24 subnets (4 concurrent connections each, churn unlimited); entry node.rs:2098; input one wrong-order message per fresh key; expected wrong result: every unseen peer refused for the length of the attack plus one hour.
Known: none

### P36-F09
File: src/network/peer_manager.rs:314-322, 357-366
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
peer_manager.rs:316:  self.prune_spent_records();
peer_manager.rs:360:  self.peers.retain(|id, s| {
peer_manager.rs:361:      s.is_banned()
```
Description: while the table is at the ceiling and no record is spent, every message from a peer with no record triggers a full retain over 10 000 entries (each calls Instant::now) under the peer manager lock that the event loop takes.
Impact: CPU amplification during the F08 condition: each refused message costs a 10 000-entry scan, executed on the single swarm task. Bounded by connection churn, so Low.
Trigger: as F08.
Known: none

### P36-F10
File: src/network/privacy.rs:236-254, 289-301, 318-324
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
privacy.rs:322:  let host = lower.rsplit_once(':').map_or(lower, |(host, _port)| host);
privacy.rs:323:  host.contains('.') && host.bytes().any(|b| b.is_ascii_alphabetic())
privacy.rs:300:  _ => {}
```
Description: classify_target returns Opaque for a dotless host name ("localhost", "seed"), a bracketed IPv6 target ("[2001:db8::1]:4001"), and URL style targets ("http://seed.example"). check_egress lets Opaque pass a Direct route under every policy except require_private_route, so allow_direct_dns = false and allow_direct_ip = false are bypassed by those shapes (public_node_no_dns_leak, devnet-like presets).
Impact: when the seam is wired, a configured "no DNS leak" or "no direct IP" policy still allows a direct dial for these target forms. The module is a known unwired seam (STATUS.md:55), so impact is latent; the classifier should be fixed before the dial path is connected.
Trigger: precondition operator config with an IPv6 or dotless seed; entry privacy.rs:264; input "[::1]:4001"; expected wrong result: approval with target_kind Opaque on a Direct route.
Known: unwired is known (STATUS.md:55); the classifier gap is not

### P36-F11
File: src/network/privacy.rs:211-213, 338
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
privacy.rs:212:  write!(f, "invalid network privacy route id: {route_id}")
privacy.rs:338:  Err(NetworkPrivacyError::InvalidRouteId(route_id.to_string()))
```
Description: the module states that the concrete route id is intentionally omitted from approvals and logs, but the error carries and prints the full route id, and the id is accepted up to 64 characters before it is checked for being path or URL like (it may be a rejected URL with credentials).
Impact: a mistyped proxy URL containing user and password reaches logs and error output. Low because the id is operator configuration.
Known: none

### P36-F12
File: src/network/peer_manager.rs:622-649
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
peer_manager.rs:632:  let entry = self.peers.entry(pid).or_default();
```
Description: reload_banned_peers (and reload_banned_peers_legacy) insert records without going through get_or_create, so the max_tracked_peers ceiling and the prune step do not apply. A persisted ban list larger than 10 000 entries enlarges the table past the ceiling; the legacy path gives every entry a full new hour.
Impact: bounded by the local ban file, so memory is not at risk, but the invariant "the map never exceeds the ceiling" asserted by the tests (peer_manager.rs:1152-1196) does not cover this entry point. The table can also start full of old bans, which feeds F08.
Known: none

### P36-F13
File: src/network/protocol.rs:23-28
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
$ rg -n 'VersionMismatch' src/network
src/network/protocol.rs:27:    VersionMismatch { expected: u32, got: u32 },
```
Description: MessageError::VersionMismatch is never constructed or matched in src/network or node.rs. The wire format has no version number at the envelope (the only version is ProtoTransaction.wire_version, hard-coded 2 on encode at proto_conversions.rs:472 and ignored on decode).
Impact: a peer on a different wire version is not detected at decode time; the variant gives the impression that it is. Dead code and a missing version check on the transaction wire.
Known: none

### P36-F14
File: src/network/proto_conversions.rs:19
Severity: Info
Axis: invariant(Z3)
Confidence: Certain
Evidence:
```
proto_conversions.rs:19: #[allow(clippy::all, clippy::pedantic, clippy::nursery)]
proto_conversions.rs:20: pub mod pb {
```
Description: blanket lint suppression on the generated prost module. It covers generated code only, not hand written code, but it is a Z3 pattern; check that it is inside the suppression budget and that the include! target holds no hand written logic.
Known: none (budget file not read)

### P36-F15
File: src/network/proto_conversions.rs:182, 190, 198, 355, 363, 371; 28-36
Severity: Info
Axis: LABEL-12.9
Confidence: Certain
Evidence:
```
proto_conversions.rs:182:  data: bincode::serialize(identity_tx).unwrap_or_default(),
proto_conversions.rs:34:   Vec::new()
```
Description: six bincode serialization failures on the encode path turn into an empty payload with no log; serialize_payload_or_log (line 28) logs but still returns an empty Vec. A peer then sends a transaction or message whose payload cannot decode, instead of the sender failing. No fix proposed here.
Known: none

### P36-F16
File: src/network/mobile.rs:445-450
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
mobile.rs:446:  self.nat_status.using_relay = true;
mobile.rs:447:  self.nat_status.relay_address = Some(relay_address);
mobile.rs:448:  self.nat_status.hole_punched = false;
mobile.rs:449:  self.nat_status.validate()
```
Description: set_relay_address writes the state first and validates afterwards, so an empty, whitespace or over 256 character address returns Err but stays stored, and using_relay stays true. update_nat in the same file validates a candidate first (lines 423-430), so the two setters disagree. The function is in the dead-pub baseline (no caller), so the defect is latent.
Impact: a failed call leaves a profile that MobileNodeProfile::validate (line 362) refuses; a node built on it would be rejected by with_mobile_profile only if validated again.
Known: set_relay_address unused is in the baseline; the half-finished state is not

### P36-F17
File: src/network/gossip_dedup.rs:150-161, 182-190, 357-359
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
gossip_dedup.rs:182:  if self.seen_order.len() >= self.window_size {
gossip_dedup.rs:183:      if let Some(old) = self.seen_order.pop_front() {
gossip_dedup.rs:358:  self.seen_order.len() as f64 / self.window_size as f64
```
Description: GossipDedup::new(0) is public and accepted. With window_size 0 the eviction branch pops nothing and every message is inserted, so the window is unbounded; window_utilization divides by zero and returns NaN or infinity. The only production caller uses the default 10 000, so this is a defence in depth gap.
Known: window_utilization unused is in the baseline; the zero window is not

### P36-F18
File: src/network/gossip_dedup.rs:20-383 and src/network/peer_manager.rs:27-43
Severity: Info
Axis: integration
Confidence: Certain
Evidence:
```
gossip_dedup.rs:85:  pub struct PeerScore { pub score: f64, ...
peer_manager.rs:28:  pub struct PeerScore { pub score: i32, ...
```
Description: two peer scoring systems with the same type name and different scales (f64 ban at -10 and 1000 duplicates; i32 ban at -100) are active in the same receive path (node.rs:2097-2137). Only the dedup side bans on duplicates and only the manager side bans on rate limit, invalid data and timeouts. They share no state, so a peer near the ban line in one system looks clean in the other, and the metrics gauge peer_connection_quality (node.rs:1952) reports the mean of the dedup score only.
Known: none

### P36-F19
File: src/network/node.rs:2114 vs 2147-2163 (outside part), src/network/gossip_dedup.rs:167-192
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: confirm that the libp2p gossipsub config does not hold the message until the application validates it; validate_messages was not found by rg in node.rs, so the message is forwarded without waiting)
Evidence:
```
node.rs:2114:  match dedup.check_and_record(&message.data, &peer_id) {
node.rs:2152:  if !is_handshake_msg && !is_handshaked {
node.rs:2155:      warn!("Peer {} sent {:?} before completing handshake, dropping.", ...
```
Description: the dedup window records a message before the ban, handshake and decode checks decide to drop it. A peer that has not handshaked, or any message that is dropped for a state dependent reason, still marks its bytes as seen. Gossipsub suppresses later copies with the same id, and the app dedup treats a later copy of the same bytes as a Duplicate.
Impact: a connected but unhandshaked peer that delivers a valid block, transaction or vote to the victim first, for example from inside the victim's mesh, makes the victim drop that message for the whole window (10 000 distinct messages) and charges the honest relays that later deliver it as duplicates (F05). The attacker pays -5 per message (report_invalid_tx) and about 20 messages per throwaway key. Missed blocks are recovered by sync, missed votes and transactions are not re-requested.
Trigger: precondition a connection in the victim's gossip mesh that wins the delivery race; entry node.rs:2114; input a copy of a valid message M seen on the network; expected wrong result: M never processed by the victim.
Known: none

## Counts

Total 19. Critical 0. High 2 (F01, F02). Medium 6 (F03, F04, F05, F06, F08, F19). Low 8 (F07, F09, F10, F11, F12, F13, F16, F17). Info 3 (F14, F15, F18).

Notes for the verifier:
- F01 and F02 are the two to check first; both are cheap to confirm with a unit test (wire round trip with execution_program_hash set; two-step check_rate_limit then check_vote_rate_limit with a sleep between messages).
- F05, F07 and F19 share one root: the dedup key is raw bytes and runs before validation, and the penalty lands on the forwarder.
- Test coverage axis: peer_manager.rs tests exercise each method alone with a zero refill rate (several set msg_refill_rate = 0.0); none drives the gossip order that mixes check_rate_limit and check_vote_rate_limit, which is why F02 is invisible to them. proto_conversions.rs has no negative test for oversized counts or non-canonical timestamp strings.
