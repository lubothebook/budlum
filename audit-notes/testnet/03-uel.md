# How to build: Universal Execution Layer (UEL)

Testnet index Section 3. Work packages IP-09, IP-26 through IP-33, IP-35. TLG gates TLG-12 through TLG-16.

---

## Current state

The UEL coordinator (`src/execution/coordinator/mod.rs`) manages five EE types: EvmEe, WasmEe, SvmEe, MoveEe, BudZero. Worker supervision (`src/execution/coordinator/worker.rs`) uses Unix socket isolation with 4 GiB RLIMIT_AS. All five EEs are compiled unconditionally (F-0027).

The executor (`src/execution/executor.rs`, 5076 lines) handles all transaction types including cross-EE Universal Calls. The AI execution backend is gated by `AI_EXECUTION_BACKEND_PLONKY3` constant. The RPC API (`src/rpc/api.rs`, 1801 lines) exposes 40+ JSON-RPC methods.

Six RPC compatibility layers exist: Ethereum, Solana, Aptos, Cosmos, StarkNet, Sui. StarkNet and Sui are closed on testnet. The Ethereum compat log delivery answers -32004 (F-0096).

The Universal Call precompile is at address 0xB001 (`contracts/uel/BudlumUniversal.sol`).

## What must be built

1. **Ethereum log delivery** (TLG-13). Wire the EVM execution environment log query path into `src/rpc/compat/ethereum/filters.rs`. The filter table and limits are ready. Only the log data source is missing. Without this, `eth_getFilterChanges` and `eth_getLogs` fail, blocking Hardhat, Foundry, ethers, and viem tool tests.

2. **Tool test suite** (TLG-13). Green nightly for: Foundry (`forge test`), Hardhat (`npx hardhat test`), viem, ethers.js, CosmJS, Anchor, solana CLI, aptos CLI. Each must connect to the Budlum node's compat RPC and execute standard operations.

3. **Cross-EE atomic test** (TLG-14). An EVM contract calls a CosmWasm contract atomically in the same transaction. This exercises the Universal Call precompile (0xB001) and the cross-EE message passing.

4. **Gas calibration and determinism** (IP-35, TLG-15, TLG-16). Conformance suite for each EE. Cross-architecture determinism (x86_64 vs aarch64 must produce the same state root for the same input). The determinism CI workflow (`.github/workflows/determinism.yml`) exercises this.

5. **Configurable compat ports** (F-0028). Aptos compat at 127.0.0.1:18549 and Solana compat at 127.0.0.1:18548 are hardcoded. Make configurable.

## Key files

- `src/execution/coordinator/mod.rs`: UEL coordinator
- `src/execution/coordinator/worker.rs`: worker supervision
- `src/execution/executor.rs`: transaction execution (5076 lines)
- `src/rpc/api.rs`: JSON-RPC API definitions
- `src/rpc/server.rs`: RPC server (8803 lines)
- `src/rpc/compat/ethereum/`: Ethereum gateway
- `src/rpc/compat/solana/`: Solana gateway
- `src/rpc/compat/aptos/`: Aptos gateway
- `src/rpc/compat/cosmos/`: Cosmos gateway
- `contracts/uel/BudlumUniversal.sol`: Universal Call precompile
- `crates/uel-abi/`: execution environment ABI

## Blocking findings

- F-0026: Unix-only worker isolation
- F-0027: all EEs compiled unconditionally
- F-0028: hardcoded compat ports
- F-0096: Ethereum filter log delivery missing
