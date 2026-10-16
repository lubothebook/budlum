# P49 scan: src/tests 5 (read-only audit, Tier R1 files, R3 subjects)

Base commit: 3df8863 (HEAD e7f75d6 at scan time). Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14), notes/P45-scan.md, notes/P47-scan.md.

## COVERAGE

Files 71 to 86 of `ls src/tests/*.rs`, each read in full (every line):
- src/tests/relayer_worker_locks.rs 1-1014
- src/tests/replay_audit.rs 1-219
- src/tests/security_auditor.rs 1-199
- src/tests/settlement_prod.rs 1-2208 (two reads: 1-1100, 1100-2208)
- src/tests/sharding_e2e.rs 1-157
- src/tests/slashing_matrix.rs 1-266
- src/tests/snapshot_chaos.rs 1-474
- src/tests/socialfi.rs 1-205
- src/tests/storage_tx_door.rs 1-865
- src/tests/target_700.rs 1-102
- src/tests/tokenomics.rs 1-688
- src/tests/tokenomics_proptest.rs 1-196
- src/tests/v95_v98_canaries.rs 1-44
- src/tests/vault_tx_door.rs 1-297
- src/tests/wall_clock_locks.rs 1-169
- src/tests/zkvm.rs 1-255

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md 110-165 (sections 5 and 6); notes/P29-scan.md 1-30.

Outside the part (rg first, short ranges only): src/tests/mod.rs 46-52 and 98-106 (registration); Cargo.toml features (rg); .github/workflows/ci.yml (rg for wallet-ml-dsa); src/sharding/mod.rs 215-262; src/registry/evidence.rs 229-251, 480-505 (rg); src/registry/permissionless.rs 690-720 (rg); src/core/account.rs burn_from (rg -A12); src/domain/storage_deal.rs open_deal (rg); .github/idle-code-baseline.txt (rg on sharding); docs/audit/FINDINGS.md and STATUS.md (rg only).

Not read: the production bodies behind each test (blockchain.rs, executor.rs, manifest.rs, snapshot.rs, worker.rs). Every claim about production comes from the test text plus the listed rg. No cargo, no test run.

Extra-check summary:
- Vacuous asserts: found in tokenomics_proptest.rs (P49-F02, F03), security_auditor.rs (F04), v95_v98_canaries.rs (F11), target_700.rs (F12), slashing_matrix.rs (F06).
- Tests that pin a production bug as correct: storage_tx_door.rs manifest id collision (F05); sharding_e2e.rs fee vanishes (F14); replay_audit.rs masks four roots (F08).
- #[ignore] or #[allow] (Z3): no attribute in the 16 files. One equivalent bypass found: `#![cfg(false)]` on a whole file (F01). replay_audit.rs:111 and mod.rs:50 only mention a removed ignore and a removed cfg(false) in comments.
- Hardcoded secrets (Z9): none. Fixed keys appear only as test scalars and seeds (settlement_prod.rs:108 and :170, dead file; test keypairs elsewhere are generated). No 64 hex literal, no PEM, no mnemonic.
- Clean files (no finding beyond the notes above): vault_tx_door.rs, socialfi.rs, tokenomics.rs (one doc note, F15), zkvm.rs (one note, F16).

## FINDINGS

### P49-F01
File: src/tests/settlement_prod.rs:1-3 (whole file, 2208 lines); src/tests/mod.rs:103-104
Severity: High
Axis: test (Z3 equivalent)
Confidence: Certain
Evidence:
  src/tests/settlement_prod.rs:1-3
    // Devnet expects the old FinalityProof API and is incompatible with upstream.
    // To be rewritten against the new API.
    #![cfg(false)]
  src/tests/mod.rs:103-104
    #[cfg(test)]
    pub mod settlement_prod;
  rg -n 'cfg\(false\)' src  ->  src/tests/mod.rs:50 (comment only) and src/tests/settlement_prod.rs:3
  The file also does not match its own helpers: commitment_with_proof is defined with 5 parameters (line 193-199) and called with 6 (line 586 `..., &weak_proof, Address::zero())`); verify_domain_event_proof is called with 6 arguments at 792-799 and with 7 elsewhere (765-773). It would not compile if enabled.
Description: An inner `#![cfg(false)]` removes the whole module, including the second module `zk_finality_real_proof` (line 2000), from every build. This is a disabled-test bypass of the same class as `#[ignore]` (Z3), and `rg '#\[ignore'` does not see it. mod.rs still lists the module as if it ran.
Impact: About 45 tests never run: PoW header chain, PoA quorum, BFT cert and ZK finality through submit_verified_domain_commitment, bridge lock/mint/burn/unlock lifecycle, replay protection, global header hash chain, storage reload of domains, commitments and bridge roots. A regression in any of these paths passes CI. The file claims coverage of the multi-consensus settlement layer, an R3 area.
Known: none (P62 Z3 scan looks for `#[ignore` and `#[allow(`; not this form)

### P49-F02
File: src/tests/tokenomics_proptest.rs:33-67
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
  tokenomics_proptest.rs:22-25   fn arb_balance() -> impl Strategy<Value = u64> { 0..10_000_000u64 }
  tokenomics_proptest.rs:34      balances in prop::collection::vec((arb_address(), arb_balance()), 1..50)
  tokenomics_proptest.rs:63-66   assert!(supply <= u128::from(BUD_TOTAL_SUPPLY), ...)
  tokenomics.rs:38-40            BUD_TOTAL_SUPPLY == bud(100_000_000), with bud() scaling by 10^6 (state.circulating_supply() == u128::from(BUD_TOTAL_SUPPLY))
Description: The generator can add at most 49 * 10^7 = 4.9e8 base units. The cap is 10^14 base units. The bound can never be reached, so "total supply never exceeds 100M" cannot fail. The test also adds balances with `add_balance`, which has no cap, so it checks the sum of its own inputs (the equality assert at 58-62), not a protocol property.
Impact: The 100M fixed-supply invariant (CLAUDE.md section 1, double burn) has no property test at the state level. A mint path that exceeds the cap is not caught here. tokenomics.rs covers genesis total and single burns, but there is no test that runs fee burn and timed burn together across blocks and asserts supply = 100M minus all burns.
Known: none

### P49-F03
File: src/tests/tokenomics_proptest.rs:74-92, 100-122
Severity: Low
Axis: test
Confidence: Certain
Evidence:
  tokenomics_proptest.rs:83-90
    let _ = state.burn_from(&addr, burn_amount);
    let final_balance = state.get_balance(&addr);
    assert!(final_balance <= initial_balance, "Balance should not increase after burn");
  src/core/account.rs:2082-2087 burn_from clips with `amount.min(account.balance)` and returns the burned value (ignored here).
  tokenomics_proptest.rs:103-121 burn_mint_consistency builds only TokenomicsParams, never an AccountState.
Description: Invariant 2 asserts `<= initial_balance`, which holds even if burn_from does nothing. The return value (the burned amount) is dropped, so "burn removes exactly min(amount, balance) from supply" is never checked. Invariant 3 is named "burn and mint consistency" but compares only two parameter formulas to their inputs; no state, no supply.
Impact: Both properties pass for any burn implementation that never raises a balance. They give false assurance for the burn paths.
Known: none

### P49-F04
File: src/tests/security_auditor.rs:58-199 (and 14-27)
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
  security_auditor.rs:63-72
    let a = addr($idx);
    let mut tx = Transaction::new(a, addr(0), 0, vec![]);
    tx.nonce = $idx;
    // ... Without balance it fails.
    let state = crate::core::account::AccountState::new();
    assert!(state.validate_transaction(&tx).is_err());
  security_auditor.rs:78-199  60 invocations sec_test_1 ... sec_test_60 (rg -c 'sec_test_' gives 60 names)
Description: The macro produces 60 tests with the same body; only the sender byte and nonce change. The transaction is never signed and the state is empty, so validate_transaction fails on the first check it reaches (missing signature or balance) whatever the nonce. The assert is `is_err()` with no error text, so it cannot say which rule fired. This is the same padding pattern that target_700.rs documents as removed on 2026-07-18 (lines 1-11).
Impact: 60 passing tests add no coverage and inflate the test count; the file header promises "range boundaries". The three real tests at the top (lines 14-56) also assert only `is_err()`: `security_reject_max_u64_amount_plus_fee` would pass if the failure came from the signature or chain id instead of the overflow check, so the overflow rule has no pinned negative test. validate_transaction overflow guards are called correct in the P20 scan (FINDINGS line 1005) without a test that pins them.
Known: none (target_700.rs autopsy covers a different file)

### P49-F05
File: src/tests/storage_tx_door.rs:429-446
Severity: Medium
Axis: test (pins a production defect as correct)
Confidence: Certain (test text); Likely for the production cause, missing step: read ContentManifest::verify_id preimage in src/storage/manifest.rs
Evidence:
  storage_tx_door.rs:436-445
    let a = manifest_owned_by(alice).with_source(crate::storage::ContentSource::Generated(spec));
    let mut b = a.clone();
    b.source = crate::storage::ContentSource::Stored;
    b.dictionary_id = Some(digest);
    // The id preimage appends the source bytes (tag 1 then the digest) and
    // the dictionary bytes (1 then the id) without separating them ...
    // The second manifest therefore verifies under the first one's id.
    b.verify_id().expect("the real derivation collides");
    assert_eq!(a.manifest_id, b.manifest_id);
Description: The test asserts that two different manifests (generated source with no dictionary, versus stored source with a dictionary equal to the spec digest) share one manifest_id and both pass verify_id. The id preimage concatenates variable fields without length prefixes. The test records this as expected and only checks that the signing hash differs.
Impact: Content-addressing binds one id to two logical manifests. The state registry is keyed by manifest_id (storage_tx_door.rs:74), so first-writer-wins: a later registrant with a colliding manifest is refused, or a verifier trusting the id alone accepts either shape. A fix of the preimage would turn this green test red, so the test blocks the fix. Mainnet blocker for the B.U.D. content id binding if the preimage is not corrected.
Known: none (rg 'manifest_id|collid' in FINDINGS.md finds no manifest id preimage entry)

### P49-F06
File: src/tests/slashing_matrix.rs:26-59, 82-131, 135-166, 236-265
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
  slashing_matrix.rs:103-104   let result = reg.slash_from_report(&report); assert!(result.is_ok(), "liveness fault must be processed");
  slashing_matrix.rs:129-130   assert!(result.is_ok(), "invalid sig spam must be processed");
  slashing_matrix.rs:161       let _ = result;   (unverified case: result discarded)
  slashing_matrix.rs:257-258   let r2 = reg.slash_from_report(&report); let _ = r2;
  slashing_matrix.rs:34-42     consensus_double_sign(val, 100, "hash_aaa", "hash_bbb", vec![0xAA; 64], vec![0xBB; 64], ...) then slashed
  src/registry/evidence.rs:229-251 consensus_double_sign stamps ProofProvenance::ConsensusVerified; is_actionable (480-486) only checks provenance
  rg -n 'consensus_double_sign\(' src --glob '!src/tests/*'  ->  only evidence.rs:591-665 (its own tests)
Description: Liveness and spam cases only assert Ok(_), which also holds for Ok(None) (no slash). They never check stake, jail, penalty or history. The double-sign "positive" case slashes with non-hex hashes and a 0xAA repeated signature, so it proves only that the registry trusts the provenance flag; verify_double_sign (evidence.rs:524) is not called anywhere in the matrix. The honest-validator case never calls slash at all (the assert is true by construction). The "unverified" case would pass if the slash were silently skipped for any reason.
Impact: The "matrix" gives no protection for slash ratios by condition (RegistryParams::slash_ratio), for the zero-penalty record path (permissionless.rs:699-711), or for a wrongly accepted forged proof. The convenience constructor has no production caller, so the matrix covers a path production does not use (blockchain.rs:2854, 3519, 4406 build reports differently).
Known: none

### P49-F07
File: src/tests/snapshot_chaos.rs:108-134, 224-243, header 1-10
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
  snapshot_chaos.rs:141-222  recompute_v2_hash_for_test (legacy re-implementation of the digest)
  snapshot_chaos.rs:237-242
    snap.balances.insert(eve, 9_000_000);
    snap.snapshot_hash = recompute_v2_hash_for_test(&snap);
    // Schema-4 digest has a domain-separated canonical field manifest;
    // The legacy helper cannot recreate a valid schema-4 digest.
    assert!(!snap.verify(), "schema-4 rejects legacy rehash forgery");
  snapshot_chaos.rs:3-10  header says the "_gap" tests "are INVERTED when those tasks land"; test names still end in _gap (113, 225).
Description: The rehash test rejects a forged snapshot only because its own helper does not reproduce the schema-4 digest (the helper's own comment says so). An attacker who uses the real algorithm still passes verify, because the digest has no key or signature. The test therefore does not show that authenticity exists; it shows the helper is stale. The header and test names still describe the old gap pins while the assertions are already inverted. The helper also uses `bincode::serialize(..).unwrap_or_default()` (lines 198, 215), a hash input that silently becomes empty on error (LABEL-12.9 pattern, test side).
Impact: A reader sees "rehash forgery rejected" and may believe snapshot authenticity is covered. It is not (related: P11-F05). The only test of the real gap is gone. Tests 2 and 8 cover only bns_registry and bridge_state; the other schema-4 fields (tokenomics, registry, liveness, invalid_votes, nft, marketplace, budlumxyz, storage_registry, ai_registry, message_registry, external_roots, finality_certificates) have no tamper test.
Known: partly P11-F05 (FINDINGS.md:794, different fault)

### P49-F08
File: src/tests/replay_audit.rs:62-69, 88-97, 99-106
Severity: Medium
Axis: test (pins a live/replay divergence as acceptable)
Confidence: Likely (missing step: read calculate_state_root and replay loop to confirm the four roots feed the real state root)
Evidence:
  replay_audit.rs:62-69
    // Compare masked: executable consensus surface only (overlay fields
    // Are commit-path projections ...
    live_masked.bridge_root = [0u8; 32];
    live_masked.message_root = [0u8; 32];
    live_masked.settlement_root = [0u8; 32];
    live_masked.global_header_summary = [0u8; 32];
  replay_audit.rs:92-97  same four fields zeroed on the reloaded state before comparing roots.
Description: The test zeroes four roots on both sides, so it cannot detect a difference between the live path and the replay path in bridge, message, settlement or global header state. The comment says the replay loop does not mirror them. The audit criterion (AUDIT_PLAN section 5, "replay path and live path give the same root") is therefore only proven for a Transfer-only chain (five transfers, line 54-60) plus a BnsRegister test (line 135). No stake, unbonding, tokenomics burn, storage or bridge transaction is replayed.
Impact: If those roots are consensus-relevant, a node restarting from disk can hold different bridge or settlement state than a node that stayed live, with no test failing. Mainnet blocker class if the overlay is meant to be consensus state.
Known: none

### P49-F09
File: src/tests/wall_clock_locks.rs:50-125
Severity: Low
Axis: test
Confidence: Likely (missing step: run the scan against a planted case in CI)
Evidence:
  wall_clock_locks.rs:101   if !line.contains("duration_since") || !line.contains("UNIX_EPOCH") { continue; }
  wall_clock_locks.rs:106-108  window.contains(".unwrap()") && !window.contains(".unwrap_or") ...
  wall_clock_locks.rs:46      walk(&repo_root().join("src"), &mut out);  (only src/, skips every directory named tests)
Description: The scan needs `duration_since` and `UNIX_EPOCH` on one line, looks only at `.unwrap()` (not `.expect(...)`), and covers only src/. A rustfmt-split call, an `.expect("clock")` or a crate in budzero/ or crates/ is not seen. `without_test_modules` drops the next braced item after any `#[cfg(test)]`, so a `#[cfg(test)] use ...;` line makes the scan skip the next production block (false negative). Unlike the relayer lock, there is no planted-violation canary for this scan.
Impact: The lock can pass while a clock panic is in the tree. Also the "falls back to zero fails closed" claim (header lines 15-18) is tested only through PoAEngine::full_validate (line 150); PoW, PoS and BFT validation, and non-block uses of a zero clock (expiry, deadlines in RPC handlers), are not tested.
Known: LABEL-12.9 style (unwrap_or_default), not mixed into this part

### P49-F10
File: src/tests/relayer_worker_locks.rs:1001-1013, 553-571, 641-698
Severity: Low
Axis: test
Confidence: Certain
Evidence:
  relayer_worker_locks.rs:1002-1008
    assert!(msg.contains("does not speak to Ethereum") || msg.contains("does not read Ethereum")
        || msg.contains("Merkle") || msg.contains("leaf") || msg.contains("forgery"), ...);
  relayer_worker_locks.rs:564-570  the placeholder test repeats the no-adapter refusal already pinned at 300-314 and 773-793.
  relayer_worker_locks.rs:662-664  `line.contains("success: true")` on trimmed lines of worker.rs before the first `#[cfg(test)]`.
Description: The reason check for the stubbed EVM adapter accepts any of five substrings, including the generic "Merkle" and "leaf" the comment says were the old incidental refusal; the later `matches!(SubmissionFailed)` is the real guard. Source-text locks (include_str of worker.rs, main.rs, commands.rs) pass for `success:true` without a space or a result built in another file. Three tests assert the same no-adapter refusal.
Impact: Low: guards are partly redundant and fragile, not wrong. The behavioural tests (lines 341-571) are sound and each has a matching accept case (533).
Known: none

### P49-F11
File: src/tests/v95_v98_canaries.rs:9-22
Severity: Low
Axis: test
Confidence: Certain
Evidence:
  v95_v98_canaries.rs:9-21
    /// Canary: ConsensusDomainRegistry::new starts empty. Inside try_reorg there is an
    /// assignment domain_registry = ConsensusDomainRegistry::new, so stale state is cleared.
    let registry = ConsensusDomainRegistry::new();
    assert!(registry.domains().is_empty(), ...);
Description: The test checks that a new registry is empty. It never calls try_reorg, so it does not test that a reorg resets the registry, which is what the name and comment claim. The second test (seed determinism) is real.
Impact: Reorg handling of the domain registry (R3 chain path) has no test here; the canary stays green if try_reorg stops resetting it.
Known: none

### P49-F12
File: src/tests/target_700.rs:88-102
Severity: Low
Axis: test
Confidence: Certain
Evidence:
  target_700.rs:90-94   let r = UniversalRelayer::new(RelayerConfig::default()); assert_eq!(r.pending_count(), 0);
  target_700.rs:98-102  let s = AccountState::new(); assert_eq!(s.epoch_index, 0);
Description: Two of the six remaining behaviours test a constructor default. They cannot catch a logic fault. The header states the consolidation but the tests named after "relay" and "state" are the leftover of the 20-copy groups.
Impact: Counting only; no protection. Info-level for coverage, kept Low because the header sells them as preserved behaviours.
Known: none

### P49-F13
File: src/tests/storage_tx_door.rs:481-865 (module open_deal), 499-510, 746-753
Severity: Info
Axis: integration
Confidence: Certain
Evidence:
  storage_tx_door.rs:481   #[cfg(feature = "wallet-ml-dsa")]  (default feature set: Cargo.toml:14 default = ["pq-ml-dsa", "wallet-ml-dsa"]; ci.yml:48-79 enables it)
  storage_tx_door.rs:500-508  ProofEnvelope { backend: "test-backend", proof_bytes: vec![0xABu8; 96], ... } accepted: test 614 opens a deal with it.
  storage_tx_door.rs:747-753  mainnet_refuses_to_open_a_deal_in_block (chain id 45260, expects "mainnet").
Description: (a) The open-deal tests run only under the wallet-ml-dsa feature; it is on by default, so they run in CI, but a `--no-default-features` build skips them without notice. (b) The accept test passes a junk envelope, so it shows OpenDeal in block accepts a format-only proof (the known storage_deal F04 class, FINDINGS.md:1068). (c) The mainnet test pins that in-block OpenDeal is refused on mainnet: B.U.D. deals cannot open through blocks at mainnet launch. This is an open mainnet gate, not a defect, and it needs an owner decision (CLAUDE.md section 9 item 1).
Impact: Information for the mainnet gate list.
Known: format-only proof: FINDINGS.md:1068 (F04 behind which these paths are dead)

### P49-F14
File: src/tests/sharding_e2e.rs:113-157, 32-111
Severity: Low
Axis: invariant (supply accounting) and integration
Confidence: Certain (test text, baseline); Likely for the stale entry, missing step: confirm blockchain.rs:4653 and 4968 are on the live produce and validate paths
Evidence:
  sharding_e2e.rs:147-156
    assert_eq!(bc.state.get_balance(&addr(0x01)), before_from - 305, "source must lose amount + fee");
    assert_eq!(bc.state.get_balance(&addr(0x02)), before_to + 300, "destination must gain the amount");
  Nothing is credited with the 5 fee and no burn counter moves; the test pins this as correct.
  .github/idle-code-baseline.txt:1856-1857  apply_cross_shard_transfer, is_cross_shard (no production caller)
  rg -n 'shards_commitment' src/chain/blockchain.rs  ->  4653 (produce) and 4968 (validate)
Description: The test drives `apply_cross_shard_transfer` directly. The function has no production caller, so the "atomic cross-shard transfer" claim in the header is not an executor behaviour. It also pins that the fee leaves the sender and goes nowhere (supply drops by 5 without burn accounting, FINDINGS P20 F04 notes the same, unwired). Separately, FINDINGS P20-F03 says nothing sets or validates Block.shards_root outside tests; sharding_e2e.rs:33-111 and blockchain.rs:4653 and 4968 indicate production and validation do exist, so that entry looks stale.
Impact: If the cross-shard path is wired later, the fee loss lands in consensus. The stale P20-F03 text could cause a wrong mainnet decision.
Known: P20-F03 and P20 F04 (FINDINGS.md:1018, 1021); idle-code-baseline.txt:1856-1857

### P49-F15
File: src/tests/tokenomics.rs:293-295, 7-12; src/tests/zkvm.rs:10-14
Severity: Info
Axis: test (hygiene)
Confidence: Certain
Evidence:
  tokenomics.rs:293  /// Team vesting is ENFORCED on transfers through the real executor path.   (sits above fixed_supply_tokenomics_disables_epoch_yield_minting, line 295)
  tokenomics.rs:7-12 and zkvm.rs:10-14 each define test_addr_from_byte (also settlement_prod.rs:10 and core/transaction.rs:2-5, core/account.rs:4-7)
Description: A misplaced doc comment describes the wrong test. The same helper is copied into several files. No behaviour issue. Coverage of the other tokenomics paths is solid: the cliff, annual burn and mainnet-template tests (579-688) are paired with "still fires" tests, so they are not vacuous.
Impact: Maintenance only.
Known: none

### P49-F16
File: src/tests/zkvm.rs:56-68, 189-216
Severity: Info
Axis: test
Confidence: Certain
Evidence:
  zkvm.rs:67   assert!(receipt.proof_bytes > 0);
  zkvm.rs:196-198  blockchain.fund_development_account(&from).expect("devnet faucet");  then balance == 1_000_000_000 - 7
Description: The contract path asserts only that a proof exists (non-zero size); nothing verifies it, and no test submits a tampered receipt to block validation. The block test relies on the devnet faucet (blockchain.rs:4798), whose mainnet gating this file does not test.
Impact: Execution and gas are tested; proof verification of contract calls in blocks is not, here. Check the other test files for it before treating as a gap.
Known: none

### P49-F17
File: docs/audit/FINDINGS.md:226 (evidence reference) against src/tests/settlement_prod.rs:1812-1874
Severity: Info
Axis: test
Confidence: Certain
Evidence:
  FINDINGS.md:226  "Test settlement_prod.rs:1840-1870 runs lock, burn, unlock and asserts no supply."
  settlement_prod.rs:3  #![cfg(false)]
Description: The cited test for P05-F01 (bridge lock, mint, burn, unlock, net plus one transfer amount) is inside the file removed from the build. The test has no supply assertion in its text either (1812-1874 ends with `assert_ne!(final_header.bridge_state_root, [0u8; 32])`).
Impact: P05-F01's red-test requirement ("total supply before equals after a full cycle") has no live test to extend; the fix handoff must add a new test outside this file.
Known: P05-F01 (FINDINGS.md:226)
