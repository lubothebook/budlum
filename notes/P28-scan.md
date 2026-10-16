# P28 scan (Storage derived: derived, emit, generated, render, transformed, view_grant)

Base commit 6066547 (HEAD at scan time not re-checked). Read-only scan, tier R3. No cargo, no git write.
Excluded as already known: STATUS.md section 4.5 and docs/audit/FINDINGS.md (including F12 at storage_deal.rs:1544-1545, the no-op SealedGenerated commitment check).
Counts: Critical 0, High 0, Medium 9, Low 9, Info 1 (19 findings, P28-F01 to P28-F19).

## COVERAGE

Read in full, in chunks under 1500 lines:
- src/storage/derived.rs 1-1329 (all)
- src/storage/emit.rs 1-1689 (all)
- src/storage/generated.rs 1-1601 (all)
- src/storage/render.rs 1-1084 (all)
- src/storage/transformed.rs 1-703 (all)
- src/storage/view_grant.rs 1-975 (all)
- docs/AGENT_MAP.md 1-56, docs/AUDIT_PLAN.md sections 5 and 6 (lines 110-165), STATUS.md 50-80.

Outside the part, read by rg or sed -n (some ranges went over the 80 line limit: storage_deal.rs 1440-1660 is 220 lines, rpc/server.rs 3300-3420 is 120 lines; I read them to see the registration path and the RPC entries):
- src/domain/storage_deal.rs 775-800, 1255-1270, 1440-1660, 1690-1760, 2080-2110
- src/rpc/server.rs 3300-3420, 6721-6740
- src/chain/chain_actor.rs 4285-4330
- src/gateway/service.rs 40-75, 307-347
- src/storage/three_pipe.rs 135-175, 273-291 (via rg -A)
- src/storage/mod.rs 72-80; .github/idle-code-baseline.txt and dead-pub-api-baseline.txt by rg only.

Not read: the bodies of three_pipe.rs, qr_*.rs, three_*.rs, payload_crypt.rs, social_delete.rs, reveal_gateway.rs, fixed_point.rs, content_id.rs, ContentManifest (storage/mod.rs). Claims about them are marked Likely where they matter. I did not check which RPC listener (operator or public) serves bud_storageQrFeed*, bud_storageIssueViewGrant, bud_gatewayRenderContent.
Invariant greps over the six files: Z7 no `unsafe`; Z3 no `#[allow` and no `#[ignore`; Z8 one float use (P28-F12); Z9 no key or token literals (a seed field printed by Debug, P28-F05). LABEL-12.9: no `to_bytes` with unwrap_or_default in these files (the `unwrap_or_default` hits at emit.rs:959, 1005, 1025, 1038 are error-report fields, not hashed).
Extra checks asked: content id binding (F02, F03, F08, F09), persistence format (F10, F17), open with corrupt data (F17), parts not wired (F09, F14, F07), resource limits before decode (F04, F11, F17).

## FINDINGS

### P28-F01
File: src/storage/emit.rs:1395-1429 (qr_feed_frames_burst); src/storage/three_pipe.rs:168-171; src/rpc/server.rs:3374-3410
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: emit.rs:1408-1409
    let key = policy.seal_seed.map(|seed| PayloadKey::derive(&seed));
    let encoded = encode_qr_video(content, policy.block_len, key.as_ref())?;
three_pipe.rs:170-171
    // A fresh CSPRNG nonce per call: two seals under the same key would share a
    let sealed = seal_payload_csprng(key, &prepared.bytes)?;
Description: Every burst call re-encodes the whole body. With a seal seed the nonce is fresh each time, so the packed container, stream_commitment and every frame differ between two calls with the same inputs. Burst 0..32 and burst 32..64 therefore come from two different streams.
Impact: A progressive player that pages through a sealed feed with bud_storageQrFeedFrames collects frames of different streams and cannot reassemble. The doc claim "a client cannot be handed a frame nobody emitted" holds only inside one call.
Trigger (security only): not security.
Known: none

### P28-F02
File: src/storage/emit.rs:803, 843-892, 1184-1186
Severity: Medium
Axis: security
Confidence: Certain
Evidence: emit.rs:803
    let declared_ciphertext = manifest.is_some_and(|m| m.encryption.is_encrypted());
emit.rs:1184
    if !publicly_reemitable && key.is_none() && !declared_ciphertext {
        return Err(EmitError::UnsealedGated);
Description: For an edition that admits a body (Classic), the body is never compared with the manifest (no shard hash, total_size or manifest_id check; only the Three edition path regenerates and compares, lines 874-890). The manifest's own encryption claim alone switches off the UnsealedGated guard.
Impact: A caller passes plaintext plus any manifest with ClientSide encryption and no seal seed. The preview succeeds and the burst path (no manifest check at all) emits clear frames under sealed-recipe and gated-metadata promises, which is the exact case the guard says it refuses. The reported provider_commitment and content_id describe a manifest built from the supplied bytes (1227-1231), not the caller's manifest.
Trigger (security only): precondition: access to bud_storageQrFeedPreview (listener class not checked). Entry: emit.rs:786 via rpc/server.rs:3338. Input: data_hex of plaintext, manifest with encryption ClientSide, no seal_seed. Expected wrong result: Ok(FeedPreview) with a_agreement true and publicly_reemitable false over clear frames.
Known: none

### P28-F03
File: src/storage/emit.rs:1220-1245
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: emit.rs:1242-1244
    } else {
        let packed = pack_payload(PayloadKind::ContentBytes, content)?;
        (ContentId::of(&packed), [0u8; 32])
Description: With no manifest the reported content_id is the id of a plain ContentBytes container of the clear body, not of pipe.packed, the container the carousel and the publish path use. Without a manifest, publicly_reemitable is false (seed_is_public needs a manifest), so line 1184 forces a seal seed: the only reachable no-manifest case is sealed, where pipe.packed is an EncryptedContent container with a random nonce.
Impact: FeedPreview.content_id is documented as "the thing a publish stores" (line 127). For every reachable no-manifest preview it names an object that no publish of this feed produces, and it is a deterministic hash of the plaintext container returned next to a sealed feed.
Trigger (security only): not security (the caller already holds the body).
Known: none

### P28-F04
File: src/storage/emit.rs:786-1382, 1395-1429; src/rpc/server.rs:3338-3410
Severity: Medium
Axis: security
Confidence: Likely (missing step: which listener serves these methods, and any per-IP rate limit in the RPC layer, not read)
Evidence: emit.rs:792 `let mut meter = ThreeMeter::with_budget(policy.meter_budget);` (a new meter per call, default 4096); qr_feed_frames_burst (1395-1429) has no meter at all.
Description: The A9 budget is local to one preview call and does not exist in the burst path. Each call does a full encode of up to 1 MiB (plan() only bounds size, not count); the preview additionally runs the video verify (60 s deadline), a full second re-emit pass, PNG render and, with a Generated manifest, generate_content up to 4 MiB after the encode already ran (manifest checks at 843-892 come after line 813).
Impact: Repeated calls cost the node the full encode each time with no cross-call accounting; the "meter" does not meter anything an endpoint caller can hit twice, contrary to the field doc at line 102.
Trigger (security only): precondition: RPC access to the two methods. Entry: rpc/server.rs:3338 and 3370. Input: 1 MiB body, block_len 64, repeated. Expected wrong result: sustained CPU and memory use per request with no refusal.
Known: none

### P28-F05
File: src/storage/emit.rs:96-110
Severity: Low
Axis: invariant(Z9)
Confidence: Certain
Evidence: emit.rs:96-107
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct EmitPolicy { ... pub seal_seed: Option<[u8; 32]>, ...
Description: The seal seed is the payload key (PayloadKey::derive(&seed)) and sits in a struct that derives Debug and PartialEq (non-constant-time compare). No current call site prints it (rg EmitPolicy: rpc/server.rs 3360, 3393 only). The seed also arrives as a plain RPC string parameter (server.rs:1337).
Impact: A future `{:?}` of the policy or of an RPC request log writes the key. Defense in depth.
Known: none

### P28-F06
File: src/storage/emit.rs:1417-1424
Severity: Low
Axis: integration
Confidence: Certain
Evidence: emit.rs:1422-1423
    let recipe = ThreeRecipe::Public(encoded.pipe.recipe.clone());
    let session = RevealSession::open(&recipe, None, &encoded.pipe.packed, true)?;
Description: The read path opens the reveal gate with a literal `true` for "grant" and always a Public recipe form, also when a seal seed is set. The view-grant layer (view_grant.rs, may_view) is not consulted by emit.rs at all; the module doc (lines 21-27) says that belongs to another path.
Impact: The reveal session adds no authorization on this path; any caller who supplies the body and seed gets frames. Not a leak by itself (the caller holds the body), but the gate is decoration here.
Known: none

### P28-F07
File: src/storage/emit.rs:819, 1343; src/storage/transformed.rs:7-11, 283-295, 163-166 (encode_payload in three_pipe.rs:155-165)
Severity: Low
Axis: test
Confidence: Certain
Evidence: transformed.rs:294 `pub apply_zlib: bool,` documented "default false"; emit.rs:1343 `transform_shrank: a0.codec_flags.contains(CodecFlags::PRE_SHRUNK),` with a0 from `transform_content(content, TransformOpts::default())` (emit.rs:819).
Description: With default options A0 never runs zlib, so FeedPreview.transform_shrank is always false for this call site. verify_hash at the A1 handoff checks a payload that transform_content hashed a few lines earlier in the same call, so it cannot fail on the production path. The module doc (transformed.rs:9-11) says the path "is not yet reachable from a binary"; it is, through the RPC.
Impact: A report field that never changes and a guard that cannot trigger give false assurance. Stale doc.
Known: none

### P28-F08
File: src/storage/generated.rs:303-315, 327-343; src/domain/storage_deal.rs:1527-1546, 1646, 2029
Severity: Medium
Axis: security
Confidence: Likely (missing step: deal pricing in deal_open.rs:270 and storage_deal.rs:2029 read held_bytes as the priced size; I read only the grep line, not the pricing arithmetic)
Evidence: generated.rs:305 `ContentSource::Generated(_) | ContentSource::SealedGenerated(_) => 1,` and 330 `ContentSource::Generated(_) | ContentSource::SealedGenerated(_) => Some(0),`. storage_deal.rs:1527-1545 checks only one shard, non-zero commitment, non-zero output_len; the sealed output_len and step_budget are not checked against MAX_GENERATED_BYTES or the step ceiling, and nothing is regenerated.
Description: A Generated source earns the single replica and zero held bytes because registration runs the recipe (storage_deal.rs:1515). A SealedGenerated source gets the same discount but the seed is off chain, so the recipe cannot be run, nothing is verified, and the network cannot regenerate the bytes after a loss ("a lost copy is regenerated" is false without the seed). The sealed public fields are also unbounded (u32 output_len, u32 step_budget), unlike the public path (generated.rs:701-718).
Impact: Anyone who registers an edition Three manifest with a SealedGenerated source and a self-chosen shard id gets one replica and zero billed bytes for content nobody can verify or rebuild. This is the class forgery the S2.2 comment (generated.rs:581-593) says the regeneration exam closes.
Trigger (security only): precondition: ability to register a manifest (fee only). Entry: storage_deal.rs:1457 register_manifest_with_source. Input: edition Three, source SealedGenerated with any non-zero commitment and output_len up to u32::MAX, one arbitrary shard. Expected wrong result: Ok, required_replicas_for returns 1, held_bytes 0.
Known: partly FINDINGS.md Part 7 F12 (the commitment check is a no-op); the missing replica rule and bounds are not in it. Tests: generated.rs tests cover required_replica_count for Generated and Derived only, none for SealedGenerated (test gap).

### P28-F09
File: src/storage/derived.rs:130-132, 633-810; src/storage/generated.rs:309-310; src/domain/storage_deal.rs:1567-1597
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: rg -n 'MasterRegistry|hold_master|acquire_master|release_master' src/ outside derived.rs: only a comment at generated.rs:309. storage_deal.rs:1594-1598
    return Err(StorageError::InvalidManifest {
        reason: "Derived source cannot be verified here; ...
Description: Registration of a Derived source always ends in Err, so no Derived manifest can exist on chain. MasterRegistry (reference count and grace window that stop a master being released) has no caller and is not a field of StorageRegistry, so it is not in the state root. The module header says "WIRING: wired" and generated.rs:309 says MasterRegistry "is what stops it being released"; neither is true in production.
Impact: Mainnet blocker class "not wired": the derived-content lever (Crop/Prefix) is unreachable, and the master-release protection that its replica rule relies on does not exist. If derived content is later wired without wiring the registry, a master can be released under live derivations.
Known: idle-code-baseline.txt lines 1872, 1873, 1891 (DERIVED_*_BYTES, MasterRegistry). The Derived-always-refused path and the false WIRING text are not in STATUS.md.

### P28-F10
File: src/storage/derived.rs:445-496, 524-529, 622, 744-752, 729-741
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: derived.rs:455-457
    if let Some(span) = self.prefix { return Self::check_prefix_span(span); }
derived.rs:748-750
    entry.derivations = entry.derivations.saturating_sub(1);
    if entry.derivations == 0 { entry.releasable_at_epoch = Some(now_epoch.saturating_add(MASTER_GRACE_EPOCHS)); }
Description: (a) For a Prefix spec check_region returns before looking at block_x/y/w/h and master_blocks_*, yet derivation_commitment_tag hashes them (587-592): the same prefix bytes can be registered under many manifest ids by varying unused fields. kept_bytes and master_bytes have no upper bound. (b) release_derivation on a master with count 0 re-arms the window each call (it slides forward) and opens a window for a master that was never derived; acquire_master uses saturating_add on u32, so after saturation a release undercounts. (c) MasterRegistry has no Serialize, no version and is not in any root: nothing persists it, so a node restart would forget every reference. DERIVED_SPEC_BYTES (58) is a hand sum, not the serialized size (an enum tag and Option add bytes).
Impact: Id malleability for prefix derivations; logic faults for the day the registry is wired.
Known: MasterRegistry and the constants are in idle-code-baseline.txt (no caller).

### P28-F11
File: src/storage/render.rs:296-363; src/gateway/service.rs:342-344
Severity: Medium
Axis: security
Confidence: Likely (missing step: whether a manifest with output_len 4194304 passes manifest registration size policy, and the RPC listener class)
Evidence: render.rs:346 `let mut svg = String::with_capacity(pixels.len() * 12 + 128);` then one `<rect .../>` of about 64 bytes per input byte (351-359). gateway/service.rs:342-344
    let bytes = crate::storage::render::render(spec, format)...?;
    let bytes = checked_gateway_content("on-demand render", bytes)?;
Description: generate_content allows output_len up to MAX_GENERATED_BYTES (4194304 = 2048 squared, which square_side accepts). Svg then builds a string of about 270 MB (the String doubles while growing), while the 10 MiB gateway cap is applied only after render returns. render_png is capped (render.rs:376-379); render_svg and render_frame are not.
Impact: One request for format "svg" on a name whose recipe has the maximum square length allocates several hundred MB on the node. Precondition: such a recipe is registered (cost: one generate run at registration) and the BNS name points at it.
Trigger (security only): Entry: BudGateway::render_name_content (service.rs:307) from the bud_gatewayRenderContent RPC. Input: name bound to a Generated manifest with output_len 4194304, format "svg". Expected wrong result: process memory spike of several hundred MB per concurrent request, then refusal by the 10 MiB check.
Known: none

### P28-F12
File: src/storage/render.rs:376
Severity: Low
Axis: invariant(Z8)
Confidence: Certain
Evidence: render.rs:376
    let max_side = ((u64::from(MAX_GENERATED_BYTES) / 3) as f64).sqrt() as u64;
Description: A float square root in a module whose header (lines 34-37, 316-319) states floating point is banned for determinism. The input is a constant (1398101), so the result is 1182 on any IEEE machine and no consensus value depends on it today; the repo-wide Z8 scan does not cover src/storage.
Impact: Maintenance risk only: the PNG size bound is the only limit on a hostile size, and it is a float expression next to a comment that forbids floats. An integer sqrt already exists at render.rs:312-337.
Known: none

### P28-F13
File: src/storage/render.rs:296-337, 351-359, 388-394; src/storage/generated.rs:822-873
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: generated.rs:836-867 draw_avatar writes 3 bytes per pixel on a side from square_side(len) = isqrt(len/3). render.rs:312-337 requires output_len to be a perfect square and render.rs:351-359 and 391-393 treat each byte as one grey pixel (`raw.extend_from_slice(&[b, b, b])`).
Description: The generators emit RGB with side = isqrt(len/3) and a zero padded tail. The renderer reads the same buffer as a grey square of side isqrt(len). For the generators' own natural size (3 x side squared, for example 3072, the size in the frozen vectors) square_side returns an error, and for a perfect square such as the test size 1024 the picture is a re-striped grey version of an 18 x 18 RGB image.
Impact: bud_gatewayRenderContent cannot render RGB sized recipes and renders other sizes as a different picture than the generator drew. Deterministic, so ids stay consistent; the feature does not do what the doc says. The tests (png_round_trips_the_generator_pixels_exactly) compare the buffer with itself, not with a drawing.
Known: none

### P28-F14
File: src/storage/render.rs:456-511, 550-584, 615-625
Severity: Low
Axis: dead code
Confidence: Certain
Evidence: rg -n 'render_and_verify|qr_stream_content_id' src/ outside render.rs: only the pub use at storage/mod.rs:121 (render_and_verify). idle-code-baseline.txt lines 1983, 1984.
Description: qr_stream_content_id and render_and_verify have no production caller. In addition the frame digest that the stream id folds is the first 4 bytes of a hash (render.rs:507-508), so each frame is bound by 32 bits; an id check over such a stream accepts a substituted frame after about 2^32 tries per frame.
Impact: Dead code today; if wired as the verification of a recipe addressed stream it is weak.
Known: idle-code-baseline.txt (qr_stream_content_id, render_and_verify, format_tag, RenderError).

### P28-F15
File: src/chain/chain_actor.rs:4289-4330; src/domain/storage_deal.rs:1264-1266, 1690-1751, 2080-2110; src/storage/view_grant.rs:618-626
Severity: Medium
Axis: invariant(consensus state outside blocks)
Confidence: Certain (behaviour); the precondition is a non-mainnet node
Evidence: chain_actor.rs:4289-4313: ChainCommand::IssueViewGrant and RevokeViewGrant call `self.blockchain.state.storage_registry.issue_view_grant(...)` directly, guarded only by storage_economics_disabled_on_mainnet(). storage_deal.rs:1264 `if self.view_grants.issued() > 0 { hasher.update(self.view_grants.root()); }`.
Description: The view grant book is folded into the storage registry root, but grants are issued, revoked and social-deleted by RPC commands on one node, not by a block transaction. The comment at storage_deal.rs:1258-1262 itself says grants are "enforced while a block is applied, so a node holding a different book accepts a block its peers reject".
Impact: On testnet an owner's RPC call on one node changes that node's state root; its next block, or its validation of peers' blocks, diverges. Mainnet refuses the commands.
Known: partly STATUS.md 4.5 class ("consensus state written outside blocks ... storage maintenance") and 4.4 ADIM 6 (names StoragePrune); view grants are not named.

### P28-F16
File: src/storage/view_grant.rs:229-276, 254-263; src/domain/storage_deal.rs:1690-1731, 2080-2110
Severity: Medium
Axis: security
Confidence: Likely (missing step: process_social_delete in social_delete.rs was not read; the claim assumes it retires every live grant of the content at any later time)
Evidence: view_grant.rs:243-251 grant_issue_digest folds tag, content, issuer, grantee, key_id, policy, opened_epoch; no chain id, no nonce. view_grant.rs:270-275 social_delete_digest folds content, caller, at_epoch only. Nothing records a consumed digest.
Description: The signed authorizations carry no chain id, no replay nonce and no freshness bound (opened_epoch and at_epoch are values the RPC caller supplies, storage_deal.rs:1696, 2082). A signed issue can be replayed once the grant is revoked (the duplicate check, view_grant.rs:444-453, looks only at live rows). A signed social delete can be replayed at any later time and retire grants issued after it. The same signature is valid on any network sharing the owner's key and content id.
Impact: Whoever captured one signed request (RPC operator, proxy, log) can restore a revoked grant, or wipe the live grants of that content, repeatedly. Revoke is safe (digest includes the unique grant_id, AlreadyRevoked).
Trigger (security only): precondition: a captured authorization JSON. Entry: storage_deal.rs:1690 (issue) or 2080 (social delete) through the RPC. Input: the same authorization and epoch resent after revoke or after new grants. Expected wrong result: Ok, state changed.
Known: none

### P28-F17
File: src/storage/view_grant.rs:404-412, 444-472; src/domain/storage_deal.rs:786-787
Severity: Low
Axis: correctness
Confidence: Likely (missing step: how StorageRegistry bytes are read back and validated in storage/db.rs was not read)
Evidence: view_grant.rs:405-412 `#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct ViewGrantRegistry { next_id, grants, by_content }`; issue() at 454-470 `self.grants.insert(grant_id, grant)` with grant_id = next_id.
Description: The three fields are stored independently and nothing validates them on load: next_id may be at or below an existing grant id (issue then overwrites a stored row, possibly a revoked one, and pushes a duplicate id into by_content), a key may differ from grant.grant_id, by_content may omit a row. The struct is part of a positional bincode row with serde(default) and no version tag. There is no cap on grants per content or in total, and the duplicate check scans every grant (O(n) per issue). next_id uses saturating_add.
Impact: A corrupt or hand edited registry row loads silently and can change who may view (a revoked grant replaced by a live one with the same id). Growth is unbounded for an owner that repeats issue calls (RPC path, mainnet disabled).
Known: none (FINDINGS.md F14 covers the storage_deal registry row having no version tag; this nested struct is separate)

### P28-F18
File: src/storage/view_grant.rs:477-495, 578-601; src/domain/storage_deal.rs:1696, 1731
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: view_grant.rs:493 `grant.revoked_epoch = Some(at_epoch);` no comparison with opened_epoch; may_view (588-600) never reads opened_epoch.
Description: opened_epoch and at_epoch are caller values that are signed but never compared with the chain epoch, with each other, or with the use time. A grant dated in the future is live at once; a revoke can carry an epoch before the open epoch.
Impact: The epoch columns in the grant row and its commitment carry no verified meaning; audit trails (rows_for_content) can be arbitrary.
Known: none

### P28-F19
File: src/storage/view_grant.rs:324-347, 686-702, 962-974; src/storage/derived.rs:130-153
Severity: Info
Axis: test
Confidence: Certain
Evidence: view_grant.rs:325-338 confidential_commit_digest maps encryption and proof_kind to bytes in its own match, while ConfidentialBodyCommit::commitment (687-702) uses `encryption.commitment_tag()` and a second proof_kind match. The last test (962-974) asserts text of its own source file with include_str!.
Description: Two separate byte encodings of the same enums can drift apart without a test failing; proof_kind values ZkStorageProof, TeeAttested, HybridZkTee are pinned on chain with "no verifier here" (line 643), so the label has no check behind it. One test asserts a doc string instead of behaviour.
Impact: Maintenance and an unverified proof claim label (a mainnet blocker if a consumer ever trusts proof_kind).
Known: none
