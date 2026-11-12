# How to build: Clients and tooling

Testnet index Section 9. Work packages IP-39 through IP-42, IP-61. TLG gates TLG-20, TLG-21, TLG-25.

---

## Current state

**Wallet**: The `bud` CLI binary is one of 8 binary targets. The wallet-core crate (`crates/wallet-core/`, 2999 lines) has V3 wallet surface unwired (F-0106) and consent screen unwired. BIP-39 wordlist is compiled into the crate.

**Explorer/Indexer**: `budlum-indexer` binary exists in `src/bin/budlum-indexer.rs`. Uses sled directly with 5 trees (F-0052). The explorer crate (`crates/budlum-explorer/`, 87 lines) provides API types.

**Faucet**: `budlum-faucet` binary exists in `src/bin/budlum-faucet.rs` with 3 TESTNET-DEFAULT markers.

**RPC**: The RPC server (`src/rpc/server.rs`, 8803 lines) has secure defaults (auth_required=true). TLS 1.3 with X25519MLKEM768. Rate limiting (120 req/min default). 40+ JSON-RPC methods.

**CI**: 36 workflow files. Nightly workflows: conformance, fuzz, air-mutants, miri, miri-storage-weekly, determinism, kani-slow. No 7-night green streak dashboard (F-0098).

## What must be built

1. **Wallet end-to-end** (IP-39, TLG-12). Verify that `bud` CLI can: generate keys, create transactions, sign with Ed25519 and ML-DSA-65, submit to a running node. If V3 wallet surface is needed, wire it (F-0106).

2. **Explorer and indexer** (IP-40, TLG-21). Verify that `budlum-indexer` indexes blocks, transactions, accounts. The indexer's sled usage (F-0052) is acceptable (not a mobile component).

3. **Faucet** (IP-41, TLG-21). Verify that `budlum-faucet` dispenses testnet BUD. Calibrate the TESTNET-DEFAULT values (drip amount, rate limit, cooldown).

4. **RPC hardening** (IP-61, TLG-20). The RPC security config defaults are good. Verify: API key authentication works, rate limiting works, CORS is configured, the operator listener is separate from the public listener, management methods are rejected on the public listener.

5. **CI green streak** (TLG-25). All nightly workflows green for 7 consecutive nights. Add a dashboard workflow (F-0098) that aggregates the status.

6. **Release artifacts** (TLG-26). Reproducible builds. SBOM. Signatures. The Dockerfile verifies toolchain version. The Docker image is digest-pinned with a Chainguard runtime base.

## Key files

- `src/bin/`: 8 binary targets
- `crates/wallet-core/`: wallet library
- `crates/budlum-explorer/`: explorer API types
- `src/rpc/server.rs`: RPC server (8803 lines)
- `src/rpc/api.rs`: RPC API definitions (1801 lines)
- `.github/workflows/`: 36 CI workflow files
- `ops/Dockerfile`: production Docker image

## Blocking findings

- F-0096: Ethereum filter log delivery missing (TLG-13)
- F-0098: no CI 7-night dashboard (TLG-25)
- F-0106: wallet V3 surface unwired
