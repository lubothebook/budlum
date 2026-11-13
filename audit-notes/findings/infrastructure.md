# Infrastructure Findings

Files reviewed: ops/, config/, .github/workflows/, Cargo.toml

## F-0083: Docker images use multi-stage builds

- **Location**: `ops/docker/Dockerfile.testnet`, `ops/docker/Dockerfile.faucet`
- **What is wrong**: Not a finding. Both Docker images use multi-stage builds: rust:1.97-bookworm for building, debian:bookworm-slim for runtime. This minimizes image size and attack surface.
- **Status**: closed (informational, positive assessment)

## F-0084: Kubernetes manifests support StatefulSet

- **Location**: `ops/k8s/testnet-deployment.yml`
- **What is wrong**: Not a finding. The validator deployment uses StatefulSet with PersistentVolumeClaims for data persistence. Each validator gets its own 50Gi volume.
- **Status**: closed (informational, positive assessment)

## F-0085: Health checks are configured at all layers

- **Location**: Docker HEALTHCHECK, K8s liveness/readiness probes, ops/scripts/testnet-healthcheck.sh
- **What is wrong**: Not a finding. Health checks are configured at Docker level (30s interval), Kubernetes level (liveness: 30s, readiness: 10s), and script level for manual verification.
- **Status**: closed (informational, positive assessment)

## F-0086: Prometheus alerting covers critical scenarios

- **Location**: `ops/prometheus-alerts-testnet.yml`
- **What is wrong**: Not a finding. Eight alert rules cover: node down, block production stalled, high finality gap, low peer count, high memory, quantum alarm triggered, consensus round timeout, and validator set shrinkage.
- **Status**: closed (informational, positive assessment)

## F-0087: CI pipeline covers 5 workflow files

- **Location**: `.github/workflows/testnet-*.yml`
- **What is wrong**: Not a finding. Five testnet-specific workflows: multi-OS build, smoke test, E2E test, audit test suites, and security scan. These complement the 29+ existing CI workflows.
- **Status**: closed (informational, positive assessment)
