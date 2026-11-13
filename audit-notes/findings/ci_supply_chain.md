# CI and Supply Chain Findings

Files reviewed: .github/workflows/*.yml, supply-chain/, .cargo/audit.toml, .quality/

## F-0025: 29+ CI workflows provide comprehensive coverage

- **Location**: `.github/workflows/`
- **What is wrong**: Not a finding. The CI matrix is extensive: core CI (ci.yml, rust.yml), security (semgrep, codeql, gitleaks, security-audit, grype, osv-scanner), supply chain (cargo-vet, dependency-review, dependency-submission), quality (extra-tooling, fmt-diff-surface, typos), testing (determinism, diverse-double-compiling, miri, fuzz-nightly, kani-slow), and operational (docker-smoke, benchmark, provenance).
- **Status**: closed (informational, positive assessment)

## F-0026: No multi-OS CI workflow existed before this audit

- **Location**: `.github/workflows/`
- **What is wrong**: The directive (Section 15, Asgama 9) requires Budlum to be built and tested on many operating systems. The existing CI runs on Ubuntu only. macOS, Windows, Alpine, Fedora, and Debian builds were not in CI.
- **Why it is wrong**: Platform-specific bugs (endianness, path separator, library availability) would not be caught.
- **What it causes**: A latent portability bug could block testnet deployment on non-Ubuntu systems.
- **How to fix it**: Added `.github/workflows/testnet-multi-os.yml` with Ubuntu 24.04, 22.04, macOS, Windows, Alpine, Fedora, Debian, and ARM64 cross-compilation targets.
- **Status**: fixed (this audit PR)

## F-0027: No testnet smoke test workflow existed

- **Location**: `.github/workflows/`
- **What is wrong**: There was no CI workflow that tested the full testnet lifecycle: genesis build, node startup, RPC health check, key generation, and binary builds for all testnet tools (faucet, indexer, relayer, light client, wallet, migration kit).
- **Why it is wrong**: Without a smoke test, a broken binary target would not be caught until manual testing.
- **What it causes**: A binary that fails to build or link would only be discovered during manual testnet deployment.
- **How to fix it**: Added `.github/workflows/testnet-smoke.yml` with genesis validation, node startup, RPC health, and all binary builds.
- **Status**: fixed (this audit PR)

## F-0028: No end-to-end multi-node test workflow existed

- **Location**: `.github/workflows/`
- **What is wrong**: There was no CI workflow that tested two-node mesh connectivity, block sync, database backup/restore, and database integrity checks.
- **Why it is wrong**: Integration bugs between nodes would not be caught until manual multi-node testing.
- **What it causes**: P2P sync bugs, consensus forks, or backup corruption would only surface in manual testing.
- **How to fix it**: Added `.github/workflows/testnet-e2e.yml` with two-node mesh, backup/restore, and integrity check jobs.
- **Status**: fixed (this audit PR)

## F-0029: BADGE_PUSH_TOKEN risk acknowledged

- **Location**: `.github/workflows/` (various)
- **What is wrong**: The directive (Section 14) and audit checklist mention BADGE_PUSH_TOKEN as a known risk. The coverage badge is updated through a push token that, if compromised, could overwrite the badge with misleading data.
- **Why it is wrong**: A compromised badge token could display false coverage percentages.
- **What it causes**: Misleading quality metrics, not a security vulnerability in the protocol.
- **How to fix it**: Document the risk. Consider using a read-only badge service or signed badges.
- **Status**: open (known risk, documented)
