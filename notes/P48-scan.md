# P48 scan: src/tests 4 (read-only audit, Tier R1 tests over R3 paths)

Base commit: 3df8863. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14), notes/P45-scan.md.

## COVERAGE

Read in full (every line), files 53 to 70 of `ls src/tests/*.rs`:
- src/tests/persistence.rs 1-336
- src/tests/poa_isolation.rs 1-472
- src/tests/poa_onboarding_matrix.rs 1-246
- src/tests/poisoned_lock_locks.rs 1-244
- src/tests/pollen_ai_data_rights.rs 1-345
- src/tests/ported_slice_redteam.rs 1-675
- src/tests/pow_light_client.rs 1-328
- src/tests/privacy_ai_execution.rs 1-218
- src/tests/private_transfer_fee_market.rs 1-61
- src/tests/proptest_core.rs 1-39
- src/tests/prover.rs 1-709
- src/tests/qcblob_quorum.rs 1-201
- src/tests/real_chain_fixtures.rs 1-295
- src/tests/regeneration_recovery.rs 1-240
- src/tests/regression_lock.rs 1-717
- src/tests/relayer_e2e.rs 1-543
- src/tests/relayer_gates.rs 1-487
- src/tests/relayer_liveness.rs 1-529

Docs read: docs/AGENT_MAP.md (head), docs/AUDIT_PLAN.md 110-165, notes/P29-scan.md 1-30.

Outside the part (rg first, at most 80 lines each): src/chain/blockchain.rs 3464-3560 (submit_registry_slashing_report) and 3885-3960 (import_qc_blob); src/consensus/qc.rs 197-340 (verify_against_snapshot); src/execution/executor.rs 55-80, 1036-1110, 2023-2075; src/ai/registry.rs 1562-1640; src/rpc/server.rs 3125-3140; src/registry/evidence.rs 470-490; src/prover/mod.rs 350-395; src/core/account.rs 2215-2230; src/privacy/submit.rs 33-108; rg only for: PoaMembershipRegistry, PoAOnboarding, IdentityTx, RegenerationLedger, register_relayer, privacy_mainnet_disabled, baseline files, docs/audit/FINDINGS.md and STATUS.md.

Not read: the rest of src/tests, src/chain/snapshot.rs beyond rg, src/network/node.rs (poisoned_lock_locks.rs scans it as text), src/registry/identity.rs beyond rg. No cargo, no test run. Every claim comes from reading source.

Extra-check summary:
- Z3: no `#[ignore]` and no `#[allow(` in the 18 files. The only rg hit is the word "ignored" in a comment (relayer_liveness.rs:179).
- Z9: no secret. Test keys are `[byte; 32]` seeds through `KeyPair::from_seed` (pollen_ai_data_rights.rs:11) and `KeyPair::generate` (random). The 64-hex literals are `Address::from_hex("00..80")` fills (regression_lock.rs:270-650) and the public keccak(rlp([])) constant (real_chain_fixtures.rs:219).
- Vacuous or wrong-reason tests: P48-F01, F02, F05, F06, F08, F10, F11, F12, F13, F14, F16, F17, F19.
- Test masks a production behavior: P48-F04 (garbage-signature report slashes), P48-F01.
- Coverage gaps against known findings: P48-F03 (privacy mainnet gate), F05 (known P14 F07 and F09 sit under these tests), F15.
- Z12/Z13: P48-F20.

## Findings

### P48-F01
File: src/tests/qcblob_quorum.rs:157-201 (with src/consensus/qc.rs:292-312, src/chain/blockchain.rs:3936-3946)
Severity: Medium
Axis: test
Confidence: Likely (missing step: run `PqKeyPair::verify` with a 32-byte key and an 8-byte signature to see the Err; its source was not read)
Evidence:
qcblob_quorum.rs:55 `pq_public_key: vec![0xAAu8; 32],` and :96 `dilithium_signature: vec![0x01u8; 8],`
qc.rs:292-312 (loop, per entry): `if !verified_indices.insert(idx) { return Err("Duplicate PQ signature ...") }` then `PqKeyPair::verify(&validator.pq_public_key, &message, &entry.dilithium_signature).map_err(...)?;`
blockchain.rs:3939-3946: `let verified_signers = blob.verify_against_snapshot(...)?; if verified_signers.len() < min_signers { return Err(...) }`
Description: Every blob in this file carries an 8-byte filler instead of a Dilithium signature, so the first entry fails at `PqKeyPair::verify` inside `verify_against_snapshot`. `import_qc_blob` returns before it reaches the `verified_signers.len() < min_signers` comparison. Tests `import_qc_blob_rejects_below_quorum_signature_count` and `duplicate_signatures_do_not_inflate_the_quorum` only assert `result.is_err()`, so they pass for the signature reason. The canary comment at 179-180 ("revert the quorum comparison ... this test is the one that fails") is false: the revert would leave both green. The fixture canary at 121-139 checks the empty set only (blob with 0 entries), not the 1-entry and duplicate blobs. The header (2-4) promises "exactly the threshold must be accepted"; no such positive test exists.
Impact: The 2/3 unique-signer quorum on QC blob import (R3 finality path, P09) has no test that reaches it. A regression of the comparison, or of the dedup order, stays green.
Known: none (P09 notes the quorum fix itself, not this test gap).

### P48-F02
File: src/tests/relayer_gates.rs:170-183 (with src/execution/executor.rs:1077, 1099)
Severity: Low
Axis: test
Confidence: Likely (missing step: confirm no earlier return than 1077 for this input; 1043-1077 were read)
Evidence:
relayer_gates.rs:175-181 `res.receipt_proof = vec![1, 2, 3]; res.external_state_root = [0x11; 32]; ... assert!(!err.is_empty(), "the error text must not be empty");`
executor.rs:1077 `"external state root has no finalized light-client anchor"` comes before :1099 `BudlumError::validation("relayer_proof_malformed", e.to_string())`.
Description: The test never inserts an anchor in `state.external_roots`, so the transaction is refused at the anchor check (1077), before the proof decode at 1099. The assert accepts any non-empty text. The comment (179-180) says the bincode error text varies; the real reason is that the decode is never reached.
Impact: A decoder that accepts garbage bytes, or a removed decode error path, keeps this test green. The sibling oversized test (202-213) is correct because its size check (1054) comes first.
Known: none.

### P48-F03
File: src/tests/privacy_ai_execution.rs:28-141 (with src/execution/executor.rs:2023-2075, 1999-2003)
Severity: Medium
Axis: test
Confidence: Certain (rg shows the codes only in executor.rs and blockchain.rs:4177)
Evidence:
`rg -n "privacy_mainnet_disabled" src` returns executor.rs:2001, 2026 and blockchain.rs:4177 only. `rg -n "private_transfer_auth|private_transfer_digest" src/tests` returns nothing.
executor.rs:2026-2029 `"private transfers are disabled on mainnet until ownership, value-conservation, and membership proofs are wired"`
Description: The only gate that keeps `PrivateTransferSubmit` and `PrivacyNoteInsert` off a chain with real value (the allowlist in `privacy_transfers_enabled`, executor.rs:63-87) has no test. The three tests here run on `DEFAULT_CHAIN_ID` and only the positive path (spend and create, nullifier reuse, note insert). No test sends a bad `authorization_sig`, a `public_digest` mismatch, a spent commitment missing from the registry, or a mainnet or unknown chain id.
Impact: Changing the allowlist back to a denylist, or dropping the gate, passes CI. Ownership, value conservation and membership proofs are unwired (known), so the gate is the only protection of the note registry on mainnet.
Known: ownership and proofs unwired is in FINDINGS (P17 section, executor.rs:63-87 Info). The missing test is not listed.

### P48-F04
File: src/tests/relayer_liveness.rs:151-170, 215-312, 453-467, 471-529 (with src/chain/blockchain.rs:3490-3560)
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
relayer_liveness.rs:139-148 `SlashingReport::consensus_double_sign(offender, 7, "aa".into(), "bb".into(), vec![1], vec![2], reporter)` (fake signatures and hashes). :165-167 `submit_registry_slashing_report(...)` returns `outcome.is_some()`. :466 `assert_eq!(outcome.penalty, stake / 2);`
relayer_liveness.rs:295-297 comment: "The RPC layer overwrites this to `Unverified`; asserted here to show the chain layer does not depend on the RPC layer to do so." The report below it is built with `ProofProvenance::Unverified` (297).
blockchain.rs:3490 refuses only `provenance == Unverified`; 3519 `slash_from_report` does no signature check for `ConsensusVerified`.
Description: The chain method trusts the `provenance` field. Four tests slash a validator (50 percent) from a report whose signatures are `vec![1]` and `vec![2]` and whose hashes are `"aa"` and `"bb"`. The one test that is named for a forged report (`a_stranger_cannot_slash_a_validator_by_asserting_provenance`, 261-312) builds it as `Unverified`, so it proves the Unverified refusal, not that a self-asserted `ConsensusVerified` report is refused. The comment claims the opposite.
Impact: No test pins that a caller-supplied `ConsensusVerified` with forged data is refused at the chain layer. Today only the RPC layer (server.rs:3136) and the consensus path (blockchain.rs:5811-5881) feed this method, so the exposure is a new caller, not an open hole.
Known: P14 F24 (endpoint always fails, Info) covers the reachability, not this test claim. The fee-and-refund tests at 151-170 and 215-258 run a path (`Some(reporter)` with `ConsensusVerified`) that no production caller can produce, because the RPC forces `Unverified`.

### P48-F05
File: src/tests/poa_isolation.rs:24-286 (with src/core/account.rs:297, 2218-2230; .github/idle-code-baseline.txt:1790-1794)
Severity: Medium
Axis: invariant(Z11)
Confidence: Certain
Evidence:
poa_isolation.rs:26-40 `let perm_state = AccountState::new(); let mut poa_reg = PoaMembershipRegistry::new(); ... poa_reg.approve(POA_DOMAIN, admin, poa_member) ... assert!(!perm_state.registry.is_active(&poa_member, roles::VALIDATOR))`
poa_isolation.rs:206-215 `let poa_domain: DomainId = POA_DOMAIN; let permissionless_domain: DomainId = 1; assert_ne!(poa_domain, permissionless_domain, ...)`
account.rs:297 `pub poa_onboarding: crate::registry::poa_onboarding::PoAOnboarding,` (the production owner of the PoA state; the tests never touch it)
Description: Seven of the eight tests in `poa_isolation_tests` build a standalone `PoaMembershipRegistry` or `PoAOnboarding` and a fresh `AccountState` and never link them. A fresh `AccountState::new()` has no validator, so "`is_active` is false" cannot fail whatever the PoA code does. Further:
- Scenario 3 (99-132) checks that the JSON of a `CrossDomainMessage` has no "kyc" substring. The struct has no such field, so the check is true by construction.
- The labels "RPC leak", "event leak", "log leak", "error message leak" (headers of tests 1, 2, 4, 5) test none of those channels. Scenario 5 contains no error.
- The `assert_ne!` at 212 compares two constants.
- `PoaMembershipRegistry` has no production user (rg shows only its own module and mod.rs re-export; its write methods are in idle-code-baseline.txt:1790-1794).
Only `poa_whitelist_independent_of_permissionless_stake` (242-286) tests a two-way property, and it also uses a detached `PoAOnboarding::new()`, not `state.poa_onboarding`.
Impact: The test-name gate for Z11 ("cargo test --lib poa_isolation", per poa_onboarding_matrix.rs:8-11) cannot fail on a real leak: for example a code change that copies a PoA member into `state.registry`, or that lets `state.poa_onboarding` admit a staking validator.
Known: P14 F07 (PoA write side has no production caller) and F09 (freeze_poa_account, whose `poa_compliance_gate` tests are at 289-472) are known. The vacuity of the isolation gate is not listed.

### P48-F06
File: src/tests/prover.rs:683-709
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
let outcome = bc.submit_zk_proof(submission(sender, 1, 10, &proof, &exact, &program));
if let Err(e) = &outcome {
    assert!(!e.contains("declared limit"), ...);
}
```
Description: The test passes on `Ok` and on every `Err` whose text lacks "declared limit". `exact` changes `gas_limit` and `gas_used` after the proof was made, so the binding hash or the proof check most likely refuses it for another reason; the test then asserts nothing. It also depends on the exact wording of another test's error ("gas", 630). A gate that wrongly refuses with a different message stays green.
Impact: The `>` versus `>=` boundary the comment (685) names is not tested.
Known: none.

### P48-F07
File: src/tests/prover.rs:1-9, 119-168, 249-280
Severity: Info
Axis: test
Confidence: Certain
Evidence:
prover.rs:1-8 header lists "registered prover: valid proof accepted AND rewarded". Tests assert `ProofAcceptance::Accepted { rewarded: false, reward: 0 }` for both the unregistered and the registered prover (132-137, 160-166, 263-269). `rewarded: true` appears only inside the unit test in src/prover/mod.rs:366.
prover.rs:272-277 `assert_eq!(second, ProofAcceptance::Idempotent); assert_eq!(bc.state.get_balance(&prover), after_first - fee);`
Description: Under the fee-only policy `rewarded` and `reward` are always false and 0; the registered-prover test (145-168) is identical to the unregistered one except for the bond, so it checks no prover-specific behavior. Also the idempotent resubmission is charged the fee again (277), which the test locks in as expected. The fields and the test header are stale.
Impact: Dead fields; a future reward path has no test. A resubmitter pays for a no-op (the sender's own cost).
Known: none.

### P48-F08
File: src/tests/relayer_e2e.rs:74-98, 219-236, 238-272, 379-443
Severity: Low
Axis: test
Confidence: Certain
Evidence:
:83-84 fresh chain `assert_eq!(bc.pending_relay_count(), 0); assert_eq!(bc.expired_relays().len(), 0);`
:95-97 `let root1 = bc.relay_ledger_root(); let root2 = bc.relay_ledger_root(); assert_eq!(root1, root2);`
:219-236 test `relay_ledger_root_changes_with_relays` ends with `assert_eq!(root_before, root_after_enqueue);` (asserts no change)
:271 `assert_eq!(bc.expired_relays().len(), 0); // not expired yet (chain len < 200)` (test `expired_relays_detects_past_expiry`)
:437-440 `assert!(!err.is_empty(), ...)` for the tampered relay proof
Description: Five tests cannot fail on a real defect. `relayer_enqueues_and_tracks_pending_relays` and `relayer_root_is_deterministic` check an empty chain and one pure function called twice. `relay_ledger_root_changes_with_relays` is named for a change and asserts equality. `expired_relays_detects_past_expiry` never makes the relay expire, so detection is not tested. The invalid-proof test accepts any error text (a wrong reason such as a missing commitment passes).
Impact: The relay expiry and the relay ledger root (consensus root) have no effective test. Missing negatives in the file: relayer not registered or slashed submitting `submit_relay_proof`, wrong `target_domain` argument, expired relay.
Known: none.

### P48-F09
File: src/tests/relayer_e2e.rs:294-301, 347-358, 373-376
Severity: Info
Axis: test
Confidence: Certain
Evidence:
:298-301 `bc.state.registry.register_relayer(relayer, 50_000_000, epoch).unwrap();` and the comment at 356-358 "registry stake tracking is bookkeeping-only at this layer; it does NOT debit the relayer's native balance".
relayer_liveness.rs:77 uses `bc.state.bond_relayer(&relayer, 2_000)` (the path that debits balance, account.rs:683-695).
Description: The relay cycle tests register the relayer through the registry API with no balance debit and assert the unbacked balance (`100_000_000 + fee`). Production onboarding goes through `bond_relayer`. The relay path is therefore never run with a relayer whose stake is real.
Impact: A bug in how the bonded balance interacts with relay fee credit is not covered. Low, test fixture only.
Known: none.

### P48-F10
File: src/tests/real_chain_fixtures.rs:84-112, 150-166, 232-255, 257-295
Severity: Low
Axis: test
Confidence: Certain
Evidence:
:84 `fn btc_merkle_root(txids: &[String]) -> String` and :71 `fn sha256d` are defined in the test file; the merkle test (158) calls only them. The halving test (233-255) only compares numbers in the fixture. The last test (258-295) checks hex lengths of fixture fields. Only `decode_header` (180, 227) is a production call.
Header (9-12): "prove that our own hash, merkle and RLP implementations match real chain values".
Description: Same class as P45-F14. The Bitcoin merkle, halving and field-length tests do not call any production code, so they cannot catch a production regression. No production Bitcoin merkle code exists under src/cross_domain except chain_adapter.rs (rg `btc|bitcoin`), so the claim in the header is only true for the ETH header hash. The MPT receipt proof, sync committee and receipt root checks are not covered by "real chain" values (see P14 evm/mpt F03).
Impact: The fixture-integrity story is weaker than the header says. No effect on funds.
Known: same class as P45-F14; P14 F03 for the MPT gap.

### P48-F11
File: src/tests/persistence.rs:35-40, 313-336
Severity: Low
Axis: test
Confidence: Likely (missing step: read snapshot.rs from_bytes and from_snapshot_v2 for the full field list)
Evidence:
:35-40 `round_trip` returns an `AccountState`; every test asserts single fields (is_active, stake, counters) and none compares `state_root()` or a hash before and after.
:319-336 `serde_json_failure_is_a_real_class_now_surfaced_not_swallowed` serializes a `HashMap<[u8; 4], u32>` and asserts `is_err()` and that `unwrap_or_default()` is empty; no budlum code is called.
Description: A field added to `AccountState` and left out of `StateSnapshotV2` is not caught, because no test compares the whole-state root across a restore (AUDIT_PLAN replay equals live). No test restores truncated, tampered (`snapshot_hash` mismatch) or garbage bytes, or a schema above `CURRENT_STATE_SNAPSHOT_SCHEMA_VERSION`. The last test proves a serde_json property.
Impact: Snapshot persistence format of an R3 path is tested for chosen fields only. Other snapshot files (migration_v2.rs, finality_adversarial.rs, integration.rs, not read) may cover the negative cases.
Known: none.

### P48-F12
File: src/tests/proptest_core.rs:21-38
Severity: Low
Axis: test
Confidence: Certain
Evidence:
:28-29 `let expected_cost = amount.saturating_add(fee); prop_assert_eq!(tx.total_cost(), expected_cost);` with transaction.rs:1123-1125 `self.amount.saturating_add(self.fee)`.
:36 `let _ = bincode::deserialize::<Transaction>(&bytes);`
`rg "bincode::deserialize::<Transaction>"` outside this file: no hit. The wire format is `ProtoTransaction` (protocol.rs:182).
Description: The cost property repeats the implementation, so it cannot fail, and it locks saturation: `amount = u64::MAX, fee = 1` gives `total_cost == u64::MAX`, not an overflow error. The "no panic" property tests bincode, which no production network path uses; the real decoder (`Transaction::try_from(ProtoTransaction)`) has no property test. The test body has no assert by design.
Impact: The decode of untrusted bytes (R2 network input) is not fuzzed through the real path. A comparison `balance >= total_cost` with a saturated cost is exact only because supply is far below `u64::MAX` (not checked here).
Known: none.

### P48-F13
File: src/tests/regeneration_recovery.rs:25-36, 108, 178
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
const RECOVERY_WINDOW_BLOCKS: u64 = 64;
const _: () = assert!(RECOVERED_HEIGHT <= DAMAGE_HEIGHT + RECOVERY_WINDOW_BLOCKS, ...);
const DAMAGE_HEIGHT: u64 = 300;  const RECOVERED_HEIGHT: u64 = 320;
```
Comments at 108 and 178: "(acceptance window pinned at compile time, see RECOVERED_HEIGHT)".
Description: The "recover within N blocks" acceptance is a compile-time check between two constants chosen by the test (320 <= 364). It measures no production recovery time. The scenario heights come from the test's own `canonical` closure. Test 3 (183-218) documents that reversion heals the node onto an attacker's canonical line when the anchor source is compromised, and defers to a "dormant-anchor / cold-committee channel"; that channel is not tested here.
Impact: The chaos acceptance claims more than it measures. All of `assess`, `revert`, `grow` have no production caller (known), so these tests cover unwired code.
Known: P14 P24-F03 (no production caller), P24-F01, F02, F07.

### P48-F14
File: src/tests/regression_lock.rs:330-356, 366-441, 595-643, 7-22
Severity: Low
Axis: test
Confidence: Certain
Evidence:
:338-341 `if registry.verifier_stake(&verifier) < MIN_VERIFIER_STAKE { let _ = registry.lock_verifier_stake(...); }` (error dropped).
:429-441 release test asserts the returned recipient and that the payment is gone; no balance is read.
ai/registry.rs:1568-1572 the expiry check is first in `release_agent_payment`; the test at 595-643 submits no result, so without the expiry check the call would still fail on "linked request has no finalized outcome yet" (1575-1580).
Header (19-22): "the non-escrowed path credits the recipient immediately" (not tested here).
Description: Three weak spots. (a) The helper drops the stake-lock error. (b) The "recipient credited" and "sender recovers" claims of the header are not tested: the tests run on `AiRegistry` alone; the credit happens in the executor. (c) `escrowed_payment_release_rejected_after_expiry` also passes if the expiry check is deleted, because another precondition fails; the error text is not asserted. Same for `non_escrowed_payment_cannot_be_released` (no reason checked).
Impact: A regression that removes the expiry guard, or that removes the payment without crediting, is not caught by the lock tests. The ZK finality locks (28-250) are correct and use `Rejected(_)` variants.
Known: none.

### P48-F15
File: src/tests/pollen_ai_data_rights.rs:114-320 (with src/execution/executor.rs:586, 618, 1495-1504)
Severity: Low
Axis: test
Confidence: Likely (missing step: read executor.rs 560-640 and `validate_ai_read_ref` to confirm which of the cases below are refused)
Evidence:
:96-111 `signed_grant(asset, grantee, max_reads)` always uses window `0, 10` and a valid owner signature; requests are always at block 0 (`submitted_at_block: 0`, :74).
:316-319 `state.marketplace.validate_ai_read_ref(&reference, &requester, 0).is_err()` for the revoked asset (calls the marketplace, not the executor).
:46 `_seed: u8` is unused in `request`.
:59-64 for an opaque input the request carries a perception declaration on `AssetId([0xEE; 32])`, and :191-193 expects `apply_transaction_checked(...).unwrap()` to succeed.
Description: Negative cases at the executor boundary: only "grant missing" and "reads exhausted". Not tested: grant used after its window ends, grant used by a requester who is not the grantee, grant with a tampered `owner_signature`, revoked grant through the executor, `declared_units` above the grant. The revoke test checks the marketplace function, not `apply_transaction_checked`. The opaque test shows a request that declares an unregistered asset id succeeds; whether that asset id is meant to be unchecked for opaque inputs was not verified.
Impact: Pollen consent (data-rights) enforcement in the AI request path is covered on two cases of at least six.
Known: none.

### P48-F16
File: src/tests/privacy_ai_execution.rs:143-218
Severity: Info
Axis: test
Confidence: Certain
Evidence:
:152-154 `let h1 = program_hash_from_words(&w); let h2 = program_hash_from_words(&w); assert_eq!(h1, h2);`
:181-218 test `ai_require_execution_proof_blocks_finalize_without_proof` expects `register_model(...)` with `require_execution_proof: true` to fail with "full AI STARK"; :217 `let _ = verifier;`
Description: The hash test calls one function twice on one input (cannot fail short of a nondeterministic hasher). The last test name says "blocks finalize", but only registration is tested; no finalize path is reached. The assertion documents that a proof-required AI model cannot be registered on any chain until AI STARK verification is live (message cites "F-04"); that is a closed feature, so a mainnet blocker for proof-backed AI inference. `L1NoteRegistry` (line 18, 159) is a public type name with the banned term (Z12, same family as P14 F24 mod.rs:1).
Impact: Proof-backed AI (execution_class and `require_execution_proof`) is not available. Observation.
Known: partly (AiAttachExecutionProof structural only is in FINDINGS P17 Info). The registration refusal as a mainnet blocker is not listed.

### P48-F17
File: src/tests/private_transfer_fee_market.rs:17-61
Severity: Info
Axis: test
Confidence: Certain
Evidence:
:25-26 `authorization_sig: vec![0u8; 64], public_digest: [0u8; 32],` and :53 `assert!(effective_fee(tx.fee_bid(), base_fee).is_ok());`
Description: The test calls `effective_fee` on a bid and never uses the mempool or the executor, so "the fee market gates private transfer inclusion" (name and header) is shown only for the pure function. The type of transaction does not change the result.
Impact: No mempool admission test for a private transfer below the base fee.
Known: none.

### P48-F18
File: src/tests/poa_onboarding_matrix.rs:163-190, 240-245
Severity: Info
Axis: test
Confidence: Certain
Evidence:
:173-182 a `fn consensus_gate(wl, who)` is defined inside the test and asserted on; :244 `const _: () = assert!(DEFAULT_KYC_HORIZON > 0 && DEFAULT_KYC_HORIZON < u64::MAX);`
Description: The "consensus-style requirement gate" is a local function, not `consensus/poa.rs`, so test 5 checks `PoAWhitelist::contains` only. Test 8 is a compile-time assert. The whole matrix runs on the write side that has no production caller (P14 F07). The header (8-11) says these tests are outside the `poa_isolation` name gate.
Impact: None beyond the known unwired state.
Known: P14 F07.

### P48-F19
File: src/tests/poisoned_lock_locks.rs:66-88, 96-109, 227-244
Severity: Low
Axis: test
Confidence: Certain
Evidence:
:70-76 line scan of src/network/node.rs for `process::exit`; :135 for `unwrap_or(true)`; :209 for `peer_manager.lock()`; :102 `src.contains("fn peer_manager_lock(")`.
:229-243 the "can still detect a violation" test re-types the filter chain and runs it on two planted strings.
Description: Four of six tests are text scans of node.rs (R3). They pass by absence, and they match one line at a time. A split `peer_manager\n.lock()` or a new lock field (not named `peer_manager`) with `process::exit` in another file passes. The `exit` scan only covers node.rs. The "scan can fail" test (227-244) does not call the scan used by the other test; it copies the filter, so a change to the real filter is not tested. The one behavioral test (29-58) uses a `Mutex<Vec<u64>>`, not `PeerManager`, so it shows the std idiom only.
Impact: Maintenance risk: the lock guarantees rest on formatting and on exact names.
Known: none.

### P48-F20
File: src/tests/relayer_gates.rs:1-9; poa_isolation.rs:183, 193, 227, 442; poa_onboarding_matrix.rs:6, 229, 236; real_chain_fixtures.rs:64, 97, 181; prover.rs:522, 537, 686; regression_lock.rs:8, 16, 25, 422, 536, 632; ported_slice_redteam.rs:11-20; regeneration_recovery.rs:9-13; qcblob_quorum.rs:6-16; poisoned_lock_locks.rs:181
Severity: Info
Axis: invariant(Z13, Z12)
Confidence: Certain
Evidence:
relayer_gates.rs:1 `// (Q-A 2026-07-16) L1 relayer proof` (banned term, comment only), :7 `Anchoring gerektirir (executor::TransactionType::RelayerResult kolu).`
poa_onboarding_matrix.rs:6 `//! Ve karar denetim (audit) izini kapsar.`; real_chain_fixtures.rs:97 `.expect("tek eleman garantili")`; regression_lock.rs:536 `"REGRESYON: non-escrowed payment release edilmemeli! ..."`; prover.rs:522 `"gelen: {err}"`.
regeneration_recovery.rs:9 `//! 1. **Corrupt derived state** ...` (markdown bold; also ported_slice_redteam.rs:11-20, qcblob_quorum.rs:6-16, prover.rs:364, 605, 610).
Description: Turkish words in comments, assert text and expect text in 9 files, markdown bold in 5 files. Rule Z13 asks for English repo text without bold. One "L1" in a comment (Z12 covers public text, so this is borderline).
Impact: Style only. No `#[ignore]`, no `#[allow]`, no unsafe, no secret in this part.
Known: none.

## Counts

Critical 0, High 0, Medium 4 (F01, F03, F04, F05), Low 10 (F02, F06, F08, F10, F11, F12, F13, F14, F15, F19), Info 6 (F07, F09, F16, F17, F18, F20). Total 20.

## Positive notes (no finding)

- ported_slice_redteam.rs, pow_light_client.rs, prover.rs (except F06, F07) and qcblob_quorum.rs (fixture canary) assert variant-specific errors, check state is unchanged on refusal, and include a control case. These are the model for the weak tests above.
- relayer_gates.rs supply-ceiling tests (322-462) check the balances, the bridge status and the replay high water after a refused mint.
