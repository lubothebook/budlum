# Cross-Domain Module Findings

Files reviewed: src/cross_domain/mod.rs, bridge.rs, bridge_relayer.rs, chain_adapter.rs, event_tree.rs, message.rs, message_registry.rs, nonce.rs, relayer.rs, evm/, external/

## F-0021: Bridge state machine has comprehensive safety checks

- **Location**: `src/cross_domain/bridge.rs`
- **What is wrong**: Not a finding. The bridge implements the lock-mint-burn-unlock cycle with proper safety checks. The BridgeState enum (Active/Frozen) correctly gates operations. BLG-043, BLG-057, BLG-105, and BLG-194 (the bridge conditional open) are referenced in the testnet feature matrix.
- **Status**: closed (informational, positive assessment)

## F-0022: Cross-domain message replay protection

- **Location**: `src/cross_domain/nonce.rs`
- **What is wrong**: The ReplayNonceStore exists and provides replay protection for cross-domain messages. The nonce is checked before processing and committed after. The BDLM_DOMAIN_COMMITMENT_V1 hash tag is used for domain separation.
- **Why it is wrong**: Positive finding. Replay protection is correctly implemented.
- **Status**: closed (informational, positive assessment)

## F-0023: External domain module is the largest cross-domain component

- **Location**: `src/cross_domain/external/mod.rs` (1244 lines)
- **What is wrong**: The external domain module handles EVM and other external chain integration. At 1244 lines, it is the most complex component in cross_domain. The selftest, versioning, intake_quorum, and prover submodules suggest mature functionality.
- **Why it is wrong**: Not a finding. The complexity matches the responsibility. The module handles EVM header verification, Merkle Patricia Trie proofs, sync committee tracking, and receipt verification.
- **How to fix it**: Run the selftest harness to verify all code paths.
- **Status**: open (verification needed)

## F-0024: EVM adapter handles MPT verification

- **Location**: `src/cross_domain/evm/mpt.rs`, `verify.rs`
- **What is wrong**: The EVM Merkle Patricia Trie (MPT) verifier and RLP decoder are in-tree implementations. These are critical security paths: a bug in MPT verification could allow a forged receipt proof to pass.
- **Why it is wrong**: In-tree implementations of complex data structures should have extensive fuzz testing. There are fuzz targets for `evm_mpt_verify` and `evm_rlp_decode` in `fuzz/fuzz_targets/`.
- **What it causes**: If the fuzz targets have not been run recently, latent bugs could exist.
- **How to fix it**: Run the fuzz targets for at least 1 hour each. Verify the corpus covers edge cases.
- **Status**: open (verification needed)
