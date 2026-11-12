# Findings: governance, crates, TESTNET-DEFAULT inventory, concept names

## F-0099

- **ID**: F-0099
- **Location**: src/core/chain_config.rs lines 158, 199, 213
- **What is wrong**: Three comments reference "Ayaz" as a decision maker: "the mainnet value is Ayaz's decision", "Mainnet waits for Ayaz", "the mainnet decision is Ayaz's". A fourth reference at src/chain/genesis.rs line 569: "is Ayaz's". A fifth at genesis.rs line 1163: "no pool until Ayaz fixes the schedule".
- **Why**: "Ayaz" is a person name used as a shorthand for a pending decision. The directive requires professional English naming. Person names in code comments create confusion when the person leaves or the decision is delegated.
- **What it causes**: TLG-24 (concept name gate) may flag "Ayaz" if it is in the concept map. Even if not, person names in code create an implicit dependency on institutional knowledge. A new contributor cannot determine what "Ayaz's decision" means.
- **How to fix**: Replace with the decision identifier: "the mainnet value is pending (DECISIONS.md)" or "the mainnet value is governed by KR-XX".

## F-0100

- **ID**: F-0100
- **Location**: src/storage/content_gateway.rs line 235
- **What is wrong**: The content gateway uses `"ayaz.bud"` as a BNS example name in a comment. This is a person name used as a domain example.
- **Why**: Example names in code should use generic placeholders, not real person names.
- **What it causes**: No functional impact. A style observation.
- **How to fix**: Replace with a generic example like `"example.bud"` or `"alice.bud"`.

## F-0101

- **ID**: F-0101
- **Location**: 50 locations across src/ (see TESTNET-DEFAULT inventory)
- **What is wrong**: 50 TESTNET-DEFAULT markers exist in production code. Top files by count: registry/params.rs (8), core/chain_config.rs (5), ai/verification_layer.rs (4), storage/bud3/fees.rs (3), chain/genesis.rs (3), bin/budlum-faucet.rs (3), validators/committee.rs (2), storage/bud3/repair.rs (2), storage/bud3/placement.rs (2), mempool/pool.rs (2), mempool/duty_lane.rs (2).
- **Why**: TESTNET-DEFAULT markers denote values that must be measured from the rehearsal network and replaced with governance-calibrated values before mainnet. They are correct for testnet.
- **What it causes**: No testnet error. These are intentional placeholder values. The governance replacement path must update them before mainnet. The count (50) indicates the scope of the calibration work.
- **How to fix**: Create a tracking document that lists all 50 TESTNET-DEFAULT values, their current values, and the measurement plan. The rehearsal network (IP-46) should exercise all of them.

## F-0102

- **ID**: F-0102
- **Location**: src/core/governance.rs lines 34-43
- **What is wrong**: Governance activation delays have compile-time assertions ensuring they are non-zero and ordered (policy > parameter). The comment at line 33 says "Two 2026 incidents turned on exactly this value being zero when it mattered."
- **Why**: The compile-time assertions prevent governance proposals from activating instantly. This is a positive finding. The incident history documents why.
- **What it causes**: No error. Good defensive design. Proposals always have a non-zero activation delay.
- **How to fix**: No fix needed.

## F-0103

- **ID**: F-0103
- **Location**: src/core/governance.rs lines 77-92
- **What is wrong**: The governance parameter whitelist has 14 entries. Any parameter not on this list is refused by governance proposals. The whitelist is hardcoded.
- **Why**: The whitelist bounds what governance can change. This is a security feature.
- **What it causes**: Adding a new governance-tunable parameter requires a code change, not a governance proposal. This is intentional for testnet but limits governance flexibility.
- **How to fix**: No fix needed for testnet. Consider whether the whitelist should be a constitution parameter itself post-testnet.

## F-0104

- **ID**: F-0104
- **Location**: src/account_abstraction/tee_attestation.rs lines 39-60
- **What is wrong**: TEE attestation is unwired. The module documents that the old implementation was broken: `TeeRuntime::sign` copied a SHA3-256 digest into a buffer and called it a signature. Verification checked `self.signature.len() != 4627`, which was always false for a `[u8; 4627]` field. The module was rewritten to use ML-DSA-87 real signatures. But it is still unwired because transactions carry no attestation field.
- **Why**: Adding an attestation field to transactions is a commitment surface change. The verifying side needs a trust root in the node configuration to check vendor certificate chains. Neither exists.
- **What it causes**: No TEE attestation on testnet. Account abstraction V6 transactions work with threshold ML-DSA signatures but without TEE binding. Not a testnet blocker.
- **How to fix**: Wire when the transaction format gains an attestation field and the node configuration gains a TEE trust root.

## F-0105

- **ID**: F-0105
- **Location**: src/account_abstraction/private_transfer_auth.rs line 36
- **What is wrong**: Private transfer authorization is unwired. The old justification for the unwired mark was stale.
- **Why**: Privacy transfers are closed on testnet (F-0065). The authorization module is part of the privacy feature.
- **What it causes**: No testnet impact. Privacy is closed.
- **How to fix**: Wire when privacy opens.

## F-0106

- **ID**: F-0106
- **Location**: crates/wallet-core/src/wallet_v3.rs line 10, crates/wallet-core/src/consent_screen.rs line 5
- **What is wrong**: The wallet-core crate has 2 unwired surfaces: the V3 wallet surface (custody path) and the consent screen (wallet UI confirmation). The wallet-core crate is 2999 lines.
- **Why**: The wallet binary (`bud` CLI) and SDK callers are not wired to the V3 surface yet. The consent screen is staged for UI integration.
- **What it causes**: The `bud` CLI wallet may not use the V3 custody path. TLG-12 (wallet features) depends on the wallet working end-to-end.
- **How to fix**: Verify that the `bud` CLI wallet uses the correct wallet surface for testnet. If V3 is the required version, wire it.

## F-0107

- **ID**: F-0107
- **Location**: crates/note-packing/src/digest4.rs line 5
- **What is wrong**: Digest4 V2 activation is staged but unwired in the note-packing crate.
- **Why**: Digest4 V2 is a privacy-layer component. Privacy is closed on testnet.
- **What it causes**: No testnet impact. Wire when privacy opens.
- **How to fix**: No fix needed for testnet.

## F-0108

- **ID**: F-0108
- **Location**: crates/bpqs/src/lib.rs line 8
- **What is wrong**: The BPQS (research post-quantum scheme) crate is unwired. The dedicated algorithm slot (SchemeId 0x0401) is registered as reserved in src/crypto/scheme.rs (F-0041).
- **Why**: BPQS is a research-level scheme. No testnet requirement.
- **What it causes**: No testnet impact.
- **How to fix**: No fix needed.

## F-0109

- **ID**: F-0109
- **Location**: crates/ai-inference/crates/ai-core/src/verification.rs line 9, crates/ai-inference/crates/ai-ops/src/logparse.rs line 4
- **What is wrong**: The AI inference crate has 2 unwired surfaces: AI verification layer activation and log parsing. The AI layer is conditional at testnet (F-0050).
- **Why**: AI inference is conditional. The crates are staged for activation.
- **What it causes**: No testnet impact if AI stays conditional.
- **How to fix**: Wire if AI opens for testnet.

## F-0110

- **ID**: F-0110
- **Location**: src/domain/storage_deal.rs (9864 lines)
- **What is wrong**: storage_deal.rs is 9864 lines. It is the third largest file in the codebase after blockchain.rs (13,063) and account.rs (7,222).
- **Why**: Storage deals are the economic foundation of B.U.D. 3.0. The file contains deal lifecycle, pricing, repair, audit, and economics.
- **What it causes**: Same code review bottleneck as F-0063 and F-0080. Not a correctness issue.
- **How to fix**: Consider splitting post-testnet.

## F-0111

- **ID**: F-0111
- **Location**: Full repository unwired inventory
- **What is wrong**: Total WIRING: unwired count across all Rust code: src/ (67), budzero/ (5), crates/ (6) = 78 total. (Updated from F-0088 which counted 72 for src/+budzero/ only.)
- **Why**: The crates/ directory adds 6 more unwired surfaces not counted in F-0088.
- **What it causes**: 78 total declared API surfaces with no production callers. The full inventory by module area: core (11), consensus (8), settlement (6), cross_domain (6), crypto (6), storage (6), network (5), ai (4), persistence (3), chain (3), security (2), account_abstraction (2), relayer (1), pollen (1), object_layer (1), light_client (1), ai_inference (1), budzero (5), crates (6).
- **How to fix**: Triage each unwired surface against TLG gates. Surfaces on the testnet critical path must be wired before testnet.

## F-0112

- **ID**: F-0112
- **Location**: src/registry/params.rs (8 TESTNET-DEFAULT markers)
- **What is wrong**: The registry parameters file has the highest concentration of TESTNET-DEFAULT values: double_sign_slash_ratio (20%), liveness_slash_ratio (1%), unbonding_epochs (100), min_undelegation_amount (100 base units), max_delegation_positions (1000), self-bond floor (min_stake), min undelegation (100 base units).
- **Why**: These are economic parameters that must be calibrated from rehearsal network measurements.
- **What it causes**: No testnet error. The values are reasonable placeholders. Mainnet requires calibration.
- **How to fix**: Document each parameter's measurement plan in the TLG report.
