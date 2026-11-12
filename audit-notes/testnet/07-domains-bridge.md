# How to build: Domains, bridge, settlement

Testnet index Section 7. Work package IP-44. TLG gate TLG-27.

---

## Current state

The domain system includes: consensus domain registry (`src/domain/`), storage deal lifecycle (`src/domain/storage_deal.rs`, 9864 lines), finality adapter (`src/domain/finality_adapter.rs`, 3030 lines), and the settlement layer (`src/settlement/`).

The bridge (`src/cross_domain/evm/`) has 5 modules all unwired (F-0033). The bridge is conditional at testnet. The settlement graft (`src/settlement/graft.rs`) has 2 unwired markers (F-0055). The PQ anchor verification (`src/settlement/pq_anchor_verify.rs`) is unwired (F-0094).

The Solidity contracts (`contracts/`) include: BudlumSlasher, BudlumRegistrar (Mikoriza/shared security), BudlumUniversal (UEL), BudlumFinalityVerifier, Graft contracts.

## What must be built

1. **Bridge decision** (TLG-27). Determine whether BLG-043, BLG-057, BLG-105, BLG-194 are closed. If all four close, the bridge opens. If any remain open, the bridge stays closed. Write the decision in the TLG report.

2. **If bridge opens**: Wire the 5 EVM bridge verification modules in `src/cross_domain/evm/`. Wire the relayer worker (`src/relayer/worker.rs`, F-0070). Deploy and verify the Solidity contracts.

3. **If bridge stays closed**: Verify that the closing tests for bridge-related features exist and pass on chain ID 45261 (TLG-02).

4. **Settlement**: The settlement layer works through `src/settlement/`. Global headers, anchor production, and the graft system are functional (graft payment proofs work through the RPC). The cold wallet state is in blockchain.rs but the rotation entry point is unwired (F-0093).

5. **Contracts rename** (F-0074). The `contracts/mikoriza/` directory uses the deprecated name. Rename to `contracts/shared-security/` and update domain tag strings in coordination with the Rust side.

## Key files

- `src/domain/`: domain registry, storage deals, finality adapter
- `src/cross_domain/evm/`: bridge verification (all unwired)
- `src/settlement/`: anchor production, graft, cold wallet
- `src/relayer/`: bridge relayer (unwired)
- `contracts/`: Solidity contracts

## Blocking findings

- F-0033: all 5 EVM bridge modules unwired
- F-0055: settlement graft unwired
- F-0070: relayer tracks bridge decision
- F-0074: Mikoriza name in contracts
- F-0094: PQ anchor light verification unwired
