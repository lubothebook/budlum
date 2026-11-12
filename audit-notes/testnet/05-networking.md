# How to build: Networking

Testnet index Section 5. Work package IP-15. TLG gate TLG-18.

---

## Current state

The network node (`src/network/node.rs`, 7058 lines) implements libp2p-based P2P with Kademlia, gossipsub, and request-response protocols. The peer manager (`src/network/peer_manager.rs`, 1945 lines) has reputation scoring with ban thresholds.

Peer authentication via ML-DSA-65 (`src/network/peer_auth.rs`) is implemented but unwired (F-0086). P2P identity stays libp2p's own PeerId. The egress privacy policy (`src/network/privacy.rs`) is also unwired (F-0087). The witness request protocol (`src/network/node.rs` line 1659) is unwired (F-0079).

## What must be built

1. **NAT traversal** (IP-15, TLG-18). Relay, DCUtR (Direct Connection Upgrade through Relay), and AutoNAT. These are libp2p protocols. Verify they are configured and working through the NAT matrix test. Mobile validators behind carrier NAT must be reachable.

2. **P2P hardening**. Bounded bincode deserialization (prevent memory-exhaustion attacks from oversized messages). Kademlia private-address rejection (prevent internal addresses from entering the DHT). Rate limiting per peer (implemented in peer_manager.rs).

3. **PQ peer authentication** (F-0086). Wire `verify_peer_proof` into the transport. This is important for security but may not block testnet if the P2P network is operated by known validators during the initial phase.

## Key files

- `src/network/node.rs`: P2P node (7058 lines)
- `src/network/peer_manager.rs`: reputation scoring
- `src/network/peer_auth.rs`: ML-DSA-65 peer proof (unwired)
- `src/network/privacy.rs`: egress policy seam (unwired)
- `src/network/witness_protocol.rs`: witness request protocol

## Blocking findings

- F-0049: peer auth 3 unwired markers
- F-0086: PQ peer authentication not enforced
- F-0087: egress privacy policy unwired
