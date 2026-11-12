# How to build: Naming and documentation

Testnet index Section 10. Work packages IP-05, IP-43. TLG gates TLG-23, TLG-24.

---

## Current state

The concept name map (`config/naming/concept_map.toml`) defines deprecated names and their replacements. The concept name counter gate (`xtask/gates/src/gates/concept_names.rs`) counts occurrences. The gate is currently in "count" mode, not "absolute" mode.

Deprecated name reference counts in production code:
- Filiz: 261 references (F-0073)
- Pollen: 455 references (F-0071)
- Mikoriza: 43 references in src/ (F-0072), plus contracts/ (F-0074), plus config/ (F-1202)
- AGD: present in handover comments (F-0022)
- Proto wire names: Filiz and Pollen in protobuf (F-1205)

Person names in code: "Ayaz" in 5 production comments (F-0099, F-0100).

Turkish in operational output: Dockerfile error message (F-0089).

## What must be built

1. **Concept name migration** (IP-05, TLG-24). Rename all deprecated names to their English equivalents. Priority order by reference count:
   - Pollen -> data_market (455 references)
   - Filiz -> hash_chain_election (261 references)
   - Mikoriza -> shared_security (43 references + contracts)
   - AGD -> instant_task_handover (few references)

2. **Wire protocol names** (F-1205). Proto enum/message names cannot change without a wire version bump. Record a KR decision stating proto names are frozen for the current wire version. Add `proto/` to the concept_map's `global_allow_paths`.

3. **Domain tag strings**. Consensus-critical domain tags (`BDLM_FILIZ_ELIG_V1`, `BDLM_MIKORIZA_BIND_V1`, etc.) must be renamed with a coordinated upgrade across Rust and Solidity sides.

4. **Metric names** (F-0071). Prometheus metric names (`budlum_pollen_*`) are public API. Rename with a migration period for Grafana dashboards.

5. **Documentation** (TLG-23). README.md must equal Appendix A. README.tr.md must equal Appendix B. Verify content matches.

6. **Switch gate to absolute mode** (TLG-24). After all renames complete, switch the concept name gate from "count" to "absolute" mode. All counters must be zero.

## Key files

- `config/naming/concept_map.toml`: deprecated name map
- `xtask/gates/src/gates/concept_names.rs`: concept name counter gate
- `proto/budlum/network/protocol.proto`: wire protocol names
- `src/core/metrics.rs`: Prometheus metric definitions
- `contracts/mikoriza/`: Solidity contracts with deprecated names

## Blocking findings

- F-0022: AGD old name in handover
- F-0023: Filiz naming (261 refs)
- F-0071: Pollen in metrics (455 refs)
- F-0072: Mikoriza in production code (43 refs)
- F-0074: Mikoriza in contracts
- F-0089: Turkish in Dockerfile
- F-0099: Ayaz person name in comments
- F-1202: Mikoriza in testnet-ethereum.toml
- F-1205: deprecated names in proto wire format
