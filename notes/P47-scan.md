# P47 scan: src/tests 3 (read-only audit, Tier R1)

Base commit: 3df8863. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14), notes/P45-scan.md.

## COVERAGE

Read in full, every line:
- src/tests/integration.rs 1-1899 (two reads)
- src/tests/liveness_consensus.rs 1-449
- src/tests/load_test.rs 1-248
- src/tests/manifest_commitment_locks.rs 1-1616 (two reads)
- src/tests/mempool_dos_locks.rs 1-184
- src/tests/migration_v2.rs 1-116
- src/tests/mod.rs 1-213 (printed whole by cat)
- src/tests/multi_consensus_locks.rs 1-419
- src/tests/network_hardening_locks.rs 1-537
- src/tests/operator_kill_chaos.rs 1-370
- src/tests/permissionless.rs 1-1066
- src/tests/permissionless_e2e.rs 1-64

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md 141-165 (section 6); notes/P29-scan.md 1-30.

Outside the part (rg first, at most 80 lines): src/lib.rs 48-54; src/registry/params.rs (rg min_stake); src/core/chain_config.rs (rg min_stake); src/chain/blockchain.rs (rg overlay roots); src/domain/storage_deal.rs (rg open_deal, verify_coding_audit 2310-2335); src/network/node.rs (rg resolve_dns_seeds 355-377, rg identity key write, rg max(1), rg multi-line lock unwrap); docs/audit/FINDINGS.md and STATUS.md (rg only).

Extra-check results:
- #[ignore] and #[allow]: none in the 12 files (rg). Z3 clean. The only attribute of note is `#[should_panic(expected = ...)]` at integration.rs:736, which is specific.
- Hardcoded secrets (Z9): none. Test-only fixed BLS scalars (sk_bytes[0] = 7, 9, 42) at integration.rs:391, 520, 589, 650, 769, 836 and fixed address bytes are not secrets. No token, seed or key path found.
- Orphan test files: every `src/tests/*.rs` file has a `mod` line in mod.rs and every `mod` line has a file (shell loop, no output). `tests` is `#[cfg(test)]` in lib.rs:52, so the mixed `pub mod` lines in mod.rs are harmless.
- Vacuous or masking asserts: F05 to F12, F15.
- Tests pinning a production defect as green: F02, F03.
- Coverage gaps against known findings: F04, F13, F14, F17.

## FINDINGS

### P47-F01
File: src/tests/load_test.rs:136-159
Severity: Medium
Axis: test
Confidence: Likely (missing step: remove the masking in CI and see whether the roots differ)
Evidence:
```
// Those are projections from Blockchain-level structures captured
// As-of-production-height; the reload replay loop does not (yet) mirror
// Them per-block ...
live_masked.bridge_root = [0u8; 32];
live_masked.message_root = [0u8; 32];
live_masked.settlement_root = [0u8; 32];
live_masked.global_header_summary = [0u8; 32];
(same four assignments on replay_h, then assert_eq! on calculate_state_root)
```
Description: The restart-and-replay state root test zeroes four root fields on both sides before comparing. The comment says replay does not mirror them. blockchain.rs:1915-1919 now derives bridge_root, message_root, settlement_root on one replay entry, so the comment may be stale, but the mask hides any divergence either way.
Impact: AGENT_MAP says live and replay must give the same state root. A live node and a restarted node can disagree on the full root and this 1000-tx test stays green. A fork after restart would not be caught.
Known: none

### P47-F02
File: src/tests/manifest_commitment_locks.rs:1056-1081
Severity: Medium
Axis: security
Confidence: Certain (the test itself states the gap)
Evidence:
```
fn a_coding_audit_answer_is_not_bound_to_the_column_that_was_asked() {
  ...
  let forged = CodingAudit { column: substituted, ..asked };
  assert!(reg.verify_coding_audit(&forged, &column, parity_byte).is_ok(),
      "an answer at a self-chosen column passes today; ...
```
storage_deal.rs:2310-2335: verify_coding_audit reads parity_index from the audit struct and never recomputes the column from entropy.
Description: The test locks in a known hole: the responder picks the column that is checked. The doc in the test (1027-1054) says "Not fixed here". The hole is not in FINDINGS.md (rg for CodingAudit and column returns only the P23-F06 unwired note).
Impact: A storage operator that keeps valid parity at one column and garbage elsewhere passes every coding audit once the audit is wired (STATUS items 4b and 4c). The (1-f)^rounds argument in the module doc no longer holds. A green test hides this open item.
Trigger: precondition, a wired AnswerCodingAudit; entry point chain_actor.rs AnswerCodingAudit arm; input, a CodingAudit with a self-chosen column plus the matching data column and parity byte; expected wrong result, Ok(()).
Known: none (STATUS says the audit is unwired, not that the column is unbound)

### P47-F03
File: src/tests/multi_consensus_locks.rs:284-303
Severity: Medium
Axis: security
Confidence: Likely (missing step: whether domain registration can leave validator_set_hash at zero for a non-bridge PoA or StorageAttestation domain)
Evidence:
```
fn a_domain_with_no_registered_set_keeps_the_old_behaviour() {
  let domain = domain_with(ConsensusKind::PoA, "poa-authority-quorum", [0u8; 32]);
  let proof = poa_proof(&domain, &commitment, &[&keys[0], &keys[1]], &addrs);
  ... == FinalityStatus::Finalized,
  "an unregistered set falls back to the quorum check alone"
```
Description: With a zero validator_set_hash and bridge_enabled false, the adapter accepts an authority list named inside the proof and sizes the quorum from it. The test pins that as intended. The attack described at lines 184-189 (self-nominated three keys, two signatures) works on this branch by design.
Impact: Any caller who can submit a commitment for such a domain finalizes it with keys it owns. Only the bridge path is protected (310-368). Domain state updates, event roots and cross-domain messages that do not go through the bridge rely on this finality.
Trigger: domain with zero set hash and bridge_enabled false; entry point PoAFinalityAdapter::verify_finality; input, FinalityProof::PoA with attacker authorities; expected wrong result, Finalized.
Known: none

### P47-F04
File: src/tests/permissionless.rs:967-984
Severity: Medium
Axis: integration
Confidence: Likely (missing step: confirm in CI that no boot path calls registry.set_params from Network::min_stake)
Evidence:
```
fn mainnet_min_stake_floor_for_onboarding() {
    assert_eq!(Network::Mainnet.min_stake(), 1_000_000);
    // Registry default min_stake may differ from network consensus min_stake;
    ...
    assert_eq!(g1.hash, g2.hash);
```
registry/params.rs:296: `min_stake: 1_000,`. chain_config.rs:129: mainnet `min_stake: 1_000_000`. rg for set_params outside tests and registry finds only the governance path account.rs:1861.
Description: The test is named a floor test but asserts no floor. It asserts a constant and genesis determinism. The comment admits the registry floor and the network floor differ. All onboarding tests (201, 906, 988, 1030) use `registry.params().min_stake`, so none exercises the 1_000_000 mainnet value.
Impact: A mainnet account that stakes 1_000 becomes an active VALIDATOR in the registry, which gates liveness slashing, invalid votes and active_members. The mainnet economic floor is 1000 times lower than the documented one.
Known: none

### P47-F05
File: src/tests/permissionless.rs:1008-1015
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
let _ = bc.add_transaction(tx);
let _ = bc.produce_block(staker);
assert!(!bc.state.registry.is_active(&staker, roles::VALIDATOR), ...
```
Description: Both results are dropped. If add_transaction fails for any reason (fee, nonce, chain id) the staker is never registered and the assert passes without testing the floor. The unit test at 232 covers the Executor path only.
Impact: The end-to-end below-floor guard can pass for the wrong reason.
Known: none

### P47-F06
File: src/tests/operator_kill_chaos.rs:246-252
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
assert!(
    reg.objects_needing_repair(2).iter().all(|(id, _, _)| id != &manifest.manifest_id)
        || reg.live_shard_count(&manifest.manifest_id) >= 4,
```
Description: Line 241 already asserts live_shard_count == 5, so the right side is always true. The assert cannot fail.
Impact: The "object still at or above k" check is a no-op.
Known: none

### P47-F07
File: src/tests/integration.rs:322-369
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
if validators.len() < 2 { return; }
...
if expected_my_slot.address == pubkey1 {
    let result = engine.prepare_block(&mut block, &state);
    assert!(result.is_ok());
```
Description: test_poa_round_robin_signed returns early or skips its only assertions on two branches, and it never checks that a block signed by the wrong slot is refused. With two validators the guard is true, but a regression in expected_proposer would skip the check silently.
Impact: The round-robin test does not fail when round-robin breaks.
Known: none

### P47-F08
File: src/tests/integration.rs:256-277
Severity: Low
Axis: test
Confidence: Likely (missing step: whether the second add_transaction error is the nonce check or the duplicate check)
Evidence:
```
blockchain.add_transaction(tx1.clone()).unwrap();
let _ = blockchain.produce_block(miner);
let result = blockchain.add_transaction(tx1);
assert!(result.is_err(), "Replay attack should be prevented");
```
Description: The produce_block result is dropped and the chain length is not checked. If no block is produced, tx1 is still in the mempool and the second add fails as a duplicate, so the assert passes without testing nonce replay after inclusion. The error text is also not checked.
Impact: Replay protection after inclusion may be untested.
Known: none

### P47-F09
File: src/tests/network_hardening_locks.rs:253-266, 200-203
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
assert!(NODE_RS.contains(".max(1)"), "the derived tick counts lost their floor ...
assert!(NODE_RS.contains("check_rate_limit"), ...
```
rg -c "\.max\(1\)" src/network/node.rs returns 2 hits in the whole file.
Description: These checks search the whole file, not the heartbeat code, so any other `.max(1)` or `check_rate_limit` keeps them green. The lock-unwrap test at 59-75 only matches the single-line text `.lock().unwrap()`; `.read().unwrap()`, `.write().unwrap()`, `.lock().expect(` and a rustfmt-split `.lock()` then `.unwrap()` are not caught (rg multi-line currently finds none, so the code is clean today). code_lines also does not strip string literals as its doc says, only text after `//`.
Impact: The locks give weaker protection than their comments claim. Most other tests in this file are fixed-window text greps and break on harmless refactors (windows of 400 to 3000 bytes at 97, 143, 274, 345, 372, 427).
Known: none

### P47-F10
File: src/tests/integration.rs:474-490
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
assert!(
    err.contains("conflicts with finalized checkpoint")
        || err.contains("height discontinuity")
        || err.contains("Block height discontinuity")
);
```
Description: The test is about finality checkpoint enforcement, but it passes on a plain height error. The comment says "accept either if ordering changes". The conflicting block (index 10, prev "wrong_prev") fails a cheap check first, so the checkpoint rule may never run.
Impact: Removing the finalized-checkpoint conflict check would not fail this test.
Known: none

### P47-F11
File: src/tests/liveness_consensus.rs:353-383, 400-449
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
fn every_liveness_path_expects_the_same_validators() { ...
    bc.observe_liveness_epoch(10, &only_producer);
    bc.record_liveness_epoch(11, &only_producer);
    assert_eq!((before, after_observe, after_record), (before, before, before), ...
```
Description: The doc (347-352) names three call sites. The test drives two (Blockchain::observe_liveness_epoch and record_liveness_epoch). apply_epoch_close_liveness and AccountState::record_liveness_epoch are not called. Also a_liveness_slash_is_releasable (401) checks jail_until but never advances epochs to prove advance_epoch frees the validator, which is the bug in its doc.
Impact: The release path that the doc calls permanent-ban prevention has no end-to-end assert.
Known: none

### P47-F12
File: src/tests/integration.rs:627-734
Severity: Info
Axis: test
Confidence: Certain
Evidence: fn test_qc_fault_proof_invalidates_cert_without_slashing ... `assert!(validator.slashed);` (line 732, comment "InvalidDilithium fault proof slashes the offender").
Description: The name says no slashing, the body asserts slashing. Test 494 (`..._does_not_slash_validator`) covers the opposite case, so the pair is correct but the name is wrong and misleads readers.
Impact: Maintenance risk only.
Known: none

### P47-F13
File: src/tests/operator_kill_chaos.rs:1-19, 351-370; src/tests/mod.rs:95-96
Severity: Info
Axis: invariant(Z13)
Confidence: Certain
Evidence: operator_kill_chaos.rs:7 `//! What it is **not** yet:`; 355 `let not_yet = "What it is ".to_string() + "**not** yet";`. mod.rs:95 `// L1 relayer proof kripto-doorulama + M5 budlumxyz fee + M4 BNS fee`, 96 `// Regresyon kapilari (Q-A, 2026-07-16).` multi_consensus_locks.rs:8, 12, 16 also use `**` bold.
Description: Z13 forbids markdown bold and non-English repo text. The test all_three_baraj_is_not_claimed_by_this_skeleton requires the bold string, so fixing Z13 breaks it. mod.rs has Turkish text and the words "L1 relayer" (Z12 is for public text; this is a repo comment). The test is also a meta-test of a doc comment and proves nothing about behaviour; its own text says the half of all_three it names is still open.
Impact: Directive drift; a fix needs the test changed in the same commit.
Known: none

### P47-F14
File: src/tests/integration.rs:1573-1584, 1673, 1256-1266, 1866-1884
Severity: Info
Axis: test
Confidence: Certain
Evidence: `let _ = metrics.finality_lag.get();` (1673). test_rpc_security_config_with_body_limits builds a struct with two fields and asserts the same two values. test_no_bls_key_returns_none (1257) repeats 1867 and 1877 for PoW and PoA.
Description: Tautological and duplicate tests that cannot fail on a real regression.
Impact: Inflated test count only.
Known: none

### P47-F15
File: src/tests/load_test.rs:163-248
Severity: Low
Axis: test
Confidence: Certain
Evidence: `let rust_result = (10u64 + 20u64) * 2u64;` against `receipt.events[0]` from one 7-instruction program.
Description: The "differential VM oracle" has one straight-line program and a constant oracle. It has no branch, memory, overflow, division by zero or out-of-gas case, and the test is named chaos.
Impact: The VM-to-native differential claim has almost no coverage here. Other files may cover the VM; not checked in this part.
Known: none

### P47-F16
File: src/tests/integration.rs:1344-1362
Severity: Low
Axis: test
Confidence: Likely (missing step: CI resolver behavior for .test domains)
Evidence: `resolve_dns_seeds(&["localhost".to_string()], 4001)` and `resolve_dns_seeds(&["invalid.nonexistent.domain.test".to_string()], 4001)`; node.rs:359 uses blocking std ToSocketAddrs.
Description: Both tests depend on the host resolver. The second can block for the resolver timeout in a sandbox and the first can return only IPv6 or nothing without a hosts file. node.rs:355-377 resolves on the calling thread.
Impact: Flaky CI or slow test; no check that the function is non-blocking.
Known: none

### P47-F17
File: src/tests/integration.rs:1319-1342
Severity: Low
Axis: security
Confidence: Likely (missing step: confirm the window in node.rs:310-319)
Evidence: test asserts only `path.exists()` and equal PeerId. node.rs:310 `std::fs::write(file_path, &encoded)` and node.rs:319 `set_permissions(file_path, ... from_mode(0o600))`.
Description: The identity key is written with default umask permissions and then chmoded, so there is a short window where the file may be group or world readable. The test does not assert mode 0600 either way.
Impact: Local user could read the node's libp2p identity key during that window; no test would notice a missing chmod.
Trigger: precondition, a shared host with permissive umask; entry point load_or_generate_identity_key first run; input, none; expected wrong result, key readable by another local user.
Known: none

### P47-F18
File: src/tests/manifest_commitment_locks.rs:50-63; src/tests/operator_kill_chaos.rs:51-62
Severity: Info
Axis: test
Confidence: Certain
Evidence: `proof_bytes: vec![0xABu8; 96], backend: "test-backend"` in both files; storage_deal.rs:1930-1933 `Self::validate_merkle_proof_format(proof_bytes, &root)?;`.
Description: Every deal-open fixture in this part uses the same forged envelope, and open_deal validates shape only. No test in the part has a case where a bad proof is refused or a good one verified. The helper is copied twice.
Impact: The VerifyMerkle gate (CLAUDE.md section 9 item 2) has no coverage in these files. STATUS 4.4 lists the proof check as waiting (Q5), so this is known work.
Known: STATUS.md (Waiting for the proof check, Q5)

### P47-F19
File: src/tests/mempool_dos_locks.rs:165-183; src/tests/manifest_commitment_locks.rs:795-880, 1217-1262
Severity: Low
Axis: test
Confidence: Certain
Evidence: `let body = &src[at..at + 400];` then `body.find("validate_pool_transaction")` and `body.find("self.mempool")`; chain_actor handler checks use `handler.contains("last_block().hash.as_bytes()")` on the first 2000 bytes after a fixed anchor string.
Description: These locks check text order or presence inside a byte window. A comment, a dead branch or a renamed local keeps them green; a longer function signature pushes the call out of the 400-byte window and turns them red without a behaviour change. The behaviour tests in mempool_dos_locks.rs:60-157 already cover the real property.
Impact: Maintenance noise; no protection against a real regression beyond the behaviour tests.
Known: none

### P47-F20
File: src/tests/permissionless_e2e.rs:50-59; src/tests/liveness_consensus.rs:1-10
Severity: Info
Axis: test
Confidence: Certain
Evidence: permissionless_e2e.rs asserts only `absentee_reg.stake < 20_000` and not-active; liveness_consensus.rs header says "crossing the miss threshold is logged/reported But NEVER slashed" while tests at 206 and 253 assert real slashing.
Description: The module doc of liveness_consensus.rs is stale (observe mode is now one of two modes). The e2e test does not check the exact penalty or that the producers' stake is unchanged.
Impact: Doc drift; weak e2e assert.
Known: none

## Summary

20 findings: Critical 0, High 0, Medium 4 (F01 to F04), Low 11 (F05 to F11, F15, F16, F17, F19), Info 5 (F12, F13, F14, F18, F20).
