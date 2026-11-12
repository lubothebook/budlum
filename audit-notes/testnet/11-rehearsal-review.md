# How to build: Rehearsal and review

Testnet index Section 11. Work packages IP-46, IP-47, IP-48. TLG gate TLG-22.

---

## Current state

The rehearsal infrastructure exists in `ops/rehearsal/`. Two topology files: `topology.toml` (chain ID 45262, devnet standalone) and `rehearsal-topology.toml` (chain ID 45261, testnet ceremony rehearsal). These have confusing names (F-1210).

The ceremony rehearsal genesis (`config/ceremony-rehearsal-genesis.json`) has a truncated allocation address (F-1200).

The Docker compose setup (`ops/docker-compose.yml`, `ops/testnet-local.yml`) supports multi-node local testnet with validators, RPC, faucet, indexer, Prometheus, and Grafana.

## What must be built

1. **Fix ceremony rehearsal genesis** (F-1200). The truncated address at line 14 of `config/ceremony-rehearsal-genesis.json` must be extended to 64 hex characters.

2. **Ceremony rehearsal** (IP-46, TLG-17). Run the full ceremony on chain ID 45261 with at least 8 Full validators. The `bud genesis assemble` tool produces the genesis. Each validator registers consensus keys (Ed25519 + ML-DSA-65). Supply check passes. The ceremony creates a valid genesis that can boot a network.

3. **Rehearsal network** (IP-46). Boot the testnet network from the ceremony genesis. Run for 72 hours. Measure: block production rate, finality lag, mobile validator participation, storage deal throughput, and all TESTNET-DEFAULT parameter behavior.

4. **Independent review 1** (IP-47, TLG-22). Core and storage review. The reviewer must verify: state transition correctness, supply conservation (G-1), custody proof correctness, and the B.U.D. 3.0 pipeline. Key challenge: `blockchain.rs` (13063 lines), `account.rs` (7222 lines), `storage_deal.rs` (9864 lines) are review bottlenecks.

5. **Independent review 2** (IP-48, TLG-22). Execution, mobile, clients review. The reviewer must verify: EE sandboxing, cross-EE atomicity, mobile share persistence, wallet key management, RPC security.

6. **Rename topology files** (F-1210). Clarify `ops/rehearsal/topology.toml` vs `ops/rehearsal/rehearsal-topology.toml` to prevent operator confusion.

## Key files

- `ops/rehearsal/`: rehearsal infrastructure
- `ops/docker-compose.yml`: local testnet compose
- `ops/testnet-local.yml`: testnet local with monitoring
- `config/ceremony-rehearsal-genesis.json`: ceremony genesis
- `config/ceremony-rehearsal-template.json`: ceremony template

## Blocking findings

- F-1200: truncated allocation address in ceremony genesis
- F-1210: confusing topology file names
- F-0080: blockchain.rs 13063 lines (review bottleneck)
- F-0063: account.rs 7222 lines (review bottleneck)
- F-0110: storage_deal.rs 9864 lines (review bottleneck)
