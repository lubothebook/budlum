# P44 scan: Test harness (read-only audit, Tier R1 to R3 by target)

Base commit: 69fc58a. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14).

## COVERAGE

Read in full (every line):
- benches/domain_throughput.rs 1-715
- benches/single_node/internal_pipeline_tps.rs 1-142
- benches/micro/merkle_update.rs 1-62
- benches/micro/merkle_scaling.rs 1-59
- benches/micro/sig_verify.rs 1-48
- benches/micro/ratio_rayon.rs 1-142
- benches/micro/erasure_rows.rs 1-253
- benches/micro/timing_safe.rs 1-484
- fuzz/fuzz_targets/block_deserialize.rs 1-7
- fuzz/fuzz_targets/transaction_deserialize.rs 1-7
- fuzz/fuzz_targets/snapshot_deserialize.rs 1-9
- fuzz/fuzz_targets/evm_rlp_decode.rs 1-14
- fuzz/fuzz_targets/evm_mpt_verify.rs 1-41
- fuzz/fuzz_targets/fuzz_blockchain_serialize.rs 1-44
- fuzz/fuzz_targets/reputation.rs 1-52
- fuzz/fuzz_targets/budl_compile.rs 1-56
- fuzz/fuzz_targets/budl_compile_then_run.rs 1-50
- fuzz/fuzz_targets/bpqs_wots_reject.rs 1-69
- fuzz/fuzz_targets/consensus_validate.rs 1-80
- fuzz/fuzz_targets/vm_execute.rs 1-90
- fuzz/fuzz_targets/consensus_state_transition.rs 1-117
- fuzz/fuzz_targets/relayer_escrow.rs 1-192
- fuzz/fuzz_targets/zk_verifier.rs 1-130
- fuzz/Cargo.toml 1-158, fuzz/README.md 1-87 (part of the P44 neighbourhood, read for wiring)
- kani/Cargo.toml 1-34, kani/src/lib.rs 1-990 (two reads: 1-500, 500-990)
- crates/bpqs/examples/bench_perm.rs 1-82
- crates/bpqs/examples/bench_differential.rs 1-249
- crates/bpqs/examples/checksum_domination_exact.rs 1-476 and checksum_domination_exact.expected.txt 1-29
- crates/bpqs/examples/dudect_poseidon2.rs 1-761
- bud/tests/test_bud2.rs 1-138
- bud/tests/test_integration.rs 1-568
- bud/fuzz/Cargo.toml 1-23, bud/fuzz/fuzz_bud_format.rs 1-112
- bud/kani/kani_bud_format.rs 1-53

Not read in full (binary or data): fuzz/corpus/* seed files (sizes and names listed with wc; only two with xxd attempted, tool missing), fuzz/Cargo.lock (5732 lines, lock file), bud/tests/fixtures/{video.yuv,image.png} (binary), bud/tests/fixtures/text.log (first 300 bytes only, no secrets by rg).

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 110-165 (sections 5 and 6) and the P44 row; notes/P29-scan.md lines 1-30.

Outside the part (rg first, at most 80 lines each): budzero/bud-proof/src/adapter.rs 195-285; budzero/bud-proof/src/plonky3_prover.rs 1835-1985; src/network/protocol.rs 150-185; src/network/proto_conversions.rs 1000-1034 and 1336-1354 (rg hits); src/chain/snapshot.rs 105-125 and 1108-1130; src/storage/db.rs 100-118; src/chain/finality.rs 696-730, 826-850, 938-965; src/core/chain_config.rs 298-310; src/registry/permissionless.rs 620-665 and 1490-1570; src/consensus/qc.rs 1236-1262; src/chain/blockchain.rs 5198-5228; src/consensus/pow.rs 471-490; src/consensus/mod.rs:46; src/cross_domain/bridge.rs signatures (rg -A); src/execution/proof_verifier.rs 105-140; src/rpc/server.rs 726-760; Cargo.toml 150-235 and feature list; .github/workflows/ci.yml 1310-1395, 1470-1512; .github/workflows/fuzz-nightly.yml 1-80; .github/workflows/extra-tooling.yml 375-520; .github/workflows/kani-slow.yml 1-40; .github/workflows/benchmark.yml 1-70 (rg); xtask/gates/src/gates/tree_is_english.rs 1-40 and fuzz_targets_wired.rs (rg); docs/SECURITY.md 244-254; crates/bpqs/SECURITY-ARGUMENT.md 236-246 and 434-442; crates/bpqs/src/wots.rs (rg -A); ops/Dockerfile (rg).

No cargo, no test run, no git commit: every claim comes from reading source. Nothing was compiled, so "does not compile" claims (F09, F12) are derived from signature comparison and are marked Likely where compilation could differ.

Extra-check summary:
- Tests that cannot fail: consensus_validate (F03); the three overshoot-style Kani harnesses and others (F07); bud integration `let _ =` discards (F13); the Kani bitmap harness proves a function that is not production (F05).
- Fuzz targets not reaching the production decoder: zk_verifier structured mode stops at the envelope shape check (F01); the wire decoder NetworkMessage::from_bytes_validated has no target and block/transaction targets cover the on-disk bincode path (F02); consensus_state_transition cannot reach the MAX_REORG_DEPTH branch (F04).
- Kani vacuity: F05, F06, F07, F08, F12.
- allow / ignore (Z3): `#![allow(clippy::unwrap_used, clippy::expect_used)]` in four bench files (timing_safe.rs:4, sig_verify.rs:4, internal_pipeline_tps.rs:4, domain_throughput.rs:4), each with a stated reason at the top of the file (bench binary, not node code). No `#[ignore]`, no other `#[allow]`, no `continue-on-error` or `|| true` on the steps that run this part (ci.yml fuzz-quick, timing-safe job, kani-proofs). The slow Kani job is explicitly "NOT A GATE" (F08).
- Secrets (Z9): `rg` over the whole part plus corpus and fixtures for PRIVATE KEY blocks, 64-hex strings, token prefixes and mnemonic words found nothing. The only key-like value is the generated 64 character test string in timing_safe.rs:277-286, derived from a fixed xorshift seed, not a credential.
- Z12: one hit, `L1` in fuzz/fuzz_targets/consensus_state_transition.rs:6 (F19).

## Findings

### P44-F01
File: fuzz/fuzz_targets/zk_verifier.rs:65-87,119 (against budzero/bud-proof/src/plonky3_prover.rs:1893-1909)
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
zk_verifier.rs:73  let n = (take(data, &mut i) % 16) as usize;      // backend: at most 15 bytes
zk_verifier.rs:81  let n = (take(data, &mut i) % 12) as usize;      // fri_params_id: at most 11 bytes
plonky3_prover.rs:1893  if envelope.backend != "Plonky3-Keccak-Goldilocks" {   // 25 bytes
plonky3_prover.rs:1903  if envelope.fri_params_id != "test_fri_params" {       // 15 bytes
plonky3_prover.rs:1909  return Err(VerifyError::PublicInputsMismatch);          // hash never computed into the envelope
zk_verifier.rs:115  let _ = inputs.hash();                                       // result discarded
```
Description: In the structured modes (1 and 2) the fuzzer-built backend string is at most 15 bytes and the fri_params_id at most 11 bytes, but `verify` requires strings of 25 and 15 bytes. Every structured input therefore ends in `InvalidEnvelope` at the backend check. Even with matching strings, `public_inputs_hash` is a random 32 bytes and `inputs.hash()` is computed and thrown away, so the hash check at 1909 would also fail, and the program-hash check at 1923 needs the keccak of the fuzzed program. The program-hash, activation, `postcard::from_bytes` (1963) and `stark_verify_with_preprocessed` (1966) stages are never reached. Mode 0 only runs bincode deserialize of the envelope and never calls verify.
Impact: The target is documented (header, fuzz/README.md:178) as "ProofEnvelope bincode + DefaultAdapter::verify fail-closed" and runs 60 s in PR CI and 4 h nightly. Green runs give no evidence about the postcard proof decoder or the STARK verifier on hostile bytes, which is the R3 acceptance path of the whole settlement layer.
Trigger (security only): attacker-chosen proof bytes inside a well-formed envelope (right backend, version, fri id, matching hash, matching program hash) reaching postcard::from_bytes at plonky3_prover.rs:1963; no fuzz target generates that shape.
Known: none

### P44-F02
File: fuzz/fuzz_targets/block_deserialize.rs:6, fuzz/fuzz_targets/transaction_deserialize.rs:6 (missing target for src/network/protocol.rs:156-168)
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
block_deserialize.rs:6        let _: Result<budlum_core::core::block::Block, _> = bincode::deserialize(data);
protocol.rs:156-160           pub fn from_bytes(bytes) { ProtoNetworkMessage::decode(bytes) ... Self::try_from(proto_msg) }
protocol.rs:163               pub fn from_bytes_validated(bytes) -> Result<Self, MessageError>
src/core/transaction.rs:1101  pub fn to_bytes(&self) -> Vec<u8> { serde_json::to_vec(self) ...   // Transaction::to_bytes is JSON
src/storage/db.rs:111-112     fn decode<T>(value) { bincode::deserialize(value) ...               // bincode Block is the disk path
```
Description: Network input is decoded by prost (`ProtoNetworkMessage::decode`) and then `TryFrom` conversions in src/network/proto_conversions.rs, with nested `bincode::deserialize(&payload.data)` per transaction type (proto_conversions.rs:1010, 1020, 1030, 1340, 1350). No fuzz target calls `NetworkMessage::from_bytes_validated`, `from_bytes` or any `TryFrom<pb::*>`. The two "deserialize" targets exercise bincode over `Block` and `Transaction`, which is the local database encoding, not the wire.
Impact: The remote, unauthenticated decoder of every gossip message and every block has no fuzz coverage, while the harness names suggest block and transaction decoding is fuzzed. A panic or unbounded allocation in a `TryFrom` arm would be a remote single-node crash (High by the audit scale) that this harness set cannot find.
Trigger (security only): any peer can send bytes to the gossip handler that calls `from_bytes_validated`; the fuzz set never feeds that function.
Known: none

### P44-F03
File: fuzz/fuzz_targets/consensus_validate.rs:46-80
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
consensus_validate.rs:79  let _ = bincode::serialize(&header);
```
Description: The target named `consensus_validate` builds a `BlockHeader` from the input and only serializes it. Serializing a plain struct with strings, `Vec<u8>` and integers cannot fail or panic, and no consensus or validation function is called. The target cannot find a bug.
Impact: It occupies a 60 s PR slot and a 4 h nightly matrix job (fuzz-nightly.yml:33) and is listed as covering consensus validation. Header, block and QC validation (`BlockHeader` checks, `validate_block`, finality certificate verify) has no fuzz target at all.
Known: none

### P44-F04
File: fuzz/fuzz_targets/consensus_state_transition.rs:16-17,69-95,111-112
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
consensus_state_transition.rs:16  const MAX_PRODUCE: usize = 12;
consensus_state_transition.rs:76  let depth = MAX_REORG_DEPTH + 1 + (take_u8(data, &mut i) as usize % 8);
src/consensus/mod.rs:46           pub const MAX_REORG_DEPTH: usize = 100;
src/chain/blockchain.rs:5208-5214 let reorg_depth = self.chain.len().saturating_sub(fork_point);
                                  if reorg_depth > MAX_REORG_DEPTH { return Err(...) }
consensus_state_transition.rs:112 let _ = chain.try_reorg(candidate);
```
Description: The local chain never exceeds 13 blocks (genesis plus at most 12 produced), so `reorg_depth` is at most 12 and the `> 100` branch of `try_reorg` is unreachable from this target. Mode 2 ("Deep DIVERGENT reorg attempt past MAX_REORG_DEPTH") builds a candidate of at most 20 blocks forked from genesis, which is rejected earlier (fork_point 0 or invalid blocks), not by the depth rule. The target also asserts nothing about the result: `Ok(false)`, `Ok(true)` and any `Err` pass.
Impact: The finality-irreversibility rules (depth cap, fork point at or below `finalized_height`, finalized checkpoint hash at blockchain.rs:5216-5226) are the R3 safety rules for reorgs; the header and comments claim they are fuzzed, and the only check is panic freedom on at most 12-block chains.
Known: none

### P44-F05
File: kani/src/lib.rs:304-326,463-490 (production src/chain/finality.rs:828-840)
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
kani/src/lib.rs:304-306  /// Maximum validators the bounded bitmap mirror accepts. The mirror test runs
                         /// the same sizes against the production walk, so a drift is caught.
kani/src/lib.rs:315-321  let mut voted_stake: u64 = 0; ... voted_stake = voted_stake.saturating_add(stakes[idx]);
finality.rs:828,837      let mut voted_stake: u128 = 0; ... voted_stake += u128::from(validator.stake);
rg -n 'bitmap_voted_stake|saturating_stake_sum|MAX_MIRROR_VALIDATORS' --glob '!kani/**' .
  -> only docs/SECURITY.md:250 (prose); no Rust hit
```
Description: The mirror accumulates in u64 with saturation, production accumulates in u128 without saturation. No test compares the two (the claimed mirror test does not exist; the three real mirror tests are bond, merkle and PQ length). The proven statement `voted <= total` is true for any sum of non-negative terms, so it holds for any accumulation and says nothing about `FinalityCert::verify`.
Impact: docs/SECURITY.md:250 states that a certificate "can never vote more stake than its set holds" is proved by this harness. The statement is not a property of production code and cannot fail if production changes.
Known: none

### P44-F06
File: kani/src/lib.rs:20-28,41-80,132-238; src/registry/permissionless.rs:1496-1520; src/consensus/qc.rs:1239-1262; src/crypto/primitives.rs:1137-1160
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
kani/src/lib.rs:26-29    /// A copy can rot. Two things stop it: `budlum-core`'s `bond_arithmetic_matches_the_kani_mirror` ...
permissionless.rs:1506   fn kani_mirror(stake: u64, slash_ratio_fixed: u64) -> u64 {   // inline copy inside the test
qc.rs:1242,1246,1254     fn kani_combine / kani_sibling / kani_promote                  // inline copies inside the test
primitives.rs:1153       fn kani_classify(len: usize)                                   // inline copy inside the test
rg -n 'budlum_kani|budlum-kani' --glob '!kani/**' --glob '!*.lock' .   -> no hit
```
Description: The kani package has no dependency link to budlum-core and budlum-core has none to it. Each "mirror test" compares production against a second hand-written copy placed inside the test, never against `penalty_for`, `merkle_*_u64` or `classify_pq_signature_len` in kani/src/lib.rs. Production slashing now calls `slash_penalty` (chain_config.rs:298) which is a third copy. The three copies (lib.rs, test, production) can drift apart with every test green and the proofs still passing. The only guard on lib.rs is the harness-count gate in xtask.
Impact: The proofs hold for the lib.rs text, and the claim that they hold for production depends on a drift test that does not look at lib.rs. This is the exact failure mode lib.rs:26 says it prevents.
Known: none

### P44-F07
File: kani/src/lib.rs:181-222,279-291,456-461,454-490
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
kani/src/lib.rs:189-194   let penalty = stake.checked_mul(ratio).expect(...);  let bond = stake.checked_mul(SCALE).expect(...);
kani/src/lib.rs:196      assert!(penalty > bond, "a ratio above FIXED_POINT_SCALE must take strictly more than the bond");
kani/src/lib.rs:286-290  let quotient = (u128::from(stake) * u128::from(ratio)) / u128::from(FIXED_POINT_SCALE);  assert!(quotient > u128::from(stake), ...)
kani/src/lib.rs:458-460  assert_ne!(PQ_SIG_LEN_DILITHIUM5, PQ_SIG_LEN_ML_DSA_65); ...
```
Description: `overshoot_at_ratio` (used by four proofs) and `an_unbounded_ratio_can_strictly_exceed_the_bond` re-derive `stake * (SCALE + k) > stake * SCALE` in local variables and call no function of the crate; they prove integer arithmetic and would pass if `penalty_for` were deleted. The distinct-length harness compares three constants. `merkle_proof_rejects_out_of_range_leaves` is the guard at lib.rs:209. The long doc comment above `overshoot_at_ratio` (lib.rs:93-179 region, "attempt 1 to 6" tables) describes a different harness and is stale; the `>=` versus `>` discussion no longer matches the code.
Impact: About 8 of the roughly 22 harnesses add no assurance about code; counting harnesses (the CI gate compares counts) overstates coverage.
Known: none

### P44-F08
File: kani/src/lib.rs:341-434 (harnesses at 499,516,562 region), .github/workflows/kani-slow.yml:19-33
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
kani-slow.yml:19-22  # This job is NOT A GATE, it is a measurement. When it goes red it says not
                     # "the property broke" but "still outside the budget"
kani/src/lib.rs:430-434  # These three are not CI-budget harnesses ... They are left in place, timing out honestly
```
Description: `ratio_endpoints_are_exact`, `penalty_is_monotonic_in_the_ratio` and `penalty_is_monotonic_for_full_stakes` have never completed in any recorded run (timeouts at 22 minutes to 90 minutes), run weekly, and failure is tolerated by design. The monotonicity of the slash penalty, which the docs present as proven, is therefore unproved. The `ratio == 0` half is cheap and is not split out.
Impact: Documentation and harness names read as proofs; the only recorded status is "timeout". No incentive inversion in slashing (a higher ratio costing less) is excluded by tooling.
Known: none (documented in comments, not in STATUS.md)

### P44-F09
File: Cargo.toml:217-222; benches/domain_throughput.rs:557-604,497-517; .github/workflows/benchmark.yml:17-53
Severity: Low
Axis: dead code
Confidence: Likely (compilation not run; derived from signatures)
Evidence:
```
Cargo.toml:221   required-features = ["__bench_domain_throughput_disabled"]     // no such feature in [features]
domain_throughput.rs:511  bridge.mint(&message).unwrap();             // bridge.rs:432 mint(message, current_height)
domain_throughput.rs:513  bridge.unlock(transfer.message_id, 2).unwrap();   // bridge.rs:592 unlock(id, domain, settled_height)
domain_throughput.rs:580-586  ProofEnvelope { proof_format_version, backend, public_inputs_hash, proof_bytes, degree_bits }  // proof_verifier.rs:110-118 also has p3_version, fri_params_id
domain_throughput.rs:557-570  ExecutionPublicInputs { ... event_digest }   // proof_verifier.rs:134-135 also has state_writes_digest
rg -n '__bench_domain' . -> Cargo.toml:221 only
```
Description: The 715-line Criterion bench is permanently switched off by an undefined feature, and its calls no longer match current signatures (missing `current_height`, `settled_height`, two envelope fields, one inputs field), so it would not compile if enabled. benchmark.yml describes "Criterion benchmarks" and "performance regression tracking" but builds only the four micro bins and the budzero benches, and compares nothing.
Impact: Throughput numbers for transactions, registry, bridge and proof verify cannot be produced; any performance claim citing this file is unbacked. Dead code with a misleading header.
Known: none

### P44-F10
File: benches/micro/timing_safe.rs:51-62,442-481
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
timing_safe.rs:57-59  /// 5 percent is not arbitrary but derived from measurements: the real leak was 22.36 ns while
                      /// the constant-time path differed by 1.03 ns (4.6 percent). The threshold sits just above
                      /// that, so a run that passes today goes red only if the difference DOUBLES.
timing_safe.rs:24-27  constant_time_eq_str : ... |t|=7.62      // above the 4.5 threshold
timing_safe.rs:462    if t_ct.abs() >= T_THRESHOLD && effect_ratio >= EFFECT_RATIO_THRESHOLD { ... exit 1 }
```
Description: The effect-size threshold was set from the observed passing run, so a statistically significant (|t| 7.62) 1 ns difference passes and a leak up to 5 percent of the naive first-versus-last leak is accepted. Only equal-length inputs differing at the first or last byte are measured; the length-mismatch branch in `constant_time_eq_str` (src/rpc/server.rs:735-738) and middle positions are not.
Impact: The CI gate (ci.yml:1493) cannot detect a small residual leak in the RPC api key comparison. Remote exploitation of a sub-nanosecond difference is impractical, so this is defense in depth.
Known: none

### P44-F11
File: crates/bpqs/examples/dudect_poseidon2.rs:73-80,370-393,535-560,626-643
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
dudect_poseidon2.rs:75-79   // `current` mirrors src/poseidon2.rs exactly. It is duplicated here ... `parity_with_crate` below pins this copy against the real one
dudect_poseidon2.rs:544-557 crate_permute(&mut a); ... if current::fe_add(x, y) != ((u128::from(x) + u128::from(y)) % ...) ... std::hint::black_box(a);
dudect_poseidon2.rs:378-381 FixVsRandom separates the SHIPPED implementation at |t| = 489 ... FixVsFix on the same code reads 0.65
dudect_poseidon2.rs:606-613 positive control: Mode::CraftedLeakPair      dudect_poseidon2.rs:631-637 subject: Mode::FixVsFix
```
Description: (1) `parity_with_crate` runs the crate permutation and discards the result, then compares the local `current::fe_add/fe_mul` against the legacy `%` arithmetic, not against the crate's field operations, so the measured subject is a copy whose equality with shipped code is not checked. (2) The classic fix-versus-random mode flags the shipped code at |t| = 489; the verdict was moved to a fix-versus-fix mode that does not, with a residency explanation recorded in comments. (3) The positive control (leaky arithmetic) is only run in the crafted-pair mode; the subject sweep uses random constant pairs, and the comment at lines 399-407 records that detection with arbitrary constant pairs was 3.85 at 120k measurements and 19.56 at 200k, so the power of the verdict mode is unvalidated. (4) A flagged pair is cleared if one re-measure does not cross 4.5 (lines 662-679), and the slowest 10 percent of samples is cropped. (5) No workflow runs it (rg over .github finds no reference).
Impact: SECURITY-ARGUMENT.md section 8 cites this run as the statistical half of the constant-time claim for Poseidon2 (lines 434-442). The evidence is a one-off manual run on a copy with a sensitivity that is not shown for the mode used.
Known: none

### P44-F12
File: bud/kani/kani_bud_format.rs:1-53; bud/fuzz/Cargo.toml:1-23
Severity: Medium
Axis: test
Confidence: Certain (wiring) / Likely (compile failure)
Evidence:
```
kani_bud_format.rs:5   #[cfg(kani)] mod kani { use crate::bud_format::{BudFile, ...};
kani_bud_format.rs:48  let data: Vec<u8> = kani::any();
rg -n 'kani_bud_format|bud/kani' --glob '!bud/kani/*' -> docs/AUDIT_PLAN.md and notes only
rg -n 'bud/fuzz|bud-core-fuzz|fuzz_bud_format' .github xtask -> no hit
fuzz_targets_wired.rs:43  let targets_dir = root.join("fuzz/fuzz_targets");    // gate scans only the root fuzz dir
```
Description: bud/kani/kani_bud_format.rs is not a module of bud-core (no `mod` line, no Cargo target) and no workflow runs Kani on it; `crate::bud_format` would not resolve, and `Vec<u8>` is not `kani::Arbitrary` in the Kani version pinned for the repo (0.67.0), so it would not build as written. Two of its three harnesses use constant data (4 and 32 bytes) with no symbolic input; the third asserts nothing. bud/fuzz/fuzz_bud_format.rs (the only fuzz target for the B.U.D. container parsers, Huffman, PoR and the losslessness roundtrips) is a separate cargo-fuzz package that no workflow builds or runs and the `fuzz-targets-are-wired` gate does not look at.
Impact: The B.U.D. decoders for untrusted .bud bytes (BudFile, BudV2File, HuffmanCoder) have a well written fuzz target that never runs, and a Kani file that reads as verification but cannot execute. bud tests in CI (ci.yml:601) are the only coverage.
Known: none

### P44-F13
File: bud/tests/test_integration.rs:288-336,338-383,385-433
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
test_integration.rs:335   let _ = free;
test_integration.rs:382   let _ = free;
test_integration.rs:431-432  let back_free = restore_json_columnar(&free, ColumnarMode::OrderFree).expect("free restore");  let _ = back_free;
test_integration.rs:288 fn json_columnar_ratio_gain_documented()   test_integration.rs:339 fn json_columnar_typed_numeric_gain()   // same corpus (20000 rows), same four asserts
```
Description: The OrderFree restore result is produced and dropped in three tests, so the record-set preservation of OrderFree is never checked in the integration suite (comments defer to a module test). `json_columnar_ratio_gain_documented` and `json_columnar_typed_numeric_gain` build the identical corpus and assert the same four relations. `the_full_integration_scenario` ends the dedup step with `assert!(dup_count >= 1)`, which is satisfied by a single duplicate.
Impact: Low. Losslessness for OrderFree rests on tests outside this part; the duplicated pair adds run time without coverage. Test names and messages in Turkish (`konteyner_parcalari_dedup_uyumlu`, `rejenerasyon_zinciri_uctan_uca`, "PACT deftere") are recorded in the English gate baseline, not a new finding.
Known: partly tree_is_english baseline

### P44-F14
File: fuzz/README.md:4-9,22-45,69-85
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
README.md:4-5   > Long fuzz runs are not executed in CI;           (fuzz-nightly.yml runs 12 targets for 4 h)
README.md:15    | `fuzz_blockchain_serialize` | A minimal byte-slice harness; a placeholder ...   (target is a real roundtrip, fuzz_blockchain_serialize.rs:23-43)
README.md:62    - [x] 14 targets exist                      (15 exist; bpqs_wots_reject is missing from the table)
README.md:38-41 The ZKVM oriented seed corpus files live under `fuzz/corpus/zkvm/`        (directory does not exist; the seeds are in fuzz/corpus/vm_execute/)
```
Description: The README disagrees with the tree in four places. The `fuzz-nightly.yml` header still talks about "5x4h" for a matrix of 12.
Impact: Documentation drift in the file auditors use to judge coverage.
Known: none

### P44-F15
File: fuzz/fuzz_targets/bpqs_wots_reject.rs:33,57-67; .github/workflows/ci.yml (no crates/bpqs step); crates/bpqs/examples/checksum_domination_exact.rs:405-419,422-429
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
bpqs_wots_reject.rs:61  let lane = (selector as usize + k) % 256;      // 256 lanes
bpqs_wots_reject.rs:33  const LEN: usize = 67;                         // verify_chains uses take(P::LEN)
bpqs_wots_reject.rs:68  let _ = verify_chains::<...>(&sig, &msg);      // result not compared
rg -n 'manifest-path crates/bpqs|crates/bpqs' .github/workflows/ci.yml -> comment lines only
SECURITY-ARGUMENT.md:241  "pinned in `checksum_domination_exact.expected.txt`, verified by `--check`"
```
Description: About 74 percent of the damage (lanes 67 to 255) lands in padding that `verify_chains` ignores, and the damaged signature's result is never compared with the honest value, so the harness named "reject" cannot fail on an accepted forgery (the comment states this is deliberate). No workflow builds or tests crates/bpqs at all, so `checksum_domination_exact --check`, its four unit tests, bench_differential and dudect run only by hand, while the security argument cites `--check` as the verification.
Impact: Low (research crate, feature `bpqs-research` off by default). The pinned expected text can drift from the code with nothing red.
Known: none

### P44-F16
File: crates/bpqs/examples/checksum_domination_exact.rs:353-360 and checksum_domination_exact.expected.txt:18
Severity: Info
Axis: correctness
Confidence: Certain
Evidence:
```
checksum_domination_exact.rs:357   "  exact  P(C<=H over 3 ranks), q=1: {}  log2 {:.1}", fmt(&p1, 6), bl_diff as f64
expected.txt:18   exact  P(C<=H over 3 ranks), q=1: 0.203832  log2 -3.0
expected.txt:21   q_honest=1: exact P(C <= min over honest q, joint ranks) = 0.203832  log2 = -2.295
```
Description: The same probability 0.203832 is printed with `log2 -3.0` (a bit-length difference kept for byte parity with the retired Python script) and `log2 = -2.295` two lines later. A reader quoting the first line understates the domination probability by 0.7 bits.
Impact: Presentation only; the refined total uses the exact value.
Known: none

### P44-F17
File: fuzz/fuzz_targets/relayer_escrow.rs:3-6,122-127,187-191; fuzz/fuzz_targets/reputation.rs:12
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
relayer_escrow.rs:5   //! Oracle: panic freedom + fund/status conservation under adversarial
relayer_escrow.rs:122 assert!(bridge.unlock(mid, lock_dom, height).is_err(), "unlock from the lock (source) domain must be refused");
relayer_escrow.rs:187 // Final conservation-ish probes (no panic).
reputation.rs:12     let peer = PeerId::random();
```
Description: The only functional assertion in relayer_escrow is the wrong-domain unlock refusal; no total-locked, minted or burned balance is compared, so "conservation" is not tested. The reputation target uses `PeerId::random()` and a real clock for ban expiry, so a crash input may not reproduce.
Impact: Low. Bridge supply invariants (the R3 double-spend class) are only covered by unit tests outside this part.
Known: none

### P44-F18
File: benches/single_node/internal_pipeline_tps.rs:86-125,135-141; benches/micro/merkle_update.rs:170-202; benches/micro/sig_verify.rs:21-23
Severity: Info
Axis: test
Confidence: Certain
Evidence:
```
internal_pipeline_tps.rs:90-91   Ok(_) => ingested += 1,  Err(_) => ingest_failed += 1,     // never turns into a failing exit
internal_pipeline_tps.rs:118-119 } else { break; }
merkle_update.rs:190  let _ = state.calculate_state_root();                  // incremental root never compared with a full recompute
sig_verify.rs:21-23   println!("Generating {count} keypairs and signatures...");  // one keypair, 1000 signatures
```
Description: The pipeline bench exits 0 even if every ingest fails and no block is produced; the Merkle benches time incremental updates but never check the root against a full recomputation; the sig_verify banner says 100000 keypairs are generated while it builds one.
Impact: Throughput numbers can be reported for a broken pipeline (the bench output is only tee'd in benchmark.yml:50-53). The bench binaries are `[[bin]]` entries in the root package and are built by every `cargo build` (ops/Dockerfile:77 builds all bins, copies only budlum-core).
Known: none

### P44-F19
File: fuzz/fuzz_targets/consensus_state_transition.rs:6; fuzz/corpus/block_deserialize, snapshot_deserialize, transaction_deserialize
Severity: Info
Axis: invariant(Z12)
Confidence: Certain
Evidence:
```
consensus_state_transition.rs:6  //! Candidate reorg chains must not crash the L1 state machine. Deep reorgs
fuzz/corpus/*/seed_{zeros,ones,empty,alt,seq,long}   // 0 to 2 lines each, no structured seed
```
Description: Z12 forbids the term for layer one in public text; this is a code comment, a minor hit. Separately, the deserialize corpora hold only zero, one and sequence byte patterns: no valid bincode `Block`/`Transaction` and no valid snapshot JSON, so coverage-guided exploration starts from nothing for decoders (JSON for snapshots) whose first failure is early.
Impact: Info.
Known: none

### P44-F20
File: budzero/bud-proof/src/plonky3_prover.rs:1903 (outside the part, found while tracing F01)
Severity: Info
Axis: integration
Confidence: Certain
Evidence:
```
plonky3_prover.rs:1903  if envelope.fri_params_id != "test_fri_params" {
```
Description: The production verifier accepts only the FRI parameter id "test_fri_params". Hand to the part that owns bud-proof (P12 to P16) to confirm whether this is the intended production parameter set.
Impact: Possible test-parameter value in the production path; not judged here.
Known: none

## Counts

Critical 0, High 0, Medium 7 (F01, F02, F03, F04, F05, F06, F11, F12 counted below), Low and Info as listed.

Recount by id: Medium = F01, F02, F03, F04, F05, F06, F11, F12 (8). Low = F07, F08, F09, F10, F13, F14, F15, F17 (8). Info = F16, F18, F19, F20 (4). Total 20.
