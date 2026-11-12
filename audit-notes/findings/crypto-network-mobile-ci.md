# Findings: crypto, network, mobile, CI

## F-0039

- **ID**: F-0039
- **Location**: src/crypto/pq.rs line 1
- **What is wrong**: The PQ signature verification module is marked "WIRING: unwired". `verify_ml_dsa_single` is reached only from the PQ vector tests. The production path verifies through the batched backend dispatch.
- **Why**: The directive K-1 through K-4 (crypto invariants) require that all PQ signing surfaces are wired. The single-verification path is tested but not used in production.
- **What it causes**: If the batched verification path has a bug, the single-verification path (which passes NIST vectors) is not used. The two paths can diverge.
- **How to fix**: Verify that the batched backend dispatch covers all cases that `verify_ml_dsa_single` covers. Add a test that asserts identical results.

## F-0040

- **ID**: F-0040
- **Location**: src/crypto/tls.rs line 43
- **What is wrong**: A TLS function is marked "WIRING: unwired". TLS 1.3 with X25519MLKEM768 is the RPC transport security (src/main.rs line 1788).
- **Why**: The RPC TLS path is wired (main.rs uses `rpc::tls::acceptor_from_files`). The unwired function may be a different TLS surface.
- **What it causes**: If this is the PQ TLS surface for peer-to-peer, it means P2P does not have PQ transport security at testnet. The directive requires ML-KEM for key exchange.
- **How to fix**: Identify whether the unwired TLS function is needed for P2P transport. RPC TLS is wired.

## F-0041

- **ID**: F-0041
- **Location**: src/crypto/scheme.rs line 47-49
- **What is wrong**: `BPQS_L5` (SchemeId 0x0401) is listed as "research; reserved". It has a `SchemeStatus` entry but no implementation path.
- **Why**: BPQS is a research-level scheme. The directive does not require it for testnet.
- **What it causes**: No error. The scheme is registered but unusable. If a transaction references it, the verification path should refuse it.
- **How to fix**: Verify that the scheme status registry marks BPQS as disabled on testnet chain ID 45261.

## F-0042

- **ID**: F-0042
- **Location**: src/crypto/scheme.rs line 452
- **What is wrong**: `PQ_SCHEME_NAME` is defined as `SchemeId::ML_DSA_65.short_name()`. The genesis file carries this string in its `pq_scheme` field.
- **Why**: A node built with a different PQ backend refuses the chain at startup (src/main.rs line 412-415). This is correct behavior.
- **What it causes**: No error. This is a good safety check. A node compiled against a different ML-DSA implementation refuses a chain whose genesis names a different scheme.
- **How to fix**: No fix needed. Document this as a deliberate design decision.

## F-0043

- **ID**: F-0043
- **Location**: src/network/protocol.rs line 5
- **What is wrong**: `MAX_MESSAGE_SIZE` is 10 MiB. `MAX_BLOCK_SIZE` (transport) is 1 MiB. `MAX_CHAIN_SYNC_BLOCKS` is derived as `(10 MiB / 1 MiB) - 1 = 9`.
- **Why**: A range-sync response can carry up to 9 full-sized blocks. This is a reasonable transport limit.
- **What it causes**: No error. The derivation is correct. The comment at line 13-18 explains the relationship.
- **How to fix**: No fix needed. This is an observation for completeness.

## F-0044

- **ID**: F-0044
- **Location**: mobile/core/DB_CHOICE.md
- **What is wrong**: The mobile core stores custody shares in memory (`ShareStore`). The DB choice document says "no production phone build ships" until a crash-safe backend is measured and chosen.
- **Why**: The directive Section 7.9.5 requires mobile validators to store custody shares persistently. In-memory storage loses shares on app kill.
- **What it causes**: TLG-10 (real Android phones validate on rehearsal: store, audit, serve, earn) cannot pass if shares are lost on app restart. TLG-11 (24h service) is also blocked.
- **How to fix**: Complete the DB choice (redb or flat file + log). Implement and measure. This is a testnet blocker for mobile.

## F-0045

- **ID**: F-0045
- **Location**: mobile/core/Cargo.toml line 27
- **What is wrong**: The mobile core uses `sha3 = "0.10"` while the main crate uses `sha3 = "0.12"` (checked from Cargo.toml workspace deps).
- **Why**: The comment says it is "an implementation independent of budlum-types' sha3 0.12" for testing.
- **What it causes**: Two sha3 versions in the mobile dependency tree. The presence vector that the node verifies is computed by the mobile core. Both must produce the same hash. The version difference does not change SHA-3 output (SHA-3 is a standard), but it doubles the code size.
- **How to fix**: Unify on sha3 0.12 across all crates. The testing independence argument does not justify a version split in production.

## F-0046

- **ID**: F-0046
- **Location**: mobile/ffi/Cargo.toml (uniffi 0.32.2)
- **What is wrong**: UniFFI 0.32.2 generates the Kotlin and Swift bindings. This version must match the UniFFI version in the Android and iOS build scripts.
- **Why**: A UniFFI version mismatch between the Rust FFI crate and the Kotlin/Swift codegen produces incompatible bindings at build time.
- **What it causes**: No error if versions match. This is a build system observation. Verify version consistency across mobile/ffi, mobile/android, and mobile/ios build configurations.
- **How to fix**: Add a CI check that the UniFFI version in Cargo.toml matches the Gradle/SPM plugin version.

## F-0047

- **ID**: F-0047
- **Location**: mobile/ios/Sources/BudlumValidator/TurkishLocalization.swift
- **What is wrong**: A Turkish localization file exists in the iOS app. The directive Section 2 requires professional English naming. The app should support multiple languages but the code structure suggests hardcoded Turkish strings.
- **Why**: The concept name map (IP-05) and TLG-24 require English terms in code. Localization strings are separate from code identifiers.
- **What it causes**: If Turkish strings are in code identifiers (not just localization resources), TLG-24 fails. Localization resource files in Turkish are fine.
- **How to fix**: Verify that Turkish is only in localization resources, not in code identifiers or API surfaces.

## F-0048

- **ID**: F-0048
- **Location**: .github/workflows/ (36 workflow files)
- **What is wrong**: There are 36 CI workflow files. TLG-25 requires "CI green. All nightly workflows green for last 7 nights."
- **Why**: The CI surface is large. Workflows include: ci.yml, conformance-nightly.yml, fuzz-nightly.yml, miri.yml, miri-storage-weekly.yml, determinism.yml, kani-slow.yml, diverse-double-compiling.yml, and more.
- **What it causes**: TLG-25 requires all nightly workflows to be green for 7 consecutive nights. With 36 workflows, the probability of a flaky failure blocking the gate is high.
- **How to fix**: Identify which workflows are nightly and track their green streak. Add a dashboard or summary workflow that reports the 7-night status.

## F-0049

- **ID**: F-0049
- **Location**: src/network/peer_auth.rs (3 unwired markers)
- **What is wrong**: The peer authentication module has 3 unwired surfaces.
- **Why**: Peer authentication is required for P2P security. Unwired surfaces mean some peer auth checks are not in production.
- **What it causes**: If the unwired surfaces are authentication checks, peers may connect without full verification. TLG-18 (NAT traversal) and P2P hardening depend on peer auth.
- **How to fix**: Audit the 3 unwired markers in peer_auth.rs. Wire any that are needed for testnet P2P security.

## F-0050

- **ID**: F-0050
- **Location**: src/ai/verification_layer.rs (1 unwired), src/ai/execution/verify.rs (1 unwired), src/ai/execution/stark.rs (1 unwired), src/ai/answer_proof_bundle.rs (1 unwired)
- **What is wrong**: The AI verification layer has 4 unwired surfaces. AI on-chain inference is conditional at testnet (TLG-27).
- **Why**: The AI layer is conditional. If BLG-059 through BLG-062 close, it opens. The unwired state is expected.
- **What it causes**: No error for testnet if AI stays conditional. The decision must be recorded in the TLG report.
- **How to fix**: If AI opens for testnet, wire these surfaces. If not, document the conditional status.
