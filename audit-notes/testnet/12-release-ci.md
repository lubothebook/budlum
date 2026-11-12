# How to build: Release and CI

Testnet index Section 12. TLG gates TLG-25, TLG-26.

---

## Current state

36 CI workflow files exist in `.github/workflows/`. These include: ci.yml, conformance-nightly.yml, fuzz-nightly.yml, miri.yml, miri-storage-weekly.yml, determinism.yml, kani-slow.yml, diverse-double-compiling.yml, docker-smoke.yml, security-audit.yml, security-hardening.yml, semgrep.yml, semver.yml, solidity.yml, supply-chain-extra.yml, testnet-rehearsal.yml, and others.

The Dockerfile (`ops/Dockerfile`) is well-hardened: digest-pinned base images, non-root user, toolchain version gate, Chainguard runtime base.

The build produces 4 binaries in the Docker image: budlum-core, bud, budlum-faucet, budlum-indexer.

## What must be built

1. **7-night green streak** (TLG-25). All nightly workflows must be green for 7 consecutive nights. Current nightly workflows: conformance-nightly.yml, fuzz-nightly.yml, air-mutants-nightly.yml. Weekly: miri-storage-weekly.yml. Slow: kani-slow.yml. Add a dashboard workflow that reports the streak status (F-0098).

2. **Reproducible builds** (TLG-26). The Dockerfile verifies the Rust toolchain version at build time. The Cargo.lock is committed. The `--locked` flag is used. Verify that two builds from the same commit produce identical binaries (or document the delta if compiler non-determinism exists).

3. **SBOM** (TLG-26). Generate a Software Bill of Materials for the release. The `cargo-vet` workflow (`cargo-vet.yml`) and `dependency-submission.yml` contribute to supply chain visibility. The `provenance.yml` workflow generates provenance attestations.

4. **Release signatures** (TLG-26). The `release-signature.yml` workflow handles signing. Verify the signing key is configured and the signatures are published alongside the release artifacts.

5. **Pin Grafana image** (F-1201). The testnet-local.yml uses `grafana/grafana:11.2.0` without a digest pin. Add `@sha256:<digest>` matching the pattern used for Prometheus.

## Key files

- `.github/workflows/`: 36 CI workflow files
- `ops/Dockerfile`: production Docker image
- `ops/testnet-local.yml`: local testnet with monitoring
- `Cargo.lock`: dependency lockfile
- `supply-chain/`: cargo-vet audits
- `.quality/`: deny.toml, typos.toml, semgrep rules

## Blocking findings

- F-0098: no CI 7-night dashboard
- F-1201: Grafana image without digest pin
- F-1208: fuzz workspace libp2p rev may diverge
