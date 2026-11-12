# Audit Plan

This checklist lists every directory, module, and file to inspect. Mark each item `[x]` when done.

## Pass 1 Status: In Progress

---

## Root Files

- [ ] `.cargo/audit.toml`
- [ ] `.coderabbit.yaml`
- [ ] `.gitleaks.toml`
- [ ] `.gitleaksignore`
- [ ] `.gitignore`
- [ ] `.quality/deny-fuzz.toml`
- [ ] `.quality/deny-mobile-ffi.toml`
- [ ] `.quality/deny.toml`
- [ ] `.quality/grype.yaml`
- [ ] `.quality/typos.toml`
- [ ] `Cargo.lock`
- [ ] `Cargo.toml`
- [ ] `LICENSE.md`
- [ ] `README.md`
- [ ] `README.tr.md`
- [ ] `THE_BUDLUM_DIREKTIF.md`
- [ ] `build.rs`
- [ ] `buf.yaml`
- [ ] `clippy.toml`
- [ ] `devcontainer.json`
- [ ] `fable.py`
- [ ] `flake.lock`
- [ ] `flake.nix`
- [ ] `modelleri_dene.py`
- [ ] `osv-scanner.toml`
- [ ] `rust-toolchain.toml`

## `.github/`

- [ ] `.github/CODEOWNERS`
- [ ] `.github/PULL_REQUEST_TEMPLATE.md`
- [ ] `.github/actionlint.yaml`
- [ ] `.github/bud3-forbidden-phrases-baseline.txt`
- [ ] `.github/cargo-vet-baseline.txt`
- [ ] `.github/clippy-extra-baseline.txt`
- [ ] `.github/codeql/codeql-config.yml`
- [ ] `.github/concept-names-baseline.txt`
- [ ] `.github/coverage-badge.json`
- [ ] `.github/coverage-baseline.txt`
- [ ] `.github/dead-pub-api-baseline.txt`
- [ ] `.github/dependabot.yml`
- [ ] `.github/gate-panics-baseline.txt`
- [ ] `.github/git-dep-audit.toml`
- [ ] `.github/idle-code-baseline.txt`
- [ ] `.github/no-legacy-storage-baseline.txt`
- [ ] `.github/readme-placeholders-baseline.txt`
- [ ] `.github/rpc-method-coverage-baseline.txt`
- [ ] `.github/semver-exceptions.txt`
- [ ] `.github/tsan-suppressions-baseline.txt`
- [ ] `.github/tsan-suppressions.txt`
- [ ] `.github/turkish-baseline.txt`
- [ ] `.github/udeps-baseline.txt`
- [ ] `.github/unwired-guards-baseline.txt`
- [ ] `.github/unwritten-metrics-baseline.txt`
- [ ] ISSUE_TEMPLATE files (5 files)

### `.github/workflows/` (35 files)

- [ ] `air-mutants-nightly.yml`
- [ ] `audit-guard.yml`
- [ ] `benchmark.yml`
- [ ] `cargo-vet.yml`
- [ ] `ci.yml`
- [ ] `code-scanning-sarif.yml`
- [ ] `conformance-nightly.yml`
- [ ] `contract-lockfiles.yml`
- [ ] `dependency-review.yml`
- [ ] `dependency-submission.yml`
- [ ] `determinism.yml`
- [ ] `diverse-double-compiling.yml`
- [ ] `docker-smoke.yml`
- [ ] `extra-tooling.yml`
- [ ] `fmt-diff-surface.yml`
- [ ] `fuzz-nightly.yml`
- [ ] `grype.yml`
- [ ] `kani-slow.yml`
- [ ] `miri-storage-weekly.yml`
- [ ] `miri.yml`
- [ ] `mobile-ios.yml`
- [ ] `mobile.yml`
- [ ] `osv-scanner.yml`
- [ ] `pkcs11-pq.yml`
- [ ] `provenance.yml`
- [ ] `release-signature.yml`
- [ ] `rust.yml`
- [ ] `security-audit.yml`
- [ ] `security-hardening.yml`
- [ ] `semgrep.yml`
- [ ] `semver.yml`
- [ ] `solidity.yml`
- [ ] `supply-chain-extra.yml`
- [ ] `testnet-rehearsal.yml`
- [ ] `typos.yml`
- [ ] `watchdog.yml`

## `bud/` (BUD format library)

- [ ] `bud/Cargo.toml`
- [ ] `bud/FORMAT-V2.md`
- [ ] `bud/README.md`
- [ ] `bud/fuzz/Cargo.toml`
- [ ] `bud/fuzz/fuzz_bud_format.rs`
- [ ] `bud/kani/kani_bud_format.rs`
- [ ] `bud/mime.types.bud.json`
- [ ] `bud/src/bin/bud.rs`
- [ ] `bud/src/bin/measure_ratios.rs`
- [ ] `bud/src/bud3_live_test.rs`
- [ ] `bud/src/bud_fixed_point.rs`
- [ ] `bud/src/bud_format.rs`
- [ ] `bud/src/bud_format_autozstd.rs`
- [ ] `bud/src/bud_format_av2.rs`
- [ ] `bud/src/bud_format_bft.rs`
- [ ] `bud/src/bud_format_block.rs`
- [ ] `bud/src/bud_format_bloom.rs`
- [ ] `bud/src/bud_format_catalog.rs`
- [ ] `bud/src/bud_format_checkpoint.rs`
- [ ] `bud/src/bud_format_columnar.rs`
- [ ] `bud/src/bud_format_container.rs`
- [ ] `bud/src/bud_format_crop.rs`
- [ ] `bud/src/bud_format_culling.rs`
- [ ] `bud/src/bud_format_das.rs`
- [ ] `bud/src/bud_format_dbdelta.rs`
- [ ] `bud/src/bud_format_dedup.rs`
- [ ] `bud/src/bud_format_dictionary.rs`
- [ ] `bud/src/bud_format_economics.rs`
- [ ] `bud/src/bud_format_edge.rs`
- [ ] `bud/src/bud_format_edition.rs`
- [ ] `bud/src/bud_format_encpact.rs`
- [ ] `bud/src/bud_format_engine.rs`
- [ ] `bud/src/bud_format_erasure.rs`
- [ ] `bud/src/bud_format_exepdf.rs`
- [ ] `bud/src/bud_format_fastcdc.rs`
- [ ] `bud/src/bud_format_fidelitygate.rs`
- [ ] `bud/src/bud_format_fountain.rs`
- [ ] `bud/src/bud_format_genomic.rs`
- [ ] `bud/src/bud_format_governance.rs`
- [ ] `bud/src/bud_format_guardian.rs`
- [ ] `bud/src/bud_format_hardening.rs`
- [ ] `bud/src/bud_format_huffman.rs`
- [ ] `bud/src/bud_format_hw.rs`
- [ ] `bud/src/bud_format_integration.rs`
- [ ] `bud/src/bud_format_jpegre.rs`
- [ ] `bud/src/bud_format_lazyrepair.rs`
- [ ] `bud/src/bud_format_logfield.rs`
- [ ] `bud/src/bud_format_lowrank.rs`
- [ ] `bud/src/bud_format_lrc.rs`
- [ ] `bud/src/bud_format_markdown.rs`
- [ ] `bud/src/bud_format_matrix.rs`
- [ ] `bud/src/bud_format_media.rs`
- [ ] `bud/src/bud_format_model.rs`
- [ ] `bud/src/bud_format_msr.rs`
- [ ] `bud/src/bud_format_multifile.rs`
- [ ] `bud/src/bud_format_nvc.rs`
- [ ] `bud/src/bud_format_office.rs`
- [ ] `bud/src/bud_format_optical.rs`
- [ ] `bud/src/bud_format_optical_transfer.rs`
- [ ] `bud/src/bud_format_pact.rs`
- [ ] `bud/src/bud_format_pcap.rs`
- [ ] `bud/src/bud_format_pgm.rs`
- [ ] `bud/src/bud_format_pipe.rs`
- [ ] `bud/src/bud_format_pointcloud.rs`
- [ ] `bud/src/bud_format_por.rs`
- [ ] `bud/src/bud_format_prodcost.rs`
- [ ] `bud/src/bud_format_production.rs`
- [ ] `bud/src/bud_format_qos.rs`
- [ ] `bud/src/bud_format_qrmatrix.rs`
- [ ] `bud/src/bud_format_qrvideo.rs`
- [ ] `bud/src/bud_format_r3fix.rs`
- [ ] `bud/src/bud_format_ratioconsensus.rs`
- [ ] `bud/src/bud_format_real.rs`
- [ ] `bud/src/bud_format_realcorpus.rs`
- [ ] `bud/src/bud_format_recipe.rs`
- [ ] `bud/src/bud_format_recipe_record.rs`
- [ ] `bud/src/bud_format_regeneration.rs`
- [ ] `bud/src/bud_format_registry.rs`
- [ ] `bud/src/bud_format_repairband.rs`
- [ ] `bud/src/bud_format_revolutionary.rs`
- [ ] `bud/src/bud_format_scrub.rs`
- [ ] `bud/src/bud_format_secure_db.rs`
- [ ] `bud/src/bud_format_securededup.rs`
- [ ] `bud/src/bud_format_segment.rs`
- [ ] `bud/src/bud_format_serviceclass.rs`
- [ ] `bud/src/bud_format_shamir.rs`
- [ ] `bud/src/bud_format_social.rs`
- [ ] `bud/src/bud_format_social2.rs`
- [ ] `bud/src/bud_format_spec.rs`
- [ ] `bud/src/bud_format_telemetry.rs`
- [ ] `bud/src/bud_format_timeseries.rs`
- [ ] `bud/src/bud_format_tiny.rs`
- [ ] `bud/src/bud_format_transpile.rs`
- [ ] `bud/src/bud_format_tricore.rs`
- [ ] `bud/src/bud_format_ultra.rs`
- [ ] `bud/src/bud_format_ux.rs`
- [ ] `bud/src/bud_format_video.rs`
- [ ] `bud/src/bud_format_videopipe.rs`
- [ ] `bud/src/bud_format_view.rs`
- [ ] `bud/src/bud_format_wal.rs`
- [ ] `bud/src/bud_format_wire.rs`
- [ ] `bud/src/bud_format_zkbridge.rs`
- [ ] `bud/src/bud_format_zkproof.rs`
- [ ] `bud/src/churn.rs`
- [ ] `bud/src/cli.rs`
- [ ] `bud/src/fidelity.rs`
- [ ] `bud/src/gates.rs`
- [ ] `bud/src/lib.rs`
- [ ] `bud/src/price.rs`
- [ ] `bud/src/provider.rs`
- [ ] `bud/src/quantum.rs`
- [ ] `bud/src/quantum_chain.rs`
- [ ] `bud/src/ratio.rs`
- [ ] `bud/tests/test_bud2.rs`
- [ ] `bud/tests/test_integration.rs`

## `budzero/` (ZK proof system)

- [ ] `budzero/Cargo.toml`
- [ ] `budzero/ARCHITECTURE.md`
- [ ] `budzero/README.md`
- [ ] `budzero/deny.toml`
- [ ] `budzero/rust-toolchain.toml`
- [ ] `budzero/flake.nix`
- [ ] `budzero/bud-cli/Cargo.toml`
- [ ] `budzero/bud-cli/src/main.rs`
- [ ] `budzero/bud-cli/tests/toolchain_end_to_end.rs`
- [ ] `budzero/bud-compiler/Cargo.toml`
- [ ] `budzero/bud-compiler/src/ast.rs`
- [ ] `budzero/bud-compiler/src/codegen.rs`
- [ ] `budzero/bud-compiler/src/ir/display.rs`
- [ ] `budzero/bud-compiler/src/ir/lower.rs`
- [ ] `budzero/bud-compiler/src/ir/mod.rs`
- [ ] `budzero/bud-compiler/src/ir/verify.rs`
- [ ] `budzero/bud-compiler/src/lexer.rs`
- [ ] `budzero/bud-compiler/src/lib.rs`
- [ ] `budzero/bud-compiler/src/parser.rs`
- [ ] `budzero/bud-compiler/src/sema.rs`
- [ ] `budzero/bud-compiler/tests/regression.rs`
- [ ] `budzero/bud-isa/Cargo.toml`
- [ ] `budzero/bud-isa/src/lib.rs`
- [ ] `budzero/bud-node/Cargo.toml`
- [ ] `budzero/bud-node/src/bitswap.rs`
- [ ] `budzero/bud-node/src/discovery.rs`
- [ ] `budzero/bud-node/src/lib.rs`
- [ ] `budzero/bud-node/src/sharding.rs`
- [ ] `budzero/bud-node/src/store.rs`
- [ ] `budzero/bud-proof/Cargo.toml`
- [ ] `budzero/bud-proof/src/adapter.rs`
- [ ] `budzero/bud-proof/src/alarm_log.rs`
- [ ] `budzero/bud-proof/src/bud_stark/` (all files)
- [ ] `budzero/bud-proof/src/canonical_boot.rs`
- [ ] `budzero/bud-proof/src/canonical_recovery.rs`
- [ ] `budzero/bud-proof/src/canonical_set.rs`
- [ ] `budzero/bud-proof/src/lib.rs`
- [ ] `budzero/bud-proof/src/plonky3_air.rs`
- [ ] `budzero/bud-proof/src/plonky3_prover.rs`
- [ ] `budzero/bud-proof/src/quarantine.rs`
- [ ] `budzero/bud-proof/src/relayer.rs`
- [ ] `budzero/bud-proof/src/trace_layout_tests.rs`
- [ ] `budzero/bud-proof/src/transfer_verdict.rs`
- [ ] `budzero/bud-proof/tests/` (all files)
- [ ] `budzero/bud-pul/Cargo.toml`
- [ ] `budzero/bud-pul/src/` (all files)
- [ ] `budzero/bud-state/Cargo.toml`
- [ ] `budzero/bud-state/src/lib.rs`
- [ ] `budzero/bud-state/src/note.rs`
- [ ] `budzero/bud-vm/Cargo.toml`
- [ ] `budzero/bud-vm/src/lib.rs`
- [ ] `budzero/bud-vm/src/private_transfer.rs`
- [ ] `budzero/bud-vm/src/syscall_context.rs`
- [ ] `budzero/bud-vm/tests/trace_fixtures.rs`
- [ ] `budzero/verifier-registry/` (all files)
- [ ] `budzero/docs/` (all doc files)
- [ ] `budzero/example.bud`, `example2.bud`, `control_flow.bud`, `test_prover.bud`, `example_loop.bud`

## `config/`

- [ ] `config/archive.toml`
- [ ] `config/ceremony-rehearsal-genesis.json`
- [ ] `config/ceremony-rehearsal-template.json`
- [ ] `config/devnet-genesis.json`
- [ ] `config/devnet.toml`
- [ ] `config/fixtures/real-chain.json`
- [ ] `config/mainnet-genesis.json`
- [ ] `config/mainnet.toml`
- [ ] `config/naming/concept_map.toml`
- [ ] `config/personas/` (all files)
- [ ] `config/rehearsal-genesis.json`
- [ ] `config/testnet-ethereum.toml`
- [ ] `config/testnet-genesis.json`
- [ ] `config/testnet.toml`

## `contracts/`

- [ ] `contracts/README.md`
- [ ] `contracts/foundry.toml`
- [ ] `contracts/external-domain/BudlumFinalityVerifier.sol`
- [ ] `contracts/graft/BudlumPaymentReceiver.sol`
- [ ] `contracts/graft/BudlumRemoteAccountFactory.sol`
- [ ] `contracts/graft/GraftExecutor.sol`
- [ ] `contracts/lib/BudlumBls.sol`
- [ ] `contracts/lib/BudlumHash.sol`
- [ ] `contracts/lib/BudlumOutbound.sol`
- [ ] `contracts/mikoriza/BudlumRegistrar.sol`
- [ ] `contracts/mikoriza/BudlumSlasher.sol`
- [ ] `contracts/mikoriza/IAllocationManager.sol`
- [ ] `contracts/object-layer/BudlumFinalityVerifier.sol`
- [ ] `contracts/object-layer/BudlumSettlementEndpoint.sol`
- [ ] `contracts/object-layer/README.md`
- [ ] `contracts/object-layer/cosmwasm/` (all files)
- [ ] `contracts/object-layer/move/` (all files)
- [ ] `contracts/object-layer/svm/` (all files)
- [ ] `contracts/test/` (all files)
- [ ] `contracts/uel/BudlumUniversal.sol`

## `crates/`

- [ ] `crates/ai-inference/` (all files)
- [ ] `crates/bpqs/` (all files)
- [ ] `crates/bud-codec/` (all files)
- [ ] `crates/budlum-explorer/` (all files)
- [ ] `crates/budlum-types/` (all files)
- [ ] `crates/note-packing/` (all files)
- [ ] `crates/uel-abi/` (all files)
- [ ] `crates/wallet-core/` (all files)

## `docs/`

- [ ] `docs/AI_VERIFICATION_LAYER.md`
- [ ] `docs/AI_VERIFICATION_STATUS.md`
- [ ] `docs/APP_BINDING.md`
- [ ] `docs/APTOS_RPC.md`
- [ ] `docs/ARCHITECTURE.md`
- [ ] `docs/BUDZERO_RECURSION_AND_BATCH_PROOFS.md`
- [ ] `docs/BUD_CODEC_CONTEXT_AND_DEDUP.md`
- [ ] `docs/CODE_OF_CONDUCT.md`
- [ ] `docs/CONTRIBUTING.md`
- [ ] `docs/DELEGATION.md`
- [ ] `docs/DOMAIN_ADAPTER_GUIDE.md`
- [ ] `docs/EVM_MIGRATION.md`
- [ ] `docs/EXECUTION.md`
- [ ] `docs/EXECUTION_LIMITS.md`
- [ ] `docs/EXECUTION_PARALLEL_SCHEDULER.md`
- [ ] `docs/FSF_ADAPTATION_PLAN.md`
- [ ] `docs/FSF_PROJECT_FIT.md`
- [ ] `docs/LIGHT_CLIENT.md`
- [ ] `docs/MAINNET_READINESS.md`
- [ ] `docs/MOBILE_DEVICE_MATRIX.md`
- [ ] `docs/MOBILE_STORE_DISTRIBUTION.md`
- [ ] `docs/NAMING.md`
- [ ] `docs/NOTICE`
- [ ] `docs/OBJECT_LAYER.md`
- [ ] `docs/OPERATIONS.md`
- [ ] `docs/PARALLEL_EXECUTION.md`
- [ ] `docs/PQ_ANCHOR_RESEARCH.md`
- [ ] `docs/RPC_COMPAT.md`
- [ ] `docs/SECURITY.md`
- [ ] `docs/SETTLEMENT.md`
- [ ] `docs/SHARED_SECURITY.md`
- [ ] `docs/SOLANA_RPC.md`
- [ ] `docs/SPECIFICATION.md`
- [ ] `docs/STORAGE.md`
- [ ] `docs/STORAGE_ENCRYPTION.md`
- [ ] `docs/STORAGE_MEASUREMENTS.md`
- [ ] `docs/STORAGE_REDUCTION.md`
- [ ] `docs/TESTNET.md`
- [ ] `docs/VALIDATOR_GUIDE.md`
- [ ] `docs/VALIDATOR_ROLES.md`
- [ ] `docs/audit/` (all files)
- [ ] `docs/budzero/HASH_SUITES.md`
- [ ] `docs/explorer-api.openapi.yaml`
- [ ] `docs/pq/` (all files)
- [ ] `docs/rehearsal/` (all files)

## `execution/`

- [ ] `execution/budzero/` (all files)
- [ ] `execution/cairo/` (all files)
- [ ] `execution/conformance/` (all files)
- [ ] `execution/evm/` (all files)
- [ ] `execution/mock/` (all files)
- [ ] `execution/move-sui/` (all files)
- [ ] `execution/move/` (all files)
- [ ] `execution/polkavm/` (all files)
- [ ] `execution/svm/` (all files)
- [ ] `execution/wasm/` (all files)

## `fuzz/`

- [ ] `fuzz/Cargo.toml`
- [ ] `fuzz/README.md`
- [ ] `fuzz/fuzz_targets/` (all 20 target files)

## `kani/`

- [ ] `kani/Cargo.toml`
- [ ] `kani/src/lib.rs`

## `mobile/`

- [ ] `mobile/android/` (all files)
- [ ] `mobile/core/` (all files)
- [ ] `mobile/ffi/` (all files)
- [ ] `mobile/ios/` (all files)
- [ ] `mobile/tools/log-analyzer/` (all files)

## `ops/`

- [ ] `ops/Dockerfile`
- [ ] `ops/Caddyfile.testnet`
- [ ] `ops/TESTNET_FIRST_RUN.md`
- [ ] `ops/BACKUP_AND_RESTORE.md`
- [ ] `ops/budlum-core.service`
- [ ] `ops/docker-compose.ci.yml`
- [ ] `ops/docker-compose.yml`
- [ ] `ops/hsm.env.example`
- [ ] `ops/monitoring/` (all files)
- [ ] `ops/prometheus.yml`
- [ ] `ops/rehearsal/` (all files)
- [ ] `ops/softhsm2.conf`
- [ ] `ops/testnet-local-validator.py`
- [ ] `ops/testnet-local.yml`

## `proto/`

- [ ] `proto/budlum/execution/v1/ee.proto`
- [ ] `proto/budlum/network/protocol.proto`
- [ ] `proto/budlum/storage/v3/bud3.proto`

## `src/` (main Rust codebase)

- [ ] `src/lib.rs`
- [ ] `src/main.rs`
- [ ] `src/error.rs`
- [ ] `src/project_manifest.rs`
- [ ] `src/account_abstraction/` (all files)
- [ ] `src/ai/` (all files)
- [ ] `src/ai_inference/` (all files)
- [ ] `src/app_registry/mod.rs`
- [ ] `src/bin/` (all files)
- [ ] `src/bns/` (all files)
- [ ] `src/budlumxyz/` (all files)
- [ ] `src/chain/` (all files)
- [ ] `src/cli/` (all files)
- [ ] `src/consensus/` (all files)
- [ ] `src/core/` (all files)
- [ ] `src/cross_domain/` (all files)
- [ ] `src/crypto/` (all files)
- [ ] `src/data_market/mod.rs`
- [ ] `src/deed/` (all files)
- [ ] `src/domain/` (all files)
- [ ] `src/education_domain/mod.rs`
- [ ] `src/execution/` (all files)
- [ ] `src/explorer_evidence/mod.rs`
- [ ] `src/light_client/mod.rs`
- [ ] `src/mempool/` (all files)
- [ ] `src/network/` (all files)
- [ ] `src/object_layer/` (all files)
- [ ] `src/persistence/` (all files)
- [ ] `src/pollen/` (all files)
- [ ] `src/privacy/` (all files)
- [ ] `src/prover/mod.rs`
- [ ] `src/registry/` (all files)
- [ ] `src/relayer/` (all files)
- [ ] `src/rpc/` (all files)
- [ ] `src/sdk/mod.rs`
- [ ] `src/security/` (all files)
- [ ] `src/settlement/` (all files)
- [ ] `src/sharding/mod.rs`
- [ ] `src/socialfi/` (all files)
- [ ] `src/storage/` (all files)
- [ ] `src/tests/` (all test files)
- [ ] `src/tokenomics/` (all files)
- [ ] `src/validators/` (all files)
- [ ] `src/wallet/` (all files)

## `supply-chain/`

- [ ] `supply-chain/audits.toml`
- [ ] `supply-chain/config.toml`
- [ ] `supply-chain/imports.lock`

## `tests/`

- [ ] `tests/churn/main.rs`
- [ ] `tests/rehearsal/main.rs`
- [ ] `tests/vectors/` (all files)

## `xtask/`

- [ ] `xtask/gates/Cargo.toml`
- [ ] `xtask/gates/src/main.rs`
- [ ] `xtask/gates/src/sha256.rs`
- [ ] `xtask/gates/src/gates/` (all ~165 gate files)
- [ ] `xtask/gates/tests/repository_locks.rs`
- [ ] `xtask/release/` (all files)
- [ ] `xtask/tools/` (all files)

## `benches/`

- [ ] `benches/domain_throughput.rs`
- [ ] `benches/micro/` (all files)
- [ ] `benches/single_node/internal_pipeline_tps.rs`

## `examples/`

- [ ] `examples/pq_measure.rs`
- [ ] `examples/print_genesis_hash.rs`

## `release/`

- [ ] `release/README.md`
- [ ] `release/RELEASE_PUBKEY.hex`
