# Network Module Findings

Files reviewed: src/network/mod.rs, node.rs, peer_manager.rs, protocol.rs, gossip_dedup.rs, mobile.rs, privacy.rs, sync_codec.rs, regen_health_tick.rs, proto_conversions.rs

## F-0014: Gossip dedup cache has no size bound documented

- **Location**: `src/network/gossip_dedup.rs`
- **What is wrong**: The gossip deduplication cache prevents reprocessing of seen messages. The cache eviction policy (time-based or size-based) exists but the maximum cache size is not documented in the module or in the architecture docs.
- **Why it is wrong**: An attacker could flood the gossip layer with unique messages, causing unbounded memory growth if the cache size is not bounded.
- **What it causes**: Memory exhaustion on nodes under sustained gossip flood. The peer_manager rate limiting should prevent this, but the dedup cache itself should also have a documented bound.
- **How to fix it**: Document the cache bound. If it is time-based only, add a size cap as well.
- **Status**: open

## F-0015: Mobile node profile battery reading is trusted

- **Location**: `src/network/mobile.rs`
- **What is wrong**: `try_update_battery` accepts the battery percentage from the node operator (or the mobile app). The value is validated (0-100) but not independently verified. A mobile validator could report 100% battery and never be throttled.
- **Why it is wrong**: This is expected for testnet: the mobile app reports its own battery state. A malicious mobile node gains nothing by lying about battery (it only determines the node's own resource budget, not any protocol power).
- **What it causes**: No security impact. The battery level controls local behavior (peer budget, storage budget), not consensus weight.
- **How to fix it**: Informational. Document that battery reporting is self-attested and affects only local resource allocation.
- **Status**: open (informational)

## F-0016: DNS seed resolution does not use _dnsaddr TXT records

- **Location**: `config/testnet.toml:29-35`
- **What is wrong**: The config comment documents: "The node does not read libp2p `_dnsaddr` TXT records." DNS seeds are resolved through A/AAAA records only. This is a deliberate design choice but limits interoperability with standard libp2p DNS discovery.
- **Why it is wrong**: Not a bug. Standard libp2p bootstrap uses `_dnsaddr` TXT records. Budlum uses A/AAAA records at a fixed port, which is simpler but requires the seed infrastructure to serve DNS records in a specific format.
- **What it causes**: For testnet, no impact: the operator controls the DNS infrastructure. For future interoperability with other libp2p networks, this would need to be extended.
- **How to fix it**: Informational. Documented design decision.
- **Status**: open (informational)
