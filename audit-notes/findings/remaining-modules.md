# Findings: remaining modules

## F-0062

- **ID**: F-0062
- **Location**: src/sharding/mod.rs line 16
- **What is wrong**: The sharding module states "Enforcement is not wired: no validation rule recomputes the commitment yet, so today a block with a wrong `shards_root` is not rejected for that reason."
- **Why**: Sharding parameters are consensus data. src/main.rs lines 1193-1201 refuse any non-default sharding configuration. The feature is not active.
- **What it causes**: If sharding were enabled, blocks with wrong shard roots would not be rejected. This is correctly guarded by the main.rs fail-closed check.
- **How to fix**: No fix needed for testnet. Sharding is disabled. The validation rule must be wired before sharding activates.

## F-0063

- **ID**: F-0063
- **Location**: src/core/account.rs (7222 lines, 4 unwired markers)
- **What is wrong**: account.rs is 7222 lines. It is the largest single file in the codebase. It contains AccountState, the state root computation, validator management, storage deal state, domain state, and settlement state.
- **Why**: All chain state lives in one file. This makes it difficult to review, test, and reason about state transitions.
- **What it causes**: High coupling. A change to any part of the state affects the entire file. Code review of account.rs is a bottleneck. Not a correctness issue but a process concern for TLG-22 (independent review).
- **How to fix**: Consider splitting into sub-modules: account_state.rs, validator_state.rs, domain_state.rs, settlement_state.rs. This is a refactoring suggestion for post-testnet.

## F-0064

- **ID**: F-0064
- **Location**: src/light_client/mod.rs line 24
- **What is wrong**: The light client module says "a set change needs a new trust anchor out-of-band". This means the light client cannot follow validator set changes without external trust.
- **Why**: The Handoff Certificate (src/consensus/handoff.rs) is designed to allow light clients to follow set changes. The comment suggests this is not yet integrated.
- **What it causes**: Light clients cannot independently verify epoch transitions. They must be re-bootstrapped after each validator set change. TLG-21 (explorer) may use the light client.
- **How to fix**: Integrate `follow_handoffs` from src/consensus/handoff.rs into the light client module. This enables chain following across epoch boundaries.

## F-0065

- **ID**: F-0065
- **Location**: src/privacy/mod.rs (entire module, 3 files)
- **What is wrong**: The privacy module is minimal: note registry and private transfer submit. Private transfer is closed on testnet (proven by `private_transfer_is_closed_on_testnet` test).
- **Why**: KR-26 states private transfer is closed until PTP-1 merges. The code correctly gates this.
- **What it causes**: No error. The closing test exists. The privacy module is correctly scoped for testnet.
- **How to fix**: No fix needed.

## F-0066

- **ID**: F-0066
- **Location**: src/mempool/ (5 files: authority_cache.rs, duty_lane.rs, lanes.rs, mod.rs, pool.rs)
- **What is wrong**: The mempool has a duty lane system. No unwired markers found in the mempool.
- **Why**: The mempool is on the critical path for block production. It must accept, prioritize, and serve transactions.
- **What it causes**: No error from the audit perspective. The mempool appears to be wired into the block production path.
- **How to fix**: No fix needed. This is a positive observation.

## F-0067

- **ID**: F-0067
- **Location**: src/security/quantum_canary.rs (2 unwired markers)
- **What is wrong**: The quantum canary has 2 unwired surfaces. The quantum canary is an early-warning system for quantum computing threats.
- **Why**: The canary monitors for quantum breaks. Unwired means it cannot alert in production.
- **What it causes**: If a quantum computing threat materializes, the canary cannot raise an alarm. This is a defense-in-depth feature, not a testnet blocker.
- **How to fix**: Wire the quantum canary surfaces before mainnet. Not a testnet requirement.

## F-0068

- **ID**: F-0068
- **Location**: src/pollen/data_rights.rs (1 unwired marker)
- **What is wrong**: The data rights module (Pollen data market) has an unwired surface.
- **Why**: The data market is part of the B.U.D. 3.0 ecosystem. It is listed in testnet index Section 14 (open at launch: BNS, SocialFi, Data Market).
- **What it causes**: If the data market is open at launch, the unwired surface may be needed.
- **How to fix**: Verify that the unwired surface is not on the critical path for data market operations at testnet.

## F-0069

- **ID**: F-0069
- **Location**: src/object_layer/ (entire module, closed at testnet)
- **What is wrong**: The Universal Object Layer is listed as closed at testnet (Section 14). The closing test `object_layer_is_closed_on_testnet` exists. The module has 1 unwired marker in metrics.rs.
- **Why**: Correctly closed feature with a closing test.
- **What it causes**: No error. TLG-02 compliance for this feature is met.
- **How to fix**: No fix needed.

## F-0070

- **ID**: F-0070
- **Location**: src/relayer/worker.rs (1 unwired marker)
- **What is wrong**: The relayer worker has an unwired surface. The relayer is for the bridge (conditional at testnet).
- **Why**: The bridge is conditional. If it opens, the relayer must be wired.
- **What it causes**: Same as F-0033. The relayer depends on the bridge decision.
- **How to fix**: Track with TLG-27 (bridge decision).
