# Audit Plan

This plan lists every directory, module, and file area to inspect. Each area
is marked `[ ]` when pending and `[x]` when done.

## Note on the directive file

The task references `THE_BUDLUM_DIREKTIF.md`. This file does not exist in the
repository. The audit uses the following files as the directive for testnet
requirements:

- `docs/SPECIFICATION.md` (core protocol specification)
- `docs/TESTNET.md` (testnet operator notes and requirements)
- `docs/MAINNET_READINESS.md` (readiness checklist)

## Areas to inspect

### 1. Root files and build configuration
- [ ] `Cargo.toml` (workspace manifest)
- [ ] `Cargo.lock`
- [ ] `build.rs`
- [ ] `clippy.toml`
- [ ] `buf.yaml`
- [ ] `flake.nix`, `flake.lock`
- [ ] `osv-scanner.toml`
- [ ] `rust-toolchain.toml`
- [ ] `README.md`, `README.tr.md`
- [ ] `LICENSE.md`
- [ ] `.gitignore`, `.gitleaks.toml`, `.gitleaksignore`
- [ ] `.coderabbit.yaml`

### 2. Cargo and supply chain
- [ ] `.cargo/audit.toml`
- [ ] `supply-chain/` (audits.toml, config.toml, imports.lock)

### 3. Quality and linting
- [ ] `.quality/` (deny.toml, deny-fuzz.toml, deny-mobile-ffi.toml, grype.yaml, typos.toml)

### 4. GitHub configuration
- [ ] `.github/CODEOWNERS`
- [ ] `.github/PULL_REQUEST_TEMPLATE.md`
- [ ] `.github/ISSUE_TEMPLATE/` (all templates)
- [ ] `.github/dependabot.yml`
- [ ] `.github/actionlint.yaml`
- [ ] `.github/codeql/codeql-config.yml`
- [ ] `.github/git-dep-audit.toml`
- [ ] `.github/semver-exceptions.txt`
- [ ] `.github/tsan-suppressions.txt`, `tsan-suppressions-baseline.txt`
- [ ] All baseline files in `.github/`

### 5. CI workflows
- [ ] `.github/workflows/ci.yml`
- [ ] `.github/workflows/rust.yml`
- [ ] `.github/workflows/security-audit.yml`
- [ ] `.github/workflows/security-hardening.yml`
- [ ] `.github/workflows/semgrep.yml`
- [ ] `.github/workflows/grype.yml`
- [ ] `.github/workflows/osv-scanner.yml`
- [ ] `.github/workflows/code-scanning-sarif.yml`
- [ ] `.github/workflows/cargo-vet.yml`
- [ ] `.github/workflows/solidity.yml`
- [ ] All remaining workflows

### 6. Configuration files
- [ ] `config/testnet.toml`
- [ ] `config/mainnet.toml`
- [ ] `config/devnet.toml`
- [ ] `config/archive.toml`
- [ ] `config/testnet-ethereum.toml`
- [ ] `config/testnet-genesis.json`
- [ ] `config/mainnet-genesis.json`
- [ ] `config/devnet-genesis.json`
- [ ] `config/ceremony-rehearsal-genesis.json`
- [ ] `config/ceremony-rehearsal-template.json`
- [ ] `config/rehearsal-genesis.json`
- [ ] `config/fixtures/real-chain.json`
- [ ] `config/naming/concept_map.toml`
- [ ] `config/personas/` (all files)

### 7. Protobuf definitions
- [ ] `proto/budlum/execution/v1/ee.proto`
- [ ] `proto/budlum/network/protocol.proto`
- [ ] `proto/budlum/storage/v3/bud3.proto`

### 8. Core source (`src/`)
- [ ] `src/main.rs`
- [ ] `src/lib.rs`
- [ ] `src/error.rs`
- [ ] `src/project_manifest.rs`

#### 8a. Core types
- [ ] `src/core/` (account, address, block, chain_config, constitution, encoding, governance, hash, map_keys, metrics, money, transaction, tx_bundle, bounded_read)

#### 8b. Consensus
- [ ] `src/consensus/` (mod, pos, poa, pow, filiz, handoff, qc, registration, block_signature, budget, merkle_tree, split_resolver)

#### 8c. Chain
- [ ] `src/chain/` (blockchain, chain_actor, fee_market, finality, genesis, genesis_ceremony, snapshot, state_recovery/, storage_economics_tests, storage_maintenance)

#### 8d. Crypto
- [ ] `src/crypto/` (all files: address_v3, composite, ctx, domain_tags, facts, hpke, key_derivation, key_file, key_set_preimage, mainnet_policy, pkcs11, pq, primitives, scheme, secp256k1_recover, signer, tls)

#### 8e. Network
- [ ] `src/network/` (node, protocol, peer_manager, peer_auth, gossip_dedup, bounded_bincode, deferred_votes, fork_rewalk, mobile, privacy, proto_conversions, sync_codec, witness_protocol)

#### 8f. RPC
- [ ] `src/rpc/` (server, api, mod, tests, client_auth, method_class, subscriptions, tls, compat/)

#### 8g. Domain and settlement
- [ ] `src/domain/` (all files)
- [ ] `src/settlement/` (all files)
- [ ] `src/cross_domain/` (all files including evm/ and external/)

#### 8h. Execution
- [ ] `src/execution/` (executor, ee, mod, proof_verifier, zkvm, coordinator/, budzero_ee/)

#### 8i. Storage
- [ ] `src/storage/` (all files including bud3/)

#### 8j. Other modules
- [ ] `src/mempool/`
- [ ] `src/persistence/`
- [ ] `src/validators/`
- [ ] `src/tokenomics/`
- [ ] `src/registry/`
- [ ] `src/relayer/`
- [ ] `src/wallet/`
- [ ] `src/sdk/`
- [ ] `src/security/`
- [ ] `src/prover/`
- [ ] `src/light_client/`
- [ ] `src/sharding/`
- [ ] `src/object_layer/`
- [ ] `src/privacy/`
- [ ] `src/data_market/`
- [ ] `src/deed/`
- [ ] `src/education_domain/`
- [ ] `src/pollen/`
- [ ] `src/socialfi/`
- [ ] `src/bns/`
- [ ] `src/budlumxyz/`
- [ ] `src/ai/`
- [ ] `src/ai_inference/`
- [ ] `src/app_registry/`
- [ ] `src/account_abstraction/`
- [ ] `src/explorer_evidence/`

#### 8k. Binaries
- [ ] `src/bin/budlum-faucet.rs`
- [ ] `src/bin/budlum-indexer.rs`
- [ ] `src/bin/budlum-light.rs`
- [ ] `src/bin/budlum-relayer.rs`
- [ ] `src/bin/budlum-domain-producer.rs`
- [ ] `src/bin/budlum-domain-reporter.rs`
- [ ] `src/bin/bud.rs`
- [ ] `src/bin/bud-migrate.rs`
- [ ] `src/bin/support/`

#### 8l. Tests
- [ ] `src/tests/` (all test files)

### 9. Bud format crate (`bud/`)
- [ ] `bud/Cargo.toml`, `bud/Cargo.lock`
- [ ] `bud/src/lib.rs`
- [ ] `bud/src/bin/bud.rs`
- [ ] `bud/src/bud_format.rs` and all `bud_format_*.rs` files
- [ ] `bud/src/` other files (churn, cli, fidelity, gates, price, provider, quantum, quantum_chain, ratio)
- [ ] `bud/fuzz/` and `bud/kani/`
- [ ] `bud/tests/`

### 10. BudZero (`budzero/`)
- [ ] `budzero/Cargo.toml`, `budzero/Cargo.lock`
- [ ] `budzero/bud-compiler/` (ast, codegen, ir/, lexer, parser, sema)
- [ ] `budzero/bud-isa/`
- [ ] `budzero/bud-vm/` (lib, private_transfer, syscall_context)
- [ ] `budzero/bud-proof/` (bud_stark/, adapter, canonical_*, plonky3_*, quarantine, relayer, trace_layout_tests, transfer_verdict)
- [ ] `budzero/bud-pul/` (keccak_chip, mldsa_ntt, pul_air, threshold)
- [ ] `budzero/bud-node/` (bitswap, discovery, sharding, store)
- [ ] `budzero/bud-state/`
- [ ] `budzero/bud-cli/`
- [ ] `budzero/verifier-registry/`
- [ ] `budzero/docs/`

### 11. Contracts (`contracts/`)
- [ ] `contracts/foundry.toml`
- [ ] `contracts/lib/` (BudlumBls.sol, BudlumHash.sol, BudlumOutbound.sol)
- [ ] `contracts/external-domain/BudlumFinalityVerifier.sol`
- [ ] `contracts/graft/` (GraftExecutor, BudlumPaymentReceiver, BudlumRemoteAccountFactory)
- [ ] `contracts/mikoriza/` (BudlumRegistrar, BudlumSlasher, IAllocationManager)
- [ ] `contracts/uel/BudlumUniversal.sol`
- [ ] `contracts/object-layer/` (Solidity, CosmWasm, Move, SVM)
- [ ] `contracts/test/`

### 12. Crates
- [ ] `crates/ai-inference/` (all sub-crates)
- [ ] `crates/bpqs/` (post-quantum signatures)
- [ ] `crates/bud-codec/`
- [ ] `crates/budlum-explorer/`
- [ ] `crates/budlum-types/`
- [ ] `crates/note-packing/`
- [ ] `crates/uel-abi/`
- [ ] `crates/wallet-core/`

### 13. Execution environments (`execution/`)
- [ ] `execution/budzero/`
- [ ] `execution/cairo/`
- [ ] `execution/conformance/`
- [ ] `execution/evm/`
- [ ] `execution/mock/`
- [ ] `execution/move/`
- [ ] `execution/move-sui/`
- [ ] `execution/polkavm/`
- [ ] `execution/svm/`
- [ ] `execution/wasm/`

### 14. Fuzz targets (`fuzz/`)
- [ ] `fuzz/Cargo.toml`
- [ ] All fuzz targets
- [ ] All corpus seeds

### 15. Kani verification
- [ ] `kani/`

### 16. Mobile (`mobile/`)
- [ ] `mobile/core/`
- [ ] `mobile/ffi/`
- [ ] `mobile/android/`
- [ ] `mobile/ios/`
- [ ] `mobile/tools/log-analyzer/`

### 17. Operations (`ops/`)
- [ ] `ops/Dockerfile`
- [ ] `ops/docker-compose.yml`, `ops/docker-compose.ci.yml`
- [ ] `ops/testnet-local.yml`
- [ ] `ops/Caddyfile.testnet`
- [ ] `ops/budlum-core.service`
- [ ] `ops/softhsm2.conf`
- [ ] `ops/hsm.env.example`
- [ ] `ops/monitoring/`
- [ ] `ops/rehearsal/`
- [ ] `ops/testnet-local-validator.py`
- [ ] `ops/scripts/`

### 18. Documentation (`docs/`)
- [ ] `docs/ARCHITECTURE.md`
- [ ] `docs/SPECIFICATION.md`
- [ ] `docs/TESTNET.md`
- [ ] `docs/MAINNET_READINESS.md`
- [ ] `docs/SECURITY.md`
- [ ] `docs/SETTLEMENT.md`
- [ ] `docs/OPERATIONS.md`
- [ ] `docs/VALIDATOR_GUIDE.md`
- [ ] `docs/VALIDATOR_ROLES.md`
- [ ] All remaining docs

### 19. Xtask (`xtask/`)
- [ ] `xtask/gates/` (main.rs and all gate files)
- [ ] `xtask/tools/` (all tool files)
- [ ] `xtask/release/`

### 20. Benchmarks (`benches/`)
- [ ] All benchmark files

### 21. Examples
- [ ] `examples/pq_measure.rs`
- [ ] `examples/print_genesis_hash.rs`

### 22. Tests
- [ ] `tests/churn/`
- [ ] `tests/rehearsal/`
- [ ] `tests/vectors/`

### 23. Release
- [ ] `release/README.md`
- [ ] `release/RELEASE_PUBKEY.hex`
