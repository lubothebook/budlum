# Findings: missing CI gates and protections for testnet launch

This file lists every CI gate, workflow, or automated check that the TLG
(Testnet Launch Gate) and the directive require but the repository does not
have yet. Each finding is a gap that must be closed before testnet.

## F-0132

- **ID**: F-0132
- **Location**: .github/workflows/ (all 36 files)
- **What is wrong**: No CI 7-night streak dashboard exists. TLG-25 requires all nightly workflows green for 7 consecutive nights. The nightly workflows exist (conformance-nightly, fuzz-nightly, air-mutants-nightly, watchdog every 6h, Kani weekly, Miri weekly) but no workflow or gate measures or reports the streak.
- **Why**: The nightly workflows report pass/fail per run but do not aggregate across days.
- **What it causes**: TLG-25 cannot be verified automatically. An operator must manually check 7 days of CI history across multiple workflows.
- **How to fix**: Add a `nightly-streak.yml` workflow that runs daily, reads the status of all nightly/scheduled workflows for the past 7 calendar days, and reports the streak. The gate should fail when any night in the window is red.

## F-0133

- **ID**: F-0133
- **Location**: .github/workflows/, xtask/gates/src/gates/
- **What is wrong**: No CI gate enforces that closing tests exist for every feature listed as "Closed" or "Conditional" in the testnet feature table (Section 14 of the directive). The `gating_flags.rs` gate exists and checks gating flags, but there is no gate that maps the Section 14 feature list to the `*_is_closed_on_testnet` tests.
- **Why**: The 28 closing tests (F-0058) were added individually. No automated inventory compares the directive's feature table against the test list.
- **What it causes**: A feature can be listed as "Closed" in the directive without a corresponding closing test, or a closing test can exist for a feature not in the directive list. TLG-02 requires coverage. The gap is invisible without manual cross-referencing.
- **How to fix**: Add a gate `closing_tests_cover_feature_table.rs` that reads the list of closed/conditional features from a configuration file (or the testnet feature table source) and verifies that each one has a matching `*_is_closed_on_testnet` test in the test log.

## F-0134

- **ID**: F-0134
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow verifies reproducible binary builds (TLG-26). The `determinism.yml` workflow checks genesis hash reproducibility but does not check binary reproducibility. Two builds from the same commit may produce different binaries due to compiler nondeterminism, timestamps, or build-path leakage.
- **Why**: Binary reproducibility is a separate property from genesis reproducibility. The Dockerfile hardening (digest-pinned bases, locked dependencies, toolchain gate) sets up the preconditions but does not verify the outcome.
- **What it causes**: TLG-26 requires reproducible builds. Without this check, the release pipeline cannot prove that a published binary matches the audited source.
- **How to fix**: Add a job to `determinism.yml` (or a new `reproducible-binary.yml`) that builds the Docker image twice from the same commit and compares the binary hashes. If they differ, the job must report the delta (embedded timestamps, build paths, etc.).

## F-0135

- **ID**: F-0135
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow runs third-party tool conformance tests (TLG-13). TLG-13 requires: Foundry, Hardhat, viem, ethers (EVM tools); CosmJS (Cosmos); Anchor and solana CLI (SVM); aptos CLI (Move). The `conformance-nightly.yml` runs UEL conformance suites (gas calibration, execution root) but not the external tool compatibility tests.
- **Why**: The external tools require running external tool frameworks against a testnet node, which is more complex than a cargo test.
- **What it causes**: An incompatibility with a standard tool (e.g. Hardhat deployment, ethers.js transaction formatting) would not be caught until manual testing.
- **How to fix**: Add a `tool-conformance.yml` that boots a devnet node, runs a minimal test suite for each tool, and reports pass/fail. The test can start with the EVM tools (Foundry, Hardhat, viem, ethers) as these have the most users.

## F-0136

- **ID**: F-0136
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow tests cross-EE atomic calls (TLG-14). TLG-14 requires proving that an EVM contract can call a CosmWasm contract atomically in the same transaction. The `conformance-nightly.yml` runs per-EE conformance but not cross-EE calls.
- **Why**: Cross-EE atomicity requires a running UEL with multiple execution environments active.
- **What it causes**: TLG-14 cannot be verified automatically. A regression in the cross-EE dispatch path would not be caught.
- **How to fix**: Add a test to the conformance suite that deploys an EVM contract and a CosmWasm contract, makes an atomic cross-call, and verifies both state changes are committed or rolled back together.

## F-0137

- **ID**: F-0137
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow tests NAT traversal (TLG-18). TLG-18 requires relay, DCUtR, and AutoNAT working. The `network_hardening_gate` checks network security parameters but not NAT functionality.
- **Why**: NAT traversal requires network topology simulation (multiple nodes behind NAT, relay nodes).
- **What it causes**: A regression in NAT traversal would not be caught until manual multi-node testing or rehearsal.
- **How to fix**: Add a test in the testnet-rehearsal workflow that boots nodes behind simulated NAT and verifies they can discover and communicate through relay nodes.

## F-0138

- **ID**: F-0138
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow tests the mobile validator end-to-end (TLG-10, TLG-11). The `mobile.yml` and `mobile-ios.yml` workflows build and test the mobile crate, but do not run a mobile validator against a testnet node. TLG-10 requires real Android phones validating on rehearsal. TLG-11 requires device matrix results.
- **Why**: Real phone testing requires hardware or emulators and a running testnet.
- **What it causes**: The mobile build compiles and passes unit tests, but actual mobile validation (store, audit, serve, earn) is untested in CI. The `rehearsal_and_field_matrix.rs` gate checks for the existence of docs/MOBILE_DEVICE_MATRIX.md but not the actual device results.
- **How to fix**: Add Android emulator testing to `mobile.yml` that connects a mobile validator to a devnet node and runs basic validation operations. Real-device testing on rehearsal is out of CI scope but must happen before TLG-10 sign-off.

## F-0139

- **ID**: F-0139
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow tests validator departure scenarios (TLG-07, TLG-08, TLG-09). TLG-07 requires all mobile validators depart with no object lost. TLG-08 requires 10% of full validators depart with finality delay at most 2 checkpoints. TLG-09 requires rehearsal drills T-3, T-4, T-5, T-7 passing.
- **Why**: Departure scenarios require multi-node orchestration with controlled shutdown.
- **What it causes**: The `rehearsal_and_field_matrix.rs` gate checks for the existence of the drill evaluator function (`evaluate_rehearsal_drills`) but does not run it. The actual departure behavior is untested in CI.
- **How to fix**: Add a departure drill to the testnet-rehearsal workflow. The native E2E job already boots a multi-node testnet. Add steps that stop mobile validators, verify no object is lost, then stop 10% of full validators and measure finality delay.

## F-0140

- **ID**: F-0140
- **Location**: xtask/gates/src/gates/
- **What is wrong**: No gate enforces that the concept name count is zero (TLG-24). The `concept_names.rs` gate exists and is in "count" (ratchet) mode. TLG-24 requires "absolute" mode with zero counts. The gate itself documents this transition but cannot enforce it until the renames are complete.
- **Why**: The renames (759 references) are in progress. The gate mode switch is a manual step.
- **What it causes**: Until the gate switches to "absolute" mode, CI will not enforce TLG-24. The switch can only happen after all renames are done.
- **How to fix**: Complete the concept name renames. Lower the baseline to zero. Switch the mode from "count" to "absolute" in `config/naming/concept_map.toml`.

## F-0141

- **ID**: F-0141
- **Location**: docs/audit/TLG_REPORT.md (does not exist), docs/audit/FINDINGS.md (does not exist), docs/audit/LIVE_STATUS.md (does not exist)
- **What is wrong**: The `launch_evidence.rs` gate requires three audit documents: TLG_REPORT.md (27 TLG gate verdicts), FINDINGS.md (per-finding evidence), and LIVE_STATUS.md (live status dashboard). None of these files exist yet. The gate will fail on every CI run that invokes it.
- **Why**: These are outputs of the audit and review process, not source code. They must be written before launch.
- **What it causes**: The launch evidence gate (`launch_evidence.rs`) cannot pass. It checks for presence, shape, and per-gate evidence rows. Without these files, TLG-01 (no open Critical/High findings) cannot be signed off.
- **How to fix**: Create the three files in `docs/audit/`. The TLG_REPORT.md must have one row per TLG gate (TLG-01 through TLG-27) with verdict and evidence. FINDINGS.md must have one row per finding (F-001 through F-NNN) with severity and disposition. LIVE_STATUS.md must have the rehearsal and operational status.

## F-0142

- **ID**: F-0142
- **Location**: .github/workflows/ci.yml (coverage job), xtask/gates/
- **What is wrong**: No CI gate enforces a minimum test coverage threshold. The `coverage.rs` gate file exists (xtask/gates/src/gates/coverage.rs) and the CI has a coverage job, but the gate only measures and reports coverage, it does not enforce a floor. There is no "coverage must be above X%" gate.
- **Why**: The coverage is measured with tarpaulin/llvm-cov but the threshold is informational.
- **What it causes**: Test coverage can decrease without CI failing. A module with zero tests can be merged if its gate canary passes. The independent review (TLG-22) must assess coverage manually.
- **How to fix**: Set a minimum coverage threshold (a ratchet, not an absolute, to avoid blocking all PRs). The existing `coverage.rs` gate can be extended to read a baseline file and fail when coverage drops.

## F-0143

- **ID**: F-0143
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow tests the SBOM against known vulnerabilities. The `dependency-audit` job generates a CycloneDX SBOM and uploads it as an artifact, but does not validate the SBOM against a vulnerability database. The `cargo audit` check runs against the Cargo.lock directly. The SBOM is produced but not consumed by any verification step.
- **Why**: SBOM generation and vulnerability scanning are separate operations. The workflow generates the artifact for compliance but does not close the loop.
- **What it causes**: TLG-26 asks for supply chain visibility. The SBOM is visible but not verified. A dependency with a known vulnerability could pass cargo audit (if the advisory is format-specific) but show up in the SBOM.
- **How to fix**: Add a `grype` or `trivy sbom` scan step after the SBOM generation that reads the `sbom.cdx.json` and fails on high/critical findings. The `grype.yml` workflow already exists but runs against the filesystem, not the SBOM.

## F-0144

- **ID**: F-0144
- **Location**: .github/workflows/, config/
- **What is wrong**: No CI gate validates the testnet.toml, mainnet.toml, and devnet.toml configuration files against a schema. The genesis schema gate (`genesis_schema.rs`) validates genesis JSON but the TOML node configuration files have no schema or gate. A typo in a config key (e.g. `auth_requied` instead of `auth_required`) would not be caught.
- **Why**: The TOML configs are read by serde deserialization at runtime. Invalid keys are silently ignored by default unless `#[serde(deny_unknown_fields)]` is on every struct.
- **What it causes**: An operator config typo would not be caught until runtime. The testnet.toml is 96 lines and the mainnet.toml is similar. A silent key drop could disable security (e.g. auth_required not taking effect because the key is misspelled).
- **How to fix**: Add a gate that deserializes every `config/*.toml` file against the expected struct and fails on unknown or invalid fields. Alternatively, ensure `#[serde(deny_unknown_fields)]` is on every config struct used by the node.

## F-0145

- **ID**: F-0145
- **Location**: .github/workflows/, xtask/gates/
- **What is wrong**: No CI gate verifies that the `ALLOCATION_KIND` domain tags in the Solidity contracts match the Rust domain tags. The `domain_tags.rs` gate inventories Rust domain tags but does not cross-reference them against the Solidity contract constants. The shared_security module has `ALLOCATION_KIND = b"budlum/mikoriza/allocation/v1"` in Rust and `bytes internal constant ALLOCATION_KIND = "budlum/mikoriza/allocation/v1"` in Solidity. These are manually kept in sync.
- **Why**: Rust and Solidity are separate compilation targets. No automated cross-check exists.
- **What it causes**: A domain tag change on one side without the other breaks the settlement protocol. The Graft contracts also have `FULFILLED_KIND = "budlum/graft/fulfilled/v1"` which must match the Rust side. A mismatch would cause proof verification failures on the Ethereum side.
- **How to fix**: Extend the `domain_tags.rs` gate to also scan `.sol` files for domain tag constants and verify they match the Rust inventory.

## F-0146

- **ID**: F-0146
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow verifies that release signatures are valid (TLG-26). The `release-signature.yml` workflow exists but runs on push/PR to main, not on a release event. There is no post-release verification that the signature matches the published binary.
- **Why**: The signing and verification are separate steps. The workflow signs during CI but the verification is not automated.
- **What it causes**: A corrupted or tampered release binary would not be caught by CI. The signature exists but nobody verifies it automatically.
- **How to fix**: Add a verification step to `release-signature.yml` that downloads the signed artifact and verifies the signature before the artifact is published.

## F-0147

- **ID**: F-0147
- **Location**: src/domain/storage_deal.rs (11 references)
- **What is wrong**: 11 WIRING comments in storage_deal.rs use "ADIM" (Turkish for "step") to refer to block effect pipeline phases. Examples: "ADIM 4b", "ADIM 4c". This is a deprecated Turkish concept name not listed in the concept_map.toml.
- **Why**: The WIRING comments were written using the original Turkish architecture terminology.
- **What it causes**: The concept name gate does not catch "ADIM" because it is not in the concept map. Non-Turkish readers cannot understand what "ADIM 4b" refers to.
- **How to fix**: Add "ADIM" to `config/naming/concept_map.toml` with new_code "phase" or "step". Replace "ADIM 4b" with "phase 4b" or "block_effects step 4b" in the 11 WIRING comments.

## F-0148

- **ID**: F-0148
- **Location**: .github/workflows/
- **What is wrong**: No CI workflow tests that the RPC authentication actually refuses unauthenticated operator calls (TLG-20). The `network_hardening_gate` checks configuration parameters and the `security_parameters.rs` gate checks security constants, but no test actually starts a node, sends an unauthenticated operator RPC call, and verifies it is refused.
- **Why**: RPC refusal is a runtime behavior requiring a running node.
- **What it causes**: A regression in the RPC auth middleware would not be caught until manual testing. The testnet.toml sets `operator_auth_required = true` and `auth_required = false`, but this is a configuration that must be tested at runtime, not just checked as a file.
- **How to fix**: Add an RPC security test to the testnet-rehearsal or a dedicated integration test that boots a node with testnet config and verifies: (1) public RPC allows reads without auth, (2) operator RPC refuses without API key, (3) operator RPC accepts with correct API key.
