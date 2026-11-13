# Consensus Extended Findings

Files reviewed: src/consensus/mod.rs (832 lines), src/consensus/pow.rs, src/consensus/pos.rs, src/consensus/poa.rs, src/consensus/bft.rs

## F-0050: Consensus constants are well-bounded

- **Location**: `src/consensus/mod.rs:26-292`
- **What is wrong**: Not a finding. Block size (1MB), max transactions (5000), reorg depth (100), future block time (15s), past block time (2h), round timeout (3s), max leader rounds (16), and slashing evidence (64KB) are all bounded and internally consistent.
- **Status**: closed (informational, positive assessment)

## F-0051: RoundPriceBook prevents round-buying attacks

- **Location**: `src/consensus/mod.rs:99`
- **What is wrong**: Not a finding. The RoundPriceBook tracks tip blocks and computes an affordable leader round for each timestamp, preventing validators from claiming arbitrarily high rounds.
- **Status**: closed (informational, positive assessment)

## F-0052: Multiple consensus engines coexist

- **Location**: Multiple files in `src/consensus/`
- **What is wrong**: The "consensus of consensuses" design allows PoW, PoS, PoA, BFT, and ZK engines to coexist. Each consensus domain has its own finality rules. Cross-domain finality is settled through the settlement layer.
- **Why it is wrong**: Not necessarily wrong, but the interaction between consensus engines (especially during network partitions) needs thorough testing.
- **What it causes**: A partition between PoS and BFT domains could lead to conflicting finality signals. The settlement layer must resolve these.
- **How to fix it**: Ensure the settlement layer's conflict resolution is tested with adversarial scenarios.
- **Status**: open (testing needed)

## F-0053: Slashing evidence size is bounded

- **Location**: `src/consensus/mod.rs:290`
- **What is wrong**: Not a finding. MAX_SLASHING_EVIDENCE_BYTES = 65,536 (64KB) bounds the size of equivocation proofs and bad-repair evidence. This prevents oversized evidence from bloating blocks.
- **Status**: closed (informational, positive assessment)
