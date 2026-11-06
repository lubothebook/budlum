# WP-04 live verification: lane D

## Verification base and protocol

| Field | Verified value |
|---|---|
| Date | 2026-11-06 UTC |
| Branch | `arena/bfdb31fd-budlum`, branched from `main` |
| Base commit | `cccf1da1f4bcfe97503a3a00cfba24b172b49b31` |
| Live main check | `git ls-remote origin refs/heads/main` returned the same SHA at 2026-11-06T05:19:25Z |
| Open PRs read | #8 (storage/codec stream), #9 (WP-12), #11 (WP-06), #12 (WP-01/02/03), #13 (WP-20) |
| Main CI | Red: run 42050659038; failed jobs and all workflow states are in `ci-baseline.md` |
| Delivery | WP-04, first delivery. The consolidated final delivery waits for WP-01/02/03 to merge. |

The directive is the source record supplied with this snapshot. Its analysis base is older than this branch. Every anchor below was located by path and identifier on the base commit, not by an old line number. The external historical ledger and original `testnet-quantum` ZIP are not present in the checkout; no claim is made to have re-read those missing sources. The directive section 14 rows are the available source descriptions.

## Status rule

- **Closed (pre-fixed)**: the live implementation has the required behavior and a permanent regression test exists or is added here. Merge and green CI are still required before this becomes closure evidence.
- **Open**: the work remains, or a grouped finding has at least one unresolved leg. Source inspection is evidence of the anchor, not a behavioral reproduction.
- **Confirmed** is reserved for a red reproduction with its output and CI run. No row in this delivery earns that status. Red inherited CI is recorded separately and does not prove an unrelated historical finding.
- A test marked **owed** is a selected name, not an existing test. The closing package must write it, observe it red before the fix, and carry the output in its PR.
- Archive S1/S2/S3 labels are preserved. They are not silently converted into Critical/High/Medium without the historical mapping.

## Summary

| Status | Count |
|---|---|
| Closed (pre-fixed) | 9 |
| Open | 61 |

| Severity | Count |
|---|---|
| Critical | 4 |
| High | 26 |
| Low | 4 |
| Medium | 25 |
| S1 (archive) | 1 |
| S3 (archive) | 10 |

The counts describe these 70 assigned records, not the whole 311-record ledger. Grouped findings count once. Shared ownership does not duplicate a row.

## Findings

| Identity | Severity | English title | Live status | Regression test | Live evidence (path and function/setting) | Closing package |
|---|---|---|---|---|---|---|
| BLG-004 | Medium | Rewriting a key file retains its existing wide permissions | Closed (pre-fixed) | `audit_lane_d_blg_004_key_rewrite_is_owner_only` | `src/crypto/primitives.rs::write_key_file` | WP-61 |
| BLG-009 | Medium | PQ API lacks a mandatory registered FIPS context and explicit scheme identifier | Closed (pre-fixed) | `pq_verify_requires_context_and_explicit_scheme` | `src/crypto/pq.rs::verify`; `src/tests/regression_ops_crypto.rs::pq_verify_requires_context_and_explicit_scheme` | WP-61 |
| BLG-010 | Medium | PKCS#11 key label is applied after an unlabeled key has been bound | Open | `blg_010_pkcs11_label_selects_before_binding` (owed) | `src/crypto/pkcs11.rs::extract_ed25519_public_key`, `private_key_handle`, `verify_key_binding` | WP-61 |
| BLG-011 | Medium | systemd unit omits the runtime RPC credential environment and enables core dumps | Closed (pre-fixed) | `audit_lane_d_blg_011_service_supplies_credentials_and_disables_dumps` | `ops/budlum-core.service`: `LimitCORE`, `Environment`, `EnvironmentFile` | WP-41 |
| BLG-012 | High | Coverage ratchet reads the first file instead of the total and accepts a NaN baseline | Closed (pre-fixed) | `the_total_is_read_not_the_first_file`; `baseline_must_be_a_finite_percentage` | `xtask/gates/src/gates/coverage.rs::percent_from_json`, `parse_baseline` | WP-42 |
| BLG-013 | Medium | The prepush helper runs the intentionally disabled all-features combination | Open | `blg_013_prepush_uses_supported_features` (owed) | `xtask/tools/src/prepush.rs` | WP-42 |
| BLG-014 | Medium | Backup drill ignores the check-db exit code and accepts substring matches | Open | `blg_014_backup_drill_propagates_check_failure` (owed) | `xtask/tools/src/backup_drill.rs` | WP-42 |
| BLG-015 | Medium | Three standalone Rust packages are outside the root workspace test run | Open | `blg_015_standalone_tests_are_required` (owed) | `.github/workflows/ci.yml`: standalone crate jobs; `crates/*/Cargo.toml` | WP-42 |
| BLG-016 | High | Fresh RustSec scanning finds patchable advisories in six tracked lockfiles | Open | `blg_016_lockfiles_clear_fresh_advisories` (owed) | tracked `Cargo.lock` files; CI 42050659038 jobs 127488354574 and 127488354282 fail at cargo deny; per-advisory output not retrieved | WP-42 |
| BLG-022 | Medium | Light-client checkpoint selection loops at the height boundary and scales with the interval | Closed (pre-fixed) | `audit_lane_d_blg_022_checkpoint_selection_handles_boundaries` | `src/light_client/mod.rs::LightClient::next_checkpoint_height` | WP-11 |
| BLG-026 | Medium | Standalone wallet/AI quality gaps and the 12-word, non-NFKD wallet seed path | Open | `wallet_refuses_12_word_seed`; `wallet_mnemonic_is_nfkd_normalised` (owed) | `crates/wallet-core/src/lib.rs::mnemonic_to_entropy`, `Wallet::from_mnemonic`; standalone CI jobs | WP-39 |
| BLG-029 | High | Forwarded-for parsing and rate-limit eviction undermine IP quotas and allowlists | Open | `blg_029_forwarded_ip_and_quota_are_bound` (owed) | `src/rpc/server.rs::extract_client_ip`, `is_per_ip_rate_limited` | WP-61 |
| BLG-030 | Critical | Public RPC writes consensus state outside blocks; the development faucet remains on testnet | Open | `audit_lane_d_blg_030_testnet_development_credit_is_closed` locks the fixed leg; full method audit owed | `src/rpc/method_class.rs::METHOD_CLASSES`; `src/chain/blockchain.rs::fund_development_account` | WP-61, WP-41 |
| BLG-032 | Medium | Gateway protection and retrieval disagree, its cache resets, and Prepare templates carry invalid nonce/fee | Open | `prepare_templates_carry_valid_nonce_and_fee` covers one leg; gateway/cache repro owed | `src/gateway/service.rs`; `src/rpc/server.rs::finish_template` | WP-61 |
| BLG-068 | High | Explorer verification badges, bud:// resolution, address-bar, CID and scoring defects | Open | `blg_068_explorer_badge_needs_bound_proof` (owed) | `crates/budscan/src/{evidence,query,content_id,name_rule}.rs` | WP-40 |
| BLG-081 | High | Security gates miss standalone workspaces and misclassify first-party unsafe code | Open | `blg_081_gates_cover_first_party_workspaces` (owed) | `xtask/gates/src/gates/ast_security_gates.rs`; `.github/workflows/supply-chain-extra.yml` | WP-42 |
| BLG-082 | High | AST gate loop-decoy, UTF-8 slicing, reachable-guard and chain-id verification defects | Open | `blg_082_ast_gates_reject_bypasses` (owed) | `xtask/gates/src/gates/{ast_security_gates,gates_do_not_panic,guards_reachable}.rs` | WP-42 |
| BLG-083 | Medium | Supply, ratio-consensus, metrics, indexing, badge and actionlint gate gaps | Open | `blg_083_quality_gate_canaries` (owed) | `xtask/gates/src/gates/{economy_invariants,multi_ratio_consensus,metrics_are_written,indexing_is_not_new,badges_current,actionlint}.rs` | WP-42 |
| BLG-084 | High | Relay verification fails open; state-recovery gate rewrites a deleted file; tree-pin is omitted | Open | `blg_084_integrity_gates_fail_closed` (owed) | `xtask/gates/src/gates/relay.rs`; `xtask/gates/src/{main.rs,gates/regeneration.rs}` | WP-42 |
| BLG-085 | High | Gate roots omit wallet-core; standalone release profiles and security scans are incomplete | Open | `blg_085_security_profiles_cover_workspaces` (owed) | `xtask/gates/src/gates/`; standalone `Cargo.toml` release profiles | WP-42 |
| BLG-086 | Medium | Gate UTF-8 panic, predictable temporary paths, test-name substring and literal-escape defects | Open | `blg_086_gate_parser_canaries` (owed) | `xtask/gates/src/gates/{no_upstream_brands,named_tests,rust_literals,tree_is_english}.rs` | WP-42 |
| BLG-087 | High | Fuzz, Kani and benchmark targets miss their intended deep validation paths | Open | `blg_087_validation_targets_reach_oracles` (owed) | `fuzz/fuzz_targets/`; `kani/src/lib.rs`; `benches/domain_throughput.rs` | WP-42 |
| BLG-092 | High | Public framing, private-key argv exposure and stale documentation paths; definition block is exempt | Open | `blg_092_documentation_never_exposes_private_key_argv` (owed) | `README.md`; `README.tr.md`; `docs/{CONTRIBUTING,SECURITY,NOTICE}` | WP-43 |
| BLG-095 | Medium | Supply-chain/Dependabot lockfile coverage, audit exceptions, CODEOWNERS and stale workflow references | Open | `blg_095_operations_cover_every_lockfile` (owed) | `.github/dependabot.yml`; `.cargo/audit.toml`; `.github/CODEOWNERS`; `ops/scripts/` | WP-41 |
| BLG-097 | High | Missing standalone lockfiles, Kani cfg build error, orphan proof and explorer CI gaps | Open | `blg_097_standalone_validation_builds_locked` (owed) | `kani/{Cargo.lock,src/lib.rs}`; `bud/fuzz/`; `bud/kani/`; `.github/workflows/ci.yml` | WP-42 |
| BLG-098 | Critical | NodeConfig arg skips reset RPC security; auth startup and validator operator listener defects | Open | `blg_098_node_config_preserves_rpc_security` (owed) | `src/cli/commands.rs::NodeConfig`, `apply_secure_rpc_defaults`; `src/main.rs` listener selection | WP-39 |
| BLG-099 | High | API-key length mismatch uses a self-comparison that optimized code can eliminate | Closed (pre-fixed) | `audit_lane_d_blg_099_secret_comparison_refuses_length_mismatch`; timing-safe CI owns the timing claim | `src/rpc/server.rs::constant_time_eq_str` | WP-61 |
| BLG-117 | Critical | Wallet Poseidon/recipient-tag mismatch, short spending entropy and recovery replay defects | Open | `blg_117_wallet_privacy_and_recovery_are_bound` (owed) | `crates/wallet-core/src/{privacy_crypto,privacy_transfer,lib}.rs` | WP-39 |
| BLG-121 | High | Explorer ENS proof width, expired BNS badge, unbound balance proof and patch traversal defects | Open | `blg_121_explorer_proofs_are_bound_and_current` (owed) | `crates/budscan/src/{ens,bns_proof,light_client,patchset}.rs` | WP-40 |
| BLG-143 | High | Explorer refuses real content manifests; HTTPS validation, evidence reasons and profile decoding defects | Open | `blg_143_explorer_accepts_only_valid_bound_content` (owed) | `crates/budscan/src/{fetch,query,evidence}.rs`; `src/cross_domain/external/profile.rs` | WP-40 |
| BLG-150 | Critical | Consensus signer domain separation/address defects and BudZero failed-nullifier proof acceptance | Open | `blg_150_signatures_and_nullifiers_fail_closed` (owed) | `src/crypto/{signer,pkcs11}.rs`; `src/chain/chain_actor.rs`; `budzero/bud-proof/src/{transfer_verdict,relayer}.rs`; `budzero/bud-vm/src/private_transfer.rs` | WP-61 |
| BLG-169 | Medium | Project manifest silently accepts retired sections; explorer name validation and tier-label defects | Open | `blg_169_manifest_and_names_refuse_ambiguous_input` (owed) | `src/developer_os.rs`; `crates/budscan/src/{punycode,name_rule}.rs`; `crates/ai-inference/crates/` | WP-40 |
| BLG-170 | High | AST/source gates have concurrent-edit, comment and character-literal blind spots | Open | `blg_170_source_gates_reject_syntax_bypasses` (owed) | `xtask/gates/src/gates/{ast_security_gates,rust_literals,zero_address,gov_slash,tee_trust_boundary,quantum_account,zkvm_immune}.rs` | WP-42 |
| BLG-171 | High | Workflow security gates accept swallowed failures, comment canaries and multiline downloads | Open | `blg_171_workflow_gates_reject_masked_failures` (owed) | `xtask/gates/src/gates/{security_scans_can_fail,ci_workflow_guards,pinned_downloads}.rs` | WP-42 |
| BLG-172 | Medium | Developer tools emit invalid PoA config/port collisions and incomplete fuzz/backup/prepush checks | Open | `blg_172_developer_tools_reject_invalid_output` (owed) | `xtask/tools/src/{devnet,backup_drill,prepush}.rs`; `fuzz/` | WP-42 |
| BLG-174 | High | ZK/AIR gates miss zero-multiplier rows, one-sided canonicality and fake forgery tests | Open | `blg_174_air_gates_reject_zero_row_bypasses` (owed) | `xtask/gates/src/gates/{accumulators_pinned,bit_decompositions,logup_multipliers,cross_table_checks,every_opcode_forgery}.rs` | WP-42 |
| BLG-175 | High | Supply-chain, Docker toolchain and domain-tag gates miss pinned Git/standalone/multistage cases | Open | `blg_175_supply_chain_gates_cover_all_inputs` (owed) | `xtask/gates/src/gates/{git_deps_audited,cargo_vet,docker_toolchain,domain_tags}.rs` | WP-42 |
| BLG-176 | Medium | Internal benchmark rejects every transaction; canonical-program, counter and test-module hygiene gaps | Open | `blg_176_benchmarks_accept_their_own_transactions` (owed) | `benches/`; `budzero/bud-proof/benches/canonical_programs.rs`; public test modules | WP-42 |
| BLG-177 | Medium | Address-agreement, device-admission, role-id and magic-registration gate gaps | Open | `blg_177_address_and_role_gates_refuse_collisions` (owed) | `xtask/gates/src/gates/wallet_node_address_agree.rs`; `src/network/mobile.rs`; `src/registry/role.rs` | WP-42 |
| BLG-178 | High | Zero-test witness, gas-calibration, forgery-test discovery and required-test gate gaps | Open | `blg_178_test_discovery_cannot_be_vacuous` (owed) | `xtask/gates/src/gates/{zero_tests_witness,gas_calibration,forgery_tests,required_tests}.rs` | WP-42 |
| BLG-179 | High | Snapshot map-key, genesis-schema, storage-proof and evidence-provenance gate gaps | Open | `blg_179_schema_and_evidence_gates_bind_inputs` (owed) | `xtask/gates/src/gates/{serialize_map_keys_are_strings,genesis_schema,storage_proof_boundary,evidence_provenance}.rs` | WP-42 |
| BLG-180 | Medium | Content-encryption, coding-audit, AI-prompt and idle-code gates miss declared/test-only variants | Open | `blg_180_gates_see_declared_and_test_only_paths` (owed) | `xtask/gates/src/gates/{content_encryption_is_declared_and_bound,coding_audit_samples_the_relationship,ai_inference_prompt_is_true,no_idle_code}.rs` | WP-42 |
| BLG-181 | Medium | Fuzz chunking, module coverage, shell-gate scope and public test-module gaps | Open | `blg_181_fuzz_and_module_scope_are_complete` (owed) | `fuzz/fuzz_targets/evm_mpt_verify.rs`; `xtask/gates/src/gates/{module_coverage,no_new_shell_gates}.rs` | WP-42 |
| BLG-187 | High | Consensus, relayer, VM, WOTS and snapshot fuzz targets lack deep oracles | Open | `blg_187_fuzz_oracles_reject_deep_invalid_state` (owed) | `fuzz/fuzz_targets/{consensus_state_transition,relayer_escrow,vm_execute,bpqs_wots_reject,snapshot_deserialize}.rs` | WP-42 |
| BLG-188 | High | Uncheckable-proof runtime inversion and mempool/bridge/binding/repair gate bypasses | Open | `blg_188_runtime_gates_cannot_be_inverted` (owed) | `xtask/gates/src/gates/{uncheckable_proof,mempool_mev_free,bridge_dual_tree,binding_claims,repair_fires}.rs` | WP-42 |
| BLG-189 | Medium | Storage pricing/penalty/placement gate gaps and benchmark wire-size/gauge defects | Open | `blg_189_pricing_and_placement_gates_bind_values` (owed) | `xtask/gates/src/gates/{storage_priced,storage_penalties,threshold_rates,value_transfers,shard_placement}.rs`; `crates/bpqs/examples/bench_differential.rs` | WP-42 |
| BLG-192 | High | Gas estimation omits proportional transfer fees and folded governance proposal fees | Open | `blg_192_estimate_matches_execution_fee` (owed) | `src/rpc/server.rs::bud_estimate_gas`; `src/chain/blockchain.rs::required_fee` | WP-61 |
| BLG-193 | Medium | Remaining quality gates have syntax/comment bypasses and accept fabricated test output | Open | `blg_193_gate_output_cannot_be_fabricated` (owed) | remaining `xtask/gates/src/gates/` parser and test-output modules | WP-42 |
| BLG-200 | High | BLS zero/identity keys, identity-signature poisoning, false-green test and unbounded RLP depth | Open | `blg_200_bls_and_rlp_reject_invalid_inputs` (owed) | `src/crypto/primitives.rs::BlsKeypair`; `src/chain/finality.rs::FinalityAggregator`; RLP helpers | WP-61 |
| BLG-231 | Medium | BudZero documentation claims recursive aggregation that the code does not implement | Open | `blg_231_recursive_aggregation_claim_is_wired` (owed) | `budzero/bud-state/src/lib.rs`; `budzero/ARCHITECTURE.md` | WP-43 |
| BLG-241 | Low | BudZero comments state Poseidon width 8 while the code uses width 12 | Open | `blg_241_poseidon_width_documentation_matches_code` (owed) | `budzero/bud-isa/src/lib.rs`; `budzero/bud-vm/src/lib.rs` | WP-43 |
| BLG-254 | High | Legacy CI gates enforce the incorrect recipe-only model for B.U.D. 3.0 | Open | `blg_254_legacy_storage_model_gates_are_removed` (owed) | `xtask/gates/src/gates/{three_is_recipe_only,zero_storage_frozen,generated_content}.rs` | WP-07, WP-42 |
| BLG-255 | Low | Retired concept names remain spread across the tree | Open | `concept_names_gate_catches_injected_name` (owed) | `config/naming/concept_map.toml`; `.github/concept-names-baseline.txt` | WP-05, WP-43 and each owner package |
| BLG-257 | High | No mobile validator application exists; a CLI profile is not a phone client | Open | `mobile_core_builds_for_all_targets` (owed) | `src/main.rs::MobileNodeProfile`; no `mobile/` tree | WP-11, WP-36 |
| BLG-259 | Medium | Per-seat PQ certificate size and the mobile light-client trust model are unresolved | Open | `light_client_follows_handoff_chain` (owed) | `src/chain/finality.rs`; `src/consensus/qc.rs`; no `docs/LIGHT_CLIENT.md` | WP-12, WP-11 |
| BLG-265 | Medium | Mobile power/challenge model omits Android foreground-service timeout constraints | Open | `android_specialuse_service_runs_24h` (owed) | `src/network/mobile.rs`; no Android service | WP-36 |
| BULGU-011 | S3 (archive) | CORS wildcard reflects request headers | Closed (pre-fixed) | `cors_wildcard_does_not_reflect_headers` | `src/rpc/server.rs::cors_outcome`, `apply_cors_headers` | WP-61 |
| BULGU-012 | S3 (archive) | Serialization failure becomes a silent null response | Closed (pre-fixed) | `audit_lane_d_bulgu_012_serialization_failure_is_explicit` | `src/rpc/server.rs::to_json` | WP-61 |
| TEST-GAP-02 | Medium | Kani covers only a subset of the SECURITY.md targets | Open | `kani_covers_every_security_target` (owed) | `kani/src/lib.rs`; `docs/SECURITY.md` | WP-42 |
| TEST-GAP-04 | Low | Domain-producer binary has no direct test coverage | Open | `domain_producer_success_and_refusal` (owed) | `src/bin/budlum-domain-producer.rs`; `.github/workflows/ci.yml` | WP-42 |
| TEST-GAP-07 | Low | RPC endpoints lack direct success/refusal coverage | Open | `rpc_every_method_has_success_and_refusal_test` (owed) | `src/rpc/{api,server,tests}.rs` | WP-61 |
| DOC-GAP-01 | S1 (archive) | Testnet first-run SoftHSM setup is incomplete | Open | `ops_compose_testnet_profile_boots` (owed) | `config/testnet.toml`; `src/main.rs`; `ops/` | WP-41 |
| DOC-GAP-02 | S3 (archive) | Port documentation disagrees across CLI, profiles, Compose and systemd | Open | `docs_port_table_matches_profiles` (owed) | `src/cli/`; `config/`; `ops/{docker-compose.yml,budlum-core.service}` | WP-43 |
| DOC-GAP-03 | S3 (archive) | README/CONTRIBUTING/operations reference broken scripts | Open | `docs_links_resolve` (owed) | `README.md`; `docs/CONTRIBUTING.md`; `ops/scripts/` | WP-43 |
| DOC-GAP-04 | S3 (archive) | systemd unit names the wrong repository and omits HSM setup | Open | `ops_systemd_unit_matches_runtime_requirements` (owed) | `ops/budlum-core.service`: `Documentation`, HSM environment | WP-41 |
| DOC-GAP-05 | S3 (archive) | Validator/testnet/operations guides contain incomplete sentences | Open | `docs_have_no_incomplete_steps` (owed) | `docs/{VALIDATOR_GUIDE,TESTNET,OPERATIONS}.md` | WP-43 |
| DOC-GAP-06 | S3 (archive) | Testnet and validator resource tables have no measured source | Open | `rehearsal_results_back_resource_tables` (owed) | `docs/{TESTNET,VALIDATOR_GUIDE}.md`; no rehearsal results | WP-46 |
| DOC-GAP-07 | S3 (archive) | The indexer lacks an explicit binary registration | Closed (pre-fixed) | `audit_lane_d_doc_gap_07_indexer_is_registered_and_tested` | `Cargo.toml`: `[[bin]] budlum-indexer`; `.github/workflows/ci.yml` | WP-42 |
| DOC-GAP-08 | S3 (archive) | CLI fee default disagrees with the network base fee | Open | `cli_fee_default_reads_network_base_fee` (owed) | `src/bin/bud.rs` fee defaults; `src/rpc/server.rs::bud_gasPrice` | WP-39 |
| DOC-GAP-09 | S3 (archive) | Archived ledger detail requires live verification | Open | `archived_detail_has_live_evidence` (owed) | directive section 14.6 archive rows | WP-43 |

## Pre-fixed evidence in this delivery

The new `src/tests/audit_lane_d.rs` locks BLG-004, BLG-022, the testnet development-credit leg of BLG-030, BLG-099 correctness, BULGU-012, BLG-011 and DOC-GAP-07. Existing permanent tests lock BLG-009, BLG-012 and BULGU-011. BLG-030 remains Open because the complete RPC surface and faucet-service requirements still need their closing-package audit. BLG-099 timing remains a CI measurement claim; the new unit test asserts correctness only.

No Rust compiler is installed in the workspace. The new Rust files pass a tree-sitter Rust parse and `git diff --check`; compilation, formatting, clippy and execution are assigned to CI. No local result is presented as a Rust test pass.

## Archive-family reconciliation

All 44 archive identities in directive section 14.6 were read. Lane D owns the 14 rows below; the other 30 belong to lanes A/B/C and must be reconciled in the final WP-04 delivery after PR #12 merges. No archive identity is reused as a BLG identity.

| Archive identity | Live location | Disposition |
|---|---|---|
| BULGU-011 | `src/rpc/server.rs::cors_outcome` | Distinct archive row; pre-fixed behavior locked by `cors_wildcard_does_not_reflect_headers`. |
| BULGU-012 | `src/rpc/server.rs::to_json` | Distinct archive row; explicit error locked by the new lane D test. |
| TEST-GAP-02 | `kani/src/lib.rs` | Open; coverage expansion belongs to WP-42. |
| TEST-GAP-04 | `src/bin/budlum-domain-producer.rs` | Open; CI lists the binary but no direct test command was found. |
| TEST-GAP-07 | `src/rpc/tests.rs` | Open; existing endpoint tests do not establish success/refusal coverage for every method. |
| DOC-GAP-01 | `config/testnet.toml`, `ops/` | Open; first-run HSM setup belongs to WP-41. |
| DOC-GAP-02 | CLI/config/Compose/systemd port settings | Open; document only after profile reconciliation. |
| DOC-GAP-03 | README, contributing guide, `ops/scripts/` | Open; link/script reconciliation belongs to WP-43. |
| DOC-GAP-04 | `ops/budlum-core.service` | Open; `Documentation=https://github.com/rade/budlum-core` remains stale and HSM setup is absent. BLG-011 credential/dump legs are fixed independently. |
| DOC-GAP-05 | validator/testnet/operations guides | Open; editorial verification belongs to WP-43. |
| DOC-GAP-06 | testnet/validator resource tables | Open; no rehearsal measurement source exists. WP-46 owns the measurements. |
| DOC-GAP-07 | `Cargo.toml`, `.github/workflows/ci.yml` | Pre-fixed; explicit `[[bin]]` and executed binary tests are locked here. |
| DOC-GAP-08 | `src/bin/bud.rs` | Open; CLI default must read the network fee base. |
| DOC-GAP-09 | directive archive table | Open until the historical detail and live evidence are reconciled; do not invent missing history. |

## Blockers and remaining WP-04 work

1. GitHub Issues is disabled. `gh repo view` reports `hasIssuesEnabled=false`; `gh repo edit --enable-issues` returned HTTP 403 (`Resource not accessible by integration`). The repository owner must enable Issues or grant the integration the required repository administration permission. The 64 package records and CLAIM/in-progress issue protocol cannot be completed until then. Labels `work-package`, `in-progress` and `blocked` were created successfully.
2. The directive requires reproduction branches named `audit/repro-d`; this session is constrained to the fixed Arena branch. No other branch is created. Open-row reproductions remain owed to their closing packages, and this delivery does not claim package completion.
3. The final WP-04 PR must wait for WP-01/02/03 to merge, then reconcile all four lanes into `LIVE_STATUS.md`, complete the English `FINDINGS.md`, update `PACKAGES.md`, and verify all 64 GitHub records. PR #12 is open, not merged.
4. The inherited red CI must be fixed by the packages owning the failing paths. This delivery neither raises a baseline nor weakens a gate. See `ci-baseline.md` for the observed failures and ownership.
5. The binding directive says it must stay outside the repository, but the supplied base already tracks `THE_BUDLUM_DIREKTIF.md`. Open PRs #8/#11 carry its removal. This package does not delete a file locked by those streams and adds no copy.

## Locks

- `docs/audit/live/lane-d.md`
- `docs/audit/live/ci-baseline.md` (LOCK-ADD; no open PR owns it)
- `src/tests/audit_lane_d.rs`
- `src/tests/mod.rs` (append only)
- `docs/work/PACKAGES.md` (shared append-only; reconcile with PR #13)

No production implementation path is changed. The tests are compiled by the existing `cargo test --lib --verbose` step; no workflow edit or new dependency is needed.
