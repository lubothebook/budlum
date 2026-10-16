# P39 scan: B.U.D. 3 (bud-core files 47 to 76, read-only audit, Tier R2)

Base commit: 1b92a08. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md sections 2 and 4.4, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt. The idle-code baseline (.github/idle-code-baseline.txt) lists the public items of every file in this part, so pure dead-code points are marked Known there. P37-F01 (bud-core is linked by nothing in the node) and P37-F20 (4 GiB zstd ceiling per chunk) are referenced, not repeated.

## COVERAGE

The part is `ls bud/src/*.rs`, entries 47 to 76 (30 files, 7310 lines by `wc -l`; the brief said about 7764). All read in full, every line, tests included (all under bud/src/, prefix bud_format_, line ranges 1-N): nvc 160, office 403, optical 126, optical_transfer 206, pact 390, pcap 422, pgm 191, pipe 324, pointcloud 159, por 197, prodcost 182, production 259, qos 85, qrmatrix 219, qrvideo 596 (two reads), r3fix 268, ratioconsensus 383, real 224, realcorpus 104, recipe 165, recipe_record 253, regeneration 319, registry 279, repairband 106, revolutionary 592, scrub 85, secure_db 88, securededup 136, segment 296, serviceclass 93. Exact paths: bud/src/bud_format_<name>.rs for each of these 30 names.

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md 110-165 (sections 5 and 6); notes/P29-scan.md 1-30.
Outside the part (rg first, at most 80 lines each): bud/Cargo.toml 50-160; bud/src/lib.rs 195-225 and `pub mod` lines; bud_format_container.rs content_id (138-144) and FormatCodec::from_u16; bud_format_ux.rs qr_capacity_bytes head; bud_format_spec.rs 76-135; notes/P37-scan.md (F01, F20); .github/workflows/ci.yml 574-625; .github/idle-code-baseline.txt (rg only). Callers located with rg over bud/src, bud/tests, bud/fuzz, src, budzero, crates, xtask.
Not read: bud_format_container.rs, bud_format_columnar.rs, bud_format_engine.rs, bud_format_block.rs, bud_format_ux.rs beyond the ranges above (other parts). The real erasure coder (Cauchy, GF(2^8)) is not in this part: the only erasure-related code here is the fountain code in bud_format_qrvideo.rs and the model numbers in bud_format_repairband.rs and bud_format_prodcost.rs. No cargo, no test run: every claim comes from reading source and tracing by hand.

Extra-check summary:
- Erasure and repair correctness: the qrvideo fountain decoder (peeling plus GF(2) elimination) is arithmetically sound (hand-checked pivot and reduction loop) but has no payload integrity (F01), no input bound (F02) and no self-describing stream (F03). repairband has wrong formulas (F22).
- Content id binding: scrub uses a different hash domain than content_id (F08); three unrelated commitment schemes for the same word (F09); optical join has no whole-content digest (F15); gates that do not bind (F13).
- Bounds: segment, pact, production, ratioconsensus, regeneration blob parsers check lengths before slicing; office, pcap restore use `.get` or checked_add. Asymmetric ceilings found (pcap F06, optical F15).
- Determinism (Z8): no HashMap iteration in encoded output (registry pin sorts keys). f64 appears in 91 lines of 15 files; the record hashes that include f64 bytes are listed in F26.
- Panics from external bytes: every unwrap and expect hit in the 30 files is under #[cfg(test)] (rg checked). No index or slice panic path found in production code of the blob parsers (checked each `bytes[...]` against its length guard). qrvideo `add_frame` indexes `frame[10..20]` after a `len < 20` guard. The bud profile sets panic = "abort" (bud/Cargo.toml:98-101), so any allocation failure is a process abort.
- Unwired parts: 19 of the 30 modules have no caller outside lib.rs and their own file; see F27.
- Z3: one `#[allow(dead_code)]` (realcorpus.rs:13). No `#[ignore]` and no `|| true` in the part. Z7: no `unsafe`; optical_transfer.rs lacks the file-level forbid (crate lint forbids it anyway). Z9: one documented example key shape in a test fixture (F29).

Counts: 29 findings: Medium 17 (F01-F17), Low 11 (F18-F28), Info 1 (F29). No Critical, no High.
## FINDINGS

### P39-F01
File: bud/src/bud_format_qrvideo.rs:250-264, 267-288, 290-314, 371-385, 218-222, 553-566
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
qrvideo.rs:218-222   /// If the rank is insufficient it returns None - it NEVER produces wrong data.
qrvideo.rs:260-262   let payload = &frame[DAMLA_HDR_LEN..];
                     let idx = Karusel::composition(self.k, seq, flags & 0x01 != 0);
                     self.add_drop(&idx, payload);
qrvideo.rs:293-296   if self.solved[b].is_some() { continue; }
                     self.solved[b] = Some(w.clone());
Description: The decoder checks only the 10-byte constant and the length of a frame. The payload is XORed in with no per-frame checksum, no content id and no final hash, and the first value for a block wins while a later conflicting value is dropped silently. `assemble` returns whatever was solved. The module header (lines 7-10) promises "verify SHA" at the end; nothing implements it, and the only negative test (lines 553-566) corrupts the magic and the length, never a payload byte.
Impact: One flipped or hostile payload byte in any frame (camera noise past the QR layer, a malicious screen) yields a wrong body that `assemble` returns as success; a poisoned repair frame spreads the error into every block it resolves. Any caller that trusts the "never wrong data" comment stores corrupt content.
Trigger: precondition: an attacker or a noisy channel supplies one frame with a valid header; entry bud_format_qrvideo.rs:250 `add_frame`; input: a systematic frame for block i with a wrong payload sent before the real one; result: `assemble` (371) returns data differing from the original.
Known: none (STATUS.md 4.4 covers the root crate QR modules, not this one)
### P39-F02
File: bud/src/bud_format_qrvideo.rs:267-288, 318-343
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
qrvideo.rs:285-286   1 => self.resolve(rem[0], pay),
                     _ => self.pending.push((rem, pay)),
qrvideo.rs:323-329   for (idx, pay) in &self.pending {
                         let mut mask = vec![0u64; words];
qrvideo.rs:335-339   rows.iter().enumerate().find(|(ri, (m, _))| m[w] & bit != 0 && !piv_rows.contains(ri))
Description: `pending` has no cap and no de-duplication, so every accepted repair frame adds a row. `eliminate` (called by every `assemble`) clones all rows into bitmask rows of k/64 words (8 KiB at k = 65535) and runs one linear scan with a linear `contains` per unknown column, so time is about unknowns x rows x pivots and memory is rows x (k/8 + 200) bytes. `new` allows k up to 65535 (line 232).
Impact: A frame stream of a few hundred thousand cheap repair frames makes `assemble` request gigabytes and spin for a long time; under panic = "abort" an allocation failure ends the process. Local or camera-side DoS for any receiver.
Trigger: precondition: none beyond a valid 10-byte frame constant; entry add_frame (250) then assemble (371); input: 10^6 distinct repair seq values with k = 65535; result: allocation of about 8 GB in `eliminate`.
Known: none
### P39-F03
File: bud/src/bud_format_qrvideo.rs:1-11, 169-203, 132-146, 402-409
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
qrvideo.rs:8-10   //!   payload -> zlib-9 (ONLY if it shrinks) -> container (magic, version, flags, orig_len, sha256)
                  //!   ... RECEIVE: decode frame -> drop pool -> peel -> decompress -> verify SHA.
qrvideo.rs:191    out.extend_from_slice(&k.pack(seq, (t % 2) as u16, 0, &b)?);
qrvideo.rs:199    out.extend_from_slice(&k.pack(seq, (t % 2) as u16, 0x01, &b)?);
Description: `derive_stream` writes frames with flags 0 or 0x01 only. The compressed flag 0x02 and the last flag 0x04 are never set, there is no container header, no k, no total_len, no hash. `KaruselDecoder::new(k, total_len)` needs k and total_len from outside, and nothing tells the receiver whether the body is zstd or raw (line 176 picks raw when zstd does not shrink). QRV_MAGIC and QRV_VERSION are used only inside `qrv_digest`. The comment says zlib-9; the code is zstd-19.
Impact: A stream produced by `derive_stream` cannot be decoded without side information that no function produces, and a receiver cannot tell a raw body from a zstd body. The stated pipeline is not implemented; only `bud_format_spec.rs` (feature bud3, unwired) calls it.
Known: none
### P39-F04
File: bud/src/bud_format_qrmatrix.rs:97-137, 1-13
Severity: Medium
Axis: correctness
Confidence: Certain (traced by hand on version 1; Likely for other versions)
Evidence:
qrmatrix.rs:110-120  for &c in &cols {
                         let bit = if bit_idx < total_bits { ... } else { 1 /* padding */ };
                         // Do not overwrite the function modules.
                         if !self.is_function(row, c) { self.modules[row * d + c] = bit; }
                         bit_idx += 1;
                     }
Description: `bit_idx` advances for a module that is skipped as a function module, so the data bits that map onto finder or timing modules are consumed and never written. For version 1 (dim 21) the first column pair (20, 19) writes rows 20 down to 8 (26 bits) and then rows 7..0 lie inside the top-right finder zone (`r < 8 && c >= d - 8`), so stream bits 26 to 41 (bytes 3 to 5) never reach the matrix. The matrix also has no mode indicator, no length field, no error correction, no format information and no alignment pattern, although the header says "REAL QR FRAME GENERATION" and the doc mentions alignment; no QR reader can decode it and the module has no decoder. `data_bytes` is stored beside the modules, which is why the tests (lines 171-211, which only count dark modules) pass.
Impact: Content placed in this matrix is lost silently; "verifies it" in the header is false. Nothing calls `QrMatrix` (rg: no hit outside the file), and the root crate has the real QR path (src/storage/qr_*.rs, P29).
Known: idle-code-baseline.txt (dead code only); the data loss is not known
### P39-F05
File: bud/src/bud_format_por.rs:44-51, 56-66, 72-81, 87-106
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
por.rs:44-50   pub fn tag(&self, block: &[u8], index: u64) -> PorTag {
                   h.update(Self::DOMAIN); h.update(self.0); h.update(index.to_le_bytes()); h.update(block);
por.rs:72      pub fn respond(&self, blocks: &[Vec<u8>], challenge: &PorChallenge) -> Option<PorResponse> {
por.rs:87-92   pub fn verify(&self, blocks: &[Vec<u8>], challenge: &PorChallenge, response: &PorResponse) -> bool {
por.rs:84-86   /// ... in this simple version verify expects the nonce in the challenge record (replay is out of scope).
Description: The tag depends only on (key, index, block), never on the challenge nonce, and `verify` recomputes the expected tag from the full blocks, so the verifier must hold the whole data. The prover side (`respond`) uses the same secret key. A prover that keeps only the 32-byte tag of each block (computed once) answers every later challenge without the data; a prover that holds the key can precompute all tags and delete the blocks.
Impact: This is not a proof of retrievability: it proves nothing about current possession and cannot be run by a verifier that does not store the data. It is re-exported (lib.rs:157) and used by the CLI test and the fuzz target as the "PoR core".
Trigger: precondition: the prover saw tags once; entry por.rs:87; input: a stored tag table with the blocks deleted; result: `verify` returns true.
Known: none (idle-code baseline lists PorKey as unused)
### P39-F06
File: bud/src/bud_format_pcap.rs:21, 62-95, 82-83, 146-150, 246-247, 371
Severity: Medium
Axis: correctness
Confidence: Certain (all three by reading; none run)
Evidence:
pcap.rs:62    while pos + 16 <= data.len() {
pcap.rs:96-98 if records == 0 { return None; }
pcap.rs:148   if records > PCAP_MAX_RECORDS { return None; }   // restore only
pcap.rs:246-247  let ts_sec = ts_abs.div_euclid(1_000_000);
                 let ts_usec = ts_abs.rem_euclid(1_000_000);
Description: The header promises that `pcap_restore` reproduces the original byte for byte. Three inputs break it with no error from `pcap_transform`: (1) up to 15 trailing bytes after the last record are dropped by the loop condition; (2) a record whose ts_usec is 1,000,000 or more is re-normalised on restore (ts_sec + 1, usec - 1e6), so the header differs; (3) a capture with more than 1,048,576 records transforms fine (no cap on `records`, line 56) but `pcap_restore` refuses the result (line 148), so the stored form is unrecoverable. Neither function round-trips internally and the only corruption test (line 371) asserts `is_none() || is_some()`, which cannot fail.
Impact: If the transform is wired into storage it can destroy a user's data for any capture over about 1M packets (a 16 MB file of headers only reaches it); worst case lasting data corruption (would be High once wired). Today unwired.
Known: idle-code-baseline.txt (dead code only)
### P39-F07
File: bud/src/bud_format_office.rs:33-64, 71-78, 83-100, 104-198, 11-13, 230-232
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
office.rs:76-77   let _ = (data, expected);
                  None
office.rs:85      entries.sort_by(|a, b| a.name.cmp(&b.name));
office.rs:11-12   //! It is LOSSLESS: `office_restore` reproduces the original ZIP, in entry order
office.rs:184     offset = local_start + 30 + nb.len() as u32 + data.len() as u32;
office.rs:192     out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
Description: (a) `inflate_raw` is a stub that always returns None, so any DEFLATE entry (every real docx, xlsx, pptx) makes `zip_read` fail; the module works only on STORE zips it produced itself. (b) `office_transform` sorts entries by name, so `office_restore` cannot return the original order; times, version fields, extra fields, comments and the original central directory are discarded, so the restore is a different ZIP, not the original. The round-trip test passes only because its sample is already in name order and was built with the restore layout (lines 230-291). (c) `zip_read` reads local headers only and ignores the data-descriptor flag: a STORE entry written with bit 3 has compressed size 0 in the local header, is read as empty, and the loop ends at the next non-signature bytes, returning `Some` with truncated content. `uncomp_len` and the CRC are never checked for STORE. (d) restore casts the entry count to u16 (n may be 1,000,000), the name length to u16 (limit 64 KiB allows 65536, which wraps to 0), and writes u32 offsets with unchecked adds (wraps in release, panics in dev with overflow checks on a 4 GiB input). Names pass through `from_utf8_lossy`, so non-UTF-8 names do not round-trip. The transform also appends every entry body twice (lines 94-98), doubling the size and not authenticating the copy.
Impact: Lossless claim is false for real files; truncated content is accepted silently (c). Unwired.
Known: idle-code-baseline.txt (dead code only)
### P39-F08
File: bud/src/bud_format_scrub.rs:36-44, 76-84
Severity: Medium
Axis: correctness (content id binding)
Confidence: Certain
Evidence:
scrub.rs:38-41   h.update(b"BDLM_BUD_CONTENT_V1");
                 h.update((data.len() as u64).to_le_bytes());
                 h.update(data);
container.rs:138-143  pub fn content_id(bytes: &[u8]) -> [u8; 32] { ... h.update(b"BDLM_CONTENT_V1"); ...
Description: The header says the check compares with the content_id "already in every .bud". The scrub function hashes with the tag `BDLM_BUD_CONTENT_V1`, while the container content_id uses `BDLM_CONTENT_V1`. `rg 'BDLM_BUD_CONTENT_V1'` finds the tag only in this file. `verify_content(data, &content_id(data))` is therefore always false. The unit test builds its own digest with the scrub tag (lines 76-81), so the mismatch is not exercised.
Impact: A scrub job wired to real container ids reports every intact record as bit rot (false repair storm) or, if the ids were recomputed with the scrub tag, scrubs against a different identity than the one chain records hold.
Known: none
### P39-F09
File: bud/src/bud_format_recipe_record.rs:74-96, 99-106; bud/src/bud_format_r3fix.rs:100-120, 38-72; bud/src/bud_format_pact.rs:101-114
Severity: Medium
Axis: correctness (content id binding)
Confidence: Certain
Evidence:
recipe_record.rs:76   h.update(b"BDLM_TARIF_COMMIT_V1");   // commit(): hash of the BODY for Bodied (line 100)
recipe_record.rs:85-88 h.update(b"BDLM_TARIF_GENERATOR_V1"); h.update(generator.to_le_bytes()); h.update(seed); h.update(&params);
r3fix.rs:106          let commitment = crate::bud_format_container::content_id(original);
r3fix.rs:111          compress(original)        // body from a caller closure, never checked against commitment
r3fix.rs:47-54        m.contains("jpeg") || m.contains("png") ... => Self::Avif   // Avif is lossy (line 29)
Description: The word "commitment (K3)" names three unrelated schemes: content_id(original) in R3Recipe and PactRecord, hash of the compressed body in `RecipeRecord::bodied`, and a hash of (generator, seed, params) in `RecipeRecord::generative`. The generative commitment does not commit to the generated bytes at all (the comment at line 82 says it is "bound to the content"); nothing re-runs a generator. `R3Recipe::produce` stores whatever the closure returns as `body` next to `content_id(original)` with no decompress-and-compare, and maps png and jpeg to the lossy AVIF codec, from which the original bytes (and thus the commitment) cannot be reproduced. `Codec::None` is unreachable because `for_mime` never returns it, so the "encrypted is a user choice" path in the header does not exist. None of these hashes equals the node ContentId (budzero/bud-node/src/store.rs:34 and crates/budscan/src/content_id.rs:47 use SHA-256 over `hash_fields_bytes`; bud-core uses SHA3-256).
Impact: A record can claim a commitment that its stored body or recipe cannot satisfy, and two modules disagree on what identifies the content, so a chain commitment from one cannot be checked by another.
Known: none (P37-F01 covers the crate not being linked)
### P39-F10
File: bud/src/bud_format_recipe_record.rs:44-49, 65-70, 115-126; bud/src/bud_format_r3fix.rs:160-193
Severity: Medium
Axis: invariant (pricing; Z8 if on chain)
Confidence: Certain
Evidence:
recipe_record.rs:46    Self::Generative { .. } => 0,                          // held_bytes
recipe_record.rs:68    Self::Generative { params, .. } => 32 + 2 + 32 + params.len() as u64,   // record_bytes
recipe_record.rs:116-119  let held = recipe.held_bytes(); if held == 0 { return 0.0; }
recipe_record.rs:120-124  let ratio = if compression_ratio > 1.0 { compression_ratio } else { 1.0 };
r3fix.rs:178           Codec::Av1 => "h264-raw", // the raw-video proxy (AV1-like, high)
r3fix.rs:168           ("h264-raw", 3393.61),
Description: A Generative record holds `params: Vec<u8>` with no length cap, yet `held_bytes` is 0 and `rent` is 0, so arbitrary data stored in `params` is free. `rent` trusts a caller-supplied `compression_ratio` (any value above 1, including infinity, drives rent to zero); nothing derives the ratio from the body. `r3_real_rent` charges every video mime type the ratio measured on raw YUV (3393.61x), although a stored mp4 is already compressed; the comment "a claim ABOVE these ratios is REFUSED" (line 163) has no implementing code (rg: no function compares a claim to the table). Rent for any video is then about 1/3400 of the floor.
Impact: If this price function feeds a rent meter, storage can be bought at near-zero cost by shaping the record or the mime type. Floats also make the result platform-dependent if it ever reaches consensus (see F26).
Known: none
### P39-F11
File: bud/src/bud_format_ratioconsensus.rs:44-52, 71-135, 157-180, 208-243
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
ratioconsensus.rs:79    evidence: [1u8; 32],
ratioconsensus.rs:172-179  // final: the best candidate takes the quorum vote (a deterministic simulation)
                           Some(RatioConsensus { ..., votes: n, quorum, epoch })
ratioconsensus.rs:239   if bytes[pos..] != rec.consensus_hash() { return None; }
Description: "Multi-agent ratio consensus with a BFT vote" is `max_by` over hard-coded constants. `votes` is set to the argument `n`; no vote is cast, signed or counted. The `evidence` fields are constants like `[1u8; 32]` that nothing checks (the K19 "invented ratios are eliminated" claim has no code). `from_blob` accepts any votes and quorum (votes may be below quorum) as long as the unkeyed trailing hash matches, and anyone can compute that hash. Only `class_of` and `ContentClass` are used by another module (bud_format_engine.rs:30); the rest has no caller. `bud_format_bft.rs` already holds the BFT ratio vote (P37-F12).
Impact: Any reader that treats a `RatioConsensus` record as proof of validator agreement is trusting a record anyone can forge.
Known: idle-code-baseline.txt (dead code only)
### P39-F12
File: bud/src/bud_format_regeneration.rs:62-186; bud/src/bud_format_pact.rs:117-129, 183-242; bud/src/bud_format_production.rs:99-161; bud/src/bud_format_ratioconsensus.rs:194-243
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
regeneration.rs:182   if bytes[HDR..] != rec.record_hash() { return None; }
pact.rs:197           out.extend_from_slice(&self.record_hash()); // digest (kurcalama RED)
production.rs:157     if bytes[pos..] != rec.record_hash() { return None; }
segment.rs:112-113    // The SHA3 check above does not protect: the digest is keyless and DOMAIN is a public constant
Description: Four record types close with an unkeyed SHA3 of their own fields and are described as "writable on chain" or "tamper rejected". The hash detects accidental corruption only. `RegenerationRecord{epoch, pact_hash, outcome: Verified, cost_units}` can be minted by anyone for any pact hash, with a self-chosen cost, because no signer, no challenge nonce and no link to the produced bytes is in the record. The segment module states the same limit (lines 112-113); these four do not. `RegenerationChallenge::verify` also needs the full produced bytes from the caller and never runs the producer named by producer_id and seed, so "Verified" means only "the caller supplied bytes with this hash".
Impact: A reputation, slashing or payment rule keyed on these records can be driven by forged "Verified" or "Mismatch" records.
Known: none
### P39-F13
File: bud/src/bud_format_registry.rs:56-100, 126-151, 154-184, 46-53; bud/src/bud_format_optical.rs:86-107; bud/src/bud_format_revolutionary.rs:410-415; bud/src/bud_format_secure_db.rs:18-36; bud/src/bud_format_r3fix.rs:143-146; bud/src/bud_format_qrvideo.rs:388-392
Severity: Medium
Axis: test (a gate that cannot fail)
Confidence: Certain
Evidence:
registry.rs:81     merkle_proof: vec![ph, oh],            // "proof of ratio": ratio is never checked in verify (85-99)
registry.rs:142-150 pub fn verify_sorted(...) { for w in hashes.windows(2) { if w[0] > w[1] { return false; } } true }
optical.rs:93      if !prompt.holds_resolution(prompt.width, prompt.height) {   // compares a value with itself
revolutionary.rs:411  if chunk.content_hash == [0u8; 32] { return Err("K-BUD-SQLITE: hash zero"); }
secure_db.rs:22    encrypted: true,                        // constructor sets the flag; verify() (28-36) only reads it
r3fix.rs:144       pub fn qr_is_not_stored(&self) -> bool { true }
qrvideo.rs:390     pub fn qr_cannot_be_stored() -> Result<(), &'static str> { Err("K-QR-GENISLEME: ...") }
Description: Gates named after properties that they do not check. `RatioProof::verify` ignores `ratio` and `merkle_proof`; the "merkle" struct is the two input hashes. `k_bud_second_preimage` checks only that a slice is sorted and there is no tree builder. `holds_resolution(prompt.width, prompt.height)` is always true and the ratio comes from a caller-supplied `original_size`. `k_bud_sqlite` accepts any non-zero hash and never hashes `content`. `SecureEmbeddedDb::new` sets encrypted, wireguard and radicle to true, so `k_bud_secure_db` passes for any non-empty path with no database. The two "QR is not stored" gates are constants. `MimeRegistry::verify_pin` compares the stored `hash` field, not a recomputation, and `entries` is pub, so the pin survives edits; `DictLoopDetector::depth` counts total visits and never decreases, so "depth > 10" means "11 distinct hashes ever".
Impact: Reports of "gate passed" give assurance about encryption, ratio proofs and non-storage that the code does not provide. Unwired (rg: no caller outside lib.rs).
Known: idle-code-baseline.txt (dead code only)
### P39-F14
File: bud/src/bud_format_securededup.rs:19-36, 42-75, 79-85
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
securededup.rs:46-48   let key = convergent_key(data); let cid = cipher_content_id(data, &key);
securededup.rs:48-58   // PoW: H(cid || nonce) ...  for nonce in 0..1_000_000u64 { h.update(cid); h.update(nonce.to_le_bytes()); ...
securededup.rs:74      Some((cid, found))
Description: Nothing is encrypted: both functions take plaintext and return hashes, and the "encrypted identity" is a deterministic unkeyed function of the plaintext (a guessable-file oracle: anyone can test a candidate plaintext against a known cid). The "ownership proof" hashes only `cid` with a nonce, and `cid` is the public value being claimed, so a party that knows only the cid, not the data, finds the same nonce. The nonce is not returned and there is no verify function, so the proof cannot be sent or checked. A difficulty above about 20 bits silently gives `found = false`.
Impact: As the dedup ownership check it protects nothing: knowing a hash is enough to claim ownership of the content (the classic dedup attack the PoW was added to stop).
Known: none
### P39-F15
File: bud/src/bud_format_optical_transfer.rs:48-66, 73-107, 124-134, 12-15
Severity: Medium
Axis: security (content id binding)
Confidence: Certain
Evidence:
optical_transfer.rs:32-40  fn digest(index, total, data) -> H("BDLM_OPTX_SEG_V1" || index || total || len || data)
optical_transfer.rs:77-83  let total = segs[0].total; if total == 0 || total > 1_000_000 { return None; }
optical_transfer.rs:54    let total = data.len().div_ceil(seg_capacity) as u32;
Description: Each segment digest covers only that segment and is unkeyed, so an attacker can build a valid segment for any index. `join_optical` has no digest of the whole `.bud` and no content id to compare with; segments of two different transmissions with equal `total` mix and join without error. The header promises "verified joining, through a SHA3-256 digest" and a "content id per segment". Ceilings are asymmetric: `split_optical` has no limit on the segment count (and truncates with `as u32`), while `join_optical` refuses more than 1,000,000 segments, so a large `.bud` with a small per-code capacity is split into a set that cannot be joined.
Impact: A receiver cannot tell a spliced or replaced stream from the intended one; a large transfer fails at the end.
Known: idle-code-baseline.txt (dead code only)
### P39-F16
File: bud/src/bud_format_recipe.rs:26, 39-54, 58-74, 76-84
Severity: Medium
Axis: security (Z8 guard)
Confidence: Certain
Evidence:
recipe.rs:26   const FORBIDDEN_OPS: &[&str] = &["fdiv", "fadd", "fmul", "fsqrt", "fpow", "float", "div"];
recipe.rs:41-42 let txt = String::from_utf8_lossy(&self.recipe).to_lowercase();
               !FORBIDDEN_OPS.iter().any(|op| txt.contains(op))
recipe.rs:47-52 let cid = content_id(produced); &cid == commitment
recipe.rs:73   Some((saving as f64 * RECIPE_BOUNTY_RATIO).round() as u64)
Description: The integer-only gate is a substring blacklist over lossy text. `fsub`, `fneg`, `fma`, `sqrt`, `exp`, `sin`, `f64` and any binary opcode encoding pass; `individual` or `video` fail on "div". The Y15 rule exists so recipes stay deterministic across validators (Z8). `verify` never runs the recipe: `recipe`, `seed`, `miner` and `pact_id` are not used, so a bounty claim is satisfied by presenting the original bytes with any integer-only string as the recipe. `bounty` takes the new recipe size as a caller number and computes the payout in f64.
Impact: A float recipe passes the gate; a miner can claim a bounty without a recipe.
Known: idle-code-baseline.txt (dead code only)
### P39-F17
File: bud/src/bud_format_revolutionary.rs:179-193, 256-258, 280-296, 340-347, 429-444
Severity: Medium
Axis: security
Confidence: Certain (hand trace)
Evidence:
revolutionary.rs:257   ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.' | '/' | '+')
revolutionary.rs:292   } else if !(ch.is_whitespace() || matches!(ch, '"' | '\'')) { state = KeyState::default(); }
revolutionary.rs:342-345  COMPOUND_SECRET_KEYS.iter().find(|(name, _)| lower.contains(name))
                          .or_else(|| BARE_SECRET_KEYS.iter().find(|(name, _)| lower == *name))
Description: `password=p@ss!word` becomes `password=[REDACTED]@ss!word`: the value token ends at the first character outside the token set, and the next character resets the state, so the rest leaks. A multi-word value (`token: correct horse battery`) is cut after one word. The bare key names must equal the whole key, so `DB_PASSWORD=`, `SECRET_KEY=` and `MY_TOKEN=` are not recognised; PEM private key blocks, `ASIA` and other AWS prefixes, Slack tokens and 64-hex keys are not matched. `k_bud_secret_redact` (429-444) re-uses `secret_tokens`, i.e. the same tokenizer, so it passes the leaking output.
Impact: Secrets reach `CompactTable` rows and `SqliteChunk.content` while the gate reports success. Unwired today (rg: no caller), so Medium as a not-wired security component.
Known: none
### P39-F18
File: bud/src/bud_format_pgm.rs:112-124, 63-77, 33-35
Severity: Low
Axis: correctness
Confidence: Certain (arithmetic example); Likely for frequency
Evidence:
pgm.rs:71    let err = (offsets[k] as f64 - pred).abs().round() as u64;   // err is rounded
pgm.rs:121   let pred = (seg.a * key as f64 + seg.b).max(0.0) as u64;      // pred is truncated
pgm.rs:122-123  let e = seg.err.max(1); Some((pred.saturating_sub(e), pred.saturating_add(e)))
Description: Build measures the error against the real-valued prediction and rounds it; search truncates the prediction. With pred 10.9 and offset 13, err = round(2.1) = 2, truncated pred = 10, range 8..=12 which excludes 13. The doc promise "the true offset lies inside it" fails by one. Keys and offsets above 2^53 lose precision in the f64 fit. Growing a segment re-checks all its points per step (O(n * segment length), O(n^2) worst case) with no input bound. `pgm_digest` hashes f64 bytes.
Impact: A lookup that trusts the range can miss by one entry; large inputs cost quadratic time. Unwired.
Known: idle-code-baseline.txt (dead code only)
### P39-F19
File: bud/src/bud_format_pointcloud.rs:27-62, 64-83, 85-94, 52-60
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
pointcloud.rs:38     let cx = (x / grid).round() as i64;
pointcloud.rs:54-56  if !seen.contains(c) { seen.push(*c); }
pointcloud.rs:88-92  for c in cells { h.update(c.0.to_le_bytes()); ... }   // residual and grid are not hashed
Description: The "bit for bit" lossless path returns `+0.0` for an input of `-0.0` (cell 0 plus residual `-0.0`), and a NaN or infinite coordinate maps to cell 0 or `i64::MAX` with no refusal. The lossy dedup is a linear `contains` over a Vec (O(n^2)) and stores no count although the comment promises one. `pc_digest` covers cells only, so two clouds with equal cells and different residuals share a digest.
Impact: Small loss of fidelity and an unbound digest; quadratic time on a large cloud. Unwired.
Known: idle-code-baseline.txt (dead code only)
### P39-F20
File: bud/src/bud_format_optical.rs:51-76, 29-34, 1-5
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
optical.rs:56-62   if token.parse::<f64>().is_ok() { ... template = template.replace(token, "<NUM>");
                   } else if token.contains('.') && token.chars().filter(|c| *c == '.').count() == 3 { ... "<IP>"
optical.rs:69      u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
Description: `replace(token, ...)` replaces every substring occurrence, so a token `1` also rewrites the 1 inside `10`, `2000` and an IP. `NaN`, `inf` and `1e5` parse as numbers. Any dotted token with three dots (for example `a.b.c.d`) becomes `<IP>`. `variables` keeps tokens in scan order without positions, and no function rebuilds the line, so the template is not invertible. The id is the first 4 bytes of a hash (32-bit collisions). The "factor of 50000" and "factor of 50" in the header are unmeasured claims.
Impact: Mined templates cannot be reversed to the original log; not usable as lossless storage. Unwired.
Known: idle-code-baseline.txt (dead code only)
### P39-F21
File: bud/src/bud_format_pact.rs:148-159, 201-242, 101-114
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
pact.rs:149-152  match self.mode { PactMode::PureProduction => self.residual_len == 0, ... }
pact.rs:155-157  PactMode::ResidualOnly => { self.residual_len > 0 && self.residual_commitment == self.commitment }
pact.rs:131-134  /// ... `to_blob`/`from_blob` ... are CANONICAL: the same logical record -> the SAME bytes
Description: `verify` for PureProduction does not require `residual_commitment == EMPTY_RESIDUAL` or `byte_budget == 0`, so `from_blob` accepts many distinct blobs (and record hashes) for one logical record. `residual_only(b"")` builds a record that its own `verify` rejects, so `to_blob` then `from_blob` returns None for an empty original. `producer_id` and `seed` are free in ResidualOnly. `ts_unix` is part of the identity.
Impact: The "canonical, no effect on the state root" claim does not hold for the accepted input set; a consumer that keys on `record_hash` sees duplicates.
Known: none
### P39-F22
File: bud/src/bud_format_repairband.rs:27-50, 53-64
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
repairband.rs:41-42  // MSR has the form alpha(n-1)/(n-k); simplified to about k/(n-k).
                     Some(k as f64 / (n - k) as f64)
repairband.rs:47     Some((k as f64) * 0.75)
Description: The comment states (n-1)/(n-k) and the code returns k/(n-k), which is smaller for every k < n-1 ((6,4): 2.0 instead of 2.5). The MBR value is an invented factor 0.75 k. `k == n` passes the guard and returns infinity. `best_repair_model` therefore ranks code families on made-up numbers; the header says they are model inputs, not measurements, but the decision function does not carry that caveat. The real erasure repair path is not in this part.
Impact: A placement or pricing decision that uses `best_repair_model` is biased toward MSR. Unwired.
Known: idle-code-baseline.txt (dead code only)
### P39-F23
File: bud/src/bud_format_prodcost.rs:111-117; bud/src/bud_format_qos.rs:25-46; bud/src/bud_format_serviceclass.rs:28-38
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
prodcost.rs:114   .filter_map(|s| step_cost(s))                 // unknown step silently prices at 0
Description: A pipeline with a misspelled or new step is priced as free. `decide_qos` is a stateless function with no tenant id; a quota overrun is reported as Throttled when the rate is also exceeded, so Denied is hidden until the rate recovers. `select_class` sends an object read 9 times a month and older than a year to tape (Archive), and "erasure level" in the doc has no mapping. `cost_digest` omits `cpu_sec_per_tb` and the USD constant. All three modules have no caller outside lib.rs.
Impact: Low; economics heuristics with no production use.
Known: idle-code-baseline.txt (dead code only)
### P39-F24
File: bud/src/bud_format_pipe.rs:38-47, 141-161; bud/src/bud_format_real.rs:23-53; bud/src/bud_format_qrvideo.rs:205-216
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
pipe.rs:43      let t = String::from_utf8_lossy(data);                 // copies the whole input
pipe.rs:141     pub fn store_json_columnar(data: &[u8], mode: ColumnarMode, _min_chunk: usize)
Description: `detect` allocates a lossy copy of the full input for a 4 KiB decision; for GB inputs the copy doubles peak memory. `store_json_columnar` ignores `_min_chunk`. `store_compressed`, `store_zstd`, `store_json_columnar` and `restore_json_columnar` have callers only in tests and the fuzz target (the CLI imports `store_with_min` and `restore`; rg over bud/src/bin). Two zstd helpers exist with different ceilings (100 MiB in qrvideo, 4 GiB in real; the 4 GiB per-chunk issue is P37-F20) and different compressors (`bulk::Compressor` versus `encode_all`).
Impact: Maintenance risk; inconsistent bomb ceilings for the same operation.
Known: P37-F20 (ceiling); idle-code-baseline.txt (dead code)
### P39-F25
File: bud/src/bud_format_qrvideo.rs:66-103, 267-288, 169-203, 43-53
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
qrvideo.rs:102   (abs_seq as u32, sym)                       // header seq is 32 bit; composition seed is the u64
qrvideo.rs:279   } else if !rem.contains(&i) { rem.push(i); }  // duplicate index in a pub add_drop: counted once, XORed once if unsolved
Description: `repair_drop(abs_seq)` seeds from the full u64 but the receiver derives the composition from the u32 in the header, so for abs_seq at or above 2^32 sender and receiver diverge (only reachable by a direct call; `derive_stream` would need over 10^12 bytes). `add_drop` is pub and treats a duplicated index as present once while a solved duplicate would XOR twice. `derive_stream` accepts any `turns` and appends to an unbounded Vec. Payloads over 65535 blocks are refused after compression with no error detail. The degree distribution is uniform 2..24, not a robust soliton, which is why F02's elimination dominates for large k.
Impact: Low; caller-visible footguns on a pub API.
Known: none
### P39-F26
File: bud/src/bud_format_nvc.rs:35-54, 56-68, 70-79
Severity: Low
Axis: invariant(Z8)
Confidence: Likely (missing step: build the table on two libm implementations and compare)
Evidence:
nvc.rs:6-7   //! "no floats" rule - once integerized, an NVC can perform consensus-safe deterministic production
nvc.rs:39    let s = 1.0 / (1.0 + (-x).exp());
nvc.rs:49-53 let norm = x as f64 / K2 as f64 * SIGMOID_RANGE; ... sigmoid_lut()[idx]
nvc.rs:57    /// ... (no overflow: 256 inputs x 2^15 x 2^15 < 2^31)
Description: Also (all Low): records meant for chain hash or compare f64 values (production.rs:77 claimed_ratio bytes, ratioconsensus.rs:187 final_ratio bytes and from_blob accepts NaN, recipe.rs:73 bounty, regeneration.rs:103); segment.rs keeps pub ledger fields so `to_blob` can emit a blob (entry over 16 MiB or empty) that `from_blob` refuses. The module claims no floating point but builds the LUT with `f64::exp` (libm, not bit-identical across targets) and maps every input through f64 division and `round`. The table is rebuilt on every `sigmoid_int` call (2048 `exp` calls per evaluation). The overflow comment is wrong: 256 x 2^30 is 2^38; the saturating add hides it (the test at line 146 asserts saturation to i32::MAX). A length mismatch in `dense_int` returns the bias without an error.
Impact: Cross-device determinism, the stated purpose, is not guaranteed; very low practical risk (a table entry flips only if `s * 255` falls within one ulp of x.5). Unwired.
Known: idle-code-baseline.txt (dead code only)
### P39-F27
File: bud/src/lib.rs (module list); bud_format_nvc.rs, _office.rs, _optical.rs, _optical_transfer.rs, _pcap.rs, _pgm.rs, _pointcloud.rs, _prodcost.rs, _qos.rs, _qrmatrix.rs, _realcorpus.rs, _recipe.rs, _registry.rs, _repairband.rs, _revolutionary.rs, _scrub.rs, _secure_db.rs, _securededup.rs, _serviceclass.rs
Severity: Low
Axis: integration
Confidence: Certain
Evidence: `rg -l "bud_format_<name>\b" src tests fuzz kani` per module returns only `src/lib.rs` for the 19 files above. Callers of the rest: pact (bin/bud.rs:486, 512, 682; engine; block), pipe (bin/bud.rs; tests; fuzz), production (bin/bud.rs:571; engine:372; videopipe), regeneration (block.rs:118; bin/bud.rs; tests), segment (bin/bud.rs:551; tests), por (tests and fuzz only), qrvideo and recipe_record (bud_format_spec.rs, itself feature bud3 and unwired), r3fix (spec.rs, bud3_live_test.rs), real (container.rs:515 uses zstd_decompress_safe), ratioconsensus (engine uses `class_of` and `ContentClass` only). `rg 'bud_core' src budzero crates xtask` has no `use` or dependency line (same result as P37-F01).
Description: 19 modules, about 3800 lines, have no production call site even inside bud-core; the rest are reached only by the bud-core CLI and tests. CI builds, lints and tests bud (ci.yml:594-600), so these compile and pass their own tests.
Impact: Per STATUS.md section 5 an unwired item is a finding; every claim above this line about on-chain or economic effect has no effect on a node.
Known: .github/idle-code-baseline.txt lines 361-749 (items); P37-F01 (crate)
### P39-F28
File: bud_format_pcap.rs:371; bud_format_recipe_record.rs:226-233; bud_format_serviceclass.rs:87-92; bud_format_realcorpus.rs:30-103; bud_format_qrmatrix.rs:200-211; bud_format_qrvideo.rs:553-566; bud_format_office.rs:230-291; bud_format_r3fix.rs:252-268
Severity: Low
Axis: test
Confidence: Certain
Evidence:
pcap.rs:371            assert!(pcap_transform(&p).is_none() || pcap_transform(&p).is_some()); // no panic
serviceclass.rs:91     assert!(ServiceClass::Hot as u8 <= ServiceClass::Archive as u8);
realcorpus.rs:39-41    } else { eprintln!("SKIP: no real ELF was found (test environment)"); }
Description: Assertions that cannot fail: pcap (371), serviceclass (91), the recipe forgery "canary" (recipe_record.rs:147-159 hashes unrelated guesses against a hash and can only return true). realcorpus.rs reads `/bin/bash`, `/usr/share/fonts` and `/etc` files and passes with a SKIP print when they are missing, so it is host-dependent; its font scan loop (lines 52-59) does nothing and `real_file` carries `#[allow(dead_code)]`. Missing negative tests that would have caught findings: payload corruption in qrvideo (F01), reading data back from `QrMatrix` (F04), a non-sorted or non-STORE zip for office (F07), usec >= 1e6 and over 1M records for pcap (F06), an intact record through scrub `verify_content` (F08). `the_real_r3_rent_measurements` (r3fix.rs:252) sits outside `mod tests`, under no `#[cfg(test)]` module, which compiles but is inconsistent.
Impact: Weak tests let F01 to F08 ship.
Known: none
### P39-F29
File: bud/src/bud_format_revolutionary.rs:496; bud/src/bud_format_realcorpus.rs:13; bud/src/bud_format_optical_transfer.rs:1-17
Severity: Info
Axis: invariant(Z3, Z7, Z9)
Confidence: Certain
Evidence: `rg -n '#\[allow|#\[ignore|unsafe|BEGIN' ` over the 30 files.
Description: Z3: one suppression, `#[allow(dead_code)]` at realcorpus.rs:13 (no `#[ignore]`, no `|| true`; history not checked). Z7: no `unsafe` in the part; optical_transfer.rs has no file-level `#![forbid(unsafe_code)]` (every other file has it; the crate lint `unsafe_code = "forbid"` in bud/Cargo.toml:53-54 covers it). Z9: revolutionary.rs:496 holds the well-known AWS documentation example access key id (type: AWS access key id shape, public placeholder), and line 518 assembles a GitHub token shape at run time; no real key, seed or token found. por.rs `PorKey` derives Debug and Copy (line 19), so the secret prints in any `{:?}` and is not zeroized; low while unwired.
Impact: None today.
Known: none
