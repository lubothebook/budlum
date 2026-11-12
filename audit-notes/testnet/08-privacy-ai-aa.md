# How to build: Privacy, AI, account abstraction

Testnet index Section 8. Work package IP-45. TLG gate TLG-27.

---

## Current state

**Privacy**: The privacy module (`src/privacy/`, 3 files) has note registry and private transfer submit. Private transfer is closed on testnet (closing test exists). Private transfer authorization (`src/account_abstraction/private_transfer_auth.rs`) is unwired (F-0105). Digest4 V2 in `crates/note-packing/` is unwired (F-0107).

**AI on-chain inference**: The AI layer has 4 unwired surfaces in `src/ai/` (F-0050) and 2 unwired in `crates/ai-inference/` (F-0109). The AI layer is conditional at testnet. `ai_layer_enabled()` returns true only for devnet. The executor gate (`check_ai_layer`) refuses AI transactions on non-devnet chains with a detailed error message.

**Account abstraction**: V6 transactions with threshold ML-DSA signatures work. TEE attestation (`src/account_abstraction/tee_attestation.rs`) is unwired (F-0104). The old broken implementation was rewritten to use real ML-DSA-87 signatures.

## What must be built

1. **AI layer decision** (TLG-27). Determine whether BLG-059 through BLG-062 are closed and VL-1 conditions are met. If all conditions pass, the AI layer opens. Write the decision in the TLG report.

2. **If AI opens**: Wire the 4 surfaces in `src/ai/` and the 2 in `crates/ai-inference/`. Enable `ai_layer_enabled()` for testnet chain ID 45261.

3. **If AI stays closed**: Verify closing tests exist for all AI transaction types on chain ID 45261 (TLG-02). The executor gate (`check_ai_layer`) already refuses them.

4. **Privacy**: No build needed for testnet. Privacy is closed. Closing test exists (`private_transfer_is_closed_on_testnet`).

5. **Account abstraction**: V6 threshold signatures work. TEE attestation is a future feature. No build needed for testnet.

## Key files

- `src/privacy/`: note registry, private transfer
- `src/ai/`: verification layer, execution, registry
- `src/account_abstraction/`: TEE attestation, private transfer auth
- `src/execution/executor.rs` lines 96-178: AI layer gating
- `crates/ai-inference/`: AI inference crate
- `crates/note-packing/`: privacy note packing

## Blocking findings

- F-0050: AI verification layer 4 unwired (conditional)
- F-0104: TEE attestation unwired (not needed for testnet)
- F-0105: private transfer auth unwired (privacy closed)
- F-0109: AI inference crate 2 unwired (conditional)
