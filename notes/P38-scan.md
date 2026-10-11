# P38 scan: B.U.D. 2 (read-only audit, Tier R2)

Base commit: 1b92a08. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md sections 2 and 4.4, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt. Also not repeated: notes/P37-scan.md F01 (bud-core has no consumer in the node), F02 (v1 parity is XOR constant), F15 (columnar not verified by engine_store), F20 (4 GiB per-chunk decompress cap, also in huffman).

## COVERAGE
Read in full, every line, tests included (files 21 to 46 of `ls bud/src/*.rs`; 26 files, 8080 lines by wc, the plan says about 7869):
- bud/src/bud_format_edition.rs 1-206
- bud/src/bud_format_encpact.rs 1-98
- bud/src/bud_format_engine.rs 1-873
- bud/src/bud_format_erasure.rs 1-353
- bud/src/bud_format_exepdf.rs 1-451
- bud/src/bud_format_fastcdc.rs 1-324
- bud/src/bud_format_fidelitygate.rs 1-102
- bud/src/bud_format_fountain.rs 1-312
- bud/src/bud_format_genomic.rs 1-159
- bud/src/bud_format_governance.rs 1-200
- bud/src/bud_format_guardian.rs 1-278
- bud/src/bud_format_hardening.rs 1-223
- bud/src/bud_format_huffman.rs 1-447
- bud/src/bud_format_hw.rs 1-161
- bud/src/bud_format_integration.rs 1-208
- bud/src/bud_format_jpegre.rs 1-271
- bud/src/bud_format_lazyrepair.rs 1-135
- bud/src/bud_format_logfield.rs 1-538
- bud/src/bud_format_lowrank.rs 1-255
- bud/src/bud_format_lrc.rs 1-241
- bud/src/bud_format_markdown.rs 1-452
- bud/src/bud_format_matrix.rs 1-424
- bud/src/bud_format_media.rs 1-133
- bud/src/bud_format_model.rs 1-393
- bud/src/bud_format_msr.rs 1-348
- bud/src/bud_format_multifile.rs 1-495
Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md 110-165; notes/P29-scan.md 1-30; notes/P37-scan.md (rg of headers, F01, F02, F15, F20 text).
Outside the part (rg first, at most 80 lines): bud/Cargo.toml 1-140; bud/src/lib.rs (rg of mod and pub use lines); bud/src/bin/bud.rs 520-560, 575-640 and rg; bud/src/bud_format_pact.rs 62-98; bud/src/bud_format_production.rs 41-63; bud/src/bud_format_container.rs 138-144, 340-390, 555-580; bud/src/bud_format_catalog.rs 318-345; bud/src/bud_format_economics.rs 193-215; bud/src/bud_format_real.rs 40-75; .github/idle-code-baseline.txt and dead-pub-api-baseline.txt (rg only); docs/audit/FINDINGS.md (head and rg only).
Not read: the rest of bud/src (columnar, container beyond the ranges, pipe, zkbridge, catalog), bud/tests, bud/fuzz. No cargo, no test run; every claim comes from reading source, plus two small python3 computations (gear table duplicates, matrix price table), noted where used.
Extra-check summary:
- Z3: no allow attribute, no ignore, no cfg_attr in the 26 files (rg). Z7: no unsafe use; every file except bud_format_lowrank.rs has forbid(unsafe_code), lowrank is covered by bud/Cargo.toml lints.rust unsafe_code = forbid. Z9: no key, seed or 64 hex pattern. LABEL-12.9: no to_bytes with unwrap_or_default. No unwrap, expect or panic in production code of the part (all hits are under cfg(test)); the panic found is an array index (F13).
- Z8: f64 is used in 18 of the 26 files, mostly in digests and blobs (list in F30). None of it is in the Z8 directories and bud-core is not linked into the node (P37-F01), so Info only.
- Erasure and repair: CauchyMds construction is correct (x_i = i, y_j = p + j, disjoint, MDS), the shard pack has the weak points in F01, F03, F20. Content id binding: engine PACT and production record are not bound to each other (F18), multifile delta has no base or result binding (F23).
- Bounds: logfield encode allocates before its line cap (F04); huffman decode, markdown, model, logfield, exepdf, lrc blob parsers are bounded by input length; lt_encode and lowrank dimensions are not (F14, F21).
- Unwired: see F02 (production call site check with rg over bud/src, bud/src/bin, and the root crate).

## Findings

### P38-F01
File: bud/src/bud_format_engine.rs:208-248 (open_shard_pack), 201-207 (contract), 177-199 (pack_shards)
Severity: Medium | Axis: correctness | Confidence: Certain
Evidence:
```
217:    for index in 0..6 {
218:        if pack.len() < pos + 32 + 4 {
219:            return None;
220:        }
...
225:        if pack.len() < pos + len {
226:            return None;
227:        }
234:    if pos != pack.len() || survivors.len() < 4 {
```
Description: The pack parses the six records one after the other and trusts each record's own len field to find the next. The 10 byte header (k, p, original_len) has no digest and no copy. A missing tail (partial write, truncated file) or one flipped bit in any len field returns None for the whole pack, although four or five shards are intact and all record offsets could be computed from original_len (every record is 36 + shard_len bytes). The doc comment promises recovery from "any four survivors" including a shard "truncated to nothing"; the tests only change bytes in place or empty a record and fix up the length consistently (engine.rs:668-714).
Impact: A (4,2) erasure pack that is advertised to survive two lost shards is lost completely on the two most common storage faults (truncation and a damaged length or header field). Anyone who sets erasure on engine_store for durability gets less than the stated protection. Known: none (P37-F02 is the v1 container parity, a different code path)

### P38-F02
File: bud/src/lib.rs:24-47, 127-149 (module list and re-exports); modules listed below
Severity: Medium | Axis: integration | Confidence: Certain
Evidence: `rg -n 'bud_format_<m>' bud tests fuzz -g '*.rs'` per module, excluding the module itself: hits only in bud/src/lib.rs (and Cargo.toml feature text) for encpact, exepdf, fidelitygate, fountain, genomic, governance, guardian, hardening, integration, jpegre, lazyrepair, lowrank, lrc, markdown, model, msr. edition: bud/src/bud3_live_test.rs:90-97 only (a test file). hw: bud_format_serviceclass.rs:40 is a comment ("the bud_format_hw mapping"). matrix: bud_format_economics.rs:142 is a comment. media: bud_format_economics.rs hit is a comment. The bud binary uses only engine (bud.rs:28, 594, 613), multifile (bud.rs:29, 533) and zkbridge.
Description: 19 of 26 modules (encpact, edition, exepdf, fidelitygate, fountain, genomic, governance, guardian, hardening, hw, integration, jpegre, lazyrepair, lowrank, lrc, markdown, matrix, media, model, msr) have no production call site, not even in the bud CLI. Inside the rest: engine_store_fcdc, engine_store_tiered, engine_restore, engine_restore_full and engine_restore_raw have no caller outside tests and lib.rs (rg), so FastCDC chunking is unreachable from the CLI; multifile is used only for add_file, dedup_ratio and to_blob (delta, restore, from_blob unused in bud.rs).
Impact: The gates in this part (fidelity gate, recipe gate, spam gate, guardian audit, grant state machine, honesty canaries, edition choice) do not run anywhere. Per CLAUDE.md "everything coded must be wired" each is a mainnet blocker if any claim relies on it. The crate as a whole is unwired (P37-F01). Known: P37-F01; .github/idle-code-baseline.txt lists symbols of every module here (for example 16 for matrix, 15 for guardian, 13 for governance, 12 for hardening and integration)

### P38-F03
File: bud/src/bud_format_integration.rs:99-123 (BudErasure::encode), 87-93, 141-147, 154-159, 62-67
Severity: Medium | Axis: correctness | Confidence: Certain
Evidence:
```
110:        // Cauchy MDS stub: parity = XOR of data shards with different patterns
114:            for (j, shard) in data_shards.iter().enumerate() {
115:                let pattern = ((i + j) % 256) as u8;
116:                for (o, s) in p.iter_mut().zip(shard.iter()) {
117:                    *o ^= s ^ pattern;
```
Description: Every parity shard is (XOR of all data shards) XOR a per-row constant, so any two parity shards differ by a constant byte and carry the same information. With p = 3 (the 18+3 layout in the test) the code tolerates one loss, not three, while expansion() reports (k+p)/k. There is no decode. The test only checks lengths. k_bud_erasure only compares a count with k. k_bud_living_threshold tests `tier > 2` but tier() returns only 0, 1 or 2, so the gate cannot fail. required_replicas gives 9 for the middle tier and 2 for the coldest, and decide() text says "cold (18+3)" for the same tier.
Impact: Anyone using this type to place shards believes in k-of-n repair that does not exist. Same class as P37-F02 in a second module. Known: unwired status in idle-code-baseline (struct BudErasure, IntegrationGates, others); the XOR-parity defect itself is new here

### P38-F04
File: bud/src/bud_format_logfield.rs:135-141 (encode), 26; catalog trigger bud/src/bud_format_catalog.rs:325-345; call bud/src/bud_format_engine.rs:322
Severity: Medium | Axis: security | Confidence: Likely (missing step: run `bud engine` on a 200 MB file of newlines)
Evidence:
```
136:        let mut lines: Vec<Vec<u8>> = Vec::new();
137:        for line in data.split_inclusive(|&b| b == b'\n') {
138:            lines.push(line.to_vec());
139:        }
140:        if lines.is_empty() || lines.len() > MAX_LINES {
```
Description: One heap Vec per input line is built first, and MAX_LINES (10 million) is checked after the loop. engine_store accepts up to 512 MiB; catalog_detect classifies any text whose first line starts with four digits as LOG, which engine maps to the logfield path. A file made of a first line "2026-..." followed by newline bytes yields one Vec (24 bytes plus a malloc block) per byte. 200 MB of newlines is about 10 GB before the cap refuses it; parse_nginx_line then fails on the first non-nginx line only after the allocation.
Trigger: precondition: engine_store on attacker supplied bytes (the bud CLI engine command, bud.rs:594). Entry: LogFieldColumnar::encode at engine.rs:322. Input: leading digits line plus many empty lines. Expected result: memory exhaustion; Cargo.toml sets panic = abort.
Impact: One small crafted file kills the process that stores it. Local CLI today; remote if the engine is ever wired to peer data. Known: none

### P38-F05
File: bud/src/bud_format_hw.rs:60-67, 94-107, 124-138; same constant bud/src/bud_format_economics.rs:197-198
Severity: Medium | Axis: correctness | Confidence: Certain
Evidence:
```
hw.rs:62:        usd_per_tb_month: 0.00025,
economics.rs:196: /// LTO-9 is about 5 USD/TB (30 years); ...
economics.rs:197: /// 0.003 USD/GB/year = 0.00025 USD/TB/month.
```
Description: 0.003 USD per GB per year is 3 USD per TB per year, which is 0.25 USD per TB per month, so the stated conversion is wrong by a factor of 1000. The other stated source ("about 5 USD/TB over 30 years") gives 5 / 360 = 0.0139 USD per TB per month, still about 55 times higher than 0.00025. The tape tier is the only one that makes `cheapest_tier(0.016, true)` and the test `tape_is_under_the_0_016_ceiling` pass. With the first source the tape would be 15 times above the ceiling; with the second it is just under (0.0139).
Impact: The 0.016 target for the archive class rests on a constant whose own comment does not support it. Also cheapest_tier can pick M-Disc and Silica-glass, whose notes say "future" or "a read-write drive is required", and has no availability flag; DNA is excluded only by price. Known: none

### P38-F06
File: bud/src/bud_format_matrix.rs:25-31, 301-324, 332-354, 380-391; engine path bud/src/bud_format_engine.rs:177-179
Severity: Medium | Axis: correctness | Confidence: Certain (python3 recomputation of the table, bud_format_matrix.rs constants only)
Evidence:
```
29: pub const LRC_ERASURE: f64 = 1.031; // measured LRC
317:        PHYSICAL_USD_PER_TB_MONTH * LRC_ERASURE / r
390:        assert_eq!(passing, 30);
engine.rs:178:    let mds = CauchyMds::new(4, 2)?;
```
Description: The "30 of 30 classes hold 0.016" claim uses an erasure multiplier of 1.031 from a local reconstruction code. The lrc module has parameter arithmetic only (multiplier, repair_reads, a record blob) and no encoder or decoder, and the only erasure the engine writes is (4,2), a 1.5x multiplier (erasure.rs:246-248). Recomputing the table: at 1.031 all 30 pass; at 1.1667 (the 18+3 of container v1) 21 pass; at the engine's real 1.5 only 12 pass; with single file ratios only (no dedup, culling or copy multipliers) 22 of 30 fail even at 1.031. matrix_honesty_check has no arm for multiplier_kind "copy" (lines 334-352 fall to `_ => {}`), so the multipliers 2.0, 4.0, 5.0 and 3.0 of bmp, tiff, png, jpeg and wav are never bounded, and the check and its test only walk a static table.
Impact: The central economic claim of B.U.D. 2.0 cannot be reproduced by the shipped pipeline. The honesty canary does not cover one multiplier kind. Not a code bug in a consensus path; a claim that must not be repeated in public text. Known: none (P37-F26 and F27 cover economics.rs, not this table)

### P38-F07
File: bud/src/bud_format_huffman.rs:35-91 (compress), 193-257 (lengths_by_freq), 138-141 (decompress limit), 28
Severity: Medium | Axis: correctness | Confidence: Likely (missing step: compress a 15 MB buffer with Fibonacci byte counts and call decompress)
Evidence:
```
28: pub const MAX_CODE_LEN: usize = 32;
139:        if max_len == 0 || max_len > MAX_CODE_LEN {
```
Description: compress builds an unrestricted Huffman tree and writes the code lengths as u8; it never limits them to 32. decompress refuses any table with a length above 32. A Huffman tree reaches depth 33 once symbol counts follow Fibonacci numbers over 34 symbols, a total of about 15 million bytes in one call (sum of the first 34 Fibonacci numbers is F(36) - 1 = 14,930,351). Then compress succeeds and decompress returns None. No round trip check exists in compress. Container codec Huffman is reached from bud_format_pipe.rs:115 (new_compressed), where one piece can be larger than the 16 KB default ("a large piece goes straight through", container.rs:567-569).
Impact: A stored Huffman chunk built from adversarial or unlucky bytes is permanently unreadable. The default path is zstd, so the exposure is the pipe command and any Huffman container. Known: none (P37-F20 is about the decompress ceiling)

### P38-F08
File: bud/src/bud_format_markdown.rs:72-80, 64-65
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
72:            } else if let Some(stripped) = line.strip_prefix('#') {
74:                let depth = line.len() - stripped.len();
75:                if depth <= 6 && stripped.starts_with(' ') {
```
Description: strip_prefix('#') removes exactly one character, so depth is always 1. For "## Title" the remainder is "# Title", which does not start with a space, so the line becomes MdSection::Other. Only level-1 headings are recognised, heading_tree contains only H1 lines, and Heading(2..6) can never be produced. The test md_structural_parse checks only Heading(1).
Impact: The "heading hierarchy" LLM view and context_ratio are computed on H1 only; the section type stream is wrong for every real document. Known: none

### P38-F09
File: bud/src/bud_format_markdown.rs:64, 120-126, 11-19
Severity: Medium | Axis: correctness | Confidence: Certain (std documents that lines() drops the CR of a CRLF ending)
Evidence:
```
64:        for line in md.lines() {
121:        let mut out = self.contents.join("\n");
```
Description: str::lines splits on "\n" and on "\r\n" and removes the line ending, so a CRLF document is stored without its CR and decode joins with "\n". The module states byte for byte losslessness three times and the test list has no CRLF case. encode does not compare decode() with the input before returning.
Impact: A Windows markdown file stored through this transform comes back changed, silently, and its content id differs. Unwired today (F02). Known: none

### P38-F10
File: bud/src/bud_format_fidelitygate.rs:39-72
Severity: Medium | Axis: security | Confidence: Certain
Evidence:
```
41:        "avif" | "jxl" | "webp" | "jpeg" => match crf {
44:            None => LossyKind::None,
51:        _ => LossyKind::None,
```
Description: The gate admits LossyKind::None ("lossless") for: any kind string it does not know ("AVIF" in capitals, "av1", "heic", a typo), a known lossy kind with no crf, and a zfp or sz kind with no rel_error. A negative rel_error also passes (-5.0 <= 1e-3). Only a measured bad value is refused. The module comment says every lossy transform must carry its measurement and be refused or recorded otherwise. fidelity_digest encodes missing values as u32::MAX and f64::MAX, so Some(u32::MAX) collides with None, and kind is hashed without a length prefix.
Impact: The control that is supposed to keep lossy transforms inside bounds fails open on missing or unusual input. Unwired (F02), so Medium as a mainnet part. Known: none

### P38-F11
File: bud/src/bud_format_media.rs:72-81
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
78:        Some(b) => claimed <= b.measured_ratio * tolerance.max(1.0),
79:        None => true, // an unknown measurement admits no claim; the caller must REFUSE
```
Description: The comment says an unknown name admits no claim, the code returns true. The tolerance argument is caller supplied and unbounded (infinity admits every claim). The BMP and TIFF rows share one measured value (15.84) and the JPEG to JXL row (1.56x, 36 percent saving) contradicts the 15 to 35 percent formula in jpegre (F27).
Impact: The K19 honesty canary admits unmeasured claims and large tolerances. Known: none

### P38-F12
File: bud/src/bud_format_msr.rs:126-142 (msr_decode), 133, 135-140
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
130:    let (i, a) = nodes[0];
133:    let r = |idx: usize| PSI[idx];
135:    let (r0, r2, r1) = (r(i)[0], r(i)[2], r(i)[1]);
```
Description: msr_decode takes node indices from the caller and indexes PSI (4 rows) without a bound check; an index of 4 or more panics. Duplicate indices return None. The crate sets panic = abort. Also gf_mul and gf_inv rebuild the 512 + 256 byte tables on every call (msr.rs:35, 57, 69), and encode/decode work on 4 bytes per call with no framing for data of any other length.
Impact: A panic from a bad index in a public function; unwired (F02), so Low.
Trigger: entry msr_decode(&[(7, [0, 0]), (1, [0, 0])]) at msr.rs:126.
Known: none

### P38-F13
File: bud/src/bud_format_fountain.rs:81-153 (lt_decode), 91-96, 170-181 (lt_digest)
Severity: Medium | Axis: correctness | Confidence: Certain
Evidence:
```
92:        for &i in chosen {
93:            if i < 64 {
94:                mask |= 1u64 << i;
95:            }
96:        }
```
Description: A symbol that lists an index in k..63 puts a bit in its row that no column ever eliminates, so the recovered block carries that foreign contribution; an index of 64 or more is dropped from the mask while the symbol data still contains that block, so the equation is wrong. Both give Some(wrong blocks) instead of None. There is no check that indices are below k or distinct or sorted, and no per-symbol or per-block integrity check; lt_digest is never compared (and has no framing for k or the index count, so different symbol sets can produce the same byte stream).
Impact: Corrupt or forged symbols produce silently wrong data. On an erasure channel (the stated use) the receiver cannot tell recovery failed. Known: none

### P38-F14
File: bud/src/bud_format_fountain.rs:25-35, 49-67
Severity: Low | Axis: security | Confidence: Likely (missing step: call lt_encode with n = usize::MAX / 8)
Evidence: `let mut out = Vec::with_capacity(n);` with n a caller value and no ceiling (line 35); only the block count is bounded (LT_MAX_BLOCKS = 16).
Description: n has no upper bound, so a huge n aborts on capacity overflow or allocation failure (panic = abort), and n times the block length is allocated without a cap. The generator is a 64 bit LCG used modulo small numbers (`% 7`, `% k`), whose low bits have very short periods; the decode success rate in the tests (measured in comments only for 8 blocks) is therefore specific to this generator.
Impact: Panic or memory exhaustion from a caller value. Unwired. Known: none

### P38-F15
File: bud/src/bud_format_lrc.rs:47-57, 66-78, 108-117
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
77:        Some(shard / (self.k / self.l).max(1))
```
Description: Shards are k data shards then l local parities. For k = 2000, l = 50 the group size is 40, shard 2000 (the first local parity) maps to group 50 and shard 2049 to group 51, both outside 0..l-1, although the doc says "for parity shards this is the group they represent"; the tests check only shard 0, 41 and 2100. When k is not a multiple of l the integer division makes repair_reads too small for the larger groups. record_hash and to_blob cast k, l, g to u32, so values that differ by 2^32 hash the same, and new() has no upper limit.
Impact: Wrong group for every local parity. Unwired and no codec exists (F06). Known: none

### P38-F16
File: bud/src/bud_format_lazyrepair.rs:18-64
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
41:    if lost >= tolerated {
44:        return Some(RepairAction::RepairNow { helpers: 2 });
```
Description: RebuildFromScratch is declared and never returned. When lost > tolerated the data is already beyond what the code can repair, yet the answer is RepairNow with 2 helpers. `until_epoch` in Defer holds an age threshold, not an epoch. The function returns Option but never None. A NaN budget gives a threshold of 1000. repair_digest hashes the f64 budget bytes.
Impact: A caller that follows the decision repairs data that cannot be repaired and never takes the rebuild path. Unwired. Known: none

### P38-F17
File: bud/src/bud_format_engine.rs:305-321
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
309:        (FormatCodec::Json, _) => {
310:            steps.push(PipeStep::Transform);
311:            match crate::bud_format_columnar::columnar_encode(
...
319:                None => data.to_vec(),
```
Description: For JSON the Transform step is recorded before the encode result is known; when columnar_encode returns None no transform is applied (transform_kind stays None) but steps and steps_hash still say Transform ran. The Log branch pushes the step only on success. The steps hash is described as the proof of "which transforms were applied".
Impact: The production proof for such a file is wrong. Known: none

### P38-F18
File: bud/src/bud_format_engine.rs:371-372; bud/src/bud_format_pact.rs:68-79
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
engine.rs:371:    let pact = PactRecord::pure([0xE9u8; 32], [0x11u8; 32], &container_final, ts_unix);
pact.rs:73:            commitment: Self::hash_bytes(b"BDLM_PACT_OUTPUT_V1", produced),
```
Description: Every engine result gets PactMode::PureProduction with fixed placeholder producer id and seed, and its commitment is over the stored container bytes (zstd output, possibly the shard pack), not over the original content. The production record binds the original (payload_root = content_id(original)). Nothing links the two records or the container to the original in the blob. Container bytes depend on the zstd build, so the PACT commitment is not stable across zstd versions even for identical content.
Impact: A PACT from this engine says "pure production" for stored data and cannot be checked against the original content id. Known: none

### P38-F19
File: bud/src/bud_format_engine.rs:131-143, 252-280, 468-501, 733-745
Severity: Low | Axis: test | Confidence: Certain
Evidence:
```
741:        let _ = engine_restore_raw(&blob, false);
742:        // a short blob gives None
```
Description: to_blob appends steps_hash, ratio, pact hash and production hash. engine_restore, engine_restore_full and engine_restore_raw parse only the header and the container; the 80 trailing bytes are never read or compared, and the length may exceed the minimum without error. The test engine_restore_rejects_tamper flips the last byte and asserts nothing about the result. engine_restore is documented as returning the original, but for columnar and logfield input it returns the transformed blob (its own comment, lines 275-278); it and engine_restore_raw are duplicates. measured_ratio (f64) is part of the blob that the doc calls writable on chain.
Impact: A tampered blob tail goes undetected and the test suite does not notice. Known: engine_restore, engine_restore_full, engine_restore_raw in idle-code-baseline (idle only)

### P38-F20
File: bud/src/bud_format_erasure.rs:145-179, 85-93; bud/src/bud_format_engine.rs:177-179, 355-362
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
156:        let chosen: Vec<(usize, &Vec<u8>)> = survivors
157:            .iter()
158:            .take(self.k)
...
176:                    self.cauchy(si - self.k, c).unwrap_or(0)
```
Description: decode takes the first k survivors only; a duplicate or bad index among them returns None even if other survivors would work. An index at or above k + p is not rejected: `cauchy` casts `i as u8`, the denominator can be valid, and the decoder solves with a phantom parity row and returns wrong data instead of None; a zero denominator becomes coefficient 0 by unwrap_or(0). MAX_SHARD_BYTES is enforced in encode only. Separately, engine_store with erasure on returns None when the container exceeds 256 MiB (shard_len above 64 MiB), but the CLI reports "invalid input (empty or >512MB)" (bud.rs:595).
Impact: Library misuse yields wrong bytes; the engine only passes indices 0..5, so the engine path is not affected. Known: none

### P38-F21
File: bud/src/bud_format_lowrank.rs:51-130, 11-17, 105-109
Severity: Low | Axis: correctness | Confidence: Likely (missing step: run roundtrip on a rank-2 matrix and compare residual size; the deflation gap is from reading)
Evidence:
```
57:    for k in 0..rank {
58:        let mut b = det_vec(7, c, k);
59:        for _ in 0..8 {
```
Description: Each rank step runs power iteration on the original A with no deflation (A is never reduced by the components already found), so every column converges toward the dominant singular vector and U times V adds about `rank` copies of it; the residual then grows instead of shrinking. The "lossless guarantee" comment (105-109) is contradicted by the header ("accurate to about 1 ULP"): the refinement loop is 4 steps and compares with `==`, so -0.0 versus 0.0 passes while the bits differ, and NaN never converges. u, v and the residual are stored as full f64, so total size exceeds the input. `a.len() != r * c` and later r * rank can overflow usize in release builds. No forbid(unsafe_code) line in this file (covered by Cargo lints).
Impact: Not usable as a lossless or compressing transform; claims in the header cannot hold. Unwired. Known: none

### P38-F22
File: bud/src/bud_format_model.rs:60-120, 169-221
Severity: Low | Axis: correctness | Confidence: Likely (missing step: run on a real little endian BF16 file)
Evidence:
```
86:            // IEEE: b0 = sign + exp[7:1], b1 = exp[0] + mantissa(MSB'ler)
87:            let sign = (raw[off] >> 7) & 1;
```
Description: The split assumes big endian byte order (sign and high exponent bits in the first byte). BF16 and FP32 weights in common model files are little endian, where the first byte holds mantissa bits, so the "exponent stream" is mostly mantissa and the claimed gain does not appear; the transform is still a bijection, so it stays lossless. Test data is generated big endian. Neither the blob nor the API records the byte order. from_blob accepts a rest_bits stream longer than needed (several valid blobs per content) and a count that is checked only at decode time.
Impact: Wrong assumption makes the K96 gain claim unreproducible on real files; unwired. Known: none

### P38-F23
File: bud/src/bud_format_multifile.rs:67-90, 124-190, 193-286, 487-495
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
76:            if let Some(existing) = self.chunks.iter().find(|ch| ch.content_id == cid) {
...
487: #[test]
488: fn oom_chunk_count_is_refused() {
490:     let mut bytes = vec![0u8; 64];
```
Description: (a) The delta carries no digest of the base or of the result: apply_delta against a different base of the same chunk layout returns wrong bytes without error for every referenced chunk. (b) add_file does a linear scan of all chunks per input chunk, enforces neither MAX_MULTIFILE_CHUNKS (1 << 20) nor MAX_FILES, and accepts chunk_size 1, so file_chunks grows 32 bytes per input byte and the scan cost grows with the pool; to_blob then writes a blob that from_blob refuses (counts over the limits), losing the store. (c) saved_bytes is not stored. (d) The top-level test oom_chunk_count_is_refused feeds 64 zero bytes with an invalid digest, so it returns at the digest check and never reaches the chunk_count check it is named for; it asserts nothing (`let _ =`). The bud CLI passes the chunk size from the command line (bud.rs:533-536).
Impact: Silent wrong restore for a mismatched base; resource blow-up and an unreadable blob for large tenants. Known: none

### P38-F24
File: bud/src/bud_format_edition.rs:22-30, 147-154, 119-135
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
25: pub enum Edition { Bud1, Bud2, Bud3 }   // no explicit discriminants
42: pub fn from_u8(v: u8) -> Option<Self> { 1 => Bud1, 2 => Bud2, 3 => Bud3 }
151:        h.update([self.edition as u8]);
```
Description: from_u8 maps 1, 2, 3 to the editions, but `as u8` yields 0, 1, 2, so the digest of an EditionChoice hashes a different number than the one that is chosen and recorded; an outside verifier that hashes the wire byte gets another digest. Both digests use ED_MAGIC with no per-use tag. bud1_digest concatenates id, content_uri and server with no length prefix, so a uri ending in a zero byte and a shifted server string can collide.
Impact: On-chain edition digests are not reproducible from the stored byte. Unwired. Known: none

### P38-F25
File: bud/src/bud_format_guardian.rs:68-87, 93-96, 129-137
Severity: Low | Axis: security | Confidence: Certain
Evidence:
```
86:    Some((idx, tour))     // tour is the constant DV_N
131:    if audit.shard_ok && audit.signed {
```
Description: select_guardian is labelled commit-reveal, but it only hashes seeds the caller passes in; there is no commit, no check that a seed matches a prior commitment, and the last party to reveal can try seed values and pick the selected index (grinding). The returned "round count" is always 26. audit_mini trusts two plain booleans: no signature is verified, and device_id and pact_id in the audit are never compared with the Guardian, so an audit for another device or content counts for or against this one.
Impact: Guardian selection can be steered by the last revealer, and audit results are asserted rather than proven. Unwired. Known: none

### P38-F26
File: bud/src/bud_format_governance.rs:56-62, 120-130; bud/src/bud_format_hardening.rs:48-53, 77-89, 134-139
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
governance.rs:56: pub fn activate_after_production(grant: &mut PollenGrant, produced_ok: bool) {
hardening.rs:52:     fee_usd < quota.min_creation_fee_usd
```
Description: Y7 says a grant becomes Active only after PACT production verification passes, but the function takes a bool and ignores pact_id. Refunded keeps payment_locked unchanged and nothing says where the amount goes. wiring_inventory ignores its module argument and trusts call_count and verifies from the caller, so the "inventory" is self declared. spam_check returns true to refuse: a NaN fee compares false and passes. recipe_is_unfabricable_canary hashes a fixed list of guesses under another domain than content_id, so it can only return false on a SHA3 collision; its test cannot fail. sert_digest hashes only magic and version. Money is f64 (USD) throughout hardening.
Impact: Gates do not enforce what their documentation says. Unwired. Known: none

### P38-F27
File: bud/src/bud_format_jpegre.rs:113-132, 134-183
Severity: Low | Axis: correctness | Confidence: Certain
Evidence:
```
118:        let recompress_savings_pct = 15.0 + header_ratio * 20.0;
131:        self.recompress_savings_pct >= 10.0
```
Description: The module says it MEASURES how compressible the JPEG is, but the saving is a formula of the header share (always between 15 and 35), not a measurement, and recommends_jxl is true for every parsed file. This conflicts with the measured JPEG to JXL value (1.56x, media.rs:53-54). from_blob accepts any f64 (NaN, infinite) for the saving. The parser itself is bounded: all indexing is guarded and no panic path was found.
Impact: The "decision record" has no information content. Known: none

### P38-F28
File: bud/src/bud_format_fastcdc.rs:26-155, 169-226, 198-202
Severity: Low | Axis: correctness | Confidence: Certain (python3 count on the table)
Evidence: the table has 128 entries of which 91 are distinct (37 values repeat; for example 0x3f7a9c2e5b8d6a41 at lines 58, 78 and 118) and 42 are even; the lookup is `GEAR_TABLE[(data[i] & 0x7F) as usize]` (line 200), so bytes b and b + 128 share a multiplier.
Description: The hash is `rotl(h,1) * G + b`, not the shift-add Gear hash of FastCDC, and has no normalized chunking or minimum skip after the first chunk. Weak, repeated constants reduce boundary entropy. Each chunk is copied twice (`c.clone()` at 188, 206, 216). Boundaries stay deterministic and the split is lossless, and the engine zips chunk_ids into StructuralChunk, but new_zstd then replaces them with ids of the compressed bytes (container.rs:381-383), so chunk_ids are unused.
Impact: Dedup anchors are weaker than the "FastCDC" label suggests; FastCDC is unreachable from the CLI anyway (F02). Known: engine_store_fcdc in idle-code-baseline

### P38-F29
File: bud/src/bud_format_exepdf.rs:7-13, 66-103, 128-148
Severity: Info | Axis: correctness | Confidence: Certain
Evidence: header lines 7-13 describe per section splitting of PE and ELF; lines 96-102 split at `(data.len() * 3) / 5`, and `kind` is never used after being set.
Description: The EXE transform is a fixed 60/40 byte split with no section parse; the code comment (81-95) says so, the header does not. The PDF split is lossless by construction (gap offsets are recorded and checked in decode and from_blob) and bounded (n at most 1,000,000, input at most 256 MiB, offsets monotonic); no issue found there except that a stray "stream" word outside a real stream pairs with the next "endstream" and widens a body.
Impact: Documentation mismatch only. Known: none

### P38-F30
File: 18 files: matrix (12 hits), lowrank (20), hardening (9), guardian (5), fidelitygate (5), hw (4), engine (4), jpegre (3), plus markdown, lazyrepair, multifile, fastcdc, logfield, integration, media, lrc, erasure, model (1 to 2 each)
Severity: Info | Axis: invariant(Z8) | Confidence: Certain (rg `\bf(32|64)\b`)
Evidence: digests and blobs that are described as writable on chain include f64 values: engine.rs:139 (measured_ratio), jpegre.rs:144, lazyrepair.rs:73, hw.rs:119, matrix.rs:374, media.rs:90, fidelitygate.rs:70, guardian.rs production_vs_por_ratio.
Description: None of these directories is in the Z8 list and bud-core is not linked into the node, so Z8 is not violated today. If any digest is ever consumed on chain, the float inputs must become integer or fixed point (bud_fixed_point.rs exists).
Impact: Future constraint only. Known: none

### P38-F31
File: bud/src/bud_format_genomic.rs:42-99; bud/src/bud_format_encpact.rs:1-59
Severity: Info | Axis: test | Confidence: Certain
Evidence: genomic: ref_decode and two_bit_decode bound all indexing (`data.len() < n.div_ceil(4)` check at 90); encpact: class_for_decl returns string constants and declaration_bound is a plain hash.
Description: Both files were read in full and no defect was found. Notes: two_bit_decode ignores bytes beyond ceil(n/4) and padding bits (several valid encodings per sequence); genomic accepts only upper case A, C, G, T, so soft masked (lower case) FASTA is refused whole; encpact states honestly that the encryption flag is a declaration. Both are unwired (F02).
Impact: None. Known: none
## Counts
Findings: 31. Critical 0, High 0, Medium 10, Low 18, Info 3.
Medium: F01, F02, F03, F04, F05, F06, F07, F09, F10, F13. Low: F08, F11, F12, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28. Info: F29, F30, F31.
