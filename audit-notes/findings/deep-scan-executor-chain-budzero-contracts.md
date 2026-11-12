# Findings: executor, blockchain, budzero, contracts, concept names

## F-0071

- **ID**: F-0071
- **Location**: src/core/metrics.rs lines 227-236, src/chain/blockchain.rs lines 411-421
- **What is wrong**: Five Prometheus metric names use the deprecated concept name "Pollen": `budlum_pollen_offer_rows`, `budlum_pollen_asset_rows`, `budlum_pollen_grant_rows`, `budlum_pollen_purchase_rows`, `budlum_pollen_protected_content_rows`. Per config/naming/concept_map.toml, "Pollen" must be replaced with "data_market".
- **Why**: The concept name migration (IP-05, TLG-24) requires professional English naming. There are 455 references to "pollen/Pollen" in production code (excluding tests). Prometheus metric names are public API surfaces scraped by monitoring dashboards.
- **What it causes**: TLG-24 (concept name gate in absolute mode) cannot pass with 455 occurrences. Grafana dashboards built against these metric names break on rename. The rename must be coordinated with monitoring configuration.
- **How to fix**: Rename all "pollen" to "data_market" in metrics, field names, and module paths. Coordinate with ops/testnet-local.yml Grafana dashboard configuration.

## F-0072

- **ID**: F-0072
- **Location**: src/chain/blockchain.rs line 3226, src/ directory (43 production references)
- **What is wrong**: The deprecated concept name "Mikoriza" appears in production code. blockchain.rs line 3226: "Finality weight: stake plus Mikoriza restaked weight." There are 43 references in production src/ (excluding tests).
- **Why**: "Mikoriza" / "Mycorrhiza" must be replaced with "shared_security" per concept_map.toml. The name is used in struct fields, comments, and the shared security module path.
- **What it causes**: TLG-24 blocker. The name also propagates into the Solidity contracts (F-0074) and the protobuf wire protocol (F-1205).
- **How to fix**: Rename all "mikoriza" references to "shared_security" in production code. The wire protocol (proto/) requires a separate migration plan per F-1205.

## F-0073

- **ID**: F-0073
- **Location**: src/chain/blockchain.rs lines 4622-4681, src/ directory (261 production references)
- **What is wrong**: The deprecated concept name "Filiz" appears 261 times in production code (excluding tests). blockchain.rs uses `apply_filiz_reveal`, `validator.filiz`, and error messages containing "Filiz". The module src/consensus/filiz.rs, the env var `BUDLUM_FILIZ_SEED_FILE`, and domain tags (`BDLM_FILIZ_ELIG_V1`, `BDLM_FILIZ_EPOCH_V1`) all carry this name.
- **Why**: Per concept_map.toml, "Filiz" must be replaced with "hash_chain_election" (or equivalent professional English name). This is the largest deprecated name by reference count.
- **What it causes**: TLG-24 blocker. 261 references is the highest count of any deprecated concept name. The rename touches the module filename, domain tags (consensus-critical wire format), environment variables, and all API surfaces.
- **How to fix**: Rename in phases: (1) internal comments and struct fields, (2) module name, (3) domain tags with a wire version bump, (4) environment variable with a deprecation period. The domain tags are consensus-critical and must not change without a coordinated upgrade.

## F-0074

- **ID**: F-0074
- **Location**: contracts/mikoriza/ (directory), contracts/mikoriza/BudlumRegistrar.sol lines 13-15, 85, 186, 199
- **What is wrong**: The Solidity contracts use the deprecated name "Mikoriza" in the directory name, contract comments, and domain tag strings: `"budlum/mikoriza/allocation/v1"`, `"BDLM_MIKORIZA_BIND_V1"`, `"BDLM_MIKORIZA_OPERATOR_V1"`, `"BDLM_MIKORIZA_ALLOCATION_V1"`. The test file is also named `Mikoriza.t.sol`.
- **Why**: The contracts were written before the concept name migration. The domain tag strings in the Solidity contracts must match the Rust side for cross-chain signature verification.
- **What it causes**: The domain tag strings are signature-domain separators. Changing them in Solidity without changing the Rust side breaks cross-chain verification. Changing them in Rust without changing Solidity has the same effect. The rename must be synchronized across both sides.
- **How to fix**: Coordinate the rename with the Rust-side shared_security module. Both sides must agree on the new domain tag strings. Deploy the new Solidity contracts before the Rust side starts producing signatures with the new tags.

## F-0075

- **ID**: F-0075
- **Location**: budzero/verifier-registry/src/registry.rs line 14, budzero/verifier-registry/src/evidence.rs line 7
- **What is wrong**: The budzero verifier-registry has 2 unwired markers. The registry.rs documentation explains it is a deliberate "independent second expression" of src/registry/permissionless.rs for double-compiling verification. The evidence.rs landed with K-series evidence claims.
- **Why**: The verifier-registry is a cross-check crate, not a library waiting for a consumer. It duplicates the core registry logic to detect divergence.
- **What it causes**: No production impact. The crate serves as a verification mirror. The `slash_expression` gate prevents the two registries from diverging on slashing arithmetic.
- **How to fix**: No fix needed. This is a deliberate design pattern. Document the relationship in the TLG report.

## F-0076

- **ID**: F-0076
- **Location**: budzero/bud-pul/src/pul_air.rs line 22
- **What is wrong**: The AIR shape helpers in bud-pul are unwired. No production module calls them. The prover paths consume concrete columns directly.
- **Why**: The AIR shape helpers are a proof-side surface for a future caller. The concrete column approach works for current needs.
- **What it causes**: No testnet impact. The prover works through concrete columns. The AIR shape abstraction is not needed until a new circuit type arrives.
- **How to fix**: No fix needed for testnet. Wire when a new AIR shape requires the abstraction.

## F-0077

- **ID**: F-0077
- **Location**: budzero/bud-proof/src/bud_stark/verifier.rs lines 1-5
- **What is wrong**: The BudZero STARK verifier has 2 unwired markers. The verifier is reached through prover pipeline tests and re-exported verify entry points, but has no direct production caller from the node.
- **Why**: The BudZero EE execution path (src/execution/coordinator/mod.rs) uses the prover; the verifier is called through the re-exported entry points in bud-proof, not directly from this file.
- **What it causes**: If the re-export path is the only production path, the verifier is reachable. The unwired marker means no direct call exists from outside the workspace. Verify that the re-export path in bud-proof/src/lib.rs reaches this verifier for production proof checks.
- **How to fix**: Trace the production call path from src/execution/executor.rs through bud-proof to confirm the verifier is reachable. If it is, the unwired marker is informational. If not, the BudZero EE cannot verify proofs.

## F-0078

- **ID**: F-0078
- **Location**: budzero/bud-proof/src/bud_stark/prover.rs lines 253, 323, 526; budzero/bud-proof/src/bud_stark/verifier.rs line 366
- **What is wrong**: Four TODOs reference "upstream p3-uni-stark 0.7.0". These are upstream API limitations in the Plonky3 library.
- **Why**: The BudZero STARK prover/verifier depends on Plonky3 (p3-*). The current Plonky3 version has API gaps that require workarounds. These TODOs mark where the workarounds live.
- **What it causes**: No correctness issue. The workarounds function. When p3-uni-stark 0.7.0 is released, these workarounds can be cleaned up.
- **How to fix**: Track the Plonky3 upstream release. When 0.7.0 arrives, update the workarounds.

## F-0079

- **ID**: F-0079
- **Location**: src/network/node.rs line 1659
- **What is wrong**: `request_witnesses` is unwired. The function asks a peer for transaction witnesses of one block. No caller in the tree uses it.
- **Why**: The witness protocol exists (network/witness_protocol.rs) and the request/response handling is in place, but no production code path calls `request_witnesses` yet.
- **What it causes**: If a node needs to request missing witnesses (for a pruned segment it received without witnesses), it has no way to request them on demand. The import_pruned_segment path in blockchain.rs validates blocks without witnesses when they are pruned, relying on the finality certificate instead.
- **How to fix**: Not a testnet blocker. The pruned segment import path works without on-demand witness requests. Wire when witness-on-demand is needed.

## F-0080

- **ID**: F-0080
- **Location**: src/chain/blockchain.rs (13,063 lines)
- **What is wrong**: blockchain.rs is 13,063 lines. It is the largest file in the codebase, containing the full chain state machine: block validation, block application, epoch transitions, finality aggregation, reorg handling, pruning, metrics emission, settlement observation, and domain unfreezing.
- **Why**: All chain state machine logic lives in one file. This exceeds account.rs (7222 lines, F-0063) as the largest single file.
- **What it causes**: Code review bottleneck. A change to any chain logic (epoch, finality, pruning, reorg) touches this file. TLG-22 (independent review) is harder with a 13K line file.
- **How to fix**: Consider splitting into sub-modules: block_validation.rs, epoch_transition.rs, finality.rs, pruning.rs, reorg.rs. This is a post-testnet refactoring suggestion.

## F-0081

- **ID**: F-0081
- **Location**: src/chain/genesis.rs lines 826, 1206, 1216
- **What is wrong**: Three TESTNET-DEFAULT markers in genesis.rs. Line 826: fee rates are genesis state parameters with testnet defaults. Line 1206: every economic value equals the mainnet template. Line 1216: block execution does not mint emission; the field is testnet-default.
- **Why**: Genesis parameters must be measured from the rehearsal network (IP-46) before mainnet. The testnet defaults are placeholders.
- **What it causes**: No error for testnet. The defaults are intentional. The rehearsal network exercises these values. The governance replacement mechanism must update them before mainnet.
- **How to fix**: Verify that the rehearsal network (IP-46) exercises the fee rates. Document the governance path for parameter updates.

## F-0082

- **ID**: F-0082
- **Location**: contracts/object-layer/BudlumFinalityVerifier.sol lines 48-60
- **What is wrong**: The finality verifier contract uses precompile addresses for BLS and ML-DSA operations: `0x0b` (BLS G1ADD), `0x0f` (BLS PAIRING), `0x10` (BLS MAP_FP_TO_G1), `0x12` (ML-DSA verify), `0x13` (ML-DSA ETH variant). These are EIP-2537 (Cancun) precompile addresses for BLS, but `0x12` and `0x13` (ML-DSA) are not standard EVM precompiles.
- **Why**: The finality verifier needs both BLS and ML-DSA verification. BLS precompiles exist on Ethereum (EIP-2537). ML-DSA precompiles do not exist on standard Ethereum.
- **What it causes**: The contract works only on chains that deploy custom precompiles at addresses 0x12 and 0x13. On standard Ethereum, calls to 0x12 and 0x13 will fail silently (empty return) or succeed vacuously, depending on the EVM version.
- **How to fix**: Document which target chains support the ML-DSA precompiles. If the contract is deployed on standard Ethereum, the ML-DSA verification path must use a Solidity implementation or a different precompile strategy. The contract already handles fallback (gas probing), but the ML-DSA path has no fallback.

## F-0083

- **ID**: F-0083
- **Location**: contracts/uel/BudlumUniversal.sol line 36-37
- **What is wrong**: The Universal Call precompile address is hardcoded to `0x000000000000000000000000000000000000B001`. This address is in the precompile range but is not a standard EVM precompile.
- **Why**: The UEL (Universal Execution Layer) cross-environment calls work through a Budlum-specific precompile. This precompile exists only on the Budlum EVM.
- **What it causes**: No error on Budlum. The contract is Budlum-specific. If deployed on another EVM chain, calls to 0xB001 will fail.
- **How to fix**: No fix needed. Document that BudlumUniversal.sol is Budlum-EVM-only. Consider adding a chain-id check in the constructor or a deployment guard.

## F-0084

- **ID**: F-0084
- **Location**: src/execution/executor.rs lines 46-48
- **What is wrong**: `AI_EXECUTION_BACKEND_PLONKY3` is hardcoded as `"Plonky3-Keccak-Goldilocks"`. The gate is "deliberately the same on every network" (comment at line 43). The comment explains that the string was briefly `"Plonky3"` which no prover emits, creating a dead path. The fix was to align both gates on this exact string.
- **Why**: The proof backend identifier is consensus-critical. Both the mempool and the verifier must accept the same string. A mismatch creates a dead path where valid proofs are refused.
- **What it causes**: No error today. The documentation at lines 23-44 explains the history. This is a positive finding: the single-definition pattern prevents the drift that caused the previous bug.
- **How to fix**: No fix needed. Document this as a deliberate design decision.

## F-0085

- **ID**: F-0085
- **Location**: src/chain/blockchain.rs lines 5620-5646
- **What is wrong**: Block validation clones the entire `AccountState` (`temp_state = self.state.clone()`) to validate transactions before applying them. This is a full deep copy of all chain state (accounts, validators, registries, AI models, governance, BNS, NFTs, privacy notes, etc.).
- **Why**: The validation must not mutate the canonical state until the block is fully validated (including state root check). The clone provides isolation.
- **What it causes**: Every block validation allocates a full copy of the chain state. For a large state (many accounts, storage deals, AI models), this is a significant memory allocation. On a mobile validator, this may cause memory pressure. On a full node, it is acceptable.
- **How to fix**: This is a known trade-off: correctness over performance. The alternative is a copy-on-write state or a transactional wrapper. Not a testnet blocker. Consider COW semantics for mobile validators post-testnet.
