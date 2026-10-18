# P45 scan: src/tests 1 (read-only audit, Tier R1)

Base commit: f9be68a. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14). Items that overlap a known finding are marked Known.

## COVERAGE

Read in full (every line), files 1 to 17 of `ls src/tests/*.rs`, 7299 lines:
- src/tests/adversarial_p2p.rs 1-113
- src/tests/advisory_reachability.rs 1-522
- src/tests/ai_inference_runtime.rs 1-862
- src/tests/ai_verification_status_locks.rs 1-347
- src/tests/audit_findings_locks.rs 1-238
- src/tests/bench_performance.rs 1-163
- src/tests/block_clock.rs 1-52
- src/tests/block_reward.rs 1-92
- src/tests/blockchair_fixture_locks.rs 1-847 (two reads: 1-687, 688-847)
- src/tests/bns.rs 1-59
- src/tests/bns_expanded.rs 1-221
- src/tests/bridge_lifecycle.rs 1-340
- src/tests/bridge_negatives.rs 1-236
- src/tests/bud_e2e.rs 1-702
- src/tests/byzantine_settlement.rs 1-1140
- src/tests/chain_id_registry_locks.rs 1-295
- src/tests/chaos.rs 1-1070

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md 110-165 (sections 5, 6); docs/audit/FINDINGS.md lines 1-140 (P01 to P03 format and style) plus rg hits for the topics below. notes/P29-scan.md head was read for the format.

Outside the part (rg first, then at most 80 lines): src/chain/blockchain.rs 925-940, 1295-1420, rg on apply_bridge_sweep and mint_bridge_transfer_from_verified_event; src/domain/storage_deal.rs 2808-2960, 2990-3090 (rg first); src/pollen/data_rights.rs 170-215, 455-490; src/pollen/offers.rs 360-376, 480-494; src/ai/types.rs 425-445; src/rpc/server.rs 2595-2650; src/network/protocol.rs and peer_manager.rs (rg -A12); config/personas/enterprise-poa.toml and config/archive.toml (rg on profile and listener keys); .quality/osv-scanner.toml (rg effectiveUntil); CLAUDE.md (rg Z3, Z9, Z13); src/tests/mod.rs (rg module list).

Not read: the production bodies behind the tests beyond the ranges above, .quality/*.yaml/toml files in full, Cargo.lock files, docker compose files. A few ls and sed calls on Cargo.lock, fuzz/Cargo.lock and the compose files were refused by the sandbox, so the existence of those include_str! targets is not confirmed by this scan. No cargo, no test run: every claim comes from reading source.

Extra-check summary:
- Z3 (#[ignore], #[allow]): no #[ignore] and no #[should_panic] in the 17 files. Two `#[allow(dead_code)]` at bench_performance.rs:14 and :160, behind a commented-out `#[tokio::test]` (P45-F11).
- Z9 (secrets): none. The 64-hex literals in blockchair_fixture_locks.rs are public Bitcoin and Ethereum block, txid and root values. `REV` in advisory_reachability.rs:137 is a public git commit. Test keys are `[0xNN; 32]` fills and `KeyPair::from_seed` with a one-byte seed, test only.
- Vacuous asserts and tests that pass for the wrong reason: P45-F02, F04, F05, F06, F08, F14.
- Tests that mask a production gap: P45-F01, F03, F07, F09.
- Every one of the 17 files is declared in src/tests/mod.rs (rg checked), so none is silently uncompiled.

## FINDINGS

### P45-F01
File: src/tests/advisory_reachability.rs:459-522 (with src/pollen/offers.rs:370, 488; src/pollen/data_rights.rs:198-209, 468-480)
Severity: Medium
Axis: security
Confidence: Certain for the false lock and the missing chain_id; Likely for cross-network replay (missing step: confirm one seller key is used on two networks with equal asset id and terms)
Evidence: The test doc says "neither digest is verified anywhere" and the test counts `signing_hash(` only inside the defining file (lines 506-520, `calls <= definitions + 3`). Production verifies both Pollen digests in another file:
  src/pollen/offers.rs:370: `&grant.signing_hash(),`
  src/pollen/offers.rs:488: `&authorization.signing_hash(),` inside `crate::crypto::primitives::verify_signature(...)`.
  src/pollen/data_rights.rs:198-209 (SaleAuthorization::signing_hash) hashes tag, authorization_id, asset_id, seller, unit_price, valid_from, expires_at, max_grants, terms_hash. No chain_id.
  src/pollen/data_rights.rs:468-480 (AccessGrant::signing_hash) has the same shape and no chain_id.
  rg -n 'chain_id' src/pollen/data_rights.rs src/pollen/offers.rs: no output.
Description: The lock was written for the AI digest and the Pollen digest together. The Pollen half is wrong: owner and seller signatures over these digests are checked on the register path, and the preimage carries no network. The test cannot fail when that happened because it scans one file.
Impact: A seller or owner signature made for one network (devnet, testnet) verifies on another network that holds the same asset id and terms, and a replayed authorization or grant is registered under the seller's name. Needs the same key on two networks and equal derived ids (derive_id at data_rights.rs:175-190 has no chain_id either).
Trigger (security only): precondition: same Ed25519 key used on two Budlum networks, same asset id registered on both. Entry: the Pollen register path in src/pollen/offers.rs:360-376 and 480-494. Input shape: a SaleAuthorization or AccessGrant with a valid signature copied from the other network. Expected wrong result: the object is accepted without the owner acting on this network.
Known: none (rg for SaleAuthorization, AccessGrant, BDLM_POLLEN in docs/audit/FINDINGS.md and STATUS.md: no hit). Related class: FINDINGS.md P23 and P21-F06 note storage authorizations without chain_id.

### P45-F02
File: src/tests/byzantine_settlement.rs:163-171; src/tests/chaos.rs:475-511
Severity: Medium
Axis: test
Confidence: Certain
Evidence: byzantine_settlement.rs:163-171:
  `let (mut fake_com, proof) = pow_commitments[0].clone();`
  `fake_com.state_root = [0xFFu8; 32]; ... .submit_verified_domain_commitment(fake_com, proof).is_err()`
  where the proof is `FinalityProof::PoWHeaderChain { headers: vec![] }` (line 96). chaos.rs:503-510 passes `FinalityProof::PoA { authorities: vec![], signatures: vec![] }` to a PoW domain and the reverse, then asserts only `.is_err()`.
  rg -n 'submit_verified_domain_commitment' src/tests/{byzantine_settlement,chaos}.rs: only these negative uses.
Description: The proofs are empty or empty-shaped, so the call fails whatever the tamper is. The message "Invalid state root in proof must be rejected" is not shown by the test. No test in this part calls the verified entry with a valid proof, so there is no positive control.
Impact: A regression that accepts a wrong state root, or a wrong proof type, stays green when the empty proof is rejected earlier. The finality adapters (PoW, PoS, PoA) are never run on a good proof in the 17 files.
Known: related to FINDINGS.md P08-F10 (tests skip finality), not the same item.

### P45-F03
File: src/tests/byzantine_settlement.rs (about 20 tests, 50-1041); src/tests/chaos.rs:229-266, 308-404, 513-540, 542-808; src/chain/blockchain.rs:1306-1320
Severity: Medium
Axis: test
Confidence: Certain
Evidence: blockchain.rs:1306-1319:
  `#[cfg(not(test))] pub fn submit_domain_commitment(...) { Err("Raw domain commitment submission is disabled; verified finality proof is required".into()) }`
  `#[cfg(test)] pub fn submit_domain_commitment(&mut self, commitment) { self.accept_domain_commitment(commitment) }`
  src/rpc/server.rs:2604-2612 also returns the disabled error. chaos.rs:686-808 (end-to-end transfers) accepts a PoS commitment through this test-only entry, then `mint_bridge_transfer_from_verified_event` succeeds against it (775-789).
Description: The settlement, equivocation, convergence and bridge-mint tests all feed state through a function that does not exist in the production binary. The production gate (verified proof) is not the one under test.
Impact: Green results for equivocation freeze, registry roots and bridge mint do not cover the production anchor path.
Known: FINDINGS.md:716 (P08-F10). New part: the bridge mint positive test (chaos.rs:686-808) is built on the same test-only anchor.

### P45-F04
File: src/tests/chaos.rs:965-1000, 1002-1027, 1029-1069; src/tests/block_clock.rs:48-52; src/tests/chain_id_registry_locks.rs:111-125
Severity: Medium
Axis: test
Confidence: Certain
Evidence: chaos.rs:998-999:
  `assert!(total_a > 0, "chain A must have non-zero total");`
  `assert!(total_b > 0, "chain B must have non-zero total");`
  The test (969-1000) never applies a transaction; it calls `add_balance` directly on two chains, so the "state determinism under tx reordering" claim has no basis, and the asserts cannot fail after 1000 + 1000 is added.
  chaos.rs:1033-1069 (`test_chaos_tx_validity_survives_reorg`) contains no transaction; it repeats `test_chaos_network_partition_recovery` (18-54) and is named for replay protection after reorg.
  chaos.rs:1023-1026: `assert!(result.is_err() || !result.unwrap(), ...)` accepts every error text and every non-reorg outcome, for a chain with a different chain_id (9999).
  block_clock.rs:48-52 `restored_state_carries_the_tip_clock`: a fresh chain with no storage, genesis timestamp 0, asserts `state.current_block_unix_secs == bc.current_unix_secs()`, that is 0 == 0. The restore path (blockchain.rs:938) is not run.
  chain_id_registry_locks.rs:113-125 only asserts that a constant list contains 1 and does not contain mainnet's id; it does not execute the scan it claims to check.
Description: Four tests carry names that promise a property and assert something that is true for any input.
Impact: Replay-after-reorg, determinism under reorder and the restored block clock have no real test, although the names suggest coverage.
Known: none.

### P45-F05
File: src/tests/byzantine_settlement.rs:286-348, 393-487, 763-824, 910-984
Severity: Medium
Axis: test
Confidence: Certain
Evidence: 337-340: node_a applies the domain-1 claim first, node_b the domain-2 claim first, both with `(alice, 1)`; the two claims carry the same value, so the equal final nonce (342-347) says nothing about which commitment wins.
  447-450 and 472-477: `let _ = node_a.submit_domain_commitment(com);` and `let _ = apply_state_updates(...)`. Final asserts (480-486) compare nonces only. If every submit and apply failed, all nonces are 0 on both nodes and the test passes.
  972-977: the 500-claim set is sorted by (domain, height) and applied in the same order on every node, so the gossip randomness (StdRng drops and duplicates, 925-935) cannot affect the final nonce assertion at 978-983. Only the header-hash check (945-957) depends on gossip.
  393-487 uses `rand::rng()` without a seed (442), so a failure is not reproducible.
Description: The convergence asserts compare values that are equal by construction or that ignore all errors.
Impact: A bug in order handling or in the monotonic nonce guard may not be caught; failures cannot be replayed.
Known: none.

### P45-F06
File: src/tests/bridge_negatives.rs:144-236; src/tests/bridge_lifecycle.rs:151-167
Severity: Medium
Axis: test
Confidence: Certain
Evidence: every negative case ends in `assert!(res.is_err(), ...)` (166, 187, 196-199, 210-213, 223, 235). Only N2 runs the positive path first (183-184, `.unwrap()`), the other five have no positive control and no error text check. bridge_lifecycle.rs:164-167: `assert!(bad_mint.is_err(), ...)` after a comment that says it fails "either at the forgery gate ... or at the bridge-state level ... Both are acceptable".
Description: A broken fixture (for example anchor at the wrong height) rejects every case, so all six negatives stay green with the verification logic removed.
Impact: The R3 bridge relay negatives do not prove which check rejects.
Known: none.

### P45-F07
File: src/tests/bridge_lifecycle.rs:1-5, 79-169, 171-249; src/tests/chaos.rs:686-808
Severity: Medium
Axis: test
Confidence: Certain
Evidence: tests/mod.rs header and bridge_lifecycle.rs:1-5 describe "lock, mint, burn, unlock". `bridge_lock_mint_burn_unlock_lifecycle` (79-169) has no successful mint, no burn and no unlock: after the sweep it only checks that one mint attempt fails. The sweep tests (118-133, 210-248) check the returned `(owner, amount)` list; no assert reads `bc.state.get_balance(&owner)` after the refund although apply_bridge_sweep_to_state credits it (blockchain.rs:4586-4589). chaos.rs:775-804 asserts `mint_result.is_ok()` for 100 transfers but never reads balances or `total_bud_committed()` before and after.
  rg -n 'total_bud_committed|get_balance' src/tests/bridge_lifecycle.rs src/tests/bridge_negatives.rs: only `get_balance(&owner)` before locking (bridge_lifecycle.rs:45).
Description: The tests named for lock, mint, burn, unlock never assert funds conservation. Supply and balance are the main invariants of a bridge (AUDIT_PLAN section 5).
Impact: Double credit on mint or a missing debit on lock would pass. In chaos.rs:723-737 the lock is made through `bridge_state.lock` directly, which skips the owner debit that `Blockchain::lock_bridge_transfer` does, so that test mints with no matching debit.
Known: none.

### P45-F08
File: src/tests/adversarial_p2p.rs:11-30, 33-43, 60-113
Severity: Medium
Axis: test
Confidence: Certain
Evidence: 11-30: the "sybil role rejection" test builds `Block::new(1, genesis_hash, vec![])` with no producer (the line "Block.producer = Some(attacker)" is a comment, line 23), uses `PoWEngine`, and asserts `res.is_err()` with no reason. PoW has no staked role, so the block fails on another check (state root, hash, producer). `attacker` is used only for `validators.contains_key`.
  101-112: the "clock-regressed" block is `Block::new(2, ...)` with a copied prev hash and `res.is_err()` with no reason; it is rejected for any mismatch (state_root, tx_root), not shown to be the timestamp rule.
  38-42: one oversized transaction (20 MiB) and no boundary or accepted-case test.
  45-58: 100 `report_invalid_block` calls; no check of the threshold or the ban duration.
Description: Same pattern as FINDINGS.md P01-F18: the test passes for the wrong reason.
Impact: No coverage of consensus producer eligibility or of the timestamp monotonic rule although the test names claim it.
Known: related to FINDINGS.md P01-F18 and P01-F19.

### P45-F09
File: src/tests/bud_e2e.rs:64-77, 153-166, 228-312, 534-594
Severity: Low
Axis: test
Confidence: Certain
Evidence: `valid_merkle_proof()` builds a `ProofEnvelope` with backend "test-backend" and `proof_bytes: vec![0xAB; 96]` (67-76); open_deal accepts it because it only checks the envelope shape (storage_deal.rs:3711-3731). Answers use `Some(b"test-mock-proof")` (163, 562) and storage_deal.rs:3025 has `if cfg!(test) && proof_bytes == b"test-mock-proof" { return Ok(()); }`. The "malicious operator" test (228-312) never calls `answer_challenge` with the cached hash; it only asserts the two hashes differ (304-307), then the missed-challenge path. In production `storage_challenge_proofs_are_checkable()` is false (storage_deal.rs:3087-3089) and line 2890 returns Ok for any proof when the flag is false.
Description: The answer path with a wrong hash or garbage proof is never run, and the success path runs the test-only mock.
Impact: Invariant 6 ("slash only via missed deadline", 529-594) holds only because the Mismatched path is unreachable in production.
Known: FINDINGS.md P23-F04.

### P45-F10
File: src/tests/audit_findings_locks.rs:93-131, 205-237; src/tests/ai_verification_status_locks.rs:30-263; src/tests/bns_expanded.rs:209-220
Severity: Low
Axis: test
Confidence: Certain
Evidence: these locks read source files and call `contains`: `!pos_rs.contains("let _ = store.save_checkpoint")` (122), `server_rs.contains("if !config.trusted_proxies.is_empty() && request_came_from_trusted_proxy")` (214-217), `server_rs.contains("operator RPC listener must bind to loopback")` (232), `src.contains("verify_execution_proof_full")` (39), `!tx_src.contains("BnsRenew")` (212). A comment, a string or a renamed variable satisfies or breaks them. `let _ = self.store.save_checkpoint(..)` passes the pos.rs lock. Today pos.rs has one call site (pos.rs:213, `map_err`), so the lock holds, but it does not test behavior.
Description: Text locks stand in for behavior tests. They pass when the code is changed in a way that keeps the text, and they fail on a harmless refactor.
Impact: Maintenance cost and false confidence. For the trusted-proxy item the config issue (FINDINGS.md P21-F07, `trusted_proxies ["*"]`) is not covered by the text lock.
Known: none.

### P45-F11
File: src/tests/bench_performance.rs:13-14, 160
Severity: Low
Axis: dead code
Confidence: Certain
Evidence: `// #[tokio::test]` (line 13) then `#[allow(dead_code)] async fn bench_high_tps()` (14-15) and `#[allow(dead_code)] fn index_of_sender` (160-161).
Description: The whole 163-line file compiles but never runs. Two allow attributes hide that it is dead (Z3: do not add allow).
Impact: No benchmark and no test; imports pull ChainActor, FuturesUnordered into the test build.
Known: FINDINGS.md:192 ("Dead code notes (not verified)"). This entry verifies it.

### P45-F12
File: src/tests/advisory_reachability.rs:310-363, 420-456; .quality/osv-scanner.toml:78
Severity: Low
Axis: test
Confidence: Certain
Evidence: line 353-356 asserts only `entry.contains("effectiveUntil = ")`. The waiver is `effectiveUntil = 2026-12-01` (osv-scanner.toml:78); today is 2026-10-11. The test does not parse the date, so it stays green after the waiver expires. The gossipsub test (421-455) asserts `version_at_least(version, "0.49.4")` for a git-pinned crate; its own comment (414-419) says the version string is not evidence, and the second assert is a hash string match on Cargo.toml, so a revision change plus a copied hash passes with no re-check.
Description: Two locks check text presence and not the property they name.
Impact: An expired lru waiver or an unverified gossipsub revision is not flagged by the test.
Known: none.

### P45-F13
File: src/tests/chain_id_registry_locks.rs:220-224, 248-286
Severity: Low
Axis: test
Confidence: Certain
Evidence: the RPC listeners are parsed with `listener.parse::<SocketAddr>()...is_loopback()` (220-224), and the comment at 216-219 names the `127.attacker.example` prefix bug. The metrics listener check still uses `metrics.starts_with("127.") || metrics.starts_with("[::1]")` (283). The mainnet and testnet key checks run only when `network.profile` is "mainnet" or "testnet" (248); a profile with another or missing string skips them (`unwrap_or("")`, 247). A missing `public_listener` defaults to loopback (228).
Description: The same loopback rule is checked in two ways; the weaker one is the unauthenticated metrics endpoint.
Impact: `metrics.listener = "127.example.com:9100"` passes the lock.
Known: none.

### P45-F14
File: src/tests/blockchair_fixture_locks.rs:782-847, 172-214
Severity: Low
Axis: test
Confidence: Likely (missing step: search for a Bitcoin merkle function outside src/cross_domain/external; rg there found none)
Evidence: `btc_double_sha256` and `btc_merkle_root` (783-828) are defined in the test file and used only by the two merkle tests; `bitcoin_merkle_single_leaf_equals_txid` (840-847) compares two fixture strings from the same tuple. The header (10-13) calls these "the endianness and pairing lock of our Bitcoin-side root computation". The ETH test (196-214) feeds `hash` as a given field; `verify_chain` is not shown to recompute a header hash.
Description: About 450 lines of vectors test a test-local helper, not production code.
Impact: Real vectors give no assurance about the production BTC or ETH header code (only U256 chainwork, ReplayNonceStore, verify_chain linkage are production).
Known: none.

### P45-F15
File: src/tests/ai_inference_runtime.rs:55-86, 319-394, 592-613
Severity: Low
Axis: test
Confidence: Certain
Evidence: `signature: vec![1]` in every AiInferenceResult (73, 599); the test `conflicting_signed_result_commits_evidence_and_burns_only_role8_bond` (320) is named "signed" and applies transactions with `Executor::apply_transaction` directly, which skips the transaction signature check. No test in the file submits an AiInferenceResult or AiDisputeSlash through `add_transaction` with a bad inner or outer signature.
Description: The dispute and slash tests cover executor logic and not admission. Together with the unverified inner digest (advisory_reachability.rs:459-522) there is no test that forged result evidence is refused.
Impact: Slashing of AI operators (stake and bond) has no negative test on the forged-evidence side.
Known: none.

### P45-F16
File: src/tests/chaos.rs:938, 965, 1002, 1029; src/tests/bud_e2e.rs:6, 10, 13; src/tests/advisory_reachability.rs:13; src/tests/ai_inference_runtime.rs:707-838; src/tests/blockchair_fixture_locks.rs:81, 205; src/tests/bud_e2e.rs:229-230, 453
Severity: Info
Axis: invariant(Z13)
Confidence: Certain
Evidence: markdown bold markers in comments: chaos.rs:938 `/// **Double-spend protection:**`, 965, 1002, 1029; bud_e2e.rs:6 `**e2e_three_actor_manifest_to_challenge_flow**`, 10, 13; advisory_reachability.rs:13 `**fixed**`. Turkish text in repo files (Z13 says repo text is English): test names `tavan_ilan_edilmeden_varsayilan_baseline` (708), `stakesiz_operator_tavan_ilan_edemez` (719), `tavan_state_root_a_girer` (801), `slash_tavani_da_kaldirir` (822); `.expect("hex basamak")` (blockchair 81); `"4 onay gerekince red"` (205); `Cache'lerse` (bud_e2e 230); `// (b) izleyici` (bud_e2e 453).
Description: Style rule breaks in test files.
Impact: None on behavior.
Known: none.

### P45-F17
File: src/tests/bns.rs:7; src/tests/bud_e2e.rs:20; src/tests/byzantine_settlement.rs:8; src/tests/chaos.rs:7; src/tests/audit_findings_locks.rs:74; src/tests/bridge_lifecycle.rs:97-107
Severity: Info
Axis: test
Confidence: Certain
Evidence: `test_addr_from_byte` is copied into four files, each with a redundant `#[cfg(test)]` inside a test-only module (for example bns.rs:6-7, byzantine_settlement.rs:7-8). audit_findings_locks.rs:74 reads `std::fs::read_to_string("config/mainnet-genesis.json")` by a path relative to the working directory, while the rest of the part uses `CARGO_MANIFEST_DIR`. bridge_lifecycle.rs funds the same owner twice with `fund_development_account` (100 and 106). `bud_e2e.rs:130-139` builds `req` with `opener: Some(test_addr_from_byte(3u8))` but opens the challenge as `watcher_c = addr(0xC3)`, so the request struct and the call disagree (the struct is then not used).
Description: Test hygiene.
Impact: Low. Relative path breaks if a runner changes the working directory.
Known: none.

## Coverage gaps against known findings (docs/audit/FINDINGS.md)

- P01-F18 and F19 (consensus tests pass for the wrong reason, missing negatives): the same pattern appears in adversarial_p2p.rs (P45-F08); no negative for invalid producer under PoS/PoA in this part.
- P08-F10 (test-only submit_domain_commitment): extended by P45-F03, F02.
- P23-F04 (storage challenge proofs unchecked): not detectable by bud_e2e.rs (P45-F09).
- P21-F06 and F07 (authorizations without chain_id, trusted_proxies "*"): no test in the part. The Pollen digest with the same defect is P45-F01.
- P03 and P05 to P07 bridge items: bridge_negatives.rs and bridge_lifecycle.rs have no supply or balance asserts (P45-F07).

## Counts

Medium 8 (F01 to F08), Low 7 (F09 to F15), Info 2 (F16, F17). Critical 0, High 0. Known overlap: F03 (P08-F10), F09 (P23-F04), F11 (FINDINGS.md:192).
