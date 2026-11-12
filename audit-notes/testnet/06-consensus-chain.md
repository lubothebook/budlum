# How to build: Consensus and chain

Testnet index Section 6. Work packages IP-16, IP-17. TLG gates TLG-17, TLG-19.

---

## Current state

The blockchain state machine (`src/chain/blockchain.rs`, 13063 lines) handles block validation, application, epoch transitions, finality, reorgs, pruning, and metrics. The consensus engine (`src/consensus/mod.rs`, 833 lines) implements leader election, round pricing, and block size validation.

The genesis system (`src/chain/genesis.rs`, 2219 lines) supports ceremony configuration, tokenomics, domain bootstrap, and validator consensus keys. The ceremony tool is the `bud` CLI binary.

The chain actor (`src/chain/chain_actor.rs`, 4047 lines) drives the blockchain through async message passing.

Tokenomics (`src/tokenomics/mod.rs`) defines BUD_TOTAL_SUPPLY=100M, BUD_DECIMALS=6, and the reward pool split (40/50/10%). Supply conservation (G-1) is enforced at compile time.

## What must be built

1. **Genesis ceremony rehearsal** (TLG-17). The ceremony tool (`bud genesis assemble`) must produce a valid genesis for chain ID 45261. At least 8 Full validators participate. The supply check must be green (all allocations sum to BUD_TOTAL_SUPPLY).

2. **State transition hardening** (IP-17). The block validation path (`blockchain.rs` `validate_and_add_block_with`) validates: height continuity, chain ID, tx_root, witness_root, block hash, state_root, consensus, and slashing evidence. The state root is computed after applying all transactions and compared to the block's declared root. This path is clean (no unwired markers in blockchain.rs).

3. **Economy verification** (TLG-19). Pool splits, storage fees, and rewards must preserve supply (G-1). The compile-time assertion in `src/tokenomics/mod.rs` covers the split. The dust absorption in epoch rewards (F-0054) ensures no base units are lost. Verify end-to-end on the rehearsal network: total supply before and after N epochs must be equal.

4. **Filiz Chain rename** (F-0023, F-0073). The leader election mechanism uses "Filiz" (261 references). If this is in the concept name map, it blocks TLG-24. The domain tags (`BDLM_FILIZ_ELIG_V1`, `BDLM_FILIZ_EPOCH_V1`) are consensus-critical wire format and must not change without a coordinated upgrade.

## Key files

- `src/chain/blockchain.rs`: state machine (13063 lines)
- `src/chain/chain_actor.rs`: async driver (4047 lines)
- `src/chain/genesis.rs`: genesis configuration (2219 lines)
- `src/consensus/mod.rs`: consensus engine (833 lines)
- `src/consensus/filiz.rs`: leader election (Filiz Chain)
- `src/tokenomics/mod.rs`: supply, pool splits, burn
- `src/tokenomics/reward_pool.rs`: validation reward pool

## Blocking findings

- F-0023: Filiz naming (261 references)
- F-0073: Filiz in blockchain.rs
- F-0080: blockchain.rs 13063 lines (review bottleneck)
