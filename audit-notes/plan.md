# Budlum Testnet Audit Plan

This file lists the audit steps, target operating systems, and every area in the
repository. Each area is marked done after all findings are recorded and all
fixes are committed.

## Operating Systems

- [ ] Ubuntu 24.04 LTS (x86_64)
- [ ] Ubuntu 24.04 LTS (aarch64)
- [ ] Ubuntu 22.04 LTS (x86_64)
- [ ] Debian 12 Bookworm (x86_64)
- [ ] Alpine Linux (x86_64, musl)
- [ ] Fedora Linux (x86_64)
- [ ] macOS (Apple Silicon)
- [ ] Windows Server 2022 (x86_64)

## Audit Areas

### Core Modules
- [x] `src/consensus/` - Consensus engines (PoW, PoS, PoA, BFT)
- [ ] `src/validators/` - Validator classes, handover, committee
- [ ] `src/chain/` - Blockchain, finality, genesis, snapshot
- [ ] `src/core/` - Account, block, transaction, chain config
- [ ] `src/persistence/` - Node-internal key-value store
- [ ] `src/storage/bud3/` - B.U.D. 3.0 storage system
- [ ] `src/execution/` - Execution coordinator, BudZero EE
- [ ] `src/rpc/` - RPC server and compatibility gateways
- [ ] `src/network/` - libp2p networking
- [ ] `src/registry/` - Stake registry, evidence, identity
- [ ] `src/domain/` - Domain registry and finality adapters
- [ ] `src/cross_domain/` - Bridge, messages, external domains
- [ ] `src/settlement/` - GlobalBlockHeader, PQ anchor, proof market
- [ ] `src/crypto/` - Cryptographic primitives, domain tags, signers

### Ecosystem Modules
- [ ] `src/ai/` - AI verification layer
- [ ] `src/ai_inference/` - AI inference runtime
- [ ] `src/bns/` - Budlum Name Service
- [ ] `src/socialfi/` - SocialFi and NFT folders
- [ ] `src/data_market/` - Data Market (Pollen)
- [ ] `src/pollen/` - Content gate and data rights
- [ ] `src/explorer_evidence/` - Explorer evidence model
- [ ] `src/app_registry/` - Application registry
- [ ] `src/education_domain/` - Education domain
- [ ] `src/budlumxyz/` - Application registry types

### Security and Privacy
- [ ] `src/security/` - Quantum canary, alarm effects
- [ ] `src/privacy/` - Note registry, private submit
- [ ] `src/account_abstraction/` - Quantum-safe AA
- [ ] `src/object_layer/` - Universal Object Layer

### Infrastructure
- [ ] `src/tokenomics/` - Reward pool, economics
- [ ] `src/mempool/` - Transaction pool
- [ ] `src/light_client/` - Light client
- [ ] `src/sharding/` - Sharding configuration
- [ ] `src/wallet/` - V5 command-line wallet
- [ ] `src/sdk/` - Project file schema
- [ ] `src/cli/` - CLI commands
- [ ] `src/bin/` - All binary targets

### External Crates
- [ ] `crates/uel-abi/` - Universal Execution Layer ABI
- [ ] `crates/bud-codec/` - A0 codec registry
- [ ] `crates/budlum-types/` - Shared wire types
- [ ] `crates/wallet-core/` - Wallet core library
- [ ] `crates/bpqs/` - Post-quantum signature research
- [ ] `crates/budscan/` - Budlum Explorer
- [ ] `crates/note-packing/` - Note packing
- [ ] `crates/ai-inference/` - AI inference crate

### BudZero
- [ ] `budzero/bud-compiler/` - Compiler
- [ ] `budzero/bud-isa/` - Instruction set
- [ ] `budzero/bud-vm/` - Virtual machine
- [ ] `budzero/bud-proof/` - Proof system
- [ ] `budzero/bud-state/` - State layer
- [ ] `budzero/bud-node/` - Content node
- [ ] `budzero/verifier-registry/` - Verifier registry

### Execution Environments
- [ ] `execution/evm/` - EVM execution environment
- [ ] `execution/wasm/` - WASM execution environment
- [ ] `execution/svm/` - SVM execution environment
- [ ] `execution/move/` - Move execution environment
- [ ] `execution/conformance/` - Conformance tests

### Mobile
- [ ] `mobile/core/` - Mobile core (Rust, UniFFI)
- [ ] `mobile/android/` - Android app
- [ ] `mobile/ios/` - iOS app

### CI and Supply Chain
- [ ] `.github/workflows/` - All CI workflows
- [ ] `xtask/gates/` - Quality gates
- [ ] `xtask/tools/` - Build tools
- [ ] `supply-chain/` - cargo-vet configuration
- [ ] `ops/` - Docker, systemd, prometheus

### Configuration
- [ ] `config/` - Genesis, network configs, personas
- [ ] `fuzz/` - Fuzz targets and corpus
- [ ] `kani/` - Formal verification
