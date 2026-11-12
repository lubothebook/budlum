# Findings: deep pass 2 - cross-domain, storage, AI, identity

## F-0149

- **ID**: F-0149
- **Location**: src/cross_domain/evm/ (5 files: adapter.rs, header.rs, sync_committee.rs, verify.rs, outbound.rs)
- **What is wrong**: The entire EVM bridge verification chain is unwired. Five files in `src/cross_domain/evm/` declare themselves unwired. These implement `verify_deposit_observed`, `verify_chain`, `verify_execution_block_finality`, `verify_evm_receipt`, and the outbound path. The relayer binary (`src/bin/budlum-relayer.rs`) documents that it is NOT mainnet ready and the bridge adapters are deliberately disabled.
- **Why**: The bridge is a conditional feature for testnet (TLG-27 requires a bridge decision written in the TLG report). The verification code exists but the production wiring path through the relayer is not active.
- **What it causes**: No EVM bridge deposits or withdrawals can be processed. The Ethereum side contracts exist (BudlumFinalityVerifier, BudlumSettlementEndpoint) but the Rust verification pipeline is not connected. A testnet with bridge enabled would have no deposit or withdrawal path.
- **How to fix**: This is a design decision, not a code bug. Document the bridge status in the TLG report (TLG-27). If bridge stays closed on testnet, add a `bridge_is_closed_on_testnet` closing test.

## F-0150

- **ID**: F-0150
- **Location**: src/cross_domain/external/phase_a_dual.rs
- **What is wrong**: The Phase-A dual-finality module is unwired. It is staged for recursion and dual-finality activation.
- **Why**: This is a post-launch feature.
- **What it causes**: No impact on testnet. The module is correctly marked as unwired.
- **How to fix**: No fix needed for testnet.

## F-0151

- **ID**: F-0151
- **Location**: src/ai/mod.rs lines 22-46
- **What is wrong**: The `FULL_AI_STARK_VERIFICATION_LIVE` flag is hardcoded to `false`. The module documents three specific binding gaps in the AI STARK proof: (1) `initial_state_root` is a prover claim not rebuilt node-side, (2) `event_digest` is never compared with `output_commitment`, (3) `sender`, `nonce` and `block_height` in public inputs are not bound to the attaching transaction.
- **Why**: The AI layer is gated by `ai_layer_enabled()` which returns true only on devnet. The STARK verification has known gaps that prevent it from being used as a production acceptance condition.
- **What it causes**: On devnet, AI inference proofs can be submitted but the proofs do not prove what they claim. A valid trace can be paired with substituted weights, and one proof can accompany multiple requests. This is explicitly documented and the flag is off (fail-closed). TLG-27 requires the AI layer decision written in the TLG report.
- **How to fix**: No fix needed for testnet. The AI layer must remain closed on testnet until the three binding gaps are addressed. Add a `ai_stark_verification_is_closed_on_testnet` closing test if one does not already exist.

## F-0152

- **ID**: F-0152
- **Location**: src/ai/verification_layer.rs (lines 31-37)
- **What is wrong**: The AI verification layer has 4 TESTNET-DEFAULT parameters: committee size n=5, agreement threshold k=4, challenge window 100 blocks, second-committee sampling rate 100 bps (1%). The entire module is unwired.
- **Why**: The AI verification layer is a post-launch feature.
- **What it causes**: No impact on testnet. The TESTNET-DEFAULT values are documented. If the AI verification layer is activated, these values need calibration from rehearsal.
- **How to fix**: No fix needed for testnet. Calibrate the values if/when the AI verification layer is activated.

## F-0153

- **ID**: F-0153
- **Location**: src/core/transaction.rs (7 unwired markers at lines 1047, 1068, 1089, 1109, 1582, 1783, 1984)
- **What is wrong**: Seven transaction constructor functions are unwired: `new_delegate`, `new_undelegate`, `new_withdraw_delegation`, `new_governance_vote` (likely), and others. These are convenience constructors for transaction types that have no production callers in the tree.
- **Why**: The transaction types exist in the executor (Delegate, Undelegate, WithdrawDelegation are handled by `executor.rs`). The constructors are declared surfaces for SDK, CLI, and wallet callers that do not exist in the tree yet.
- **What it causes**: The delegation flow works through the executor when transactions are constructed externally (by the wallet or SDK). The in-tree constructors are not called by any production code. No impact on testnet functionality; the executor paths are wired.
- **How to fix**: No fix needed. The constructors will be wired when the SDK and wallet crate callers land.

## F-0154

- **ID**: F-0154
- **Location**: src/storage/bud3/ (3 unwired modules: live.rs, compression_context/corpus.rs, private_dedup/committee.rs)
- **What is wrong**: Three B.U.D. 3.0 storage modules are unwired. `live.rs` (live streaming), `compression_context/corpus.rs` (shared compression context), and `private_dedup/committee.rs` (private cross-owner deduplication). All three are post-launch T-Expansion features. Each has a corresponding `*_is_closed_on_testnet` closing test.
- **Why**: These features are closed on testnet by design.
- **What it causes**: Live streaming, shared compression context, and private dedup are not available on testnet. The closing tests prove they are rejected on chain ID 45261.
- **How to fix**: No fix needed. The closing tests exist. Verify they pass.

## F-0155

- **ID**: F-0155
- **Location**: src/chain/genesis.rs lines 569, 1163; src/core/chain_config.rs lines 158, 199, 213
- **What is wrong**: Five production comments reference "Ayaz" (a person name) as a decision-maker for mainnet economic parameters. Examples: "the mainnet value is Ayaz's decision", "Mainnet waits for Ayaz", "is Ayaz's". These are the same references reported in F-0099 and F-0100.
- **Why**: Mainnet economic parameters (base fee, reward pool schedule, epoch values) are pending a decision from a named team member.
- **What it causes**: The comments tie production code to a specific person. If "Ayaz" leaves the project or changes roles, the comments become misleading. The person name should be replaced with the role or the decision document reference.
- **How to fix**: Replace "Ayaz" with "the economics lead" or "DECISIONS.md section N" in all five comments. This was already reported in F-0099 and F-0100 but the references in genesis.rs (lines 569, 1163) were found in this deeper pass.

## F-0156

- **ID**: F-0156
- **Location**: src/storage/one_share.rs
- **What is wrong**: The `one_share.rs` module is a legacy B.U.D. 1.0 marker projection retained for backward schema compatibility. It is unwired. B.U.D. 1.0 should have been removed per TLG-03 (IP-07: remove B.U.D. 1.0, B.U.D. 2.0, bud/ workspace, budzero/bud-node).
- **Why**: The module is retained for schema compatibility. Removing it may break snapshot deserialization.
- **What it causes**: TLG-03 requires removal of legacy storage code. The `no_legacy_storage.rs` gate exists (xtask/gates/src/gates/no_legacy_storage.rs) but this module may be exempted or overlooked.
- **How to fix**: Verify that `no_legacy_storage.rs` covers `one_share.rs`. If the module is needed for schema compatibility, document the exemption. If not, remove it.

## F-0157

- **ID**: F-0157
- **Location**: src/registry/identity.rs (2714 lines), src/registry/identity_fill.rs
- **What is wrong**: The identity registry module is wired ("WIRING: the identity transaction door... is the production caller") but uses PoA domain placement exclusively. The identity feature must be confined to PoA domains per the design doc. No gate verifies this confinement at the CI level.
- **Why**: The registry enforces PoA gating internally (`ConsensusKind::PoA` check). The `poa_compliance_gate.rs` gate exists in CI and checks PoA isolation for MASAK/AML hooks, but it is unclear if it covers the identity registry's PoA confinement.
- **What it causes**: If the PoA gate inside the identity registry is removed or bypassed, identity writes could land on non-PoA domains. The CI `poa_compliance_gate.rs` should verify this.
- **How to fix**: Verify that `poa_compliance_gate.rs` covers the identity registry's PoA confinement. If not, extend it.
