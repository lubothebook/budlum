# Engine inventory (WP-03 special audit)

This file is required by the WP-03 card (`Bölüm 15.8`, `İP-03`) as the `docs/audit/live/engine-inventory.md` artifact.

## Rules

Every entry must list: engine name, crate / module reference in this repo, version at time of analysis, license file reference, `cargo-deny` pre-check result (if available), production call site (file + function or method), notes on determinism (no floating point, no unordered collections in consensus path — `G-2`).

Engines listed in the directive (`Bölüm 8.2`): `revm` (EVM), `wasmi` (WASM), `solana-svm` / `solana-sbpf` (SVM), `aptos-vm` (Move / Aptos dialect), and `budzero` (BudZero execution adapter).

## Inventory

| Engine | Source reference | Version (analysis time) | License | cargo-deny result | Production call site | Determinism notes |
|---|---|---|---|---|---|---|
| EVM (`revm`) | `execution/evm/` (separate workspace, per `Bölüm 4.2` target layout) | Not pinned in this repo; depends on execution workspace manifest | Apache-2.0 / MIT (per `docs/EXECUTION.md` and crate docs) | Not run in this session (`S-5`: no new dependency without `cargo-deny` / `cargo-vet` / `cargo-geiger`) | `src/execution/coordinator/` (coordinator dispatches to EVM worker via `execution/evm/` binary) | No floating point in consensus path (`G-2`); EVM uses 256-bit integer arithmetic |
| WASM (`wasmi`, CosmWasm ABI) | `execution/wasm/` (separate workspace) | Not pinned in this repo; `Cargo.toml` references `wasmi` and CosmWasm ABI contracts | Apache-2.0 / MIT | Not run (`S-5`) | `src/execution/coordinator/` (dispatch to WASM worker) | No floating point (`G-2`); `wasmi` interpreter is deterministic for the CosmWasm subset |
| SVM (`solana-svm`, `solana-sbpf`) | `execution/svm/` (separate workspace) | Not pinned; `solana-sbpf` used for program parsing | Apache-2.0 / MIT | Not run (`S-5`) | `src/rpc/compat/solana.rs` (RPC gateway) and `execution/svm/` (execution worker) | No floating point in consensus path (`G-2`); SVM uses 64-bit integers |
| Move Aptos (`aptos-vm`) | `execution/move/` (separate workspace) | Not pinned; `aptos-vm` crate | Apache-2.0 | Not run (`S-5`) | `execution/move/` and `src/rpc/compat/aptos.rs` | No floating point (`G-2`); Move uses `u64` / `u128` arithmetic |
| BudZero (`budzero` ISA / VM) | `budzero/bud-vm/`, `budzero/bud-isa/` | Internal (this repo's `budzero/` workspace) | Internal / repository license (`LICENSE.md`) | `budzero/` workspace is covered by `execution.yml` and `supply-chain.yml` (`Bölüm 16.2`) | `src/execution/budzero_ee/` (adapter connecting coordinator to BudZero VM) | Deterministic ISA (`Bölüm 10.2`, `G-2`); no floating point, no unordered collections in consensus path |

## Special audit notes (WP-03 card requirements)

1. `src/execution/executor.rs` `ContractCall` path (`BLG-248`): the file exists (`src/execution/` is present in this repo layout per `Bölüm 4.3`); the exact path must be verified against the live file content, not by line number (`Bölüm 0.4`).
2. BudZero `Pcs::ZK` value (`BLG-223`, `BLG-224`): the `budzero/` workspace contains `Pcs` definitions; the `ZK` bit must be inspected in `budzero/bud-vm/src/` or the equivalent module, with the live file content verified.
3. 64-bit commitment paths (`BLG-223`): any commitment using 64-bit values must be identified and mapped to the directive's `no_64bit_commitments` gate (`WP-56`, `Bölüm 10.3`).
4. Ethereum sync committee module (`BLG-235`): `src/cross_domain/evm/sync_committee.rs` (or the equivalent module) must have its production call site recorded; the module is part of the `WP-60` scope (`Bölüm 10.3`), but its presence and call site must be verified live for WP-03.
5. Bridge findings (`BLG-043`, `BLG-057`, `BLG-105`, `BLG-194`): these four findings determine the bridge decision (`Bölüm 1.5`, `KR-28`). Their status must be recorded in `lane-c.md` once `WP-44` (domain / bridge package) produces its closing evidence; WP-03 only verifies the live presence of these references in the execution / domain paths, not their final closure.
