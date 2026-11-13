# Work Packages (IP-01 → IP-64)

Tracks the 64 work packages from the master directive.
Each package has an agent lane (A–D) and a dependency set.

## Critical Path

IP-07 → IP-10 → IP-18 → IP-20 → IP-21 → IP-11 → IP-36 → IP-37 → IP-46 → IP-43

## Status Legend

- `DONE` — merged to main / on fix/audit-findings
- `WIP` — in progress
- `OPEN` — not started
- `BLOCKED` — waiting on a dependency

## Lane A — Core / Storage / Crypto

| WP | Title | Status | Depends |
|----|-------|--------|---------|
| IP-01 | B.U.D. 3.0 sole storage invariant (D-1..D-15) | WIP | — |
| IP-02 | Remove B.U.D. 1.0/2.0 dead storage paths | WIP | IP-01 |
| IP-03 | RFC 6330 RaptorQ fountain code integration | DONE | — |
| IP-04 | Content addressing (ContentId, ContentRef) | DONE | IP-03 |
| IP-05 | Custody proof generation and verification | DONE | IP-04 |
| IP-06 | Segment repair protocol | OPEN | IP-05 |
| IP-07 | ValidatorClass: Full / Mobile / MobileOpportunistic | DONE | — |
| IP-08 | ML-DSA-65/87 key derivation and rotation | DONE | — |
| IP-09 | Composite signature (BLS + ML-DSA) | DONE | IP-08 |
| IP-10 | Quantum canary alarm (Kiragi/Don/Zemheri → QuantumAlertLevel) | WIP | IP-09 |
| IP-11 | Post-quantum anchor (ML-DSA-87 root seal) | DONE | IP-10 |
| IP-12 | HPKE key encapsulation for grants | DONE | IP-08 |

## Lane B — Execution / Settlement

| WP | Title | Status | Depends |
|----|-------|--------|---------|
| IP-13 | Universal Execution Layer coordinator | DONE | — |
| IP-14 | EVM runtime (revm integration) | DONE | IP-13 |
| IP-15 | WASM runtime (wasmi/CosmWasm) | DONE | IP-13 |
| IP-16 | SVM runtime stub | OPEN | IP-13 |
| IP-17 | Move runtime stub | OPEN | IP-13 |
| IP-18 | BudZero native VM | WIP | IP-13 |
| IP-19 | Gas metering unification | OPEN | IP-14, IP-15 |
| IP-20 | AppBinding (was Graft) settlement | DONE | — |
| IP-21 | Proof market settlement | DONE | IP-20 |
| IP-22 | Relayer intent settlement | DONE | IP-20 |
| IP-23 | Mikoriza restaking integration | DONE | IP-20 |
| IP-24 | Bridge shared security | DONE | IP-22 |

## Lane C — Networking / Consensus / Ops

| WP | Title | Status | Depends |
|----|-------|--------|---------|
| IP-25 | Testnet genesis (chain_id 45261) | DONE | — |
| IP-26 | Bootstrap peer discovery | DONE | IP-25 |
| IP-27 | Devnet docker-compose | DONE | IP-25 |
| IP-28 | Kubernetes StatefulSet manifests | DONE | IP-27 |
| IP-29 | Prometheus + Alertmanager monitoring | DONE | IP-28 |
| IP-30 | Grafana dashboard | DONE | IP-29 |
| IP-31 | Faucet service | DONE | IP-25 |
| IP-32 | CI: multi-OS build matrix | DONE | — |
| IP-33 | CI: smoke test workflow | DONE | IP-32 |
| IP-34 | CI: E2E test workflow | DONE | IP-33 |
| IP-35 | CI: security scan workflow | DONE | — |
| IP-36 | Stress test harness | DONE | IP-25 |

## Lane D — Documentation / Audit / Governance

| WP | Title | Status | Depends |
|----|-------|--------|---------|
| IP-37 | Concept name migration (Section 2.2) | WIP | — |
| IP-38 | Invariant gate tests (G-1..G-8) | DONE | — |
| IP-39 | Storage invariant tests (D-1..D-15) | WIP | IP-01 |
| IP-40 | Validator invariant tests (V-1..V-5) | DONE | IP-07 |
| IP-41 | Execution invariant tests (Y-1..Y-5) | DONE | IP-13 |
| IP-42 | Crypto invariant tests (K-1..K-4) | DONE | IP-08 |
| IP-43 | TLG-01..TLG-27 gate report | WIP | all above |
| IP-44 | Pre-mortem risk register | DONE | — |
| IP-45 | Unified findings (BLG-001..BLG-267) | WIP | — |
| IP-46 | Closed-on-testnet feature gate tests | WIP | IP-25 |
| IP-47 | Audit report (FINDINGS.md) | DONE | IP-45 |
| IP-48 | README rewrite | OPEN | IP-43 |

## Lane A (continued)

| WP | Title | Status | Depends |
|----|-------|--------|---------|
| IP-49 | StorageTx v3 types | WIP | IP-04 |
| IP-50 | Custody ack protocol | OPEN | IP-05, IP-49 |
| IP-51 | Record repair protocol | OPEN | IP-06, IP-49 |
| IP-52 | Content decline protocol | OPEN | IP-49 |
| IP-53 | Share release protocol | OPEN | IP-49 |
| IP-54 | Placement strategy engine | DONE | IP-07 |
| IP-55 | Access tiering engine | DONE | IP-54 |
| IP-56 | Fee model (upload/custody/bandwidth) | DONE | IP-54 |

## Lane B (continued)

| WP | Title | Status | Depends |
|----|-------|--------|---------|
| IP-57 | Data Market (Pollen) v3 settlement | DONE | IP-04 |
| IP-58 | AI inference data gate | DONE | IP-57 |
| IP-59 | Tokenomics epoch reward split | DONE | — |
| IP-60 | BNS name service | DONE | — |
| IP-61 | Education domain | DONE | — |
| IP-62 | Explorer evidence module | DONE | — |
| IP-63 | Cold wallet quorum | DONE | — |
| IP-64 | Account abstraction (ML-DSA-87) | DONE | IP-08 |
