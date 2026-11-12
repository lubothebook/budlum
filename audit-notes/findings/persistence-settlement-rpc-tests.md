# Findings: persistence, settlement, RPC, tokenomics, tests

## F-0051

- **ID**: F-0051
- **Location**: src/persistence/db.rs line 11
- **What is wrong**: The persistence layer uses `sled::Db` directly as its storage backend. sled is used for the chain state database.
- **Why**: Decision KR-37 forbids sled on mobile. The persistence layer is the core chain state store. A mobile validator that uses the same binary will pull in sled.
- **What it causes**: Mobile validators cannot use the persistence layer as-is. The mobile core (mobile/core/) uses an in-memory store (F-0044) as a workaround, but this means the mobile validator cannot persist chain state either.
- **How to fix**: Abstract the persistence backend behind a trait (src/persistence/traits.rs exists for this). Provide a sled implementation for full nodes and a mobile-compatible implementation (redb or flat file) for mobile.

## F-0052

- **ID**: F-0052
- **Location**: src/persistence/db.rs line 11, src/bin/budlum-indexer.rs lines 256-297
- **What is wrong**: sled is used in two places: the main persistence layer and the indexer. The indexer uses sled directly with 5 trees.
- **Why**: The indexer is a separate binary but shares the sled dependency.
- **What it causes**: The indexer is not a mobile component, so sled usage there is acceptable. But the persistence layer (used by the node) must abstract sled for mobile compatibility.
- **How to fix**: The indexer can keep sled. The node persistence must abstract it.

## F-0053

- **ID**: F-0053
- **Location**: src/tokenomics/mod.rs lines 54-57
- **What is wrong**: The reward pool split assertion (40% + 50% + 10% = 100%) is a compile-time `const` assert. This is correct.
- **Why**: The split must sum to 100%. The assertion catches any change that breaks the invariant.
- **What it causes**: No error. This is a positive finding. The supply conservation invariant (G-1) is enforced at compile time for the pool split.
- **How to fix**: No fix needed. Good practice.

## F-0054

- **ID**: F-0054
- **Location**: src/tokenomics/mod.rs line 76-79
- **What is wrong**: `epoch_repair_relay_reward_share` absorbs rounding dust: it computes `budget - consensus_share - storage_share`. This ensures the three shares sum to the budget with no lost or created base units.
- **Why**: Supply conservation (G-1). Integer division can lose base units.
- **What it causes**: No error. The dust absorption is correct. The repair-relay share may differ from `budget * 1000 / 10000` by up to 2 base units.
- **How to fix**: No fix needed. Document this as a deliberate design choice.

## F-0055

- **ID**: F-0055
- **Location**: src/settlement/graft.rs (2 unwired markers)
- **What is wrong**: The settlement graft module has 2 unwired surfaces.
- **Why**: Settlement grafting is part of the domain bridge infrastructure. The bridge is conditional at testnet.
- **What it causes**: If the bridge opens, these surfaces need wiring. If conditional, no error.
- **How to fix**: Track with the bridge decision (TLG-27).

## F-0056

- **ID**: F-0056
- **Location**: src/settlement/pq_anchor_verify.rs (1 unwired marker)
- **What is wrong**: PQ anchor verification has an unwired surface.
- **Why**: PQ anchors are part of the settlement layer's post-quantum root of trust (src/main.rs devnet and production anchor modes are wired).
- **What it causes**: If the unwired surface is a verification function needed for light client anchor following, light clients cannot verify PQ anchors.
- **How to fix**: Identify the specific unwired function. If it is `verify_anchor_light` or similar, it blocks the light client (TLG-21 mentions explorer).

## F-0057

- **ID**: F-0057
- **Location**: src/tests/ (116 test files)
- **What is wrong**: The test directory contains 116 files. This is a large test suite.
- **Why**: The directive requires all tests to pass (TLG-25). With 116 test files, maintenance burden is high.
- **What it causes**: No error. A large test suite is a positive signal. The README claims 4444 tests.
- **How to fix**: No fix needed. Verify the actual test count matches the README claim (F-0004).

## F-0058

- **ID**: F-0058
- **Location**: TLG-02 closing tests (28 found)
- **What is wrong**: 28 `*_is_closed_on_testnet` tests exist across the codebase. The testnet index (audit-notes/testnet/index.md Section 14) lists 9 closed features.
- **Why**: Some closed features have multiple tests (e.g. verify_merkle and verify_inference each appear 3 times in different test files). The count of unique closed features covered is approximately 14.
- **What it causes**: TLG-02 requires every closed and closing conditional feature to have a closing test. The directive's Section 14 lists: private dedup, shared compression, live streaming, SVM/Move asset pointers, Universal Object Layer, Remote App Binding, shared security, private transfer, on-chain AI inference. Cross-checking: private_dedup (yes), compression_context (yes), bud3_live + live_stream (yes), object_layer (yes), shared_security (yes), private_transfer (yes), ai_execution_proof + ai_layer (yes), ethereum_sync_committee (yes). SVM/Move asset pointers and Remote App Binding closing tests need verification.
- **How to fix**: Verify that SVM/Move asset pointers and Remote App Binding have closing tests. These may be covered under the object_layer or foreign module tests.

## F-0059

- **ID**: F-0059
- **Location**: src/rpc/compat/ (ethereum, solana, aptos, cosmos, starknet, sui)
- **What is wrong**: Six RPC compatibility layers exist. TLG-13 requires tool compatibility (Foundry, Hardhat, viem, ethers, CosmJS, Anchor, solana CLI, aptos CLI). The StarkNet and Sui compat modules have `is_closed_on_testnet` tests, meaning they are closed at launch.
- **Why**: The directive lists which external tools must work at testnet. StarkNet and Sui compat layers are not in the testnet requirements.
- **What it causes**: No error. The compat layers for closed EEs are correctly gated.
- **How to fix**: Verify that the open compat layers (Ethereum, Solana, Aptos, Cosmos) pass their respective tool tests.

## F-0060

- **ID**: F-0060
- **Location**: src/bin/ (8 binary targets)
- **What is wrong**: The repository has 8 binary targets: budlum-core (main), bud (CLI wallet), budlum-relayer, budlum-light, budlum-indexer, budlum-faucet, budlum-domain-producer, budlum-domain-reporter, plus bud-migrate.
- **Why**: TLG-21 requires explorer/indexer. TLG-21 requires faucet. The wallet is IP-39. The relayer is for the bridge (conditional).
- **What it causes**: No error. The binary targets map to testnet requirements. budlum-domain-producer and budlum-domain-reporter are for the domain system.
- **How to fix**: Verify each binary compiles and runs. The CI should build all targets.

## F-0061

- **ID**: F-0061
- **Location**: src/core/transaction.rs (7 unwired markers at lines 1047, 1068, 1089, 1109, 1582, 1783, 1984)
- **What is wrong**: The transaction module has 7 unwired API surfaces. Transactions are the core of every block.
- **Why**: These may be future transaction types or validation surfaces not yet wired into the block execution path.
- **What it causes**: If any of these are transaction validation checks that the block path skips, blocks may accept transactions that should be refused.
- **How to fix**: Audit each unwired marker. Cross-reference with the block execution path in src/execution/executor.rs to verify that all active transaction types have wired validation.
