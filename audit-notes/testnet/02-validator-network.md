# How to build: Validator network

Testnet index Section 2. Work packages IP-08, IP-12, IP-13, IP-14. TLG gates TLG-05 through TLG-11.

---

## Current state

The validator directory (`src/validators/mod.rs`, 822 lines) supports Full, Mobile, and MobileOpportunistic classes. Committee-based finality (`src/validators/committee.rs`) implements seat allocation with largest remainder. COMMITTEE_MAX_SEATS=128, SEAT_STANDBY_DEPTH=3 (both TESTNET-DEFAULT). Instant Duty Handover (`src/validators/handover.rs`) has the state machine, recovery tracker, and disaster mode constants (FINALITY_STALL_BLOCKS=40).

The handover state-transition callers are not wired (F-0019). The registration module (`src/consensus/registration.rs`) has an unwired verification path (F-0014). The committee Handoff Certificate uses COMMITTEE_SIZE=512 (distinct from finality COMMITTEE_MAX_SEATS=128, F-0021).

## What must be built

1. **Wire Instant Task Handover** (IP-14, TLG-06). Connect the handover state machine in `src/validators/handover.rs` to `Blockchain`'s block commit path. The constants and types are defined. The duty kinds (block proposal, finality seat, custody shares, repair tasks) need callers in the epoch transition and block production paths.

2. **Wire registration verification** (F-0014). Either wire `verify_registration` from `src/consensus/registration.rs` into the block execution path or verify that the inline check in the block path is identical and add an equivalence test.

3. **Presence protocol and suspension** (IP-13). The liveness tracker exists (`src/core/account.rs` liveness module, `src/validators/liveness.rs`). Verify the suspension path works end-to-end: a validator that misses blocks is jailed, loses its seat, and the handover redistributes its duties.

4. **Rehearsal drills** (TLG-09). T-3 (all mobile depart), T-4 (10% full depart), T-5 (mixed departure), T-7 (recovery from disaster mode). These must run on the rehearsal network. The handover wiring (step 1) is a prerequisite.

5. **Mobile validator field test** (TLG-10, TLG-11). At least 20 real Android phones validate on rehearsal: store, audit, serve, earn. 24h service test. Requires the mobile persistence fix (F-0044) and the mobile core (`mobile/core/`).

## Key files

- `src/validators/mod.rs`: ValidatorDirectory, registration, classes
- `src/validators/committee.rs`: seat allocation, standby
- `src/validators/handover.rs`: Instant Task Handover, disaster mode
- `src/consensus/registration.rs`: dual-signature key binding (unwired)
- `src/consensus/handoff.rs`: Handoff Certificate, PQ committee
- `src/chain/blockchain.rs` lines 3226+: finality weight calculation
- `mobile/core/`: mobile validator core library

## Blocking findings

- F-0014: registration verification unwired
- F-0019: Instant Task Handover unwired
- F-0022: old name "AGD" in handover
- F-0044: mobile share store in-memory
