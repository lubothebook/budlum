# Findings: src/consensus, src/validators, structural

## F-0014

- **ID**: F-0014
- **Location**: src/consensus/registration.rs line 1
- **What is wrong**: The module header says "WIRING: unwired". `verify_registration` is reached only from tests. The block path validates registrations through its own inline check.
- **Why**: The directive Section 7.3 requires a single registration door. If the formal verification function is not wired into the block execution path, the inline check and the module check can diverge.
- **What it causes**: Two registration verification paths exist. One is tested, one is in production. They can disagree silently. TLG-05 (validator network) depends on correct registration.
- **How to fix**: Wire `verify_registration` into the block execution path and remove the inline duplicate. Or verify that both paths are identical and add a test that asserts they produce the same result for the same inputs.

## F-0015

- **ID**: F-0015
- **Location**: 39 files across src/ (see grep for "WIRING: unwired")
- **What is wrong**: At least 39 source locations are marked "WIRING: unwired". These are declared API surfaces with no production callers. They exist only in tests or not at all.
- **Why**: The codebase marks future surfaces explicitly. This is good practice for tracking. But the count (39+) is high for a testnet-ready codebase.
- **What it causes**: Untested-in-production code paths. If any of these surfaces are required for testnet (e.g. PQ anchor verification, bridge verification, state recovery tick), they are not wired. TLG items that depend on these paths cannot pass.
- **How to fix**: Prioritize wiring for surfaces on the testnet critical path. Mark the rest as post-launch. The unwired list should be cross-referenced with TLG gates.

## F-0016

- **ID**: F-0016
- **Location**: src/consensus/mod.rs line 471-492
- **What is wrong**: `validate_block_size` serializes the entire block to JSON (`serde_json::to_vec`) to measure its size. This happens on every block validation.
- **Why**: The consensus MAX_BLOCK_SIZE (1,000,000 bytes) is measured over JSON, while the transport MAX_BLOCK_SIZE (1,048,576 bytes) is measured over protobuf. The comment at line 261-279 explains the ordering. But the serialization is expensive.
- **What it causes**: Every received block is serialized to JSON during validation. On a mobile validator with limited CPU, this is a performance concern. The block is already deserialized from protobuf; re-serializing to JSON to measure size is redundant work.
- **How to fix**: Consider measuring against the protobuf encoding that already exists at the transport layer, or caching the serialized form. This is a performance observation, not a correctness bug.

## F-0017

- **ID**: F-0017
- **Location**: src/consensus/mod.rs line 100 (RoundPriceBook)
- **What is wrong**: `RoundPriceBook` uses `std::sync::Mutex<VecDeque<(String, u128)>>`. Block hashes are stored as owned `String`.
- **Why**: The round price book is on the consensus hot path. `String` allocations for block hashes on every commit add heap pressure.
- **What it causes**: Minor allocation overhead per block commit. Not a correctness issue. Relevant for mobile validators with constrained memory.
- **How to fix**: Consider using a fixed-size hash representation (`[u8; 32]`) instead of `String`. This is a quality observation.

## F-0018

- **ID**: F-0018
- **Location**: src/validators/mod.rs (entire file, 822 lines)
- **What is wrong**: The `ValidatorDirectory` serializes itself with `bincode::serialize` to compute its state root (line 731). The root is recomputed on every access.
- **Why**: The directive invariant D-1 requires that state roots are deterministic. bincode serialization is deterministic for the same Rust type layout, but the `bincode` crate is version 1.x (held deliberately per Cargo.toml comment). A bincode 2.x migration would change the root.
- **What it causes**: The validator directory root is coupled to the bincode 1.x wire format. This is intentional but fragile. The osv-scanner.toml notes bincode 1.x is unmaintained (RUSTSEC-2025-0141).
- **How to fix**: Document the bincode 1.x dependency as a consensus-critical wire format. Consider a migration plan as part of the bincode 2.x work. This is a known tracked dependency.

## F-0019

- **ID**: F-0019
- **Location**: src/validators/handover.rs (entire module)
- **What is wrong**: The module header says "WIRING: `Blockchain` owns `RecoveryTracker` and PoS calls `proposer_round_for_timestamps`. The seat, custody, repair and presence helpers remain drill/churn utilities until their state-transition callers adopt them."
- **Why**: The Instant Task Handover (AGD, Section 7.6) is a TLG-06 requirement. The code exists but the state-transition callers are not wired.
- **What it causes**: TLG-06 (instant task handover) cannot pass if the handover logic is not wired into the block execution path. The rehearsal drills T-3 through T-7 depend on it.
- **How to fix**: Wire the handover state machine into `Blockchain`'s block commit path. The constants and types are defined; the integration is missing.

## F-0020

- **ID**: F-0020
- **Location**: src/validators/committee.rs
- **What is wrong**: The committee module implements seat allocation with largest remainder and standby lists. The `COMMITTEE_MAX_SEATS` is 128 and `SEAT_STANDBY_DEPTH` is 3. Both are marked "TESTNET-DEFAULT: governance replaces this once measured."
- **Why**: These are placeholder values. The directive Section 7.4.2 defines the committee structure but the testnet parameters need measurement from the rehearsal network.
- **What it causes**: No direct error. The constants work for testnet. But the governance replacement mechanism is not yet visible in the code (no on-chain parameter update path for committee size).
- **How to fix**: Verify that the rehearsal network (IP-46) exercises these values. Document the governance replacement path.

## F-0021

- **ID**: F-0021
- **Location**: src/consensus/handoff.rs
- **What is wrong**: `COMMITTEE_SIZE` for the Handoff Certificate is 512. This is distinct from `COMMITTEE_MAX_SEATS` (128) in src/validators/committee.rs.
- **Why**: The Handoff Certificate committee (PQ signing committee for epoch transitions) uses a different size from the finality committee (seat-based voting).
- **What it causes**: Two "committee" concepts with different sizes. A reader may confuse them. The naming overlap between `handoff::COMMITTEE_SIZE` and `committee::COMMITTEE_MAX_SEATS` is a documentation issue.
- **How to fix**: Add a comment or type alias to distinguish the two. Example: `HANDOFF_PQ_COMMITTEE_SIZE` vs `FINALITY_SEAT_COUNT`.

## F-0022

- **ID**: F-0022
- **Location**: src/registry/handover.rs line 1, line 48
- **What is wrong**: This file uses the old Turkish abbreviation "AGD" (Anlik Gorev Devri) in comments. The directive Section 2 and Appendix C require the English term "Instant Task Handover".
- **Why**: The concept name map (IP-05, TLG-24) requires all old names to be replaced.
- **What it causes**: TLG-24 (concept name gate in absolute mode, green) cannot pass if old names remain. The counter gate counts every old name occurrence.
- **How to fix**: Replace "AGD" with "Instant Task Handover" in all comments. Run the concept name counter gate.

## F-0023

- **ID**: F-0023
- **Location**: src/consensus/filiz.rs (entire module), src/main.rs lines 976-1060
- **What is wrong**: The leader election mechanism is called "Filiz Chain" throughout the codebase. The directive Appendix C lists this as a concept that needs review for the professional English name map.
- **Why**: "Filiz" is a proper noun (Turkish origin). The directive requires professional English naming for all public-facing concepts.
- **What it causes**: If "Filiz Chain" is in the old-name list (Appendix C), TLG-24 fails. The module name `filiz.rs`, the environment variable `BUDLUM_FILIZ_SEED_FILE`, and all API surfaces carry this name.
- **How to fix**: Check whether "Filiz Chain" is in the concept name map. If it is a rename target, the rename touches: module name, env var, struct names, domain tags (`BDLM_FILIZ_ELIG_V1`, `BDLM_FILIZ_EPOCH_V1`), and all references.

## F-0024

- **ID**: F-0024
- **Location**: src/consensus/mod.rs line 62-67
- **What is wrong**: `now_millis()` uses `SystemTime::now()` for consensus-critical round pricing. The comment at line 57 explains it is the wall clock.
- **Why**: System clocks can jump. NTP adjustments, VM snapshots, and suspend/resume can move the clock forward or backward. The round pricing depends on monotonic time progression.
- **What it causes**: A clock jump backward makes all rounds unaffordable (safe but stalls). A clock jump forward makes all rounds affordable at once (enables wide round claims). The code acknowledges this risk at line 234-238 and relies on fork choice rather than the clock to resolve it.
- **How to fix**: This is a known design trade-off, not a bug. Document it in the operator runbook. Consider adding a monotonic clock check or drift alarm as a metric.

## F-0025

- **ID**: F-0025
- **Location**: src/error.rs (entire file)
- **What is wrong**: `BudlumError` variants are: Storage, Consensus, Settlement, Bridge, Network, Rpc, Validation. Missing: Execution, Validator, Crypto, Privacy.
- **Why**: The codebase has modules for execution (UEL), validators, crypto, and privacy, each with their own error types. The central error type does not cover them.
- **What it causes**: Error handling is fragmented. Each module defines its own error type. Cross-module error propagation requires ad-hoc conversions. This is a design observation.
- **How to fix**: Add missing variants as UEL, validator, and crypto modules mature. Not a testnet blocker if each module handles its own errors correctly.
