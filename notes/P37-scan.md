# P37 scan: B.U.D. 1 (bud/src, first 20 files, read-only audit, Tier R2)

Base commit: 9d46064. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md sections 2 and 4.4, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt. Items present in .github/idle-code-baseline.txt are marked Known.

## COVERAGE

Read in full (every line, tests included), directory /home/user/budlum/bud/src:
- bud3_live_test.rs 1-99
- bud_fixed_point.rs 1-244
- bud_format.rs 1-1116
- bud_format_autozstd.rs 1-90
- bud_format_av2.rs 1-108
- bud_format_bft.rs 1-547
- bud_format_block.rs 1-362
- bud_format_bloom.rs 1-164
- bud_format_catalog.rs 1-445
- bud_format_checkpoint.rs 1-217
- bud_format_columnar.rs 1-785
- bud_format_container.rs 1-1176
- bud_format_crop.rs 1-212
- bud_format_culling.rs 1-295
- bud_format_das.rs 1-342
- bud_format_dbdelta.rs 1-100
- bud_format_dedup.rs 1-150
- bud_format_dictionary.rs 1-228
- bud_format_economics.rs 1-526
- bud_format_edge.rs 1-106
(20 files, 7312 lines; the task estimate of 7860 was high.)

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md 110-165; notes/P29-scan.md 1-30.

Outside the part (rg first, then at most 80 lines): bud/Cargo.toml 1-140; bud/src/lib.rs 195-225 and rg of `pub mod`; bud/src/bin/bud.rs 300-345, 425-465, 800-840 (and rg); bud/src/cli.rs 14-45; bud/src/bud_format_engine.rs 296-395 (and rg); bud/src/bud_format_pipe.rs 140-170; bud/src/bud_format_real.rs 27-53; bud/src/bud_format_huffman.rs 94-120 (rg); .github/idle-code-baseline.txt (rg only); .github/workflows/ci.yml (rg only); STATUS.md and docs/audit/FINDINGS.md (rg only).

Not read: the other 83 modules of bud/src, bud/tests, bud/fuzz bodies, the zstd, serde_json and ml-dsa crate sources. No cargo, no test run: every claim comes from reading source. Where a claim depends on a library behaviour it is stated.

Extra-check summary:
- Erasure coding correctness: the only "erasure" in the part is the v1 BudFile parity (bud_format.rs:245-246), which is not erasure coding: P37-F02. Real coding (bud_format_erasure.rs) is outside the part.
- Content id binding: gaps in v1 (F03), v2 root digest (F19), DAS root (F22), MerkleTrie root (F26), BFT votes (F12), crop flag (F31).
- Bounds: v2 container has count, per-chunk and total bounds on the compressed side but no aggregate bound on the restore side (F20). Columnar blob reader is bounded (n checked against remaining bytes); columnar encoder reads up to 1 GiB (F16).
- Determinism (Z8): no float in a consensus path of this part (bud/ is outside the Z8 directories). f64 decides acceptance or price in bft, checkpoint, economics, autozstd, culling_ratio; recorded as Low where it matters (F13, F29). The zip-bomb gate itself is integer (fixed_ratio) but is fed a vacuous input (F06).
- Panics from external bytes: no unwrap, expect or direct-index panic path found in production code of the 20 files; all unwrap and expect hits are under #[cfg(test)] (rg on the 20 files). A candidate panic (OrderFree sort with an inconsistent comparator in bud_format_columnar.rs:160) was checked and cleared: string compare of non-numbers puts every JSON string below every number and arrays, objects, bools and null above, so the order is a total preorder. panic = "abort" is set for dev and release (bud/Cargo.toml:136-139), so any OOM aborts the process (F20).
- Unwired parts: the whole crate has no consumer outside bud/ (F01). Eight in-part modules have no caller even inside the crate (F35, Known).
- Z3: rg for `#[allow`, `#[ignore`, `continue-on-error` over the 20 files: no hit. CI runs the crate (ci.yml:594-601: fmt, clippy -D warnings, tests with bud3).
- Z7: every file has `#![forbid(unsafe_code)]` (bud3_live_test.rs is test-only and inherits the crate lint); no `unsafe` hit.
- Z9: no key, seed or token value in production code of the part. Test keys `SigningKey::from_bytes(&[i; 32])` appear in bud_format_bft.rs:253 (tests only). Outside the part, the demo command bud/src/bin/bud.rs:437-440 derives its validator keys from the constants `[i+1; 32]`; a demo set, not a registry (see F12).

## FINDINGS

### P37-F01
File: bud/Cargo.toml:1-3 (crate bud-core); bud/src/lib.rs:5-96 (module list)
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: `rg -n 'bud.core|bud_core|path = "bud' Cargo.toml src crates budzero xtask` returns no dependency line and no `use bud_core`; the only hits are `src/bin/bud.rs:35 name = "bud"` (a clap name) and gate walkers that scan the directory. Users of the crate: bud/src/bin/bud.rs:21-39, bud/tests/*.rs, bud/fuzz/*.rs.
Description: bud-core is a separate workspace that nothing in the node, the registry, the storage deals or the RPC links. Its modules are reached only by its own CLI and tests, including the BFT ratio vote, the regeneration block, the checkpoint chain, the DAS and the dedup PoW.
Impact: STATUS.md section 5 says everything coded must be wired to a production call site. Every B.U.D. claim in this part (ratio consensus, DAS, ownership, economics) has no effect on a node. Written as a mainnet blocker for any claim that relies on these modules.
Known: none (STATUS.md section 4.4 covers the K1 QR gaps only)

### P37-F02
File: bud/src/bud_format.rs:189-190, 245-250, 397-406
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
189:            erasure_k: 18,
190:            erasure_p: 3,
245:        let p1 = xor(&payload, &vec![0xAA; payload.len()]);
246:        let p2 = xor(&payload, &vec![0x55; payload.len()]);
```
`to_bytes` writes `chunk.data` and then every parity shard in full (lines 397-406).
Description: The "parity shards" of a v1 BudFile are the payload XORed with a constant byte. Each one alone reconstructs the payload; there is no k-of-n structure, and the header advertises (18,3). `decode` never reads `parity_shards`, so they are not even checked.
Impact: A stored v1 file is about 3x the payload (the CLI prints data.len()/bytes.len(), bud/src/bin/bud.rs:333, about 0.33) while bud_format_economics assumes expansion 1.1667 (economics.rs:24-27 and its tests). Anyone relying on "18+3" repair for a v1 file gets none: a corrupted payload is not repaired from parity, it is only refused (content_id check, line 353).
Known: none

### P37-F03
File: bud/src/bud_format.rs:272-291, 344-374, 351-358
Severity: Medium
Axis: invariant (content id binding)
Confidence: Certain
Evidence:
```
351:        if !self.chunks.is_empty() {
352:            let payload = &self.chunks[0].data;
353:            if hash(payload) != self.header.content_id {
...
356:            if self.merkle_root != self.chunks[0].hash {
372:            Ok(self.chunks[0].data.clone())
```
Description: `decode` verifies and returns only chunk 0. `merkle_root` is compared with `chunks[0].hash`, a stored field that is itself never recomputed, so it is not a Merkle root. A file with chunk_count above 1 loses chunks 1.. silently, and those chunks are never hashed. `original_content_id` (set in `encode`, line 243) is never compared to anything, and `decode_streaming` (272-291) hands every chunk to the sink with no hash check at all.
Impact: A parsed v1 file (from_bytes accepts any chunk count, lines 488-529) can carry unauthenticated data in chunks 1.. while `decode` reports success for chunk 0; streaming consumers get unverified bytes. The "original is recoverable and verifiable" claim has no check behind it.
Known: none

### P37-F04
File: bud/src/bud_format.rs:376-427 (to_bytes), 599-608 (from_bytes), 363-365
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: `to_bytes` writes `pq_signature`, `encryption_key_wrapped` and the token but no `pq_public_key`; `from_bytes` sets `pq_public_key: None` (line 604). `decode` for a PQ_SIGNED file calls `verify_pq_signature`, which fails with "KQ-BUD-PQ: no public key" (lines 309-312).
Description: A PQ-signed file does not survive serialisation: write, read back, decode fails. `sign_pq` has no non-test caller (rg: only bud_format.rs:329 and tests), so the CLI never hits it, but the library path is broken. The roundtrip test (line 1011) never calls `to_bytes`.
Impact: Any future caller that stores a signed file gets a file nobody can open, or drops the flag to make it open.
Known: none

### P37-F05
File: bud/src/bud_format.rs:302-326, 360-365
Severity: Low
Axis: security
Confidence: Certain
Evidence: message is `b"BDLM_PQ_SIGN_V1" || content_id` (319-322); the verifying key comes from the file itself (`self.pq_public_key`, 309-312); the check runs only if `flags.is_pq_signed()` (363), a flag inside the unsigned header.
Description: The signature covers only content_id (a hash of the payload). The key is self-asserted, with no registry or pinned identity, and the flag that switches the check on is not signed. Header fields (mime, original_content_id, pipe_id, flags, version) are unsigned.
Impact: The check proves only that the signer holds some key matching the file; anyone can sign any payload and an attacker can clear the flag. "PQ signed" gives no authorship guarantee.
Known: none

### P37-F06
File: bud/src/bin/bud.rs:819-826 (caller); bud/src/bud_format.rs:818-825, 344-374
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
bud.rs:823  let out = file.decode().map_err(...)?;
bud.rs:825  BudGates::k_bud_ratio(&file, out.len())
bud_format.rs:819  let payload: usize = f.chunks.iter().map(|c| c.data.len()).sum();
bud_format.rs:820  let measured = ...fixed_ratio(original_len as u64, payload as u64);
```
Description: `decode` returns `chunks[0].data`, the stored payload, not an expanded original. The only production call of the K25 zip-bomb gate passes that payload length as `original_len`, so the ratio is payload/payload = 1 for a one-chunk file and the gate cannot fire. v1 never decompresses anything (the candidates keep the original bytes, F07), so there is no bomb to gate, and the gate is vacuous either way.
Impact: The integer-exact gate (bud_fixed_point.rs) is correct but is fed a self-comparison; "zip bomb refused" is untested end to end. Same shape as the P29 finding about a check comparing output with itself.
Known: none

### P37-F07
File: bud/src/bud_format.rs:626-772, 774-796; bud/src/bin/bud.rs:313-327; bud/src/cli.rs:23-39
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
639:   RatioCandidate { pipe_id: 2, pipe_name: "CDC16K+zstd", ratio: 15.5, payload: original.to_vec(), ...
685:   RatioCandidate { pipe_id: 41, pipe_name: "4 layers 13750x", ratio: 13750.0, payload: original.to_vec(), ...
```
Description: Every candidate carries a hand-written `ratio` literal and the original bytes as payload; `select_best` picks by the claimed ratio and the CLI stores `c.payload` under `c.pipe_id`. Nothing compresses. The v2 container module notes the same pattern was removed there ("hand-written ratios and the original bytes as its payload", bud_format_container.rs:13-17) but it remains in v1.
Impact: `bud encode --class log` writes the raw bytes (plus parity, F02) labelled pipe 41 / "13750x"; the stored header and the catalog (bud_format_catalog.rs ranges) then report ratios that were never measured. Contradicts K19 ("every component measured").
Known: none

### P37-F08
File: bud/src/bud_format.rs:382, 399, 401, 409, 421, 429-609
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `(mime.len() as u16)`, `(chunk.data.len() as u32)`, `chunk.parity_shards.len() as u8`, `(sig.len() as u16)`, `(token.len() as u16)`; `from_bytes` uses `String::from_utf8_lossy` (453, 576) and never rejects trailing bytes after the token.
Description: Writer casts truncate silently (a mime or token over 65535 bytes, 256 parity shards) and produce a file the reader parses differently; the reader normalises invalid UTF-8 and ignores trailing data, so write(read(x)) differs from x.
Impact: Corrupt or non-canonical v1 files are accepted; two byte strings parse to one file. Low because the fields are caller-controlled today.
Known: none

### P37-F09
File: bud/src/bud_format.rs:834-841
Severity: Low
Axis: test / invariant(Z3 pattern)
Confidence: Certain
Evidence:
```
834   pub fn k_bud_repo_dep(_f: &BudFile) -> Result<(), &'static str> {
835       // is there a repo hash? self-contained is OK
836       Ok(())
838   pub fn k_bud_report_exists() -> Result<(), &'static str> {
840       Ok(())
```
Description: Two named gates always return Ok without looking at anything. `k_bud_video_revolutionary`, `_image_`, `_pdf_`, `_exe_` (842-865) are threshold checks on a caller-supplied f64 and have only a test caller.
Impact: A gate that cannot fail reads as a check in reports (the module header lists them as gates). Silent success.
Known: none

### P37-F10
File: bud/src/bud_fixed_point.rs:26-31
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
26 /// Fixed point to integer, truncating toward zero, the same direction for
27 /// either sign.
29 pub const fn fixed_to_int(v: i64) -> i32 {
30     (v >> FIXED_FRAC_BITS) as i32
```
Description: `>>` on i64 is an arithmetic shift, which floors: fixed_to_int(-1.5) is -2, not -1. The `as i32` also wraps for values above 2^47. The module claims "no panics" and saturating conversions; `fixed_from_int` is documented "Saturating" but cannot overflow.
Impact: Documented behaviour differs from actual for negatives. No in-tree caller (rg: only lib.rs re-export and tests), so no live effect; the test only covers whole negatives.
Known: idle-code-baseline (fixed_to_int family)

### P37-F11
File: bud/src/bud_fixed_point.rs:116-119; bud/src/bud_format.rs:818-825
Severity: Low
Axis: security
Confidence: Certain
Evidence: `if payload_len == 0 { return FIXED_ONE; }`
Description: A file whose chunks sum to zero bytes reports ratio 1 for any claimed original length, so the bomb gate accepts "original 4 GiB, payload 0".
Impact: Only reachable with an empty payload (decode still demands hash(payload) == content_id, so the file must also hash the empty payload). Defense in depth only.
Known: none

### P37-F12
File: bud/src/bud_format_bft.rs:84-92, 130-183
Severity: Medium
Axis: security (content id binding, replay)
Confidence: Certain
Evidence:
```
84   fn message(validator_id: &str, pipe_id: u16, ratio: f64) -> Vec<u8> {
86       m.extend_from_slice(&b"BDLM_BFT_VOTE_V2"[..]);
...  validator_id, pipe_id, ratio   (no content id, epoch, domain, or chain id)
```
Description: A vote signs (validator id, pipe, ratio) only. It names no object, container, epoch or round, so a quorum certificate for "pipe 7, ratio 16.68" is valid for every file at once and is replayable forever. `RatioFinalityCert` has no field linking it to a content id.
Impact: Even if wired, a certificate proves nothing about a specific file; an old certificate can be attached to any new object. The CLI demo (bud.rs:437-440) builds its validator keys from `[i+1;32]`, so it is not a real set either. No consumer exists in the tree (F01).
Known: none

### P37-F13
File: bud/src/bud_format_bft.rs:130-183, 198-244
Severity: Low
Axis: security
Confidence: Certain
Evidence: `verify` checks `votes.len() < quorum` but has no upper bound; the membership loop `validators.contains` (linear, line 75) runs once per vote before the duplicate check (167-176); ratio is never validated (NaN, negative, infinite, subnormal accepted); equality is by bit pattern (155-163).
Description: A certificate with a very long vote list costs O(votes x n) before the duplicate rejection. A NaN or negative ratio with valid signatures finalises. Equal values with different bit patterns (0.0 and -0.0, NaN payloads) never reach quorum together.
Impact: Resource use scales with attacker-sized input; a finalised ratio can be meaningless. Low, no consumer.
Known: none

### P37-F14
File: bud/src/bud_format_block.rs:98-106, 129-131, 157-227
Severity: Medium
Axis: invariant (content id binding)
Confidence: Certain
Evidence:
```
100   pub fn verify(&self) -> bool {
101       self.hash == self.compute_hash()
102           && self.pact_challenges.iter().all(|c| c.outcome == RegenerationOutcome::Verified)
130   self.pact_challenges.iter().map(|c| c.cost_units).sum()
```
Description: `verify` is the module's "production consensus", but it checks that a self-computed hash matches and that outcomes, which are plain fields in the blob, say Verified. Nothing recomputes an exam, nothing checks `prev_hash` against a parent, `epoch` order, `segment_root`, or a signature; an empty block verifies; `from_blob` accepts any `pact_hash`, outcome and cost. The header says "any node can recompute the block", yet a block holds no bytes to recompute from.
Impact: An attacker writes a block of arbitrary pact hashes marked Verified and it passes both `from_blob` and `verify`. `total_production_cost` adds u64 from external bytes with `.sum()`; the crate sets no overflow-checks (also P59-F07), so it wraps in release.
Known: none

### P37-F15
File: bud/src/bud_format_columnar.rs:112-206, 223-246; bud/src/bud_format_engine.rs:306-322
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: keys come from the first record only (`let keys = first.keys().cloned()`, 122); other records are accepted if their sorted key set matches (130-144); decode rebuilds every record in the first record's key order (236-238) with `serde_json::to_vec` (241). engine.rs:306: "losslessness is guaranteed by transform_test", but engine_store has no comparison of the restored bytes with `data` (rg `== data` finds none).
Description: Exact mode claims decode(encode(d)) == d. It fails for records whose keys are in a different order than the first record, for pretty-printed input, escapes (`é`), numbers like `1e2` or `1.50`, and duplicate keys (last wins). preserve_order only preserves the first record's order.
Impact: `engine_store` on such JSON stores a container that restores to different bytes and reports success; the content id of the restored bytes differs from the original. Silent data change on the primary B.U.D. 2.0 path.
Known: none

### P37-F16
File: bud/src/bud_format_columnar.rs:112-120, 153-199, 475-482, 236-238
Severity: Low
Axis: security / correctness
Confidence: Likely (memory figures not measured)
Evidence: input ceiling is 1 GiB (113); the encoder holds the parsed `Value` tree, a vector of object references and a cloned copy of every cell (`columns[ci].push(v.clone())`, 197). Blob reader: F64 cells use `Value::from(f64::from_bits(bits))` (481), which turns NaN and infinity into JSON null; duplicate keys in a blob are accepted and `columnar_decode` silently drops all but the last (237).
Description: Peak memory is a multiple of the input size with no output-size or node-count guard before the clone. The blob digest is unkeyed, so a crafted blob with valid digest decodes to altered data.
Impact: Memory use up to several GiB for a 1 GiB input (missing verification step: measure peak RSS on a large array of small objects). Blob tampering is detectable only against accidental corruption.
Known: none

### P37-F17
File: bud/src/bud_format_catalog.rs:31-311, 314-363
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `catalog_detect` returns the first entry whose signature is a prefix. "RIFF" is listed for WEBP (163-169) before WAV (204-210); MKV and WebM share `\x1A\x45\xDF\xA3` (237-252); DOCX and XLSX share "PK\x03\x04" after ZIP (114-128, 295-310); MP4, AVIF, CSV, LOG, PlainText have empty signatures; "BM", "MZ", "@", "<", "{" match any file that starts with those bytes.
Description: Several entries can never be returned; a text file starting with "@" is FASTQ, one starting with "BM" is BMP, a JSON file with a BOM or leading space is Unknown. The v1 magic `BUD\x01` is not in the table (only the v2 magic is).
Impact: bud_format_engine.rs:300 chooses the transform and codec from this result, so misdetection picks a wrong pipe. Ratios in the table are hard-coded claims (see F07).
Known: none

### P37-F18
File: bud/src/bud_format_checkpoint.rs:1-107
Severity: Low
Axis: security
Confidence: Certain
Evidence: header claims "SEC 17a-4 audit trail: a hash chain plus a timestamp, an identity and a cryptographic proof" (lines 6-9); the struct has no timestamp, identity or signature; `verify_chain` recomputes unkeyed SHA3 and links prev_hash (78-95); epoch order is not checked.
Description: Anyone can build a complete valid chain with any content; there is no restore-on-restart code and no persistence in the module. The ratio is an f64 hashed by `to_le_bytes`.
Impact: "Tamper evidence" holds only against accidental change by a party that cannot recompute the chain; "durable checkpoint restored on restart" is not implemented.
Known: none

### P37-F19
File: bud/src/bud_format_container.rs:246-264, 417-427, 505-547
Severity: Medium
Axis: invariant (content id binding)
Confidence: Certain
Evidence:
```
247   let mut h = Sha3_256::new();
248   h.update(b"BDLM_BUD_V2");
249   for c in chunks { h.update(c.content_id); }
```
`chunk_codec` is written as one byte after the header (420) and is outside both the digest and the header; `codec`, `chunk_count`, `total_len` are in the header but not in the digest. For Huffman and Zstd the chunk content_id is over the compressed bytes (365-366, 383).
Description: The root content id does not cover the chunk codec byte, so the same root id can be restored with `Raw`, `Huffman` or `Zstd` handling (flip the byte; `restore_original` then returns garbage or None). The id is a function of compressed bytes, so the same original gets different ids per codec, zstd level and zstd version, and no hash of the restored original is stored or checked.
Impact: Content-addressing is not stable across compressors, so dedup across codecs fails; the restored original is not verified against any commitment. Aliasing of one id to different restored bytes is possible by an unauthenticated byte flip.
Known: none

### P37-F20
File: bud/src/bud_format_container.rs:346-348, 505-523; bud/src/bud_format_real.rs:27-53
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: build a zstd frame of 4 GiB zeros and run `bud check`)
Evidence: `MAX_TOTAL_BYTES` (4 GiB) and `MAX_CHUNK_BYTES` bound the compressed size (decode, lines 459, 474). `restore_original` calls `zstd_decompress_safe(&c.data, ZSTD_MAX_DECOMPRESSED)` per chunk with `ZSTD_MAX_DECOMPRESSED = 4 GiB` (bud_format_real.rs:27) and appends every result to one `Vec` (519).
Description: Each chunk may expand to 4 GiB and there are up to 1,000,000 chunks; no aggregate output bound applies. A zstd frame of zeros expands by a very large factor, so a file of a few hundred KB can request many GiB. The huffman path has the same per-chunk 4 GiB cap (bud_format_huffman.rs:27, 106). The K25 ceiling is per chunk, not per file.
Impact: Memory exhaustion from a small container through `bud check`, `engine_restore*` and `restore_json_columnar`; with panic = "abort" the process dies. If the node ever calls these on peer data it is a remote single-node crash.
Known: none

### P37-F21
File: bud/src/bud_format_container.rs:246-264, 391-415
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `BudV2Header::new` is `pub` and does `chunks.len() as u32` and an unchecked sum (253, 261); `new_with_codec` bounds count, total and per-chunk but never checks `c.content_id == content_id(&c.data)` for caller-supplied chunks (the Raw path, line 351-353).
Description: The public constructors trust supplied chunk ids and the unconstrained header path can truncate the count. `encode` then writes the wrong ids and `decode` refuses the file.
Impact: A locally produced file that cannot be read back; no remote effect.
Known: none

### P37-F22
File: bud/src/bud_format_das.rs:27-64, 75-129
Severity: Medium
Axis: invariant (content id binding)
Confidence: Certain
Evidence: odd-count rule duplicates the last node (`h.update(pair[0]); // odd one out, so duplicate it`, 57); the root carries no leaf count.
Description: das_root([a, b, c]) and das_root([a, b, c, c]) are identical: level 1 is [N(La,Lb), N(Lc,Lc)] in both. The leaf count is not part of any domain tag or root.
Impact: Two different data sets (3 chunks and 4 chunks, extra duplicated chunk) share one root; a verifier holding the root cannot tell the object length. Same flaw as the Bitcoin duplicate-leaf case. Also the test at line 276 only checks that proofs verify, not that counts are distinguished.
Known: idle-code-baseline (das_root, DasProof, DasSampler have no production caller)

### P37-F23
File: bud/src/bud_format_das.rs:132-153, 191-206
Severity: Low
Axis: security / correctness
Confidence: Certain
Evidence: `verify` uses `leaf_index` only for parity at each of `path.len()` levels (138-151), with no check against a leaf count or tree depth; `verify_sample` takes the full `chunks`, recomputes the whole root (192), then rebuilds every leaf hash again inside `DasProof::prove` for each sample (197).
Description: A proof verifies for any `leaf_index` with the right parity bits (high bits ignored), so a proof does not prove position. `verify_sample` needs all chunks in memory and hashes the entire object, so it is not sampling: cost is O(k x n) and the "high probability" claim in the header does not apply.
Impact: No data availability saving; a position claim from a proof is unreliable. Low, unwired.
Known: idle-code-baseline

### P37-F24
File: bud/src/bud_format_das.rs:209-254
Severity: Medium
Axis: security
Confidence: Certain
Evidence: header says "each validator declares which chunk it holds in a signed record" (line 14); `DasOwnership` has `validator_id: String`, `chunk_index`, `chunk_hash`, `ts_unix` and no signature or key; `chunk_index as u32` in the hash (239).
Description: The record is unsigned and not tied to a root or object, and the chunk hash is computed from public data. Anyone who has the chunk can create a record for any validator id; holding a chunk once proves nothing about holding it later (no challenge, no freshness).
Impact: If used for reputation or payment, custody claims are forgeable and replayable. The "missing chunk fails the DAS exam" step does not exist in code.
Known: idle-code-baseline (DasOwnership)

### P37-F25
File: bud/src/bud_format_dedup.rs:53-107
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
 53 /// PoW ownership challenge: whoever holds the chunk can solve the challenge.
 56 pub struct PowChallenge { pub chunk_id: [u8; 32], pub difficulty: u32 }
 71 fn hash_with_nonce(&self, nonce: u64) -> [u8; 32] { ... DOMAIN || chunk_id || nonce }
```
Description: The puzzle depends only on `chunk_id`, which is a public hash, not on the chunk bytes, a verifier nonce or the prover's identity. Anyone who knows the id can solve it without the data, a solution is valid forever and shareable, and difficulty 0 always passes (`leading_zero_bits >= 0`).
Impact: It does not prove ownership; it only costs CPU. A dedup index guarded by it (K20, decision 71) is open to the known dedup oracle (claim a hash, gain access to another tenant's chunk). Only used in a securededup test (bud_format_securededup.rs:97-132).
Known: idle-code-baseline (PowChallenge partly)

### P37-F26
File: bud/src/bud_format_economics.rs:271-307
Severity: Medium
Axis: invariant (content id binding)
Confidence: Certain
Evidence:
```
293   // root = hash of all keys sorted
294   let mut hashes: Vec<_> = self.entries.keys().cloned().collect();
295   hashes.sort();
```
Description: `MerkleTrie::root` hashes only the sorted keys; values (`entries` values) are not in the root, there is no tree, no proof, and every insert re-sorts all keys (O(n log n) per insert, O(n^2 log n) to build). `k_bud_trie_root` only checks the root is non-zero.
Impact: Two tries with the same keys and different values have the same root; the "Merkle trie integration" gives no commitment to content. Not wired (F01).
Known: none

### P37-F27
File: bud/src/bud_format_economics.rs:233-269; bud/src/bud_format_dedup.rs:1-9
Severity: Medium
Axis: security
Confidence: Certain
Evidence: `insert_chunk(&mut self, hash: [u8; 32], size: usize)` takes a caller-supplied hash and size, nothing hashes data; `self.total_saved_bytes += size as u64` is unchecked. bud_format_dedup.rs:5-8: "Cross-tenant convergent dedup is NOT done, for privacy" (decision 71).
Description: `GlobalDedup` is exactly the cross-tenant dedup the K20 decision excluded, with public fields and no ownership proof; a "duplicate" answer lets a caller claim bytes it never sent. `dedup_ratio` mixes u64 and f64.
Impact: If wired as a storage dedup path, a client that guesses or learns a chunk hash is told "deduplicated" and is credited the saving, and the hash set leaks membership. Contradicts a recorded decision.
Known: idle-code-baseline (check the economics lines)

### P37-F28
File: bud/src/bud_format_bloom.rs:20-62, 84-114
Severity: Low
Axis: correctness / dead code
Confidence: Certain
Evidence: struct keeps `exact: Vec<[u8; 32]>` and `insert` pushes every hash (69); `contains_exact` calls `self.exact.contains(h)` (a linear scan, 87); `new` allocates `vec![0; num_bits.div_ceil(64)]` from `expected * bits_per_entry` with only a saturating multiply (36, 41); `digest` writes `num_bits as u32` and `k as u8` (108-109).
Description: The "94 to 97 percent RAM saving" is computed by excluding the exact set that is always stored (`ram_saving_vs_full`, 97-103), and each exact lookup is O(n). Huge parameters allocate unbounded memory; large values truncate in the digest.
Impact: Claimed RAM economics not achieved; no production caller.
Known: idle-code-baseline (BloomDedupIndex)

### P37-F29
File: bud/src/bud_format_economics.rs:20-41, 75-97, 66-69
Severity: Low
Axis: correctness (Z8 caution)
Confidence: Certain
Evidence: `residual_price` has no upper bound on `coldness`: `1.0 - coldness * 0.5` is negative for coldness above 2; `holds_price(ceiling)` is `cost <= ceiling + 1e-12`, so an invalid ratio (cost = +inf) passes when `ceiling` is +inf; `fee` ignores `expansion` and the ratio check (`self.ratio > 10.0`) and returns f64.
Description: A negative price satisfies every ceiling gate; the inf case defeats the "honest refusal". The module note says residual pricing was simplified away by the flat-price decision, so `residual_price` is superseded.
Impact: If prices ever feed a deal or fee, f64 results can differ in the last bit between builds and negative prices pass. No consumer outside cli.rs/integration.rs demo; the repo rule is no economic decisions in this phase.
Known: idle-code-baseline (economics items)

### P37-F30
File: bud/src/bud_format_edge.rs:35-61
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `request(size_bytes, budget_hit_ratio)` has no object key; any request that fits the remaining capacity or is under 1 percent of capacity is a "hit"; `used` never decreases; `budget_hit_ratio` is discarded (`let _ = budget_hit_ratio;`).
Description: A first request for a never-seen object counts as a hit, there is no eviction, and a full cache still "hits" all small objects. `hit_ratio` and `egress_saving_pct` are therefore not cache measurements.
Impact: The "90 to 95 percent hit rate" figures in the tests and header are artefacts of the model. Unwired.
Known: idle-code-baseline (EdgeCache items)

### P37-F31
File: bud/src/bud_format_crop.rs:32-64, 97-138
Severity: Low
Axis: security / correctness
Confidence: Certain
Evidence: `derivation_hash` covers master id, x, y, w, h but not `aligned` (74-83); `from_blob` reads `aligned = bytes[57] != 0` and returns it without recomputing (122, 126-137).
Description: A blob with a flipped `aligned` byte still passes the hash check. There is no bound check: w = h = 0 counts as aligned, `x + w` can overflow u32, and the crop is not related to the master's dimensions. The 16 x 16 MCU assumption holds only for 4:2:0.
Impact: `is_regenerable` can be set by editing one byte, so a non-reproducible crop is classed as reproducible. Unwired, so no live effect.
Known: idle-code-baseline (CropDerivation)

### P37-F32
File: bud/src/bud_format_culling.rs:88-119, 187-236
Severity: Low
Axis: correctness / security
Confidence: Certain
Evidence: `Vec::with_capacity(count)` for two vectors with `count` up to 1,000,000 before the first record is read (199-200), so a 45-byte blob reserves about 9 MB; `from_blob` accepts any tier for any access count and any thresholds, then checks only the hash; with the documented default `cold_threshold = 1`, `a >= cold.max(1)` is true for every non-zero count so the Cold tier is unreachable (102-106).
Description: Tier and access count can disagree in a valid blob (a Culled cluster with 1000 accesses). `Culled` means "not stored at all"; the plan is only advice, but a reader trusting `tiers` would delete data.
Impact: Low: bounded allocation, no remote path today; the tiers are unauthenticated.
Known: idle-code-baseline (CullingPlan fn items)

### P37-F33
File: bud/src/bud_format_dictionary.rs:57-74, 84-94
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `dict_id` is read from `bytes[0..4]` (63-66) and the test asserts it equals 0xFD2FB528 (line 224), which is the zstd frame magic; `from_bytes` sets `sample_count: 0` and `train` never updates it (52); `compress_with` refuses input larger than `2 * max_out` and returns None when the output exceeds `max_out` (85-93).
Description: In the zstd dictionary format the dictID sits at bytes 4..8 after the 4-byte magic, so `dict_id` is a constant for all trained dictionaries. `sample_count` is always 0. The "determinism" claim (identical dictionary bytes) holds for one zstd build only; the chain would store the recipe, not the bytes.
Impact: A recipe-only dictionary can retrain to different bytes after a zstd upgrade, changing digest and every dependent content id. Unwired.
Known: idle-code-baseline (TenantDictionary)

### P37-F34
File: bud/src/bud3_live_test.rs:26-52, 54-99
Severity: Low
Axis: test
Confidence: Certain
Evidence: `R3Recipe::produce(original, mime, |d| zstd..., b"qr-derivative-bytes")` (line 28-36); `c.compress(d).unwrap_or_else(|_| d.to_vec())`; the PNG test asserts the dimensions read from the fixture's own header; `all_three_editions_exist_in_the_code` only constructs values.
Description: The "LIVE END-TO-END EXPERIMENT" passes a constant byte string as the QR derivative and never calls the carousel, so the QR commitment is not exercised; the dimension asserts check the fixture, not the chain; the final test is a constructor smoke test. Gated by `cfg(all(test, feature = "bud3"))` and run in CI (ci.yml:601).
Impact: The test does not catch a break in the QR derivative path (compare the open K1 gaps in STATUS.md section 4.4).
Known: STATUS.md section 4.4 (partly: production call compares output with itself)

### P37-F35
File: bud/src/bud_format_autozstd.rs; bud_format_av2.rs; bud_format_bloom.rs; bud_format_dbdelta.rs; bud_format_edge.rs; bud_format_crop.rs; bud_format_das.rs; bud_format_dictionary.rs; bud_format_culling.rs (public items)
Severity: Info
Axis: dead code / integration
Confidence: Certain
Evidence: `rg -n 'bud_format_autozstd|bud_format_av2|bud_format_bloom|bud_format_dbdelta|bud_format_edge' bud/src bud/tests bud/fuzz` returns only the `pub mod` lines. idle-code-baseline.txt lists their public items (lines 54-62, 70-75, 88-117, 162- and others). CropDerivation, DasProof, DasSampler, DasOwnership and TenantDictionary are re-exported in lib.rs and used only by tests.
Description: Nine modules have no production caller inside bud-core, on top of F01. bud_format_av2.rs records an external announcement (AV2 spec released 2026-05-28) and a target ratio, with no code path; bud_format_dbdelta.rs `dbd_digest` hashes pages without framing, so [ab][c] and [a][bc] share a digest.
Impact: Debt already recorded by the idle-code ratchet. Listed so the owner decides wire or delete per module.
Known: idle-code-baseline

### P37-F36
File: bud/src/bud_format_autozstd.rs:27-59; bud/src/bud_format_av2.rs:37-58
Severity: Info
Axis: invariant (Z8 note)
Confidence: Certain
Evidence: `decide(fast_ratio: f64, slow_ratio: f64, ...)` branches on f64; `autoz_digest` hashes `fast.to_le_bytes()`; `av2_digest` hashes `0.30f64.to_le_bytes()`.
Description: Decisions and digests are computed from measured f64 values. Fine for a local heuristic; if a digest of a measured ratio were ever compared across nodes it would need the integer path used in bud_fixed_point.rs.
Impact: None today (unwired, outside the Z8 directories).
Known: idle-code-baseline
