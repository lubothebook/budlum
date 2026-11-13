# Security Module Findings

Files reviewed: src/security/mod.rs, alarm_effects.rs, quantum_canary.rs, src/crypto/

## F-0011: Quantum canary module is comprehensive but lacks integration test for full alarm flow

- **Location**: `src/security/quantum_canary.rs` (56908 bytes)
- **What is wrong**: The quantum canary puzzles (P-192, P-224, Curve25519, secp256k1) are implemented. The alarm effects module handles the three alert levels (Advisory, Restricted, Isolated). However, there is no end-to-end integration test that exercises the full flow: puzzle solution submission, alarm level transition, and effect application.
- **Why it is wrong**: Individual units are tested, but the state transition from "no alarm" through "Advisory" to "Restricted" to "Isolated" as a continuous chain of events is not tested as a single scenario.
- **What it causes**: A regression in the state machine that connects puzzle verification to alarm effects would not be caught until production.
- **How to fix it**: Add an integration test in `src/tests/quantum_alarm_flow.rs` that submits a puzzle solution, verifies the alarm level changes, and checks that the alarm effects (fee multiplier, registration gate, transfer restrictions) are applied correctly.
- **Status**: open

## F-0012: Domain tag inventory audit needed

- **Location**: `src/crypto/domain_tags.rs`
- **What is wrong**: The directive (Section 2.3, Rule 7) requires that every domain tag be registered in the `domain_tags.rs` inventory and that old concept-name tags be renamed. The `domain_tags` gate checks this.
- **Why it is wrong**: Without running the gate, the inventory may be stale after the rename.
- **What it causes**: A renamed tag that is not in the inventory would fail the gate. An old tag that was not renamed would fail the concept_names gate.
- **How to fix it**: Run `cargo run --manifest-path xtask/gates/Cargo.toml -- domain_tags` and verify all tags pass.
- **Status**: open

## F-0013: PKCS#11 module path validation uses a denylist

- **Location**: `src/crypto/mainnet_policy.rs`
- **What is wrong**: `is_software_token_path` uses a denylist approach (checking for "softhsm" and "p11-kit-proxy" in the path). `is_sanctioned_module_path` uses an allowlist. The denylist approach for software tokens means a new software token implementation with a different name would not be caught.
- **Why it is wrong**: Denylist is weaker than allowlist for security decisions. However, the allowlist function (`is_sanctioned_module_path`) is the stronger check and is used for anchor keys, which are the highest-trust path.
- **What it causes**: For testnet, softhsm is explicitly allowed, so no operational impact. For mainnet, the allowlist on `is_sanctioned_module_path` is the effective gate.
- **How to fix it**: Informational. The current layered approach (denylist for general PKCS#11, allowlist for anchor keys) is reasonable. Document this design decision.
- **Status**: open (informational)

## Module Quality Assessment

The security module is well-designed. The quantum canary system is thorough and covers four classical curves. The alarm level state machine is clean. The PKCS#11 integration has both a denylist and allowlist approach, layered correctly.
