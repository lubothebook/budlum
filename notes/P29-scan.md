# P29 scan: Storage QR (read-only audit, Tier R3)

Base commit: 6066547 (HEAD b66d617 at scan time). Scanner: Sonnet, read-only. Known and not repeated: STATUS.md section 4.4 (K1 gaps), docs/audit/FINDINGS.md.

## COVERAGE

Read in full (every line, tests included):
- src/storage/qr_carousel.rs 1-1558 (two reads: 1-1500, 1500-1558)
- src/storage/qr_codec.rs 1-247
- src/storage/qr_encode.rs 1-1042
- src/storage/qr_frame.rs 1-400
- src/storage/qr_matrix.rs 1-278
- src/storage/qr_payload.rs 1-556
- src/storage/qr_png.rs 1-341
- src/storage/qr_receive.rs 1-333
- src/storage/qr_recipe.rs 1-296
- src/storage/qr_reemit.rs 1-285
- src/storage/qr_verify.rs 1-408
- src/storage/qr_verify_indep.rs 1-891
- src/storage/qr_video.rs 1-927

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 110-165 (sections 5 and 6); STATUS.md lines 31-56 (section 4.4 for the known list).

Outside the part (rg first, then at most 80 lines): src/storage/three_pipe.rs 160-305; src/storage/emit.rs 80-100, 745-760, 815-835, 890-945; src/storage/three_reveal.rs 60-130; src/storage/three_rpc.rs 100-160; src/rpc/server.rs 3700-3722; src/core/hash.rs hash_fields_bytes (rg -A14); Cargo.toml (rg on dependency lines); docs/audit/FINDINGS.md (rg only, qr lines); .github/dead-pub-api-baseline.txt and idle-code-baseline.txt (rg only); fuzz/fuzz_targets (ls and rg only).

Not read: the rest of src/storage (emit.rs, three_pipe.rs, three_recipe.rs, three_reveal.rs, three_nft.rs, reveal_gateway.rs beyond the ranges above), src/tests, the rqrr, qrcode and flate2 crate sources. No cargo, no test run: every claim comes from reading source. Git history for the constants was not usable: `git log -S` on the permillage constant returns one large import commit (e7e3a45), so the old value of ONESHOT_REPAIR_PERMILLAGE is known only from the code comment at qr_carousel.rs:59-61.

Extra-check summary:
- Decoder resource budget before decode: the carousel and A1 layers have real bounds (MAX_K 4096, MAX_BLOCK_LEN, total_len capped at 64 MiB on the wire, inflate capped, dedup map capped, PNG side capped, frame count capped). The gaps are the loose PNG ceiling, no blob or per-PNG byte cap, per-push elimination cost, and a post-hoc deadline: P29-F03.
- Malformed input panics: no unwrap, expect, panic or direct indexing in production code of the 13 files (all hits of rg are under #[cfg(test)]); one `assert!` in qr_encode.rs:93 bounded by a constant table. No panic path found from external bytes. rqrr is a third-party reader fed hostile images and is not wrapped: P29-F03.
- Content id binding: bound in qr_verify (ContentId::of body, recipe, stream, kind, independent stream recomputation). Not bound in the receiver or in decode_qr_video: P29-F01. Recipe stream pin depends on an unrecorded constant: P29-F04.
- Determinism: no HashMap, no float, BTreeMap only; PRNG is SHA-256 counter. Hazards are dependency-dependent bytes: P29-F07, P29-F04.
- Wiring: the write side (encode, emitter, reveal) is wired. The read side (decode_qr_video, QrVideo::from_bytes, ProgressiveReceiver on foreign frames) has no production caller with external input. That is the K1 reader point in STATUS.md 4.4 and is not filed again; findings below say where it changes severity.
- Z3: `rg -n 'allow\(|ignore|cfg_attr|clippy::' src/storage/qr_*.rs` finds only English comments. Z7: no `unsafe`. Z8: no f32 or f64. Z9: no key, token or seed literal; the only keys in tests are byte fills. See P29-F13.

---

### P29-F01
File: src/storage/qr_receive.rs:185-203; src/storage/three_pipe.rs:215-226, 296-302; src/storage/qr_payload.rs:262-266
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
qr_receive.rs:94  pub const fn new(stream_commitment: [u8; 32]) -> Self {
qr_receive.rs:200 pub fn finish_unpacked(&self) -> Result<(PayloadKind, Vec<u8>), ReceiveError> {
qr_receive.rs:201     let packed = self.finish_packed()?;
qr_receive.rs:202     Ok(unpack_payload(&packed)?)
three_pipe.rs:300 let (kind, raw) = decode_frames(&video.stream_commitment, &optical)?;
qr_payload.rs:255 let got = calculate_hash_bytes(&raw);   // sha of raw content only
qr_payload.rs:256 if got != expect_sha { return Err(PayloadError::ContentHashMismatch); }
```
Description: The receiver never recomputes `payload_commitment(packed)` or the stream commitment from the recovered bytes and parameters, so the stream commitment it was given is only a label. The A1 content sha is read from the container itself, and the kind byte (offset 6) and flags are outside that sha. `decode_qr_video` takes the stream commitment from the video header, so a self-consistent forged video decodes (the test `self_consistent_fake_video_is_refused` in qr_verify.rs:286 shows this).
Impact: A reader that calls `decode_frames` or `decode_qr_video` (the only readers outside qr_verify) gets bytes that match nothing the caller pinned. An attacker who can show frames to a receiver chooses content, and can flip `ContentBytes` to `EncryptedContent` or the reverse because the kind tag is not hashed by any layer the receiver checks.
Trigger: precondition: a production reader is wired to external frames (not yet). Entry: qr_receive.rs:116 push_frame then finish_unpacked. Input: frames packed with `pack_frame` under a stream commitment the attacker picks (it is only a hash input, qr_frame.rs:190-198), whose packed container is self-consistent. Expected wrong result: Ok((kind, body)) with the attacker's body.
Known: STATUS.md section 4.4 (reader point unwired, partial). The missing commitment check inside the receiver is not listed there.

### P29-F02
File: src/storage/qr_frame.rs:21-27, 188-198; src/storage/qr_receive.rs:132-154; src/storage/qr_carousel.rs:572-584, 601-603, 716-718
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
qr_frame.rs:24  //! `stream_commitment` comes from ... so a foreign stream cannot splice drops in.
qr_frame.rs:190 pub fn frame_digest(stream_commitment: &[u8; 32], seq: u32, drop_wire: &[u8]) -> [u8; 4] {
qr_frame.rs:197     [full[0], full[1], full[2], full[3]]
qr_receive.rs:134  if let Some(prev) = self.seen.get(&drop.seq) {
qr_receive.rs:139  // Conflict: the first body for this seq is already in the decoder and stays there
qr_carousel.rs:601 if degree == 0 { // Fully reduced - nothing new (or consistency check omitted).
qr_carousel.rs:716 // Fully reduced: redundant or inconsistent. Nothing to learn.
```
Description: The frame digest is 4 bytes of an unkeyed hash over inputs an observer already holds (the stream commitment is in the public recipe). It detects noise, not tampering, so the claim "a foreign stream cannot splice drops in" is not true for anyone who knows the recipe. The drop body check is FNV-1a 32 (qr_carousel.rs:956), also computable. The receiver keeps the first body per seq forever, the decoder accepts a repair or systematic equation without any consistency test, and there is no reset.
Impact: One forged drop with a valid digest, shown before the honest drop of that seq, makes the whole recovery fail permanently (`finish_unpacked` ends in a content sha error, or in `Incomplete`), and the receiver cannot recover without being rebuilt. Cost to the attacker: a few hundred bytes per poisoned equation.
Trigger: precondition: a recipe with a known stream commitment (public recipes expose it) and a receiver fed from an optical or network channel the attacker can write to. Entry: qr_receive.rs:116. Input: `pack_frame(known_commitment, &Drop{seq: s, body: junk, ..})`. Expected wrong result: the honest drop for seq s is counted as rejected (qr_receive.rs:141) and decode never completes correctly.
Known: none (STATUS.md 4.4 lists the unwired reader, not frame authentication strength).

### P29-F03
File: src/storage/qr_video.rs:56-61, 160-197, 261-363; src/storage/qr_png.rs:56-61; src/storage/qr_carousel.rs:675-766; src/storage/qr_verify.rs:229-235
Severity: Low (becomes Medium when the reader is wired)
Axis: security
Confidence: Likely (missing step: a measurement of one 8192 by 8192 RGBA frame through decode_png_grey and rqrr, and an rqrr fuzz run; neither done here)
Evidence:
```
qr_png.rs:61   pub const MAX_PNG_SIDE_PX: u32 = 8192;
qr_matrix.rs:18-20  QUIET_ZONE = 4, MODULE_PX = 4   -> largest honest side (177 + 8) * 4 = 740 px
qr_video.rs:346 let row = 1 + w * channels;           // channels up to 4
qr_video.rs:347 let expected = h.checked_mul(row)...  // 8192 * (1 + 8192*4) = 268,443,648 bytes
qr_video.rs:182 let len = u32_le(blob, off)? as usize; // per-PNG length bounded only by blob length
qr_verify.rs:230 if elapsed > deadline { return Err(VerifyError::TimedOut ...) } // after all work
qr_carousel.rs:700 let Some(pv) = self.residuals.get(idx).cloned() else { ... }  // clone per reduction
```
Description: Specifics of the K1-10 gap. (1) The PNG side ceiling is 8192 while the encoder never exceeds 740, so a hostile frame may force a 256 MiB raw buffer, a 64 MiB grey buffer and an rqrr image of 64 MiB, about 400 MiB peak per frame, for roughly 260 KB of zlib input (zeros compress about 1000 to 1). (2) `QrVideo::from_bytes` has no total blob cap and no per-PNG cap; only `n <= MAX_VIDEO_FRAMES` (50,000). `demux_optical_frames` decodes every PNG before `decode_frames` can stop. (3) `CarouselDecoder::push` is O(rank) reductions per drop, each cloning a residual (up to 512 + 8168 bytes at k=4096) and XORing a block; up to 8192 distinct seqs are accepted (qr_receive.rs:79), so the worst case is on the order of 10^11 byte operations for one receiver. The independent verifier repeats the same elimination (qr_verify_indep.rs:484-507) without clones. (4) The 60 second deadline in `verify_qr_video` is checked once, after decode and confirm, so it limits nothing. (5) rqrr runs on hostile pixel data with no `catch_unwind`; its own panic behaviour was not reviewed.
Impact: Whoever feeds `decode_qr_video`, `verify_qr_video` or `ProgressiveReceiver` external bytes can pin a worker for minutes or exhaust memory with a blob of a few megabytes. Today the only callers pass self-encoded data capped at 1 MiB content (emit.rs:88), so there is no remote path yet.
Trigger: precondition: a reader wired to external video (K1 reader point). Entry: qr_video.rs:160 then qr_video.rs:249. Input: BDLV with a few dozen PNGs whose IHDR declares 8192 by 8192, colour type 6, IDAT a valid zlib stream of zeros. Expected wrong result: hundreds of MB resident per frame and seconds of CPU per frame, no early refusal.
Known: STATUS.md section 4.4 (K1-10 resource budget before decode; fuzz targets). Only the concrete numbers are new. No fuzz target for any qr_ file exists (`ls fuzz/fuzz_targets`).

### P29-F04
File: src/storage/three_pipe.rs:185-198 (outside); src/storage/qr_carousel.rs:54-62; src/storage/qr_reemit.rs:170-185; src/storage/three_reveal.rs:87-89 (outside)
Severity: Medium
Axis: correctness
Confidence: Likely (missing step: whether any recipe pinned under the earlier value 50 was ever persisted; git history of the constant is one import commit)
Evidence:
```
qr_carousel.rs:61  the old fifty permillage left ~6 frames *in total* and failed 5 trials out of 5 at 8% loss.
qr_carousel.rs:62  pub const ONESHOT_REPAIR_PERMILLAGE: u32 = 150;
three_pipe.rs:185  let n = oneshot_drop_count(enc.params().k, ONESHOT_REPAIR_PERMILLAGE);
three_pipe.rs:198  let fold = fold_frame_digests(&digests)?;  // becomes recipe.stream_id
three_reveal.rs:87 let pass = oneshot_drop_count(public_k, ONESHOT_REPAIR_PERMILLAGE);
three_reveal.rs:89 emitter.verify_stream_id(&pass_fold)?;
```
Description: `recipe.stream_id` is the fold of a pass whose length is `k + ceil(k * permillage / 1000)`. The permillage is a process constant, not a recipe field and not in the BDLV header. Any retune (the comment shows one already happened) changes the fold that `verify_stream_id` recomputes, so every older recipe whose pin is a fold fails to open with `StreamMismatch`. `verify_stream_id` also accepts a pin equal to the A2 commitment, so the check is satisfied by a different value than the one the encoder wrote.
Impact: Owners of recipes created under another permillage lose reveal (`bud_storageOpenReveal`) after a constant change, with no versioning signal. Determinism of the recipe object depends on a value the recipe does not carry.
Trigger: none (not a hostile-input issue). Entry: three_reveal.rs:82-89. Input: recipe produced when the constant had another value. Expected wrong result: Err(StreamMismatch) for a valid recipe.
Known: none.

### P29-F05
File: src/storage/qr_video.rs:179-197, 201-203; src/storage/qr_frame.rs:158-175; src/storage/qr_codec.rs:167-183; src/storage/qr_carousel.rs:244-308; src/storage/qr_video.rs:261-363; src/storage/qr_verify_indep.rs:235-238
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
qr_video.rs:181 for _ in 0..n { ... off += len; png_frames.push(png); }   // no `off == blob.len()` check
qr_frame.rs:161 if frame.len() < drop_end { return Err(FrameError::Truncated); } // longer frame accepted
qr_codec.rs:169 for _ in 0..n { ... }                                      // trailing bytes accepted in split
qr_carousel.rs:256 // flags at 5 ignored for v1   (pad byte 19 also ignored)
qr_video.rs:274 while off + 8 <= png.len() { ... }  // duplicate IHDR, chunks after IEND, unknown critical chunks accepted
qr_verify_indep.rs:235 if b.rank == b.k { break; }                       // later frames never parsed
```
Description: Several layers accept non-canonical encodings: trailing bytes after the last PNG in a BDLV blob, after the drop inside a frame, after the last frame in a raw-concat blob, non-zero drop flags and pad, extra or repeated PNG chunks. Both decoders stop at completion, so frames after the point of completion are never checked. `QrVideo::blob_commitment` hashes the raw blob (qr_video.rs:201), so distinct byte strings decode to the same content.
Impact: Byte-level malleability of a stored video: a verified video can be padded or appended to without failing `verify_qr_video`, while its blob commitment changes. Callers that compare commitments of a re-encode (three_recipe.rs:276) stay safe; any caller that pins a received blob's commitment and then verifies it does not get a unique blob.
Trigger: precondition: a reader or pin path on received bytes. Entry: qr_video.rs:160. Input: valid BDLV plus 1 trailing byte. Expected wrong result: Ok with unchanged content and a different `blob_commitment`.
Known: none.

### P29-F06
File: src/storage/qr_carousel.rs:244-308, 558-610; src/storage/qr_verify_indep.rs:361-385, 401-425
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
qr_verify_indep.rs:405 if drop.degree != 1 { return Err("systematic drop with degree other than 1"); }
qr_verify_indep.rs:384 if total.div_ceil(usize::from(block_len)) != usize::from(k) { return Err("drop k does not fit total length"); }
qr_carousel.rs:578    // Trust recomputed selection; degree on the wire is advisory after check.
qr_carousel.rs:579    if idxs.len() != usize::from(drop.degree) && drop.degree != 1 { /* empty */ }
```
Description: The primary decoder and the independent verifier apply different rules. The primary ignores the wire degree (the `if` at qr_carousel.rs:579 has an empty body, an assert that asserts nothing), does not check that `k == ceil(total_len / block_len)`, and `push` never checks `drop.body.len() == block_len` for a hand-built `Drop` (fields are pub; `xor_into` truncates silently to the shorter slice, qr_carousel.rs:947-954). The verifier refuses all three. No test feeds one input to both and compares verdicts, so `decode_qr_video` accepts streams that `verify_qr_video` refuses, and that gap is unpinned.
Impact: The "two independent paths" claim is checked only on honest inputs (`two_paths_agree_on_the_whole_content_set`). A drift in either parser is not caught by any test.
Trigger: none (test gap).
Known: none.

### P29-F07
File: src/storage/qr_encode.rs:577-592, 679-694; src/storage/qr_payload.rs:150-194, 274-278; Cargo.toml:29, 35
Severity: Low
Axis: correctness
Confidence: Likely (missing step: a run of the encoder over many payloads under two rqrr or miniz_oxide versions; the golden tests cover one 25-byte PNG, `mask` of one 1-byte symbol, and one 16-byte A1 vector)
Evidence:
```
qr_encode.rs:577 for mask in 0..8u8 { ... if matrix_decodes_to(&matrix, data) { return Ok(matrix); } }
qr_encode.rs:686 inside.contains(&r) && inside.contains(&c) && matrix.is_dark(r - quiet, c - quiet)
qr_png.rs:19-23  It is not promised across compressor versions; a dependency bump may change the bytes
Cargo.toml:35    rqrr = "0.11"
qr_payload.rs:178 Some(z) if z.len() < content.len() => (z, FLAG_ZLIB),
```
Description: The module claims a recipe regenerates the exact same modules on every machine "and every future dependency bump" (qr_matrix.rs:3-6). The mask is chosen by whether the third-party decoder rqrr reads the symbol, so a different rqrr version can pick another mask for a payload, which changes the modules, the PNG and the BDLV blob. The A1 container embeds deflate output from miniz_oxide at level 9, so the same content has two valid containers (raw and zlib) and the compressed one changes with the compressor; `payload_commitment` and the whole stream commitment chain hash those bytes. The PNG module admits the compressor dependence (qr_png.rs:14-23) and pins one frame; the encoder module does not admit the rqrr dependence, and no golden covers a payload that needs mask above 0.
Impact: A routine dependency bump can change committed video bytes without a failing test, unless the one golden hits the changed path. Not a consensus path (video_commitment is not read by chain code: `rg video_commitment src/chain src/domain` is empty), so impact is product-level.
Trigger: none.
Known: none.

### P29-F08
File: src/storage/qr_reemit.rs:170-185
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
qr_reemit.rs:177 let pinned = self.recipe.stream_id;
qr_reemit.rs:178 if pinned == [0u8; 32] {
qr_reemit.rs:179     return Ok(());
qr_reemit.rs:181 if pinned == self.stream_commitment || pinned == *frame_fold {
```
Description: An all-zero `stream_id` passes `verify_stream_id` unconditionally, and a pin equal to the A2 commitment passes too. The module header and three_reveal.rs:84-89 present this as a check that "a recipe that pins a different stream cannot open a session"; a recipe whose pin is zero or the A2 commitment is never compared against the pass.
Impact: The stream pin is optional by construction; a recipe creator (or anyone who builds a `ThreeRecipePublic` for a sealed opening, three_reveal.rs:78) can omit it and the reveal path will not notice. The packed body is still bound by `payload_commitment` (qr_reemit.rs:96-99), so content is not at risk, only the frame-sequence pin.
Trigger: precondition: ability to supply the full public recipe at open (sealed opening). Entry: three_reveal.rs:82. Input: recipe with `stream_id = [0; 32]`. Expected wrong result: Ok.
Known: none.

### P29-F09
File: src/storage/qr_recipe.rs:16-43, 108-129, 164-166
Severity: Low
Axis: integration
Confidence: Certain
Evidence:
```
qr_recipe.rs:29-31 /// Holders without the opening cannot re-emit drops; validators still see
                    /// sizes needed for refuse-absurd checks.
qr_recipe.rs:33  #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
qr_recipe.rs:36  pub recipe_commitment: [u8; 32],   (total_len, k, block_len: plain pub fields, no constructor check)
rg 'ThreeRecipeSealed' src/: only emit.rs:56,1148 and mod.rs:152 outside the part; no size-sanity consumer
```
Description: The doc promises refuse-absurd checks on the sealed metering fields, but no code validates them: `k`, `block_len` and `total_len` of a sealed recipe can disagree (k not equal to ceil(total_len/block_len)), exceed `MAX_K`, or be zero, and `Deserialize` accepts them. `ThreeRecipePublic` is also deserializable without validation and carries `block_len` twice (once in `carousel`); only `RecipeEmitter::open` checks they agree (qr_reemit.rs:104). The comment at qr_recipe.rs:164-166 says CarouselParams might lack serde, which is stale (it derives it at qr_carousel.rs:148).
Impact: Any consumer that prices or limits by the sealed fields (the 40-byte fee schedule tied in the test at qr_recipe.rs:194) reads unvalidated numbers until the owner opens the recipe.
Trigger: none (no production consumer yet).
Known: none.

### P29-F10
File: src/storage/qr_reemit.rs:149-168; src/storage/three_reveal.rs:87-89 (outside)
Severity: Low
Axis: security
Confidence: Likely (missing step: a timing run of `RevealSession::open` at k=4096 with block_len 8168)
Evidence:
```
qr_reemit.rs:157 let mut frames = Vec::with_capacity(count as usize);
qr_reemit.rs:158 let mut digests = Vec::with_capacity(count as usize);
reveal_gateway.rs:64 pub const MAX_FRAMES_PER_CALL: u32 = 256;     (the only cap, outside the part)
three_reveal.rs:88 let (_, pass_fold) = emitter.emit_frames(0, pass)?;  // whole one-shot pass at open
qr_carousel.rs:870 /// `O(m * k * block_len)`. At k=4096 and `MAX_K` that is still well under the frame budget
```
Description: `emit_frames` takes a `u32` count with no bound and reserves two vectors from it; a count near 2^32 requests about 100 GB of capacity. The RPC caps the count at 256, so only a new caller would hit it. Separately, `RevealSession::open` builds the entire one-shot pass (k + 15%) just to compute the fold. A repair drop XORs about k/2 blocks, so at k=4096 and block_len 8168 the 615 repair drops cost about 10 GB of XOR work plus one SHA per frame, synchronously. The doc claim "well under the frame budget" is not measured in the tree.
Impact: Defense in depth for the first; for the second, an authorized opener (owner or grant holder) ties up a worker for seconds per session open on a maximal recipe.
Trigger: none for the first. Second: Entry three_reveal.rs:82; Input: recipe with k=4096, block_len=8168 (33 MB content).
Known: none.

### P29-F11
File: src/storage/qr_codec.rs:26-53, 128-184; src/storage/emit.rs:751 (outside); src/storage/qr_matrix.rs:158-170
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
rg -n 'LiveCarousel|CodecKind::H264|CodecKind::Vp9' src/ budzero/ crates/ --glob '!src/storage/qr_codec.rs'  -> no output
emit.rs:751  gate_codec(CodecKind::RawFrames)?;     // constant argument, always Ok
rg -n 'mask_pattern|ec_level' src/ --glob '!src/storage/qr_*.rs' -> only emit.rs/server.rs uses of ec_level; no mask_pattern outside tests
```
Description: Three `CodecKind` variants, and every `FrameMux` branch other than `RawFrames` (they return `MuxNotLinked`), are never used by production code. The one production `gate_codec` call passes the constant `RawFrames`, so it can never fail; the gate is not wired to a choice. `QrMatrix::mask_pattern` has no caller outside tests.
Impact: Maintenance only. Not in `.github/dead-pub-api-baseline.txt` or `.github/idle-code-baseline.txt` (rg on both finds only `qr_reemit.rs:recipe_digest` for this part).
Trigger: none.
Known: baseline file for `recipe_digest` only; the rest is not listed.

### P29-F12
File: src/storage/qr_carousel.rs:10-16, 896-897, 655-661; src/storage/qr_recipe.rs:164-166; src/storage/qr_matrix.rs:122-128; src/storage/qr_png.rs:156-176; src/storage/qr_video.rs:108-109, 145
Severity: Info
Axis: correctness
Confidence: Certain
Evidence:
```
qr_carousel.rs:12-14  pos >= k -> drop = XOR of d random blocks, d = 4 + (next_u32(seq) mod 21), capped at k
qr_carousel.rs:880    let d = 1 + (rng.next_u32() as usize % k_usize);
qr_carousel.rs:896    let degree = u8::try_from(chosen.len()).unwrap_or(u8::MAX);
qr_video.rs:108       if optical_frames.len() as u32 > MAX_VIDEO_FRAMES {
qr_png.rs:156         fn crc32_png(data: &[u8]) -> u32 { let mut table = [0u32; 256]; ... }  // table rebuilt per chunk
```
Description: (1) The module spec at the top of qr_carousel.rs still describes the old degree distribution 4..24; the code draws 1..=k. DROP_VERSION and the PRNG tag stay at V1, so the wire label does not tell the two apart. (2) The wire `degree` saturates at 255 for k above 255, so "Number of source blocks XORed" (qr_carousel.rs:214) is false for those drops; both decoders treat it as advisory or compare the saturated value. (3) `as u32` casts on lengths in `from_optical_frames` and `to_bytes` (qr_video.rs:108, 145, 149) and `RawFrameConcat::mux` (qr_codec.rs:139, 141) wrap above 4 G entries; unreachable in practice. (4) `solve_block` returns `BadK(0)` for an out-of-range index. (5) The CRC table is rebuilt for each chunk written. (6) `fn rows_of` carries `#[must_use]` and a `# Errors` section it does not have.
Impact: Documentation and maintenance. Item 1 matters if an archived stream from the earlier distribution exists; none is known.
Trigger: none.
Known: none.

### P29-F13
File: src/storage/qr_*.rs (all 13 files)
Severity: Info
Axis: invariant(Z3, Z7, Z8, Z9)
Confidence: Certain
Evidence:
```
rg -n 'allow\(|ignore|cfg_attr|clippy::' src/storage/qr_*.rs   -> 4 hits, all English comments (qr_carousel.rs:256, qr_receive.rs:112,136,270)
rg -n '\bunsafe\b' src/storage/qr_*.rs                         -> no hits
rg -n '\bf(32|64)\b' src/storage/qr_*.rs                       -> no hits
rg -n 'HashMap|HashSet' src/storage/qr_*.rs                    -> no hits (BTreeMap in qr_receive.rs:89 and qr_verify_indep.rs:204)
```
Description: Z3 (no new allow or ignore), Z7 (no unsafe), Z8 (no float; the part is not on the consensus path in any case) and Z9 (no key, token or seed literal; the test keys are byte fills such as `PayloadKey([0x42; 32])`) hold for this part at the base commit. Production code reads bytes only through `get`, `checked_add` and `try_from`. Git attribution of the absence was not needed.
Impact: none.
Trigger: none.
Known: none.

---

## Known overlap, not filed

- Reader point unwired. `rg -n 'decode_qr_video|QrVideo::from_bytes|demux_optical_frames' src/ --glob '!src/storage/qr_*.rs' --glob '!src/tests/**'` shows only tests, a re-export in mod.rs and three_pipe.rs:297-300; `ProgressiveReceiver` and `decode_frames` are called in production only from emit.rs:897 and 926 on the frames the same call just encoded, and `verify_qr_video` only at emit.rs:939 on the same blob. STATUS.md 4.4 K1-04, K1-05, Q3.
- Resource budget before decode and fuzz targets (K1-10); determinism goldens (K1-12): only concrete numbers are in P29-F03 and P29-F07.
- The shared QR symbol layer (rqrr and the PNG reader feed both decoders): stated in qr_verify_indep.rs:21-26 and in STATUS.md 4.4.

## Counts

Critical 0, High 0, Medium 3 (F01, F02, F04), Low 8 (F03, F05, F06, F07, F08, F09, F10, F11), Info 2 (F12, F13).
