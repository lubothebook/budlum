# P40 scan: B.U.D. 4 (bud/src, files 77 to 106 of `ls bud/src/*.rs`, read-only audit, Tier R2)

Base commit: 0aab775. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md sections 2 and 4.4, docs/audit/FINDINGS.md, notes/P37-scan.md (bud-core is a separate crate that the root crate does not depend on). Items listed in .github/idle-code-baseline.txt are marked Known.

## COVERAGE

The task estimate said 30 files and about 6536 lines; `ls bud/src/*.rs | sed -n 77,106p` gives 28 files, 6147 lines by `wc -l` (the listing is shorter than the range). All 28 read in full, every line, tests included, directory /home/user/budlum/bud/src:
- bud_format_shamir.rs 1-314
- bud_format_social.rs 1-256
- bud_format_social2.rs 1-230
- bud_format_spec.rs 1-240
- bud_format_telemetry.rs 1-153
- bud_format_timeseries.rs 1-382
- bud_format_tiny.rs 1-150
- bud_format_transpile.rs 1-103
- bud_format_tricore.rs 1-177
- bud_format_ultra.rs 1-119
- bud_format_ux.rs 1-269
- bud_format_video.rs 1-338
- bud_format_videopipe.rs 1-177
- bud_format_view.rs 1-369
- bud_format_wal.rs 1-195
- bud_format_wire.rs 1-272
- bud_format_zkbridge.rs 1-208
- bud_format_zkproof.rs 1-304
- churn.rs 1-240
- cli.rs 1-149
- fidelity.rs 1-154
- gates.rs 1-224
- lib.rs 1-225
- price.rs 1-166
- provider.rs 1-179
- quantum.rs 1-105
- quantum_chain.rs 1-359
- ratio.rs 1-90

Total read: 6147 lines.

Docs read: docs/AGENT_MAP.md (first 6000 bytes, whole table); docs/AUDIT_PLAN.md lines 110-165 (sections 5 and 6); notes/P29-scan.md lines 1-30; CLAUDE.md lines matching Z3, Z7, Z8, Z9 (rg only).

Outside the part (rg first, small ranges): bud/Cargo.toml 1-140; bud/src/bin/bud.rs 625-650, 770-800; bud_format_container.rs content_id (138-144); bud_format_engine.rs (rg); bud_format_qrvideo.rs (rg DAMLA_HDR_LEN, BLOCK); bud_format_recipe_record.rs 115-172; bud_format_r3fix.rs 89-106; .github/idle-code-baseline.txt, ci.yml (rg); root Cargo.toml (rg); notes/P37-scan.md 1-40.

Not read: bud/tests, bud/fuzz, the rest of engine.rs and container.rs, dependency crate sources. No cargo, no test run. Golden vector digests were not recomputed.

Extra-check summary:
- Erasure coding and repair: the part holds no real erasure code. It holds Shamir over GF(2^8) (bud_format_shamir.rs, table and interpolation are correct, k up to 255 does not overflow after the earlier fix, duplicate x gives None) and a one-parity XOR repair (churn.rs:33-44, correct for one lost block, no integrity check: P40-F12). The Shamir share carries neither k nor an integrity tag, so a wrong share or a wrong k yields a wrong seed silently: P40-F16. The "churn fixtures" are hard-coded constants: P40-F11.
- Content id binding: fidelity.rs ContentId and bud_format_container::content_id are byte-identical constructions (BDLM_CONTENT_V1, u64 length, bytes), fine. The zk witness does not bind content at all: P40-F01. The social bridge hashes the stored content_id and never recomputes it: P40-F15. HybridTx does not bind the PQ key to pq_pub_hash: P40-F05. The ladder step commitment (social2) is correctly bound to master, step, param and content id.
- Bounds: wal_restore, view from_blob, zkproof load_field_trace, timeseries from_blob parse with checked arithmetic and caps, with the gaps in F03, F04, F17, F18.
- Panics from external bytes: no unwrap, expect, panic or unreachable in production code of the 28 files (every hit of rg is under #[cfg(test)]); no `#[allow]`, no `#[ignore]`, no `unsafe` (Z3 and Z7 clean for this part). Arithmetic that can panic or wrap on external numbers: F18, F20, F11. bud/Cargo.toml sets `panic = "abort"` for dev and release but no `overflow-checks` in release, so an overflow aborts in dev and tests and wraps silently in release.
- Determinism (Z8): f64 appears in tricore, ux, ultra, price, spec, gates, provider, video, view, churn; none sits in src/consensus, src/chain, src/tokenomics, src/settlement, src/registry or src/cross_domain (bud-core is not a dependency of the root crate). Doc claims that an f64 value is "written into the block header" are recorded in F26.
- Unwired parts: root Cargo.toml has no bud-core dependency. In the part only run_video_pipeline, attempt_proof, engine_to_witness, field-trace save/load, BudCli::bench and the CLI are called by bud/src/bin/bud.rs. Everything else has no caller outside its own file and tests. Most of it is already in .github/idle-code-baseline.txt (Known); bud_format_zkproof.rs is not in the baseline because the CLI uses it.
- Z9: no key, seed or token literal in the part. DualWallet derives Debug over secret seeds: F23.

## FINDINGS
### P40-F01
File: bud/src/bud_format_zkbridge.rs:34-63, 79-91
Severity: Medium
Axis: correctness (content id binding)
Confidence: Certain
Evidence:
```
34 pub fn engine_to_witness(res: &EngineResult) -> Vec<WitnessStep> {
38     let mut init: [u8; 32] = prev.finalize().into();      // H("BDLM_ZK_INIT" || original_len)
41         out.update(init);
42         out.update([s.to_u8()]);
44         // the digest standing for the step output: the container size (a deterministic intermediate)
45         out.update(res.container.len().to_le_bytes());
...
80 pub fn verify_witness(steps: &[WitnessStep], expected_root: &[u8; 32]) -> bool {
85     for w in steps.windows(2) { if w[1].input_digest != w[0].output_digest { return false; } }
90     &witness_root(steps) == expected_root
```
Description: Every digest in the witness chain is a function of original_len, container length and the step list only. No content id, no chunk digest, no container hash enters it, so any two inputs with the same two lengths and the same steps have the same witness root. `verify_witness` only checks that the chain links and that the root matches the steps it is given; it never recomputes a step, and the first input digest is not checked against the init value, so any self-consistent chain with a matching root passes. The module header says the witness is the spec of "a proof that this .bud was produced with these transforms".
Impact: The root that "is written on chain, the proof binds to it" (zkbridge.rs:65) does not commit to the data or to the container, so a prover that proves this circuit proves nothing about the stored object. Anyone relying on `verify_witness` as the I9 cheap check gets a length check. Precondition: a caller treats the root as a production attestation.
Trigger: bud zk prove / engine_to_witness at bud/src/bin/bud.rs:759 and 775; two different 1 MB files whose containers have equal length give equal roots.
Known: none (module is partly in idle-code-baseline.txt, 4 lines; the binding gap is new)
### P40-F02
File: bud/src/bud_format_zkproof.rs:94-110; bud/src/bin/bud.rs:777-786
Severity: Medium
Axis: correctness (fail-closed claim)
Confidence: Certain
Evidence:
```
 94 let out = Command::new(prover).arg(&trace_path).output();
 96 match out {
 ...
103     Ok(o) if o.stdout.is_empty() => Ok(ZkTrust::Unproduced { ... }),
106     Ok(o) => Ok(ZkTrust::ProvenExternally { rows: row_count, proof_bytes: o.stdout.len() }),
```
Description: Exit code 0 plus any non-empty stdout (a banner, a usage line, a Python traceback printed to stdout) becomes `ProvenExternally`. The stdout is reduced to its length and dropped: it is neither returned, saved, hashed nor bound to the trace root, and the CLI prints only the byte count. The module doc says a proof is "only RECORDED".
Impact: The tree reports "external prover produced N bytes" with nothing recorded, and a misconfigured BUD_ZK_PROVER that echoes text is reported as a proof. The enum name is read by callers as a trust level (the doc calls it "the trust decision").
Trigger: set BUD_ZK_PROVER=/bin/echo; `bud zk prove <file>` prints the ProvenExternally line (bud.rs:781).
Known: none
### P40-F03
File: bud/src/bud_format_timeseries.rs:25, 179, 236-270
Severity: Medium
Axis: security (allocation from untrusted length)
Confidence: Certain
Evidence:
```
 25 pub const MAX_POINTS: usize = 100_000_000;
179 let mut out = Vec::with_capacity(self.points);
261 if points == 0 || points > MAX_POINTS { return None; }
```
Description: `from_blob` accepts any `points` up to 100 million after an integrity check whose digest is unkeyed (domain tag constant, lines 242-246), so an attacker computes it. `decode` then reserves `points * 16` bytes (1.6 GB) before reading a single bit of a stream that may be 40 bytes long; decode only stops early afterwards, when the bits run out. There is no relation check between `points` and `bits.len()` (every point costs at least 2 bits, so points <= 4 * bits.len() could be enforced).
Impact: A 70-byte blob aborts the process (`panic = "abort"`, allocation failure) or exhausts memory where decode is exposed. Not wired today (idle-code baseline lists TimeSeriesColumnar), so this is a mainnet blocker only if the module is wired.
Trigger: TimeSeriesColumnar::from_blob(blob with points=100_000_000, bits_len=16, valid digest) then .decode() at bud_format_timeseries.rs:175.
Known: idle-code-baseline.txt (unwired); the allocation issue is new
### P40-F04
File: bud/src/bud_format_view.rs:46-115, 118-139, 164
Severity: Medium
Axis: correctness (roundtrip break)
Confidence: Certain
Evidence:
```
123 out.extend_from_slice(&(self.keys.len() as u16).to_le_bytes());
164 if key_count > 512 { return None; }
```
Description: `compile` has no cap on the number of keys, `to_blob` truncates the count to u16, and `from_blob` refuses more than 512. A JSON array whose objects carry more than 512 distinct keys compiles to a view that serialises into a blob the reader refuses; at 65 537 keys the written count wraps to 1 while 65 537 key records follow, so the blob is rejected by the trailing-bytes check or, with crafted content, parsed as a different view. The writer and the reader disagree about the format limit.
Impact: A view of wide data cannot be read back (silent data loss of the view); the original content stays intact (KF2).
Trigger: CompiledView::compile(JSON array of one object with 600 keys).to_blob() then from_blob returns None.
Known: idle-code-baseline.txt (unwired); the asymmetry is new
### P40-F05
File: bud/src/quantum_chain.rs:49-90
Severity: Medium
Axis: correctness (key binding) / security
Confidence: Certain
Evidence:
```
50 pub struct HybridTx { pub ed_sig: [u8; 64], pub pq_sig: Vec<u8>, pub pq_pub_hash: [u8; 32] }
56 pub fn verify(&self, msg: &[u8], ed_pk: &[u8], pq_pk: &[u8]) -> bool {
...
85     vk87.verify(msg, &sig87).is_ok()
```
Description: `verify` takes both public keys from the caller and checks each signature against the key it was handed. The field `pq_pub_hash` is never read: nothing ties `pq_pk` to the hash in the transaction or to an address. A signer who controls only an Ed25519 key can pair it with a PQ key of their own and the hybrid check passes; the second factor adds no protection against a quantum break of Ed25519. `Verifier::verify` of ed25519-dalek is the non-strict variant (small-order keys and malleable signatures accepted).
Impact: The "hybrid" guarantee (dual_required_new) depends on a binding that is not implemented in the only verification routine. Precondition: any caller that derives the identity from the Ed25519 key alone.
Trigger: HybridTx { pq_pub_hash: [0;32], pq_sig made with a fresh ML-DSA key } and the matching fresh key as `pq_pk`; `verify` returns true (the existing test passes `pq_pub_hash: [0u8; 32]` and still verifies, quantum_chain.rs:264).
Known: idle-code-baseline.txt (27 lines for quantum_chain.rs, unwired); the missing binding is new
### P40-F06
File: bud/src/quantum_chain.rs:110-138
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
117 pub fn from_bip39_seed(seed: &[u8; 64]) -> Self {
119     ed.copy_from_slice(&seed[0..32]);
...
129 pub fn address(&self) -> [u8; 32] {
130     let mut h = Sha3_256::new();
131     h.update(self.ed_seed);
132     h.update(self.pq_seed);
```
Description: The address is SHA3 of the two SECRET seeds, not of the public keys, and carries no domain tag. A verifier holding only the public keys and a signature cannot recompute or check the address, so an address cannot be tied to a transaction signer. The Ed25519 seed is the raw first half of the BIP39 seed (no SLIP-10 or domain separation), while the PQ seed is hashed with a tag.
Impact: Wallets built on this produce addresses that no third party can validate against keys; if the API is wired, address checks must be reimplemented. Related Z9 point in P40-F23.
Trigger: DualWallet::from_bip39_seed(&[3u8; 64]).address() at quantum_chain.rs:314-320 depends only on seeds.
Known: idle-code-baseline.txt (DualWallet); the construction is new
### P40-F07
File: bud/src/quantum_chain.rs:140-235, 186-195
Severity: Medium
Axis: invariant (false assurance)
Confidence: Certain
Evidence:
```
189 pub fn handshake_ml_kem_ok() -> bool { true }
192 pub fn is_quantum_resistant() -> bool { true }
208 pub fn kq_hash(is_sha3: bool) -> bool { is_sha3 }
220 pub fn kq_p2p(ml_kem_ok: bool) -> bool { ml_kem_ok }
```
Description: Of the KQ-* gates, kq_hash, kq_tx, kq_wallet, kq_p2p, kq_stark and kq_media_device_only return their boolean argument; SnowHybrid reports a post-quantum handshake with two constants and has no handshake code. `HybridFinalityVote::verify_quorum(bls_ok, pq_ok, count, n)` verifies nothing: the caller supplies both booleans and a count that is not tied to distinct signers (the quorum formula `n - (n-1)/3` itself is correct). FiatShamirTranscript::is_complete checks that four labels were pushed, in any order, by whoever pushes them, unrelated to a real transcript. The module doc names these as the chain-wide quantum gates.
Impact: A gate whose output equals its input can never fail on real data (the file header of gates.rs says nothing counts as a gate until it catches the broken case). If any release note cites KQ-P2P or SnowHybrid as implemented, the statement has no code behind it.
Trigger: SnowHybrid::handshake_ml_kem_ok() is true on every build; no ML-KEM crate is a dependency (bud/Cargo.toml 85-96: sha3, clap, ed25519-dalek, ml-dsa, zstd, rand_core, serde_json).
Known: idle-code-baseline.txt (unwired); the vacuous gates are new
### P40-F08
File: bud/src/provider.rs:83-96, 118-128
Severity: Medium
Axis: correctness (fail-open stub)
Confidence: Certain
Evidence:
```
87 fn fetch(&self, _cid: &str) -> Result<Vec<u8>, ProviderError> { Ok(vec![]) }
122 fn fetch(&self, _cid: &str) -> Result<Vec<u8>, ProviderError> { Ok(vec![]) }
```
Description: The two live providers return `Ok(empty)` for every content id. `ProviderError::HashMismatch`, `NotFound` and `Offline` are never produced by them, the id string is not validated, and nothing compares the returned bytes with the id. A caller that treats `Ok` as the content gets an empty object for any id and no error. `NetworkFullProvider.n` is ignored (expansion is the constant 21/18).
Impact: If the trait is wired as the retrieval layer, retrieval cannot fail and no integrity check exists on that path. Today nothing outside the file calls it.
Trigger: MobileSelfProvider::new_owner("d").fetch("not-a-hash") returns Ok(vec![]).
Known: idle-code-baseline.txt (provider.rs, 11 lines)
### P40-F09
File: bud/src/gates.rs:152-173 (155-156), 99-125
Severity: Medium
Axis: test (a gate that fails open)
Confidence: Certain
Evidence:
```
155 let cost_json = price.cost_sold(1.143, 17.191).unwrap_or(0.0);
156 let cost_jpeg = price.cost_sold(1.1667, 4.885).unwrap_or(0.0);
164 Self::kf(cost_json, 0.016),
91  pub fn kf(cost: f64, ceiling: f64) -> GateResult { ... passed: cost <= ceiling + 1e-12,
```
Description: `all_gates_demo` turns an error of the price model into cost 0.0, and KF passes for cost 0.0. A change that makes `cost_sold` return Err (for example a validation added to e) turns both KF rows, including the "deliberate break" row for JPEG, into passes, and the test `gate_all_demo_has_both_pass_and_fail` still goes green because other rows fail. Also `kf2` (gates.rs:105-108) cannot fail for any in-tree format: `render` returns the canonical bytes with the original resolution for every non-thumbnail format, and the Thumbnail arm is accepted unconditionally.
Impact: The demo set is the file's own evidence that the gates can fail; two of its rows cannot fail by construction (KF on error, KF2 on every input). Not consensus.
Trigger: make PriceModel::cost_sold return Err for any input: gates rows KF(JPEG) flips to passed.
Known: idle-code-baseline.txt (gates.rs, 10 lines) for dead code only
### P40-F10
File: bud/src/fidelity.rs:83-131
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: The "fidelity core" never parses the bytes: width and height are caller-declared metadata, so `verify_fidelity` compares two caller numbers and then the content hash. The AVIF, WebP, AV1 and thumbnail renders return the original bytes (the thumbnail with a different claimed size). `FidelityError::FloatForbidden` is never produced. Public fields let `content_id` drift from `canonical`.
Impact: KF2 "resolution preserved" is a statement about the declared numbers only. Info for a skeleton; Low because lib.rs re-exports it as API.
Known: idle-code-baseline.txt (fidelity.rs, 3 lines)
### P40-F11
File: bud/src/churn.rs:127-172, 130-134
Severity: Low
Axis: test / correctness
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: `run` returns constants per fixture kind (survived true for everything except DoubleChurn at n=4, traffic as literals). No ring is built, no block is lost, no repair is run, so "N=21 (18+3) must survive a double column loss" is asserted by a literal. `self.n - 1` underflows (abort in dev, wrap in release) for the public field n=0.
Impact: Any report or gate that cites the ten fixtures as churn evidence measures nothing. Repair-width numbers feed KP2 in gates.rs (kp2(18, 21)) as literals as well.
Known: idle-code-baseline.txt (churn.rs, 5 lines)
### P40-F12
File: bud/src/churn.rs:33-44
Severity: Low
Axis: correctness (repair)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: The XOR repair is correct for exactly one lost block and now refuses unequal lengths. It does not check the number of surviving blocks against `QuadRing.k`, does not verify the recovered block against a content id, and an empty `blocks` slice returns the parity itself as "the lost block". One corrupt survivor produces a wrong block with no signal (XOR parity cannot locate it).
Impact: Silent corruption on repair when a survivor is bad; the caller must hash the result. No caller exists (unwired).
Known: idle-code-baseline.txt
### P40-F13
File: bud/src/bud_format_telemetry.rs:54-68
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: `decay` shifts by age / half-life relative to the last touch but never advances `touches`. Calling it periodically (as the module header says the producer does) re-applies the same elapsed time every call: a count touched at t=0 with half-life 100 is halved at t=100, halved again at t=101 (age 101 still gives one halving), and so on, and at t=300 a second call applies three halvings to an already decayed count. Hot clusters cool far faster than the half-life says, and `record_hash` (declared "writable to the chain") changes with the call schedule.
Impact: Wrong culling tiers if wired (CullingPlan::from_access); the record hash depends on how often decay ran. Not consensus today.
Known: idle-code-baseline.txt (AccessTracker, decay)
### P40-F14
File: bud/src/bud_format_social2.rs:117-128
Severity: Low
Axis: correctness (hash framing)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: `url` and `post_id` are variable length and are hashed with no length prefix and no source count, so (url "ab", post_id "c") and (url "a", post_id "bc") give the same digest, and so do different source boundaries. Compare `SocialBridgeRecord::record_hash` and `Sha3Hasher::hash_fields`, which do frame lengths. The `alive` flag, a self-reported liveness sample, is part of the digest.
Impact: Two different pacts share a digest; a digest used as a commitment is ambiguous. Not wired.
Known: idle-code-baseline.txt (social2_digest)
### P40-F15
File: bud/src/bud_format_social.rs:102-127
Severity: Low
Axis: correctness (content id binding)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: `record_hash` uses the stored `content_id` field and never recomputes it from `content`; with pub fields a record can carry a content_id that does not match its content and still hash, so identity and content are bound only if the caller also calls `verify_content`. `verify_source` returns true for empty input, so a failed or blocked fetch that yields zero bytes is indistinguishable from "the source was deleted" and passes the drift check. `platform` and the `Other(&'static str)` label are not in the hash. K74 "Owned" is a label chosen by the constructor caller.
Impact: Drift detection is bypassed by an empty fetch result; provenance record can be built with mismatched content.
Known: idle-code-baseline.txt (social.rs, verify_content, verify_source)
### P40-F16
File: bud/src/bud_format_shamir.rs:118-167
Severity: Low
Axis: correctness (threshold sharing)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: The GF(2^8) tables and the Lagrange step are correct, and split now uses OsRng with a k-sized coefficient buffer. But a share has no integrity tag and the blob has no k, n, or seed commitment and no parser: `combine` takes the first k shares given and returns whatever they interpolate. A modified share, or a k smaller than the real threshold, returns a wrong 32-byte seed as `Some(..)`; extra shares are ignored rather than cross-checked. `secret.is_empty()` on `&[u8; 32]` (line 83) is always false. Tables are rebuilt on every call (cost only).
Impact: A single bad node silently destroys the regenerated content unless a higher layer checks the content id of the result; with k nodes required for access, any one liar is undetectable here. Not wired (test_integration.rs only).
Known: idle-code-baseline.txt (shamir.rs)
### P40-F17
File: bud/src/bud_format_wal.rs:25-40, 63-107
Severity: Low
Axis: correctness (canonical form) / security
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: `varint_read` accepts overlong encodings (0x80 0x00) and at shift 63 silently drops the high bits of the last byte; `wal_restore` ignores bytes after the last record, and the format has no digest of its own. Many byte strings restore to the same records. The header comment promises `[ts_delta]` columns but the writer emits lengths only. The bounds (record count against remaining bytes, checked end offset) are correct.
Impact: Malleable container: two different blobs, one record list. Matters only if the blob is hashed or deduplicated as an identity. Not wired.
Known: idle-code-baseline.txt (wal.rs)
### P40-F18
File: bud/src/bud_format_timeseries.rs:122, 177-178, 236-270
Severity: Low
Axis: correctness / invariant (Z8 neighbourhood)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: (a) `*ts - prev_ts` is an unchecked i64 subtraction on caller data: it aborts in the dev profile and wraps in release (bud/Cargo.toml has no overflow-checks). A wrapped delta is written as a small code, and `decode` then fails at `checked_add`, so a series with (i64::MIN, ..., i64::MAX) encodes but cannot be decoded: lossless is violated without an error at encode time. (b) The header copies `first_ts` and `first_value` but `decode` reads them from the bit stream; `from_blob` never compares the two, and bits beyond the last point and the padding are not checked, so several blobs decode to the same series. (c) The header comment says "10 plus 6 bits" and "110 plus 8 bits"; the code writes 7 and 9.
Impact: Encode succeeds on data it cannot decode; blob malleability. Not wired.
Known: idle-code-baseline.txt
### P40-F19
File: bud/src/bud_format_videopipe.rs:40-85; bud/src/bud_format_video.rs:194-214, 226-237
Severity: Low
Axis: correctness / invariant (honest measurement claim)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: `run_video_pipeline` is the only function of the part with a CLI caller (bin/bud.rs:638). It records the codec (always AV1) and GOP of the suggestion for the class, but `video_bytes` is never inspected, so the record states a codec and GOP the bytes do not have. `original_len` is a caller number never checked against the YUV size. `verify()` recomputes the ratio from the same two numbers that built it (tautology), the plausibility gate `VideoGates::k_bud_video` (K19, "an invented claim is refused") is never called in the pipeline, and the range is wide (half the minimum to twice the maximum). `w as u32` truncates. The cap on `video_bytes` size is the CLI file size.
Impact: The CLI prints `ratio=... proof_hash=...` for any bytes with any claimed original length; the "production proof" attests the caller's numbers.
Known: none (module partly in idle-code-baseline.txt)
### P40-F20
File: bud/src/bud_format_video.rs:106-146
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: `w * h * 3 / 2` is unchecked on CLI-supplied usize (aborts in dev, wraps in release; a wrapped value only shrinks the slices, indexing stays in range). Sampling every 64th byte aliases with the frame geometry: for a width that is a multiple of 64 only a few columns are sampled, so a moving object between them is invisible and a moving scene classifies Static. The result decides the codec suggestion and GOP.
Impact: Misclassification, wrong suggestion; no memory-safety issue.
Known: none
### P40-F21
File: bud/src/bud_format_ux.rs:22-29, 75-111
Severity: Low
Axis: correctness (documentation and arithmetic)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: The comment lists v20=652, v30=1231, v40=2331, but the table below it (correct EC=L byte capacities) has 858, 1732, 2953. The frame math packs 14 droplets of 200 B into a 2953 B QR, but a droplet frame is 20 B header plus payload (DAMLA_HDR_LEN = 20, bud_format_qrvideo.rs:29, frame length check at line 257), 220 B, so 13 fit; frame counts and playback minutes are about 7 percent low. The comment at line 75 says "BLOCK=200 bytes + a 20 B header -> a 200 B payload per frame". `qr_frame_count(bytes_per_droplet)` is called with 200 in the tests as well.
Impact: The user-facing "how many QR codes" answer is low. Spec K6 "overhead 1.00" (bud_format_spec.rs:6) is really 1.10 on this header; the test accepts 1.0 to 1.25.
Known: idle-code-baseline.txt for dead code; capacity/header mismatch new
### P40-F22
File: bud/src/bud_format_view.rs:51-115, 222-228
Severity: Low
Axis: security (DoS) / correctness
Confidence: Likely (cost not measured)
Evidence: code at the File lines above (quote omitted for length).
Description: Key collection is O(records x keys x distinct keys) and the schema pass is O(distinct keys x records); an array of N objects each with many unique keys costs quadratic or worse time for a few MB of JSON (the missing verification step: a timing run). Only `serde_json`'s default depth limit and the caller's byte limit bound the input. `compact_value` truncates to 24 chars before counting, so `unique_values` merges values that share a 24-char prefix and the string "null" equals JSON null. `type_name` is the type of the last record, not a union. Non-object rows are skipped but counted in `record_count`. The `summary` embeds unescaped, unbounded keys for "an LLM context" (summary and sample come from untrusted data).
Impact: CPU DoS on an exposed compile call; wrong cardinality; prompt text under data control.
Known: idle-code-baseline.txt
### P40-F23
File: bud/src/quantum_chain.rs:111-115
Severity: Low
Axis: invariant (Z9 neighbourhood)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: A struct that holds the secret seeds derives Debug (a `{:?}` or `dbg!` prints both seeds), derives Clone, has public seed fields, and has no zeroize on drop (no zeroize dependency in bud/Cargo.toml). No key literal exists in the part (Z9 clean for committed secrets).
Impact: Seed leakage through logs if a caller formats the wallet.
Known: idle-code-baseline.txt (DualWallet)
### P40-F24
File: bud/src/bud_format_spec.rs:28-35, 55-79, 89-104
Severity: Low
Axis: test / invariant
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: K5 passes for any input under 256 bytes. K10 round-trips the zstd body only: it decompresses `t.body` and compares it to the input, never checks `t.commitment` or `qr_derivative_commit`, and the compressor closure that makes the body is the same zstd as the decompressor, so a recipe whose commitment is wrong still passes. `Decompressor::decompress(data, capacity)` reserves the full 512 MiB per call. The K13 sweep labels `Codec::Av1` as "h264-raw", and lists five of the codecs only. `carousel_overhead_k6` returns the ratio only if at most 1.25, so the "overhead 1.00" claim is checked against a 25 percent tolerance.
Impact: The conformance gates cannot fail on the failure they are named for. Not wired (idle-code-baseline.txt lists all nine functions).
Known: idle-code-baseline.txt
### P40-F25
File: bud/src/bud_format_transpile.rs:18-39, 47-57, 64-82
Severity: Low
Axis: correctness / dead code
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: The "transpile" wraps the code in `echo`, so the script is always longer than the input and `k_bud_transpile` (ratio >= 1.0) returns Err for every non-empty input; the test calls that the canary. `is_deterministic` is a substring test (code containing "update" or "date" is "non-deterministic"; `$(date)` written differently or `$SECONDS` is not caught). `echo` treats an input beginning with `-n` or `-e` as an option. The AST stub puts the first 100 chars in a JSON string without escaping, so quotes or backslashes give invalid JSON, and `k_bud_ast` only checks non-empty. `ratio()` on AstTransform divides by a length that is never zero.
Impact: Dead stub with a gate that cannot pass; no runtime risk.
Known: idle-code-baseline.txt (transpile.rs)
### P40-F26
File: bud/src/bud_format_tricore.rs:1-110; bud_format_ux.rs:129-179; bud_format_ultra.rs:7-78; price.rs:1-75; provider.rs
Severity: Low
Axis: invariant (Z8, claim only)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: The module states that the f64 energy budget and price are "a consensus metric written into the block header". They are f64 sums of f64 products hashed by raw bits (NaN payloads, -0.0 versus 0.0, different results across a changed evaluation order); Z8 requires fixed point on a consensus path. bud/src/bud_fixed_point.rs exists (re-exported in lib.rs:98) and is not used here. `ux_digest` hashes an f64 fee, `ThreeMeterK14b` is compared with `PartialEq` on floats, and `three_meter_k14b` computes rent with floats (`rent` in recipe_record.rs:115-126). No file in the part is reachable from src/consensus, src/chain, src/tokenomics, src/settlement, src/registry or src/cross_domain (root Cargo.toml has no bud-core dependency), so this is a claim to remove or a rule to meet before any wiring, not a live violation.
Impact: If the energy record or the three-meter commitment is ever put in a header, nodes may disagree on the hash.
Known: STATUS.md section 2 may cover Z8 scope; not matched in this scan
### P40-F27
File: bud/src/quantum.rs:5-76; bud/src/bud_format_ultra.rs:57-78; bud/src/ratio.rs:41-55
Severity: Info
Axis: correctness (documentation, string-typed policy)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: ML-KEM is FIPS 203 (FIPS 205 is SLH-DSA). `is_quantum_resistant` tests strings on public fields: "ML-DSA-fake" passes, any cipher containing "256" passes, the KEM is never checked, and BLAKE3 is accepted as quantum-resistant hash while the module header says SHA3-256 (BLAKE3 optional). The UltraGates compare self-declared ratios against 1000 and 100 (inputs `original_size`, `template_ratio` are fields the caller sets), so they gate a claim, not a measurement. The ratio table header says "no number without a measurement" and holds a literature estimate (2.8), and 16.68 is hard-coded in ratio.rs:61, cli.rs:22 and (as 16.68 text) price tests.
Impact: Documentation and policy hygiene; no runtime effect.
Known: none
### P40-F28
File: bud/src/bud_format_zkproof.rs:88-95, 140-178
Severity: Info
Axis: security (defense in depth)
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: The prover path comes from an argument or the BUD_ZK_PROVER environment variable and is executed with the caller's rights; there is no timeout, `.output()` buffers all stdout and stderr in memory, and a hung prover leaves the trace file in the temp dir. The temp name is guessable (8 hex of a public root plus the pid), so a pre-planted file makes `create_new` fail (DoS of the command only; the symlink case is handled correctly). `load_field_trace` reads the whole file before checking the size against the cap, and does not check that field elements are below the Goldilocks prime (the root binds the raw words, so this is only canonical-form hygiene). The row count cap (1 000 000) and length equality check are correct.
Impact: Local CLI only.
Known: none
### P40-F29
File: bud/src/lib.rs:1-225; bud/src/quantum_chain.rs:171-184; bud/src/provider.rs:49-81
Severity: Info
Axis: integration / dead code
Confidence: Certain
Evidence: code at the File lines above (quote omitted for length).
Description: bud-core is a standalone crate (root Cargo.toml has no dependency on it; CI builds it separately with `--features bud3`, ci.yml:599-601). From this part only run_video_pipeline, engine_to_witness, field trace save/load/meta, attempt_proof, and BudCli (encode, decode, encode_container, decode_container, bench) have a production caller, and that caller is the bud CLI of the same crate. The rest (shamir, social, social2, spec, telemetry, timeseries, tiny, transpile, tricore, ultra, ux, view, wal, wire, churn, fidelity, gates, price, provider, quantum, quantum_chain, ratio) is in .github/idle-code-baseline.txt or is used only by tests/bud_format_qrmatrix (bud_format_ux::qr_capacity_bytes). `MobileSelfProvider` exists twice with different rules (provider.rs:50 has an owner exemption, quantum_chain.rs:173 does not) and lib.rs re-exports only the quantum_chain one under that name at the crate root (`pub use quantum_chain::MobileSelfProvider`, lib.rs:195). The wire contract (WIRE_CONTRACT, field_verdict, GOLDEN_VECTORS) is consumed by no reader, and `conformance_pass` runs only in a unit test.
Impact: Mainnet blocker class "B.U.D. parts not wired to the node" (AUDIT_PLAN section 5, storage extra checks); recorded once here.
Known: notes/P37-scan.md and .github/idle-code-baseline.txt

## Counts

Medium 9 (F01 to F09), Low 17 (F10 to F26 minus none; F10-F26), Info 3 (F27 to F29). Critical 0, High 0. Total 29.
Top by impact: F01 (zk witness does not bind content), F02 (ProvenExternally from any stdout, proof dropped), F03 (1.6 GB allocation from a 70-byte time series blob).
