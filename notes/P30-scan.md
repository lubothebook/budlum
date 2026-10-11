# P30 scan: Storage three and reveal (read-only audit, Tier R2)

Base commit: f1b8a4a. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md section 4.4, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt (and idle-code-baseline.txt, lines 2105-2153, for the three_nft registry and three_regime).

## COVERAGE

Read in full (every line, tests included, one read each):
- src/storage/three_gate.rs 1-158
- src/storage/three_hooks.rs 1-90
- src/storage/three_meter.rs 1-152
- src/storage/three_nft.rs 1-534
- src/storage/three_pipe.rs 1-661
- src/storage/three_recipe.rs 1-830
- src/storage/three_regime.rs 1-279
- src/storage/three_reveal.rs 1-203
- src/storage/three_rpc.rs 1-363
- src/storage/three_visibility.rs 1-72
- src/storage/reveal_gateway.rs 1-976

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 108-165 (sections 5 and 6); notes/P29-scan.md lines 1-30; STATUS.md lines 31-56.

Outside the part (rg first, then at most 80 lines):
- src/rpc/server.rs 1140-1210 (check_claimed_owner, verify_view_claim), 3470-3520 (revoke), 3560-3720 (storage_may_view, storage_open_reveal)
- src/storage/qr_reemit.rs 90-190 (RecipeEmitter::open, emit_frames, verify_stream_id)
- src/storage/qr_recipe.rs 40-150 (digest, seal, open_with)
- src/storage/emit.rs 780-825, 1150-1220, 1225-1330, 1330-1385, 1405-1440
- src/storage/provider.rs 205-235 (the put gate)
- src/chain/chain_actor.rs 4285-4335 (view grant commands)
- src/domain/storage_deal.rs 1240-1270 (state root fold)
- src/core/hash.rs hash_fields_bytes (rg -A), src/storage/qr_payload.rs pack_payload (rg -A)
- .github idle-code and dead-pub baselines (rg only)

Not read: src/storage/view_grant.rs, payload_crypt.rs, transformed.rs, social_delete.rs, src/rpc/server.rs frames and close handlers (3720-3790), the chain actor MayViewContent arm, qr_* internals (P29). No cargo, no test run: every claim comes from reading source.

Extra-check summary:
- Content id binding: ThreeRecipe digests bind payload_commitment, k, block_len, total_len, stream_id (qr_recipe.rs:56-66) and RecipeEmitter::open binds packed to the commitment (qr_reemit.rs:96-99). Nothing binds the recipe or the packed body to the `content_id` used for the grant lookup: P30-F03. hash_fields_bytes is length-prefixed, so ThreeNftMeta::commitment with its optional empty fields has no concatenation collision (checked, not a finding).
- Nonce and key handling: encode_payload uses seal_payload_csprng, a fresh nonce per call (three_pipe.rs:168-175), with a test. Session ids are keyed hashes (reveal_gateway.rs:219-228). The sealed recipe keeps no key. Plaintext hash leak: P30-F02. Seed-to-key derivation (PayloadKey::derive) is outside the part and not read.
- Mainnet gating: view grant issue/revoke commands refuse on mainnet (chain_actor.rs:4298, 4316). The reveal RPCs (open, frames) and the feed RPCs have no mainnet check in the handler; on mainnet no grant can exist, so only public recipes open there, and the DoS findings P30-F04 to F06 apply on mainnet.
- Unwired: ThreeNftRegistry, PinRow, RecipeRecord, three_regime (all of it) have no production call site; Known (idle-code-baseline.txt 2141-2153, three_regime header says unwired). open_reveal_session (registry path) and RevealGateway::open have only test callers; production uses open_prechecked (server.rs:3690). decode_qr_video has no production caller: P30-F11.
- Panics from external bytes: all unwrap, expect and indexing hits in the 11 files are under #[cfg(test)] (rg checked). No panic path from external bytes in production code. Casts: P30-F10.
- State written outside blocks: P30-F01. RevealGateway is process-local memory, not consensus state.
- Z3 (allow, ignore): none in the 11 files. Z7 (unsafe): none. Z8 (f32, f64): none. Z9 (secrets): none; test keys are literal byte arrays ([42u8; 32], derive(b"facade-key")) inside cfg(test).

## Findings

### P30-F01
File: src/chain/chain_actor.rs:4289-4327; src/domain/storage_deal.rs:1264-1266; src/rpc/server.rs:3470-3500
Severity: High
Axis: invariant (state written outside blocks)
Confidence: Likely (missing step: the block-apply path was not read, to prove no block transaction applies the same grants)
Evidence:
```
chain_actor.rs:4310  ChainCommand::RevokeViewGrant { grant_id, auth, at_epoch, response } => {
chain_actor.rs:4320      let res = self.blockchain.state.storage_registry
chain_actor.rs:4324          .revoke_view_grant(grant_id, &auth, at_epoch)
storage_deal.rs:1264  if self.view_grants.issued() > 0 {
storage_deal.rs:1265      hasher.update(self.view_grants.root());
storage_deal.rs:1255  // ... Both are enforced while a block is applied, so a node holding
storage_deal.rs:1256  // a different book accepts a block its peers reject.
```
Description: Issuing and revoking a view grant is an RPC command (bud_storageIssueViewGrant, bud_storageRevokeViewGrant) that mutates `state.storage_registry` on the local node between blocks, and `at_epoch` and `opened_epoch` are caller supplied. The grant book is folded into the storage state root, and the code comment says grants are enforced while a block is applied.
Impact: Any caller who can sign as a content owner changes the local node state root without a block. Two nodes that received different RPC calls compute different roots, so the node that took the call rejects or forks from honest blocks. Mainnet refuses the command (chain_actor.rs:4298), so this is a testnet divergence and a mainnet blocker for opening storage.
Trigger: Precondition: non-mainnet chain, owner signature. Entry: ChainCommand::RevokeViewGrant (chain_actor.rs:4310) from server.rs:3487. Input: a valid revoke with any `at_epoch`. Expected wrong result: the local `view_grants.root()` changes with no block carrying the revoke.
Known: partial, STATUS.md section 4.4 (ADIM 6 "remove RPC mutation paths", and StoragePrune); grants are not named there.

### P30-F02
File: src/storage/three_recipe.rs:219-241, 245-254; src/storage/emit.rs:1247-1256, 1371; src/rpc/server.rs:1290-1291
Severity: Medium
Axis: security
Confidence: Likely (missing step: confirm a sealed feed's JSON is served to untrusted readers; the handler returns it to the caller of bud_storageQrFeedPreview and the field is named as the marketplace-visible form)
Evidence:
```
three_recipe.rs:219  content_sha256: calculate_hash_bytes(content),
three_recipe.rs:228  hash_fields_bytes(&[ b"BDLM_THREE_VIDEO_RECIPE_V1", &self.content_sha256,
three_recipe.rs:229      &self.video_commitment, &self.content_bytes.len().to_le_bytes(), ...
emit.rs:1371   recipe_commitment: sealed.recipe_commitment,
server.rs:1291 "recipeCommitment": format!("0x{}", hex::encode(feed.recipe_commitment)),
```
Description: The recipe commitment of a sealed feed is a plain SHA-256 of the plaintext (unsalted) mixed with fields that are all public for the same feed (video commitment, content length, frame count, fps, block_len, permillage, kind, transform knobs). Sealed `VideoRecipeSealed` also carries `content_sha256` and `content_len` in the clear.
Impact: Anyone who sees the commitment of a sealed (owner-only or DM) object confirms a guess of its plaintext offline. Short or templated content (messages, small JSON, avatars from a known set) is recoverable without the key, which defeats the sealed form.
Trigger: Precondition: low-entropy plaintext, reader sees `recipeCommitment` and `videoCommitment`. Entry: server.rs:1291. Input: candidate plaintext; recompute the commitment with the public knobs. Expected wrong result: equality confirms the guess.
Known: none

### P30-F03
File: src/rpc/server.rs:3600-3664; src/storage/three_rpc.rs:83-102, 154-158; src/storage/three_reveal.rs:64-91
Severity: Medium
Axis: security
Confidence: Likely (missing step: how the manifest or confidential commit relates `content_id` to a packed body is outside the read set; the handler shows no such lookup)
Evidence:
```
server.rs:3600  let recipe: crate::storage::ThreeRecipe = serde_json::from_value(recipe)
server.rs:3608  let packed = hex::decode(packed.strip_prefix("0x")...
server.rs:3655  let req = crate::storage::RevealRequest { recipe, full_public, packed, content_id, viewer, owner, key_id, meter_budget };
three_rpc.rs:157  let grant_allows = registry.may_view(&req.content_id, &req.viewer, &req.key_id, recorded_owner);
```
Description: The recipe, the sealed opening (`full_public`) and the packed body all come from the caller. `content_id` is used only as the key of the grant lookup. The signed viewer claim (server.rs:1189-1197) binds content_id and the payload commitment of `packed`, not the recipe, and nothing ties `content_id` to that payload.
Impact: The sealed-recipe grant gate protects nothing a caller does not already hold: to pass `RevealSession::open` the caller must present the opening and the exact packed bytes, and can re-emit frames with the public reemit code. A caller can also pair any grant on content X with a recipe and body of its own. The gate, the grant revoke and the T3 honesty text give a confidentiality claim the endpoint cannot back; the endpoint is a free frame-rendering service.
Trigger: Entry: server.rs:3588. Input: sealed recipe plus its opening and body for content B, `content_id` of a content A on which the caller holds a grant. Expected wrong result: a session opens and serves frames of B under A's grant.
Known: none

### P30-F04
File: src/rpc/server.rs:3665-3691; src/storage/three_reveal.rs:81-90; src/storage/reveal_gateway.rs:254, 283
Severity: Medium
Axis: security (DoS)
Confidence: Certain for the lock and the uncharged work; Likely for the cost figure (missing step: no measurement, derived from MAX_K 4096, MAX_BLOCK_LEN 8 KiB, 15 percent repair)
Evidence:
```
server.rs:3665  let mut gw = self.reveal_gateway.lock()...
server.rs:3689  let session_id = gw.open_prechecked(req, grant_allows, now)
three_reveal.rs:87  let pass = oneshot_drop_count(public_k, ONESHOT_REPAIR_PERMILLAGE);
three_reveal.rs:88  let (_, pass_fold) = emitter.emit_frames(0, pass)?;
```
Description: Every open builds the whole one-shot pass (k plus repair, up to about 4700 drops, each packed and hashed, all collected into a Vec) and throws the frames away, while the handler holds the std Mutex of the gateway table. The meter is created after this (three_rpc.rs:190) and never charges it. A public recipe needs no grant, and the claim only needs a fresh key.
Impact: One anonymous caller with a few hundred KB to MB of body repeats opens: each costs tens of MB of allocation and a SHA-256 pass per drop, serialized behind the lock, so frame serving, close and revoke (which takes the same lock, server.rs:3478-3499) for every other viewer stall. Applies on mainnet, where only public recipes open.
Trigger: Entry: server.rs:3588. Input: a public recipe with k near 4096 and a matching packed body, repeated. Expected wrong result: lock held for the whole pass per call; no budget used.
Known: none

### P30-F05
File: src/rpc/server.rs:3597-3663, 3699-3700; src/storage/reveal_gateway.rs:259-269; src/storage/three_rpc.rs:100-101
Severity: Medium
Axis: security (DoS)
Confidence: Certain
Evidence:
```
reveal_gateway.rs:265  if req.meter_budget.is_none() {
reveal_gateway.rs:266      req.meter_budget = Some(DEFAULT_REVEAL_BUDGET_FRAMES);
server.rs:3663         meter_budget,   // caller value, any u64
```
Description: The default cap applies only when the caller omits a budget. A caller who sends `meterBudget: 18446744073709551615` replaces the cap with a value the meter can never reach. The doc of the gateway says "uncapped is not a remote option", which is only true for an omitted field.
Impact: A session serves 256 frames per call for the whole TTL with no budget limit, so the 10 000 frame cap, the point of the A9 meter on the read path, is bypassed by the reader. Combined with F04 and F06 it removes the only per-session work limit.
Trigger: Entry: server.rs:3588 with `meter_budget` set high. Expected wrong result: `frameBudget` echoes the huge value and emits never trip `BudgetExceeded`.
Known: none

### P30-F06
File: src/storage/reveal_gateway.rs:290-327; src/rpc/server.rs:3675-3688
Severity: Medium
Axis: security (DoS)
Confidence: Certain
Evidence:
```
reveal_gateway.rs:297  if self.sessions.len() >= MAX_REVEAL_SESSIONS {
reveal_gateway.rs:298      return Err(RevealGatewayError::SessionLimit { max: MAX_REVEAL_SESSIONS });
```
Description: The table cap is global (1024) with a 300 s TTL, and there is no per-viewer or per-IP quota and no rate limit on opens. Sessions are not bound to the opener: the frame and close calls take only the session id (server.rs:3705, 3774).
Impact: One caller (any fresh key, public recipe, no grant) fills all 1024 slots and re-fills them every 300 s, so every other viewer gets "session table full" (-32003). Sealed sessions of legitimate grantees are refused too. Closing needs the id, which is unguessable, so the victim cannot reclaim slots.
Trigger: Entry: server.rs:3588 in a loop of 1024 opens. Expected wrong result: `activeSessions` stays at 1024 and later opens fail.
Known: none

### P30-F07
File: src/storage/emit.rs:1247-1256, 1283-1284; src/storage/three_recipe.rs:264-280, 393-403
Severity: Low
Axis: correctness (invariant, reproduction promise)
Confidence: Certain
Evidence:
```
emit.rs:1247  let recipe = VideoRecipe::from_encoded( content, RecipeTransform::pin_from(...), kind.tag(), ... &encoded);
emit.rs:1284  if kind != PayloadKind::EncryptedContent {   // reemit checked only here
three_recipe.rs:403  let packed = pack_payload(kind, &transformed.bytes)?;
```
Description: For a sealed feed the recipe is written over the plaintext with payload kind 3 while the video was built from ciphertext with a CSPRNG nonce. `VideoRecipe::reemit(content)` then packs the plaintext as "encrypted" and can never reproduce `video_commitment`; the preview skips the check and reports it as skipped. `VideoRecipeSealed::reemit_with` is a public path that always returns BodyMismatch for a sealed feed.
Impact: The "recipe plus body rebuilds the video bit for bit" claim holds for clear feeds only; for sealed feeds the recipe is not a production recipe, so a pin of it cannot be re-emitted by anyone, including the owner, after the node that holds the ciphertext is gone.
Trigger: none (correctness)
Known: none

### P30-F08
File: src/storage/three_pipe.rs:176-184; src/storage/three_recipe.rs:393-403, 471-476
Severity: Low
Axis: correctness
Confidence: Likely (missing step: run a body that A0 classifies as entropy but zlib shrinks, such as the JPEG-magic plus repeated bytes used in three_pipe.rs:494-496, through `recipe.reemit`)
Evidence:
```
three_pipe.rs:184  let packed = pack_payload_opts(kind, &body, allow_zlib)?;      // allow_zlib = class.may_try_zlib()
three_recipe.rs:403  let packed = pack_payload(kind, &transformed.bytes)?;         // pack_payload = opts(.., true)
three_recipe.rs:473  /// This is the single production path: both the first encode ... bit-equal.
```
Description: The doc says one production path, but encode_qr_video (three_pipe) and RecipeCore::from_body (three_recipe) are two implementations with different A1 zlib policy. For a body whose class skips zlib at A0 but that does shrink, the two produce different packed bytes and different videos.
Impact: `qr_feed_preview` returns ReemitMismatch (emit.rs:1292) for such a body, and any recipe made from the three_pipe output fails `reemit`. No existing test runs a recipe over an entropy-classed compressible body (the test at three_pipe.rs:491 only checks packed_is_zlib).
Trigger: none (correctness)
Known: none

### P30-F09
File: src/storage/three_gate.rs:41-58, 77-86; src/storage/provider.rs:220
Severity: Low
Axis: security (defense in depth) and correctness
Confidence: Certain
Evidence:
```
three_gate.rs:45  if bytes.starts_with(&DROP_MAGIC) { return ThreeBlobKind::CarouselDrop; }
three_gate.rs:84        | ThreeBlobKind::QrVideo => Err(kind),
three_gate.rs:84-85  ThreeBlobKind::PackedPayload | ThreeBlobKind::Other => Ok(()),
```
Description: The gate is a four-byte prefix test. A derivative with any leading byte, or inside another container, classifies as Other and is stored. The reverse also holds: honest or crafted content whose first four bytes equal one of the magics (BDLD, BDLF, BDLR, BDLV or the frame magic) is refused as a derivative, and the regime test itself uses a bare `b"BDLD"` as a derivative.
Impact: The "Three derivatives are not durable storage" rule is not enforceable against a writer who wants to evade it (header says the rule runs against bytes a reader can bring), and a shard that begins with a magic is un-storable, which lets a crafted file block its own storage at the provider.
Trigger: Entry: provider.rs:212 put. Input: shard bytes starting with `BDLV`. Expected wrong result: `DurableDerivative(QrVideo)` for a non-video shard.
Known: none

### P30-F10
File: src/storage/three_recipe.rs:221, 250
Severity: Low
Axis: correctness (truncation)
Confidence: Certain
Evidence:
```
three_recipe.rs:221  frame_count: encoded.video.png_frames.len() as u32,
three_recipe.rs:250  content_len: self.content_bytes.len() as u32,
```
Description: `content_len` in the sealed form wraps above 4 GiB, and the commitment (line 232) hashes the full `usize` length while the sealed form carries the wrapped one, so the two disagree for such content. `from_encoded` also takes `content` and `encoded` separately and does not check they belong together (its doc says "only through the nested commitment step" but it returns no Result).
Impact: Practically bounded by the 64 MiB carousel cap, so low; the unchecked pairing means a caller error pins a recipe for the wrong video silently.
Trigger: none
Known: none

### P30-F11
File: src/storage/three_pipe.rs:297-302; src/storage/three_hooks.rs:11-20; src/storage/three_visibility.rs:15-21; src/storage/three_nft.rs:34-41
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
rg -n "decode_qr_video" src --glob '!src/storage/**'   -> no output
rg -n "decode_qr_video" src/storage  -> mod.rs:158 (re-export), qr_verify.rs/qr_verify_indep.rs (comments, tests), transformed.rs:632 (cfg(test))
rg -n "ThreeHookKind::VisibilityChanged|UploadVisibility::Restricted|MetadataVisibility::Restricted" src --glob '!src/tests/**'
   -> only three_visibility.rs match arms; no emitter or constructor in production
```
Description: `decode_qr_video` (the root 3.0 decoder, the one that parses an external BDLV blob) has no production caller; the verifier calls its own steps. `ThreeHookKind::VisibilityChanged` is never emitted, and `Restricted` visibility cannot be produced (emit.rs:1160-1164 only picks Public or the default; `ThreeNftMeta::from_recipe` only yields Public or Sealed). None of these appear in the baselines.
Impact: Maintenance and an unreviewed decode path with an external-bytes entry; the "V1 restricted" product mode is documented but unreachable.
Trigger: none
Known: none

### P30-F12
File: src/storage/three_gate.rs:121-157
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
three_gate.rs:130  // drop ~25% of frames
three_gate.rs:138  let result = decode_frames(&enc.stream_commitment, &kept);
three_gate.rs:139  if let Ok((kind, raw)) = result { ... } else { ... second pass i % 5 != 0 plus kept ... expect("second pass ...") }
```
Description: The test named lossy_pipe_still_recovers (CH.4) accepts either outcome of the 25 percent loss pass. The repair margin is 15 percent (three_pipe.rs:338-342), so a 25 percent loss normally fails and the else branch runs with the union of two patterns, which keeps 95 percent of frames (only multiples of 20 missing). The test does not show recovery at the stated loss.
Impact: The lossy-channel claim for the one-shot pipe is not pinned; a margin regression below the real loss would pass.
Trigger: none
Known: none

### P30-F13
File: src/storage/emit.rs:1409-1424 (read path of reveal-adjacent feed); src/storage/three_pipe.rs:273-291
Severity: Low
Axis: security (DoS), outside the part, read for context
Confidence: Likely (missing step: the handler's content size limit and rate limit at server.rs:3370-3400 were not read)
Evidence:
```
emit.rs:1409  let encoded = encode_qr_video(content, policy.block_len, key.as_ref())?;
```
Description: `encode_qr_video` renders every optical frame to a QR PNG and the BDLV container, and the frame burst endpoint calls it for each request just to read a slice of up to the burst limit, then discards it (again re-running RevealSession::open and its full pass, F04).
Impact: Per-call cost is proportional to the whole object, not to `count`, so a client asking for 1 frame at a time pays and costs the node the full encode each time.
Trigger: Entry: server.rs:3370 storage_qr_feed_frames, repeated small `count`.
Known: none

### P30-F14
File: src/storage/three_reveal.rs:76-78; src/storage/three_hooks.rs:64; src/rpc/server.rs:3505
Severity: Info
Axis: correctness (observation)
Confidence: Certain
Evidence:
```
three_reveal.rs:77  s.open_with(full).map_err(|_| RevealError::NeedFullRecipe)?;
```
Description: A full recipe that does not open the sealed form is reported as `NeedFullRecipe`, the same as a missing one, so a client cannot tell "send the opening" from "wrong opening". The hook seam is fired with `NopThreeHook` in the revoke handler (already recorded in FINDINGS.md, P18 F-series, server.rs:3505-3514), so `emit_hook`'s "future RPC revoke paths" comment is stale.
Impact: Diagnostics only.
Trigger: none
Known: partial, docs/audit/FINDINGS.md (NopThreeHook at server.rs revoke).
