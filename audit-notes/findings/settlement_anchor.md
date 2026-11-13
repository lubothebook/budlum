# Settlement and Anchor Findings

Files reviewed: src/settlement/mod.rs (62 lines), anchor_ledger.rs, anchor_production.rs, commitment_tree.rs, cold_quorum.rs, domains.rs, global_block.rs, proof_market.rs, root_seal.rs

## F-0080: Settlement module is well-decomposed

- **Location**: `src/settlement/mod.rs`
- **What is wrong**: Not a finding. The settlement module is correctly decomposed into 15 sub-modules: anchor ledger, anchor production, cold quorum, cold wallet, commitment tree, domains, global block, graft, PQ anchor, PQ anchor verify, proof market, proof verifier, registries, reseal, and root seal.
- **Status**: closed (informational, positive assessment)

## F-0081: Commitment tree uses merkle root

- **Location**: `src/settlement/commitment_tree.rs:25`
- **What is wrong**: Not a finding. The merkle_root function computes a SHA3-256 based merkle tree over leaves. Empty input returns zero. The function is used for replay nonce roots, state roots, and shard commitments.
- **Status**: closed (informational, positive assessment)

## F-0082: Anchor key ledger supports ML-DSA-87

- **Location**: `src/settlement/anchor_ledger.rs:88`
- **What is wrong**: Not a finding. The AnchorKeyLedger holds ceremony-produced ML-DSA-87 anchor keys across HSM vendors.
- **Status**: closed (informational, positive assessment)
