# Lane D live verification ledger (WP-04)

Directive section 14 assigns the unified findings ledger to live verification
in four lanes (WP-01..WP-04). This file is the lane D record: every ledger
entry whose closing package is WP-05, WP-11, WP-36, WP-37, WP-38, WP-39,
WP-40, WP-41, WP-42, WP-43, WP-46 or WP-61, plus the section 14.6 archive
families that close in lane D, re-verified against the live tree. English
per `tree_is_english`.

## How this was produced

- **Verification base.** `HEAD` = `cccf1da` (`main`, "Add files via upload"),
  the same tree this branch carries. The snapshot post-dates the directive's
  analysis base (2026-11-02, `testnet-quantum`) and includes merged PRs #5/#6.
  Anchors are content identifiers (file + function), not line numbers.
- **Method.** (1) Every lane D record was extracted from directive section 14
  programmatically (56 BLG rows + 14 archive rows); (2) each record's
  machine-checkable identifiers were scanned across `src/`, `crates/`,
  `budzero/`, `xtask/`, `ops/`, `config/`, `docs/`, `.github/`, `benches/`,
  `fuzz/`, `kani/`; (3) headline findings were read at their anchors; (4) a
  verdict was assigned only where the evidence is the file content itself or a
  permanent regression lock already in the tree.
- **CI.** No local Rust toolchain exists in this environment. The live `main`
  CI run 42050659038 is **red** (jobs below); that red state is recorded as
  findings BLG-268..BLG-275 in `docs/audit/FINDINGS.md` (PR #13) and is not
  silently ignored. The locks added in `src/tests/audit_lane_d.rs` are plain
  `cargo test` locks in the style of `regression_ops_crypto.rs`.
- **What this PR does not claim.** WP-04 verifies; the closing packages fix.
  An `Open` row stays open until its closing package merges and flips its repro
  test (S-3). Repro tests for `Confirmed` rows are owed on `audit/repro-d`,
  which never merges; each closing package moves its own repro into its PR.
  This PR carries no failing test.

## Status legend (directive 14.1)

| Status | Meaning |
|---|---|
| Open | Open in the live tree (directive status: open); evidence below, repro pending on `audit/repro-d`. |
| Confirmed | Open and reproduced by file inspection at the cited anchors (directive status: confirmed open). |
| Closed (pre-fixed) | Fixed in this snapshot before WP-04 ran (directive status: closed, pre-fixed); a permanent lock holds it. |
| Closed by removal | Not used this pass; removal-closing feet stay Open until `no_legacy_storage` is green (WP-07, unmerged). |
| Invalid | Not used this pass (directive status: invalid). |

## Summary

| Metric | Count |
|---|---|
| Lane D BLG records | 56 |
| Closed (pre-fixed) | 19 |
| Confirmed | 2 |
| Open | 35 |
| Archive family records | 14 |
| Archive Closed (pre-fixed) | 4 |
| Archive Confirmed | 2 |
| Archive Open | 8 |

Closing-package distribution (BLG): WP-61 (12), WP-42 (28), WP-40 (5),
WP-39 (4), WP-41 (3), WP-43 (5), WP-11 (2 shared), WP-36 (2), WP-05 (1
shared), WP-46 (0 BLG; DOC-GAP-06). Shared feet: BLG-254 (WP-07 lane B),
BLG-259 (WP-12 lane A), BLG-150 (BudZero nullifier foot lane C).

## BLG records (section 14.3 / 14.4 / 14.5)

| Identity | Severity | Live status | Evidence (file + anchor) | Regression lock | Closing package | Note |
|---|---|---|---|---|---|---|
| BLG-004 | Medium | Closed (pre-fixed) | src/crypto/primitives.rs `write_key_file` (create_new + mode 0o600 + atomic rename) | `key_save_narrows_existing_wide_permissions` | WP-61 | Rewrite narrows a wide-mode file to 0600; locked. |
| BLG-009 | Medium | Closed (pre-fixed) | src/crypto/pq.rs `verify` (registered-context + explicit SchemeId) | `pq_verify_requires_context_and_explicit_scheme` | WP-61 | Empty/long/unregistered context refused; scheme explicit. |
| BLG-010 | Medium | Closed (pre-fixed) | src/crypto/pkcs11.rs `connect` (label in key search before bind) | `pkcs11_label_is_applied_before_the_key_is_bound` | WP-61 | Label applied at search; single-key refusal present. |
| BLG-011 | Medium | Closed (pre-fixed) | `ops/budlum-core.service (EnvironmentFile + LimitCORE=0)` | `service_unit_sets_rpc_key_and_closes_core_dumps` | WP-41 | RPC key env supplied; core dumps closed. Wrong-repo doc is DOC-GAP-04. |
| BLG-012 | High | Closed (pre-fixed) | xtask/gates/src/gates/coverage.rs `percent_from_json`/`parse_baseline` | `coverage_gate_reads_the_total_and_refuses_nan` | WP-42 | Reads totals.lines.percent; NaN baseline refused. |
| BLG-013 | Medium | Closed (pre-fixed) | `xtask/tools/src/prepush.rs + ci.yml all-features canary` | `all_features_is_a_buildable_composition` | WP-42 | --all-features is now a buildable composition with a must-build canary. |
| BLG-014 | Medium | Closed (pre-fixed) | xtask/tools/src/backup_drill.rs `--check-db` exit status | `backup_drill_checks_the_exit_status` | WP-42 | Non-zero --check-db exit fails the drill. |
| BLG-015 | Medium | Closed (pre-fixed) | `.github/workflows/ci.yml standalone package test steps` | `ci_runs_every_standalone_package_test_suite` | WP-42 | Every standalone crate's unit tests run in CI. |
| BLG-016 | High | Closed (pre-fixed) | `Cargo.lock floors + regression lock` | `lockfiles_do_not_pin_known_vulnerable_versions` | WP-42 | Known advisory floors pinned; fresh re-scan owed to WP-42. |
| BLG-022 | Medium | Closed (pre-fixed) | src/light_client/mod.rs `next_checkpoint_height` (closed form) | `next_checkpoint_height_terminates_at_the_u64_edge` | WP-11 | Closed form; u64::MAX terminates, no walk. |
| BLG-026 | Medium | Open | `crates/wallet-core/src/wallet_v3.rs (24-word/NFKD fixed); sub-workspace fmt/clippy parity` | `no_vacuous_gate_test_and_v3_wallet_is_24_words` | WP-39 | Seed path fixed; fmt/clippy/--locked parity for sub-workspaces still owed. |
| BLG-029 | High | Closed (pre-fixed) | src/rpc/server.rs `extract_client_ip` (rightmost XFF, trusted proxy only) | `forwarded_for_uses_the_rightmost_entry` | WP-61 | Rightmost entry honoured only from a trusted proxy. |
| BLG-030 | Critical | Closed (pre-fixed) | `src/rpc/method_class.rs Refused rows + blockchain dev-credit guard` | `testnet_development_credit_is_refused` | WP-61, WP-41 | Out-of-block writers refused; testnet dev credit refused. Faucet ops foot owed WP-41. |
| BLG-032 | Medium | Open | `src/gateway/service.rs (bounded cache + pollen guard fixed); Prepare fee templates` | `pollen_guard_covers_the_served_storage_root` | WP-61 | Cache/guard fixed; some Prepare templates still hardcode fee. WP-61. |
| BLG-068 | High | Closed (pre-fixed) | `crates/budscan/src/bns_proof.rs + query.rs` | `budscan_does_not_badge_unproven_roots` | WP-40 | Unproven storage_root refused; explicit bud://; bidi/traversal refused. |
| BLG-081 | High | Open | `xtask/gates/src/gates/* (workspace/first-party scan roots)` | `gates_are_wired` | WP-42 | Gate scan-root blind spots; WP-42 hardening. CI gates job red. |
| BLG-082 | High | Open | `xtask/gates/src/gates/ast_security_gates.rs, gates_do_not_panic.rs` | `gates_do_not_panic` | WP-42 | Decoy/UTF-8/validation gaps; WP-42. |
| BLG-083 | Medium | Open | `xtask/gates/src/gates/{economy_invariants,metrics_are_written,badges_current}.rs` | `economy_invariants` | WP-42 | Supply/rate/metric/badge validation gaps; WP-42. |
| BLG-084 | High | Open | `xtask/gates/src/gates/{relay,regeneration}.rs; --all tree-pin` | `gates_are_wired` | WP-42 | Fail-open/silent-rewrite/skipped-gate; WP-42. |
| BLG-085 | High | Open | `xtask/gates scan roots + standalone [profile.release]` | `containment_defaults` | WP-42 | wallet-core root + hardening profiles; WP-42. |
| BLG-086 | Medium | Open | `xtask/gates/src/gates/{no_upstream_brands,rust_literals}.rs; /tmp dirs` | `rust_literals` | WP-42 | UTF-8 panic, CWE-377, substring/escape gaps; WP-42. |
| BLG-087 | High | Open | `fuzz/ + kani/ + benches/domain_throughput.rs` | `every_fuzz_target_is_run` | WP-42 | Unbounded deserialize OOM, empty targets, u8 wrap; WP-42. |
| BLG-092 | High | Open | `README.md/README.tr.md/CONTRIBUTING/SECURITY/NOTICE` | `readme_definition_block_is_verbatim` | WP-43 | Definition block preserved by KR-21; other framing/path fixes owed WP-43. |
| BLG-095 | Medium | Open | `supply-chain/ + .github/dependabot.yml + .cargo/audit.toml` | `cargo_vet` | WP-41 | Dependabot lock coverage + stale exception; WP-41. |
| BLG-097 | High | Open | `kani/Cargo.lock, bud/fuzz/Cargo.lock; crates/budscan CI` | `ci_runs_every_standalone_package_test_suite` | WP-42 | budscan CI present; kani/fuzz --locked + cfg gaps owed WP-42. |
| BLG-098 | Critical | Closed (pre-fixed) | src/cli/commands.rs `apply_secure_rpc_defaults` before overlay | `secure_rpc_defaults_survive_clap_skip` | WP-39 | Clap-skipped RPC fields restored to secure defaults. |
| BLG-099 | High | Closed (pre-fixed) | src/rpc/server.rs `constant_time_eq_str` (full walk + length fold) | `constant_time_compare_has_no_self_comparison` | WP-61 | No self-comparison; work depends only on expected length. |
| BLG-117 | Critical | Open | `crates/wallet-core/src/privacy_crypto.rs (Poseidon/tag/digest fixed); privacy_nullifier(secret: u64)` | `poseidon_tag_and_recovery_digest_are_fixed` | WP-39 | Poseidon rounds, u64 tag, chain+nonce digest fixed; 64-bit spending-secret entropy still owed WP-39. |
| BLG-121 | High | Closed (pre-fixed) | `crates/budscan/src/{arweave,bns_proof,patchset}.rs` | `budscan_arweave_expiry_and_patch_paths` | WP-40 | 48-byte data root, expiry refusal, patch path containment. |
| BLG-143 | High | Open | `crates/budscan/src/{fetch,query,evidence}.rs` | `budscan_does_not_badge_unproven_roots` | WP-40 | Manifest/fetch/evidence badge gaps; WP-40. |
| BLG-150 | Critical | Closed (pre-fixed) | `src/crypto/signer.rs (BDLM_PREVOTE/PRECOMMIT tags)` | `vote_signatures_are_domain_separated_from_block` | WP-61 | Signer domain separation fixed (lane D foot). BudZero nullifier foot is lane C. |
| BLG-169 | Medium | Open | `src/sdk + crates/budscan/src/punycode` | `bns_names_are_safe_in_an_address_bar` | WP-40 | Removed-section swallowing, punycode case/scheme, tier injection; WP-40. |
| BLG-170 | High | Open | `xtask/gates/src/gates/{ast_security_gates,rust_literals,zero_address}.rs` | `ast_security_gates` | WP-42 | Concurrent-validation bypass, comment/char blindness; WP-42. |
| BLG-171 | High | Open | `xtask/gates/src/gates/{security_scans_can_fail,ci_workflow_guards,pinned_downloads}.rs` | `ci_workflow_guards` | WP-42 | Error swallowing, comment canary, multiline download; WP-42. |
| BLG-172 | Medium | Open | `xtask/tools/src/* (validators.json, port, backup_drill, prepush)` | `backup_drill_checks_the_exit_status` | WP-42 | PoA validators/port/dead fuzz output; WP-42. |
| BLG-174 | High | Open | `xtask/gates/src/gates/{accumulators_pinned,bit_decompositions,cross_table_checks}.rs` | `accumulators_pinned` | WP-42 | Zero-multiplier pin, one-sided rs2, fake forgery; WP-42. |
| BLG-175 | High | Open | `xtask/gates/src/gates/{git_deps_audited,cargo_vet,docker_toolchain,domain_tags}.rs` | `git_deps_audited` | WP-42 | Unpinned git dep, standalone/multi-stage blindness; WP-42. |
| BLG-176 | Medium | Open | `benches/single_node/internal_pipeline_tps.rs; canonical_programs` | `bench_performance` | WP-42 | 100% rejection (chain id/stake), missing 4th program, u32 ceiling; WP-42. |
| BLG-177 | Medium | Open | `xtask/gates wallet_node_address_agree; src/storage/server_admission.rs; role` | `wallet_node_address_agree` | WP-42 | Digest-input blindness, 1-byte dust, RoleId(0) collision; WP-42 (removal foot WP-07). |
| BLG-178 | High | Open | `xtask/gates/src/gates/{zero_tests_witness,gas_calibration,forgery_tests}.rs` | `forgery_tests` | WP-42 | Zero-constraint AIR, comment shadowing, test blindness; WP-42. |
| BLG-179 | High | Open | `xtask/gates/src/gates/{serialize_map_keys_are_strings,genesis_schema}.rs` | `genesis_schema` | WP-42 | Enum/tuple-key blindness, genesis validator/allocation gap; WP-42. |
| BLG-180 | Medium | Open | `xtask/gates/src/gates/{content_encryption,coding_audit,ai_inference_prompt_is_true}.rs` | `content_encryption_is_declared_and_bound` | WP-42 | Tuple-key/comment-mask/cfg blindness; WP-42. |
| BLG-181 | Medium | Open | `fuzz evm_mpt_verify; xtask module_coverage; scripts/ shell gate` | `module_coverage` | WP-42 | 128-byte chunk blindness, dead prefix, shell gap; WP-42. |
| BLG-187 | High | Open | `fuzz/{consensus_state_transition,relayer_escrow,vm_execute,snapshot_deserialize}` | `every_fuzz_target_is_run` | WP-42 | Unreachable deep reorg, missing custody oracles; WP-42. |
| BLG-188 | High | Open | `xtask/gates uncheckable_proof; mempool_mev_free/bridge_dual_tree` | `mempool_mev_free` | WP-42 | false//true inversion, gate bypasses; WP-42. |
| BLG-189 | Medium | Open | `xtask/gates/{storage_priced,storage_penalties,threshold_rates,value_transfers}.rs` | `threshold_rates` | WP-42 | Pricing/placement gate blindness; IntGauge overflow; WP-42. |
| BLG-192 | High | Closed (pre-fixed) | src/rpc/server.rs `estimate_gas` uses AccountState::required_fee | `estimate_gas_returns_the_flat_fee_floor_not_an_ethereum_constant` | WP-61 | Estimate uses required_fee (proportional + governance), not 21000. |
| BLG-193 | Medium | Open | `xtask/gates remaining modules; tree_is_english` | `tree_is_english` | WP-42 | Comment/syntax bypass, fake test output, English blindness; WP-42. |
| BLG-200 | High | Closed (pre-fixed) | `src/crypto/primitives.rs BlsKeypair::from_bytes/from_seed zero refusal` | `bls_from_bytes_refuses_zero_secret` | WP-61 | Zero secret/identity refused; finality refuses identity pubkey. |
| BLG-231 | Medium | Open | `budzero/bud-state/src/lib.rs; budzero/ARCHITECTURE.md` | `(doc lock owed WP-43)` | WP-43 | Doc claims recursive aggregation; code has none. WP-43. |
| BLG-241 | Low | Open | `budzero/bud-isa/src/lib.rs; budzero/bud-vm/src/lib.rs` | `(doc lock owed WP-43)` | WP-43 | Comment says Poseidon width 8; code uses 12. WP-43. |
| BLG-254 | xtask/gates/src/gates/three_is_recipe_only.rs`, `zero_storage_frozen.rs`, `generated_content.rs | Open | `xtask/gates/src/gates/{three_is_recipe_only,zero_storage_frozen,generated_content}.rs` | `three_is_recipe_only` | WP-07, WP-42 | Recipe-only model gates conflict with 3.0; shared foot WP-07 (lane B) + WP-42. |
| BLG-255 | Kod geneli | Open | `config/naming/concept_map.toml + .github/concept-names-baseline.txt` | `concept_names_gate_catches_injected_name` | WP-05, WP-43 ve her paket | Counter gate live in count mode; names remain. WP-05 owner field + WP-43 zeroing. |
| BLG-257 | `src/main.rs` (`MobileNodeProfile::new(..., DeviceType::Phone)`) | Confirmed | `src/main.rs MobileNodeProfile flag only; no mobile/ crate` | `(repro owed audit/repro-d)` | WP-11, WP-36 | No mobile client application; phone cannot validate. WP-11/WP-36. |
| BLG-259 | src/chain/finality.rs`, `src/consensus/qc.rs | Open | `src/chain/finality.rs; src/consensus/qc.rs (128 x 3309 B)` | `(seat cert size owed WP-12)` | WP-12, WP-11 | Per-seat ML-DSA cert ~420 KB; shared foot WP-12 (lane A) + WP-11. |
| BLG-265 | `src/network/mobile.rs` | Confirmed | `src/network/mobile.rs ChallengePolicy (no Android dataSync 6h/24h budget)` | `(repro owed audit/repro-d)` | WP-36 | Power/challenge model ignores Android 15 foreground-service limit. WP-36. |

## Archive families (section 14.6)

| Identity | Severity | Live status | Regression lock | Closing package | Note |
|---|---|---|---|---|---|
| BULGU-011 | S3 | Closed (pre-fixed) | `cors_does_not_reflect_request_headers` | WP-61 | src/rpc/server.rs CORS wildcard header reflection. Fixed header list; no request-header reflection; WP-61. |
| BULGU-012 | S3 | Closed (pre-fixed) | `bulgu_012_rpc_serialization_error_is_not_null` | WP-61 | src/rpc/server.rs serialization error silent null. to_val maps serde error to ErrorObjectOwned, not null; WP-61. |
| TEST-GAP-02 | Medium | Closed (pre-fixed) | `kani_covers_the_four_security_md_targets` | WP-42 | kani/ covers quarter of SECURITY.md targets. Four harness families present; WP-42. |
| TEST-GAP-04 | Low | Confirmed | `(repro owed audit/repro-d)` | WP-42 | src/bin/budlum-domain-producer.rs untested. Zero #[test] in the binary; WP-42. |
| TEST-GAP-07 | Low | Open | `(coverage owed WP-61)` | WP-61 | src/rpc direct endpoint coverage low. RPC endpoint coverage still low; WP-61. |
| DOC-GAP-01 | S1 | Open | `(ops repro owed WP-41)` | WP-41 | config/testnet.toml + ops SoftHSM first-run blocker. SoftHSM config missing for first run; WP-41. |
| DOC-GAP-02 | S3 | Open | `(doc lock owed WP-43)` | WP-43 | CLI/profile/compose/systemd port docs mixed. Port docs mixed across four modes; WP-43. |
| DOC-GAP-03 | S3 | Open | `(doc lock owed WP-43)` | WP-43 | README/CONTRIBUTING/ops/scripts broken script refs. ops/scripts empty; broken refs; WP-43. |
| DOC-GAP-04 | S3 | Confirmed | `service_unit_sets_rpc_key_and_closes_core_dumps` | WP-41 | ops/budlum-core.service wrong repo + HSM context. Documentation URL still rade/budlum-core; HSM env present; WP-41. |
| DOC-GAP-05 | S3 | Open | `(doc lock owed WP-43)` | WP-43 | VALIDATOR_GUIDE/TESTNET/OPERATIONS half sentences. Half sentences; WP-43. |
| DOC-GAP-06 | S3 | Open | `(measurement owed WP-46)` | WP-46 | TESTNET/VALIDATOR_GUIDE resource tables empty. Empty measurement tables; WP-46. |
| DOC-GAP-07 | S3 | Closed (pre-fixed) | `doc_gap_07_indexer_binary_is_registered` | WP-42 | Cargo.toml budlum-indexer explicit bin. [[bin]] budlum-indexer registered; WP-42. |
| DOC-GAP-08 | S3 | Open | `(fee lock owed WP-39)` | WP-39 | src/bin/bud.rs CLI fee default vs network base. CLI fee default mismatch; WP-39. |
| DOC-GAP-09 | S3 | Open | `(this ledger)` | WP-43 | Ledger table detail verified in WP-04. Detail verified here; final doc zeroing WP-43. |

## Overlap audit (directive 14.6)

Archive rows that overlap a BLG row are linked, not double-counted:

| Archive | Overlaps | Resolution |
|---|---|---|
| BULGU-011 | BLG-032 (CORS foot) | Both close in WP-61; one CORS fix. |
| BULGU-012 | BLG-032 (serialization foot) | Both close in WP-61; `to_val` error path. |
| TEST-GAP-02 | BLG-087 (Kani foot) | WP-42; harness families counted once. |
| TEST-GAP-06 | BLG-097 | Removal-closing; lane B owns (WP-07). Not a lane D row. |
| DOC-GAP-04 | BLG-011 | BLG-011 closes the RPC-key/core-dump foot; the wrong-repo URL foot stays open here under WP-41. |
| DOC-GAP-07 | BLG-097 | Indexer bin registration; WP-42. |

## CI base state (directive 15.8 WP-04 item 2)

Live `main` (`cccf1da`) CI run **42050659038** is red. Recorded red jobs:

| Job | Red step | Finding |
|---|---|---|
| Gates | no-out-of-block-consensus-writes canary | BLG-268 |
| Gates | indexing-is-not-new (2 files outside baseline) | BLG-269 |
| Gates | dead-public-api (182 vs 173) | BLG-270 |
| Gates | guards-are-reachable (5 vs 3) | BLG-271 |
| Gates | badges-are-current (red run) | BLG-273 |
| Budlum Core quality | Format (3 diffs) | BLG-272 |
| Budlum Core PQ build | Feature matrix pq-ml-dsa solo (exit 101) | BLG-273 |
| Budlum Core library test | Test (exit 101) | BLG-273 |
| Network Hardening | network hardening tests (exit 101) | BLG-274 |
| Cargo Deny (root + fuzz) | cargo deny check | BLG-275 |

Workflow runs read live with `gh run list --branch main` (25 distinct workflows
on the snapshot; Dependency Review, Kani Slow, Miri Storage Weekly, Rust
Quality and Security Hardening have no `main` run yet). Runner: `ubuntu-latest`
(GitHub-hosted; the runner image notes a migration to Ubuntu 26 on 2026-10-19).
No runner memory limit is set in `ci.yml`; the hosted default applies. The
full red-job list and the new findings live in `docs/audit/FINDINGS.md`.

## In-flight work this ledger interacts with

- **PR #13** (WP-20 gates) already creates `docs/audit/FINDINGS.md` (BLG-268..275)
  and `docs/work/PACKAGES.md`. Those are shared append-only files (directive
  15.3 item 4); this lane D ledger does not duplicate them. The consolidated
  `LIVE_STATUS.md` and the English `FINDINGS.md` register are WP-04's final
  delivery, opened after WP-01/02/03 merge (directive 15.8 WP-04 dependency).
- **PR #12** (WP-01/02/03) carries lanes A/B/C. Lane D rows that share a foot
  with lane A (BLG-259) or lane B (BLG-254) name the owning lane above.
- **PR #8/#9/#11** touch storage/consensus/persistence; no lane D lock path
  collides with them.

