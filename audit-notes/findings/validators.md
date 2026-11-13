# Validator Module Findings

Files reviewed: mod.rs, committee.rs, handover.rs

## F-0005: Mobile validator quota ceiling constant only re-exported

- **Location**: `src/validators/mod.rs:34`
- **What is wrong**: `MOBILE_OPPORTUNISTIC_MAX_QUOTA_BYTES` is re-exported from `budlum_types` but there is no enforcement in the validator registration transaction handler that checks the proposed quota against this ceiling for MobileOpportunistic class validators.
- **Why it is wrong**: The constant exists (K-4 compliance) but the check is missing from the state transition function. A MobileOpportunistic validator could register with an arbitrarily large custody quota.
- **What it causes**: On testnet, an iOS opportunistic validator could claim more storage than the class ceiling allows, breaking the resource planning invariant.
- **How to fix it**: Add a quota ceiling check in the `ValidatorTx::Register` handler when the class is `MobileOpportunistic`. The check compares the proposed quota against `MOBILE_OPPORTUNISTIC_MAX_QUOTA_BYTES` and rejects with `QUOTA_EXCEEDS_CLASS_CEILING`.
- **Status**: open

## F-0006: Seat handover divisor can produce zero max handovers

- **Location**: `src/validators/handover.rs:49`
- **What is wrong**: `SEAT_MAX_HANDOVER_DIVISOR` is 10. For a committee with fewer than 10 seats, `ceil(S / 10)` rounds up to 1 correctly. But if the committee has 0 seats (an impossible state, but one that should be guarded against), `ceil(0 / 10)` is 0 and no handover can happen.
- **Why it is wrong**: The code does not check for an empty committee before computing the divisor. Division by zero is not the issue (the divisor is 10, not S), but a zero result means the handover loop body never executes.
- **What it causes**: No impact in practice because a committee with 0 seats cannot exist in a valid chain state. Informational finding.
- **How to fix it**: Add a `debug_assert!(!committee.is_empty())` at the top of `try_seat_handover`.
- **Status**: open (informational)

## F-0007: Recovery committee derivation is documented but partially wired

- **Location**: `src/validators/handover.rs:26-28`
- **What is wrong**: The WIRING comment on line 26 states: "The seat, custody, repair and presence helpers remain drill/churn utilities until their state-transition callers adopt them." This means the disaster recovery committee derivation (Section 7.7 of the directive) is implemented as utility functions but not yet called from the production blockchain state transition.
- **Why it is wrong**: For testnet, if finality stalls for 40 blocks (FINALITY_STALL_BLOCKS), nodes should automatically derive a recovery committee. The code to derive the committee exists but is not wired to the block-processing pipeline.
- **What it causes**: On testnet, a finality stall lasting more than 40 blocks has no automatic recovery mechanism. The node raises the `budlum_finality_stalled` metric but does not act on it.
- **How to fix it**: Wire `RecoveryTracker::try_derive_recovery_committee` into `Blockchain::apply_block` at the checkpoint boundary, after checking `finality_stall_blocks >= FINALITY_STALL_BLOCKS`.
- **Status**: open

## Module Quality Assessment

The validator module is well-structured. The class system (Full, Mobile, MobileOpportunistic) is clean and the single-registration-door pattern prevents class confusion. The committee and handover code is production-grade with correct quorum math. The disaster recovery logic is sound but needs production wiring.
