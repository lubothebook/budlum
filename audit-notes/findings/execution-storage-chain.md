# Findings: execution, storage, chain, cross-domain

## F-0026

- **ID**: F-0026
- **Location**: src/execution/coordinator/worker.rs lines 62-79
- **What is wrong**: `WorkerEndpoint::UnixSocket` is gated behind `#[cfg(unix)]`. On non-Unix platforms, only `InProcess` mode is available.
- **Why**: The directive Section 8.3.2 requires separate worker processes for fault isolation. This only works on Unix.
- **What it causes**: Windows and non-Unix builds run all execution environments in-process with no address-space isolation. Mobile validators on Android (which is Unix-based) are fine. This is a platform limitation, not a bug.
- **How to fix**: Document that out-of-process EE workers require Unix. Consider whether Windows testnet nodes are a requirement.

## F-0027

- **ID**: F-0027
- **Location**: src/execution/coordinator/mod.rs (entire module)
- **What is wrong**: The coordinator imports and uses all five EE types: EvmEe, WasmEe, SvmEe, MoveEe, and BudZero. All are compiled into the binary unconditionally.
- **Why**: The directive Section 8 lists five execution environments. All are always present.
- **What it causes**: The binary includes all five VM backends even when a node only needs one. Binary size is large. Mobile validators carry all five.
- **How to fix**: Consider feature-gating non-essential EEs for mobile builds. This is a size/build optimization, not a correctness issue. The directive does not require conditional compilation of EEs.

## F-0028

- **ID**: F-0028
- **Location**: src/main.rs line 1778
- **What is wrong**: The Aptos compat listener is hardcoded to `127.0.0.1:18549` and the Solana compat listener to `127.0.0.1:18548`. These addresses are not configurable.
- **Why**: Compat RPC listeners for external tool compatibility. Hardcoded addresses may conflict with other services.
- **What it causes**: Port collision if another service uses 18548 or 18549. Not configurable by the operator. TLG-13 (Foundry, Hardhat, etc. tool tests) may need specific ports.
- **How to fix**: Make these configurable through the node config, with the current values as defaults.

## F-0029

- **ID**: F-0029
- **Location**: src/main.rs line 2314 (total lines)
- **What is wrong**: main.rs is 2314 lines. It contains the entire node startup, including genesis build, keygen, backup, migration, consensus engine selection, HSM initialization, storage node setup, RPC server setup, metrics server, and the interactive console loop.
- **Why**: All node functionality is in one function. This is a code quality observation.
- **What it causes**: Difficult to test individual startup paths. Difficult to review. Not a testnet blocker.
- **How to fix**: Extract subcommands and server setup into modules under `cli/`. This is a refactoring suggestion.

## F-0030

- **ID**: F-0030
- **Location**: src/storage/bud3/ (entire directory, 30+ modules)
- **What is wrong**: B.U.D. 3.0 is implemented in-tree under `src/storage/bud3/`. The old storage layer (`bud-node` in `budzero/bud-node`) still exists and is used by main.rs (F-0005).
- **Why**: The directive requires B.U.D. 3.0 as the sole storage system. Both old and new coexist.
- **What it causes**: Two storage systems in one binary. main.rs uses bud-node types for BitSwap and content store. The B.U.D. 3.0 modules are present but not yet the sole path. TLG-03 (B.U.D. 3.0 replaces all earlier) cannot pass.
- **How to fix**: Complete the wiring of B.U.D. 3.0 into main.rs. Replace bud-node BitSwap/ContentStore usage with B.U.D. 3.0 equivalents. Then remove bud-node.

## F-0031

- **ID**: F-0031
- **Location**: src/storage/bud3/live.rs line 28
- **What is wrong**: The live streaming module is marked "WIRING: unwired". Live streaming is listed as a closed feature (IP-55) in the testnet plan.
- **Why**: The module exists but is not wired. Since live streaming is closed at launch, this is correct.
- **What it causes**: No error. The module matches its expected testnet status (closed). Verify that a `live_streaming_is_closed_on_testnet` test exists (TLG-02).
- **How to fix**: Ensure TLG-02 compliance: the closed feature must have a test proving it is rejected on chain ID 45261.

## F-0032

- **ID**: F-0032
- **Location**: src/storage/bud3/private_dedup/committee.rs line 13
- **What is wrong**: Private cross-owner deduplication is marked "WIRING: unwired" and is listed as post-launch T-Expansion.
- **Why**: This is a closed feature (IP-54) at testnet. The module is not wired. Correct.
- **What it causes**: Same as F-0031. Verify the closing test exists.
- **How to fix**: Ensure TLG-02 compliance test.

## F-0033

- **ID**: F-0033
- **Location**: src/cross_domain/evm/ (adapter.rs, header.rs, sync_committee.rs, verify.rs, outbound.rs)
- **What is wrong**: All five EVM bridge verification modules are marked "WIRING: unwired". The bridge is conditional at testnet (opens if BLG-043, BLG-057, BLG-105, BLG-194 close).
- **Why**: The bridge verification logic exists and is tested in isolation, but the relayer path that wires it is not in this tree.
- **What it causes**: The bridge cannot function at testnet unless these modules are wired. Since the bridge is conditional, this is expected. The decision must be recorded in the TLG report (TLG-27).
- **How to fix**: If the BLG findings close, wire the bridge modules. If they remain open, document the bridge as closed and ensure the closing test exists.

## F-0034

- **ID**: F-0034
- **Location**: src/chain/finality.rs line 8 (sha2_010)
- **What is wrong**: The finality module imports `sha2_010::Sha256` (sha2 0.10.x) for the BLS hash-to-curve API. The rest of the codebase uses sha2 0.11.x (via `sha2::Sha256`).
- **Why**: bls12_381 0.9's `hash_to_curve` requires the sha2 0.10 API. osv-scanner.toml line 88-110 explains this pinning.
- **What it causes**: Two sha2 versions in the dependency tree. The osv-scanner VEX entry (GHSA-qwgh-2vcv-g2f7) covers the block-buffer 0.10.4 that sha2 0.10 brings. This is a known tracked dependency.
- **How to fix**: Tracked on the dependency roadmap. The fix requires bls12_381 to move to digest 0.11. No action for this audit beyond noting it.

## F-0035

- **ID**: F-0035
- **Location**: src/chain/finality.rs line 253
- **What is wrong**: A function at line 253 is marked "WIRING: unwired". This is in the finality module, which is a critical consensus path.
- **Why**: The specific function has no production caller yet.
- **What it causes**: If this is a finality verification function needed for committee-based finality (TLG-05), its absence from production means finality relies on a different path.
- **How to fix**: Identify whether this specific function is needed for testnet finality. If yes, wire it.

## F-0036

- **ID**: F-0036
- **Location**: src/core/transaction.rs (7 unwired markers), src/core/account.rs (4 unwired markers), src/persistence/db.rs (3 unwired markers)
- **What is wrong**: Core modules (transaction, account, persistence) have multiple unwired API surfaces.
- **Why**: These are declared surfaces for upcoming features. Some may be on the testnet critical path.
- **What it causes**: If any of these surfaces are needed for testnet transaction processing, they are not active. Need to cross-reference with TLG gates.
- **How to fix**: Audit each unwired marker against the testnet requirements. The transaction module unwired markers (7) are the most concerning because every block processes transactions.

## F-0037

- **ID**: F-0037
- **Location**: src/main.rs lines 1943-1966
- **What is wrong**: The metrics endpoint authentication uses `std::env::var("BUDLUM_METRICS_API_KEY")` and reads it on every request.
- **Why**: Environment variables are process-global state. Reading them on every HTTP request is unusual. The value does not change at runtime.
- **What it causes**: Minor performance overhead. The env var is read on every metrics scrape (typically every 15-30s from Prometheus). Not a correctness issue.
- **How to fix**: Read the env var once at startup and pass it to the metrics handler. This is a quality observation.

## F-0038

- **ID**: F-0038
- **Location**: budzero/ directory (entire workspace)
- **What is wrong**: The budzero workspace contains bud-node, bud-vm, bud-compiler, bud-isa, bud-proof, bud-state, bud-cli, and related crates. The directive IP-07 requires removing `budzero/bud-node`.
- **Why**: bud-node is the old B.U.D. 1.0/2.0 storage node. Other budzero crates (bud-vm, bud-compiler, etc.) are the BudZero zkVM and are needed for testnet (IP-33: BudZero execution environment).
- **What it causes**: The directive says remove bud-node, not the entire budzero workspace. bud-vm and the compiler are testnet requirements. The distinction matters.
- **How to fix**: Remove budzero/bud-node only. Keep bud-vm, bud-compiler, bud-isa, bud-proof, bud-state for the BudZero EE.
