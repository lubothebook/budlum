# P46 scan: src/tests 2 (read-only audit, Tier R1)

Base commit: 3df8863. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14, F17 on identity_tx_door.rs:167-215 and the text-scan note), notes/P45-scan.md.

## COVERAGE

Files 18 to 40 of `ls src/tests/*.rs`, read in full (every line), 5809 lines:
- src/tests/consensus_bypass_locks.rs 1-274
- src/tests/consensus_digest.rs 1-173
- src/tests/consensus_expanded.rs 1-195
- src/tests/consensus_lock_order_loom.rs 1-286
- src/tests/constitution_engine.rs 1-62
- src/tests/deed.rs 1-36
- src/tests/disaster_recovery.rs 1-410
- src/tests/distributed_settlement.rs 1-393
- src/tests/domain_edge_cases.rs 1-287
- src/tests/encryption_dao.rs 1-90
- src/tests/f20_priority_findings.rs 1-333
- src/tests/finality_adversarial.rs 1-874
- src/tests/finality_live_path.rs 1-224
- src/tests/fork_choice_locks.rs 1-408
- src/tests/genesis_repro.rs 1-145
- src/tests/hard_prune.rs 1-85
- src/tests/hardening.rs 1-361
- src/tests/hardening_h2_locks.rs 1-124
- src/tests/hardening_h4_locks.rs 1-187
- src/tests/hardening_h5_h7_locks.rs 1-228
- src/tests/hardening_locks.rs 1-295
- src/tests/hash_input_serialize_locks.rs 1-125
- src/tests/identity_tx_door.rs 1-214

Outside the part (rg first, small ranges): src/core/block.rs 88-100 and 204-226; src/consensus/mod.rs 140-160, 249-251; src/consensus/pow.rs 456-462; src/consensus/pos.rs 99-107; src/chain/blockchain.rs 5973-5986; src/chain/genesis.rs 15, 1262 (rg); src/network/proto_conversions.rs 176-184; Cargo.toml loom lines (rg); .github/turkish-baseline.txt head; docs/audit/FINDINGS.md (rg only).

Not read: production bodies of Blockchain::try_reorg, handle_prevote, FinalityAggregator, the executor arms, the xtask gates. No cargo, no test run. Every claim is from reading source.

Extra-check summary:
- #[ignore]: none in the 23 files (the one rg hit is a comment string at hash_input_serialize_locks.rs:81).
- #[allow]: two crate-level `#![allow(clippy::needless_range_loop)]` (P46-F09). Both come from the import commit e7e3a45, so they are old, not new.
- Hardcoded secrets (Z9): none. Keys are test seeds (`[0xA1;32]`, `[7u8;32]`, BLS scalars from small bytes) and `KeyPair::generate()`. The Dockerfile and mainnet.toml strings in f20 are public config.
- Vacuous tests: found, see F02 to F07.
- Test that masks a production fact: F01 (timestamp), F10 (hash scan blind spots).

## FINDINGS

### P46-F01
File: src/tests/consensus_expanded.rs:49-63
Severity: Medium
Axis: test
Confidence: Certain (that the drift check cannot fire); Likely for the real rejection reason (missing step: print the error string)
Evidence:
```
consensus_expanded.rs:53   let mut block = Block::new(1, bc.chain[0].hash.clone(), vec![]);
consensus_expanded.rs:56   block.timestamp = bc.chain[0].timestamp + 3600 * 1000 + 1000;
consensus_expanded.rs:59   res.is_err(), "future-timestamp block must be rejected (timestamp drift protection)"
genesis.rs:15              pub const GENESIS_TIMESTAMP: u128 = 0;
domain_edge_cases.rs:197   assert_eq!(bc.chain[0].timestamp, 0, "genesis block timestamp should be 0");
consensus/mod.rs:152       if block.timestamp > now_ms.saturating_add(MAX_FUTURE_BLOCK_TIME_MS) {
```
Description: The devnet genesis timestamp is 0, so the test block is dated 3,601,000 ms after the epoch (1970), far in the past. The future-drift gate at consensus/mod.rs:152 cannot trigger. The block is rejected for another reason (the hash was computed in Block::new before the timestamp was edited, and the slot or interval rules also apply).
Impact: The drift gate (needed for vesting, BNS expiry, bridge lock expiry, AI deadlines per consensus/mod.rs:144-146) has no test in this file. The assert message claims coverage it does not give. Deleting the gate keeps this test green.
Known: none

### P46-F02
File: src/tests/finality_live_path.rs:99-124
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
finality_live_path.rs:102  let mut agg1 = FinalityAggregator::new(1, 10, "H".into(), snap1.clone());
finality_live_path.rs:116  let mut agg2 = FinalityAggregator::new(2, 20, "H2".into(), snap2.clone());
finality_live_path.rs:123  assert_eq!(agg1.prevotes.len(), 3, "epoch 1 penceresi kirletilmemeli");
blockchain.rs:5984         self.finality_aggregator = Some(aggregator);
```
Description: The doc says "the votes of the old epoch must not leak into the new aggregator". The test builds two unrelated structs and checks that voting on one does not change the other. Two separate values cannot share state, so the test cannot fail. The real epoch switch is `Blockchain::start_prevote_task`, which replaces the single `finality_aggregator` field (blockchain.rs:5973-5984). No test in the 23 files calls it twice or sends an old-epoch vote to the new aggregator through `handle_prevote`.
Impact: A stale-epoch vote accepted after rollover (finality safety on an R3 path) has no negative test here. The header of the file also lists this as covered.
Known: none

### P46-F03
File: src/tests/domain_edge_cases.rs:124-145
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
domain_edge_cases.rs:125   fn bft_leader_deterministic_selection() {
domain_edge_cases.rs:128   let bc = setup_chain();
domain_edge_cases.rs:132   let tip = bc.chain.last().unwrap();
domain_edge_cases.rs:133   assert_eq!(tip.index, 0, "genesis tip at index 0");
domain_edge_cases.rs:137   fn bft_block_validation_rejects_wrong_chain_id() {
domain_edge_cases.rs:144   assert_ne!(bc1.chain[0].chain_id, block_from_other.chain_id);
```
Description: The first test names leader selection and asserts only that a fresh chain has height 0; no leader function is called. The second names a validation rejection and asserts only that two fixtures were built with different chain ids; no validation is called. Both run on PoWEngine, not BFT. The header comment promises "BFT view-change/leader election" and "PoS slashing triggers"; none is exercised in the file.
Impact: A reader counting coverage sees BFT leader election and chain-id rejection covered. FINDINGS.md:14 already records that single-validator tests hide the no-leader height problem; this file adds a test that looks like coverage of exactly that area and checks nothing.
Known: none (related to FINDINGS.md:14)

### P46-F04
File: src/tests/domain_edge_cases.rs:25-41, 73-89, 194-216; consensus_expanded.rs:65-79
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
domain_edge_cases.rs:26    fn pow_difficulty_adjusts_after_blocks() { ... assert!(bc.chain.len() > 1, ...) }   // PoWEngine::new(0)
domain_edge_cases.rs:85    assert!(bc.state.accounts.contains_key(&validator), "genesis account must exist ...")  // after fund_development_account
domain_edge_cases.rs:212   assert!(bc.chain[2].timestamp >= bc.chain[1].timestamp, ...)
consensus_expanded.rs:71   assert_eq!(engine.info().contains(&$diff.to_string()), $diff > 0 || engine.info().contains("0"));
consensus_expanded.rs:64   // 40 variations ... prod_test_1..40 differ only in the producer byte
```
Description: Four weak tests. (a) The adjustment test uses difficulty 0 and asserts only chain growth; no difficulty is read. (b) The "validator registration with stake" test funds a faucet account and checks the account exists; no registration runs (the comment admits this). (c) The timestamp test asserts non-decreasing, while the production rule is strictly increasing (consensus/mod.rs:159, `<=` is refused), so the assert is weaker than the rule. (d) The `pow_diff_N` macro compares `info().contains("N")` with a right side that is always true for N above 0; `info()` always prints the difficulty (pow.rs:458), so the check cannot fail. The 40 `prod_test_N` tests run the same body with a different producer byte and add no new path.
Impact: About 50 tests inflate the test count without checking behavior. Low risk, but they hide real gaps (difficulty retarget, validator registration).
Known: none

### P46-F05
File: src/tests/disaster_recovery.rs:307-347, 381-392
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
disaster_recovery.rs:307   for _i in 0..100 { ... let _ = bc.mempool.add_transaction(tx); }
disaster_recovery.rs:314   // Block production interrupted! (Simulation: Process Exit)
disaster_recovery.rs:336   let error = bc.try_reorg(longer_chain).unwrap_err();
disaster_recovery.rs:338   error.contains("state root") || error.contains("consensus validation"),
disaster_recovery.rs:380   assert_eq!(bc.state.liveness.missed_count(&silent), 1);
disaster_recovery.rs:382   let missed_before = bc.state.liveness.missed_count(&silent);
disaster_recovery.rs:388   assert_eq!(bc.state.liveness.missed_count(&silent), missed_before, ...)
```
Description: (a) The chain-halt test reads `missed_before` and then compares the same call to it with no operation between (lines 382-392); the assertion is a tautology. (b) The "sudden crash during heavy writing" phase only fills the mempool and drops the chain; no write is interrupted, and the results of the 100 adds are discarded. (c) The recovery phase builds blocks with `Block::new` and no state, so the reorg is refused for any reason; the accepted error text is two broad substrings. (d) The first test asserts that BNS and NFT state added directly is gone after restart (lines 79-80), which pins "direct writes are not persisted"; if persistence of such writes is ever added, this test fails without a bug.
Impact: Crash recovery is claimed (chaos v2) but no partial-write, torn-commit or sled-flush case is exercised; the suite gives false assurance for the persistence format (R3 chain snapshot and persistence).
Known: none

### P46-F06
File: src/tests/hardening.rs:340-360; src/tests/hardening_h2_locks.rs:104-112
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
hardening.rs:350   let from_env_no_auth = RpcSecurityConfig { auth_required: false, ..Default::default() };
hardening.rs:359   assert!(!from_env_no_auth.auth_required);
hardening.rs:341   fn main_resolved_auth_required_check_compiles()
hardening_h2_locks.rs:108  let key = [0xABu8; 32];
hardening_h2_locks.rs:111  assert_eq!(key.len() * 8, 256);
```
Description: The first test sets a field to false and asserts it is false; it never calls `from_env` or the main.rs warning branch (its own doc says it only "compiles"). The second asserts `32 * 8 == 256` on a local array; the only real check is that one insert changes the root from zero, which any trie passes, so the "256-bit keys" claim is untested (no test inserts two keys that differ only in a low bit and expects two leaves).
Impact: Two REGRESSION locks that cannot detect their regression. The RPC auth warning path and the 256-bit key depth stay unlocked.
Known: none

### P46-F07
File: src/tests/hardening_h5_h7_locks.rs:141-150, 215-227; src/tests/f20_priority_findings.rs:243-249
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
hardening_h5_h7_locks.rs:146  wf.contains("Genesis") || wf.contains("genesis") || wf.contains("determinism"),
hardening_h5_h7_locks.rs:224  lower.contains("security@") || lower.contains("advisor") || lower.contains("report"),
f20_priority_findings.rs:246  assert!(src.contains("structural_envelope_checks_only"));
```
Description: The determinism check accepts any text with "determinism"; the file is named determinism.yml and the text almost certainly contains it. The SECURITY.md check accepts any text with "report". The f05 check only proves that three strings appear somewhere in rpc/server.rs (a comment or dead branch would satisfy it); no RPC call is made. Text-scan tests of this kind pass even when the behavior is removed.
Impact: These locks give a green result on regression. The f05 lock does not pin that readiness avoids claiming STARK verification (an R2 RPC surface).
Known: none (related to FINDINGS F17 note on text scans)

### P46-F08
File: src/tests/consensus_lock_order_loom.rs:47-239, 251-286
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
consensus_lock_order_loom.rs:56  struct Engine { seen_blocks: RwLock<u32>, checkpoints: ..., slashing_evidence: ..., epoch_seed: ... }
consensus_lock_order_loom.rs:259 .filter(|l| l.contains("RwLock<") && l.trim_end().ends_with(','))
consensus_lock_order_loom.rs:264 declared.len(), 4,
Cargo.toml:235  unexpected_cfgs = { level = "warn", check-cfg = ['cfg(loom)'] }
```
Description: The loom tests run on a hand-copied model (`Engine` of u32 locks), not on `PoSEngine`; they pass whatever pos.rs does. The only guard that ties model to engine counts field lines containing `RwLock<` and checks four names. It does not check the acquisition order, `Mutex` fields, or locks inside `signer` or `validator_keys` (pos.rs:104-105). A new call site that takes `slashing_evidence` before `seen_blocks` keeps every test green. The loom tests also run only under `RUSTFLAGS=--cfg loom` in the nightly job (security-hardening.yml:775), so a normal CI run exercises only the inventory guard.
Impact: The claim "lock order proven" holds for the model only. A lock inversion in pos.rs (R3 consensus) would not be caught.
Known: none

### P46-F09
File: src/tests/finality_adversarial.rs:24; src/tests/finality_live_path.rs:31
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence:
```
finality_adversarial.rs:24  #![allow(clippy::needless_range_loop)]
finality_live_path.rs:31    #![allow(clippy::needless_range_loop)]
git log -S'needless_range_loop' -- src/tests/finality_live_path.rs  ->  e7e3a45 (import commit)
```
Description: Two crate-level allows silence a Clippy style lint. They predate this audit (import commit), so they are not new, but Z3 forbids softening the CI by allow. The loops pair `sks[i]` with `snap.validators[i]`; a `zip` removes the need.
Impact: Z3 hygiene only.
Known: none

### P46-F10
File: src/tests/hash_input_serialize_locks.rs:30-60
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
hash_input_serialize_locks.rs:49  .filter(|(_, line)| line.contains("bincode::serialize"))
hash_input_serialize_locks.rs:50  .filter(|(_, line)| line.contains("unwrap_or_default"))
hash_input_serialize_locks.rs:46  for rel in ["src/core/transaction.rs", "src/chain/snapshot.rs"] {
rg -n -U 'serialize\([^;]*\)\s*\n\s*\.unwrap_or_default' src --glob '!src/tests/*':
src/network/proto_conversions.rs:182:  data: bincode::serialize(identity_tx).unwrap_or_default(),
```
Description: The scan needs both tokens on one line and covers two files. rustfmt splits long chains onto separate lines, so a reintroduced swallow written as `bincode::serialize(x)` then `.unwrap_or_default()` on the next line passes. Other hash-input files (block.rs, cross_domain, registry) are not scanned. A live swallow exists outside the scope: proto_conversions.rs:182 turns an Identity transaction serialize failure into empty bytes on the wire conversion (not a hash input, but the same silent-empty shape). The multi-line hits in block.rs:97-99 are guarded by `unwrap_or_else`, so they are not this bug.
Impact: The lock can stay green while the bug class returns in a wrapped line or another file. The proto site is Low (the receiver gets an empty payload and a decode error, not a hash collision).
Known: none (MODEL_ROUTING section 12 item 9 keeps the `unwrap_or_default` pattern out of focused ADIM work, so this is recorded only)

### P46-F11
File: src/tests/distributed_settlement.rs:87-145, 270-318, 320-392
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
distributed_settlement.rs:89   NodeHarness::new(5101, ...)   // fixed ports 5101-5105, 6101, 7101, 8101, 9101
distributed_settlement.rs:63   sleep(Duration::from_millis(1000)).await;
distributed_settlement.rs:123  sleep(Duration::from_secs(10)).await;
distributed_settlement.rs:313-315  let n2 = NodeHarness::new(8101, ...); header = n2...build_global_header(None); assert_ne!(header.domain_commitment_root, [0u8; 32]);
distributed_settlement.rs:348-354  err.contains("mismatch") || err.contains("empty") || err.contains("rejected") || err.contains("Rejected") || err.contains("finality")
```
Description: (a) Fixed TCP ports and fixed sleeps make the tests port-collision and timing dependent (the convergence test needs about 25 s of sleeps). (b) `test_conflicting_commitment_audit_persistence` is named for the audit record of a conflicting commitment, but after restart it only asserts the global root is non-zero; the conflict record (PoW vs PoS commitments for the same alice update) is never read. (c) The first adversarial assert accepts any of five substrings including the generic "finality", so a wrong rejection path passes. (d) `stop()` is called only at the end; a failed assert leaves the tasks and ports bound for the rest of the process.
Impact: Flaky risk in CI and a named persistence property (conflict audit trail) with no real check.
Known: none

### P46-F12
File: src/tests/fork_choice_locks.rs:40-232; src/tests/f20_priority_findings.rs:106-110
Severity: Info
Axis: test
Confidence: Certain
Evidence:
```
fork_choice_locks.rs:59-62  engine.add_checkpoint(&honest[32], None).expect("checkpoint at height 32 records");
FINDINGS.md:29              ### P01-F04 Checkpoint is written at local commit time with no finality. Critical. Confirmed.
f20_priority_findings.rs:101  assert!(!d.verify_merkle_enabled, "F-13");
f20_priority_findings.rs:107  toml.contains("verify_merkle = false")
```
Description: The fork-choice tests treat a locally added checkpoint as an absolute lower bound, which is the K3 behavior recorded in P01-F04. When K3 is fixed (checkpoint only after finality), these seven tests and their `add_checkpoint(.., None)` fixtures will need a finality cert and will fail by design. Likewise f06_f13 pins verify_merkle off in code and in config/mainnet.toml; CLAUDE.md section 9 item 2 plans to turn the 64-depth gate on (a Seviye 2 decision), and this test must change in the same step.
Impact: None today. Recorded so the fix PR for P01-F04 and the VerifyMerkle gate update these tests on purpose.
Known: STATUS.md K3 / FINDINGS P01-F04

### P46-F13
File: src/tests/finality_live_path.rs:131-156; src/tests/finality_adversarial.rs:307-338, 345-384
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
finality_live_path.rs:148  err_lower.contains("invalid") || err_lower.contains("signature") || err_lower.contains("mismatch") || err_lower.contains("height")
finality_adversarial.rs:308 fn finality_prevents_split_brain_on_partition()   // two separate aggregators, each bound to one hash
finality_adversarial.rs:372 agg.add_precommit(late).expect("new late precommit ingested");   // fn name: finality_ignores_late_votes_after_cert
```
Description: (a) The wrong-height test accepts four unrelated error words, and its own comment says the rejection is the signature check, not a height check, so no height mismatch branch is tested. (b) The split-brain test creates two aggregators with different expected hashes and checks each stays below quorum; a split-brain needs one validator set signing two hashes at one height through the production path, and that is not tried (each side has exactly 2 of 4, below 2667 by construction). (c) The late-vote test is named "ignores" but asserts that the late precommit is accepted; it does not check `signer_count` of `cert2`, so a cert that silently grew to 4 signers or kept 3 both pass.
Impact: Gaps on the finality quorum path (R3). Real double-cert safety is not exercised against `Blockchain::handle_precommit`.
Known: none

### P46-F14
File: src/tests/identity_tx_door.rs:209-213
Severity: Info
Axis: test
Confidence: Certain
Evidence:
```
identity_tx_door.rs:209  // The failed recovery still spent its nonce and paid its fee: refusal
identity_tx_door.rs:210  // is not refund, and the door must not let retries of one nonce
identity_tx_door.rs:212  assert_eq!(state.get_nonce(&subject), 1, "only the register moved it");
identity_tx_door.rs:213  assert_eq!(state.get_balance(&subject), 999, "only the register paid");
```
Description: The comment says the failed recovery spent a nonce and a fee. The asserts say it did not (nonce stays 1, balance stays 999, so only the register moved them). Either the comment is wrong or the executor lets a refused identity tx be replayed with the same nonce. The earlier test (line 109-112) says refusal before the epilogue consumes nothing, which matches the asserts, not the comment.
Impact: A reader cannot tell the intended rule for a refused-but-signed identity transaction (retry safety, fee grinding). Not duplicated in F17.
Known: related to FINDINGS F17 (same test, different point)

### P46-F15
File: src/tests/finality_adversarial.rs:548, 743; domain_edge_cases.rs:242; finality_live_path.rs:123; disaster_recovery.rs:364; genesis_repro.rs:5, 41, 52; consensus_digest.rs:19-20
Severity: Info
Axis: LABEL (Z13, language)
Confidence: Certain
Evidence: `"double-sign %50 kesmeli (10000 -> 5000)"`, `"varsayilan alan operator tasir"`, `"epoch 1 penceresi kirletilmemeli"`, `liveness_consensus.rs'te sabitlenir`, `Yoksa GenesisConfig::new fallback'i`, `Okur; produce_block timestamp'i`.
Description: Repo text must be English (Z13). These ASCII Turkish phrases sit in comments and assert messages. None of the files is in .github/turkish-baseline.txt, so the language gate does not catch ASCII Turkish without special characters. Also "L1 account Merkle trie" in hardening_h2_locks.rs:104,106 uses the banned term (Z12 targets public text; this is a code comment).
Impact: Language hygiene only.
Known: none

## Coverage gaps against known findings (no new id)
- FINDINGS.md:14 (no-leader height with equal stakes): no test in the 23 files runs more than one PoS validator through block production; domain_edge_cases.rs:124 is the named but empty slot (F03).
- P01-F04 / K3: fork_choice_locks.rs encodes the current behavior (F12).
- No test in this part checks a mainnet-profile genesis pin: genesis_repro.rs compares only same-build runs. This is covered elsewhere (genesis.rs:1262 compares mainnet JSON; docker_smoke.rs pins devnet), so not a finding.
- consensus_digest.rs prints a digest but pins none; cross-OS equality lives in determinism.yml (not read). Info only.
