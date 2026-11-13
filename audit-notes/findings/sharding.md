# Sharding Findings

Files reviewed: src/sharding/mod.rs (547 lines)

## F-0054: Sharding supports up to 64 shards

- **Location**: `src/sharding/mod.rs:33`
- **What is wrong**: Not a finding. MAX_SHARDS = 64, which provides headroom for horizontal scaling while remaining manageable for cross-shard coordination.
- **Status**: closed (informational, positive assessment)

## F-0055: Shard assignment is deterministic

- **Location**: `src/sharding/mod.rs:86`
- **What is wrong**: Not a finding. `shard_of(address, num_shards)` uses the address bytes to deterministically assign a shard. The same address always maps to the same shard for a given `num_shards`.
- **Status**: closed (informational, positive assessment)

## F-0056: Cross-shard transfers are atomic

- **Location**: `src/sharding/mod.rs:211`
- **What is wrong**: Not a finding. `apply_cross_shard_transfer` atomically debits the source shard and credits the destination shard. The function is called within block execution.
- **Status**: closed (informational, positive assessment)

## F-0057: Shards commitment is computed over all shard roots

- **Location**: `src/sharding/mod.rs:178`
- **What is wrong**: Not a finding. `shards_commitment` computes a merkle root over all shard state roots. This is included in the block header as `shards_root`.
- **Status**: closed (informational, positive assessment)

## F-0058: Sharding activation is height-gated

- **Location**: `src/sharding/mod.rs:74`
- **What is wrong**: Not a finding. `ShardingConfig::is_active_at(height)` ensures sharding only activates at or after the configured activation height. This allows coordinated rollout.
- **Status**: closed (informational, positive assessment)
