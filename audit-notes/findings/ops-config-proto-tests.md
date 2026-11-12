# Audit Findings: Ops, Config, Proto, Tests, Fuzz, Kani, Quality

Findings from the audit of: `ops/`, `config/`, `proto/`, `tests/`, `fuzz/`,
`kani/`, `benches/`, `examples/`, `release/`, `supply-chain/`, `xtask/`,
`.quality/`, `docs/` (structural review), `assets/`.

---

## F-1200 — Truncated allocation address in ceremony-rehearsal-genesis.json

| Field | Value |
|---|---|
| **ID** | F-1200 |
| **Location** | `config/ceremony-rehearsal-genesis.json:14` |
| **What is wrong** | The team allocation address `5353…5353` is 60 hex characters (30 bytes) while every other address in the same file is 64 hex characters (32 bytes). Line 7 (`5151…`), line 11 (`5252…`), line 19 (`5454…`), and line 23 (`5555…`) are all 64 hex chars. Line 14 is 4 characters short. |
| **Why** | Two bytes (`5353`) were likely lost during a manual edit or copy-paste of the repeating pattern. |
| **Effects** | A genesis loader parsing this address as a 32-byte value will either reject it or zero-pad it, producing a different address than the one the tokenomics plan intended. Any balance credited to this truncated address is unreachable under the 32-byte address scheme. The ceremony-rehearsal-genesis.json is referenced by `ops/rehearsal/rehearsal-topology.toml` line 10 as `genesis_fixture`, so it feeds the testnet ceremony rehearsal path. |
| **Fix** | Append `5353` to the line 14 address, making it `"5353535353535353535353535353535353535353535353535353535353535353535353"` (64 hex chars, 32 bytes), matching the repeating-byte pattern of the other allocations. |

---

## F-1201 — Grafana image tag without digest pin

| Field | Value |
|---|---|
| **ID** | F-1201 |
| **Location** | `ops/testnet-local.yml:221` |
| **What is wrong** | The Grafana service uses `image: grafana/grafana:11.2.0` — a mutable tag pin with no `@sha256:` digest. The Prometheus image on line 206 uses a full digest pin (`prom/prometheus:v3.14.0@sha256:5ce7...`). The comment on line 219 acknowledges this gap: "Tag pin; move it to a reviewed digest before this file serves anything beyond a local rehearsal." |
| **Why** | The digest was not added when the Prometheus image was pinned. |
| **Effects** | A `docker pull` of `grafana/grafana:11.2.0` after the upstream publisher replaces the tag gets a different binary than the one tested locally. In a testnet rehearsal this is a supply-chain integrity gap. TLG-25 (CI reproducibility) and the `docker_toolchain` gate require reproducible images. |
| **Fix** | Pin the image with a reviewed digest, e.g. `grafana/grafana:11.2.0@sha256:<digest>`, the same pattern used for Prometheus. |

---

## F-1202 — Deprecated concept name "mikoriza" in testnet-ethereum.toml

| Field | Value |
|---|---|
| **ID** | F-1202 |
| **Location** | `config/testnet-ethereum.toml:27` |
| **What is wrong** | The TOML section header is `[mikoriza]`. Per `config/naming/concept_map.toml`, "Mikoriza" / "Mycorrhiza" is a deprecated concept name that must be replaced with `shared_security`. The same file also references "Ayaz" in inline comments (lines 20, 29) which is a person/code name, not a professional term. |
| **Why** | The Ethereum testnet config was written before the concept name migration was enforced. |
| **Effects** | The concept-name gate (`xtask/gates/src/gates/concept_names.rs`) counts occurrences of deprecated names. While `config/` may be in the global_allow_paths list for some concepts, the `[mikoriza]` section header is a structural reference that tooling and documentation will need to match. When the gate switches from "count" mode to "absolute" mode (concept_map.toml line 2), this becomes a blocker. |
| **Fix** | Rename the section to `[shared_security]` and update any code that parses this section name. Replace "Ayaz" references with the work-package or decision identifier (e.g., "KR-42" or "IP-44"). |

---

## F-1203 — Inconsistent genesis allocation schema across network profiles

| Field | Value |
|---|---|
| **ID** | F-1203 |
| **Location** | `config/devnet-genesis.json:4-9`, `config/rehearsal-genesis.json:4-9`, `config/ceremony-rehearsal-genesis.json:4-25` |
| **What is wrong** | Genesis allocation entries use two different schemas across config files: (a) `devnet-genesis.json` and `ceremony-rehearsal-genesis.json` use `[address, amount]` two-element arrays; (b) `rehearsal-genesis.json` uses `{address, amount, ...}` objects with additional fields (name, region, provider, class, stake). The `testnet-genesis.json` and `ceremony-rehearsal-template.json` have empty allocation arrays so no schema is visible. Similarly, the `validators` field uses bare hex strings in some files and objects with metadata in `rehearsal-genesis.json`. |
| **Why** | The rehearsal-genesis.json was written to carry richer metadata (region, provider, class) for topology validation, while the simpler files follow the minimal format the node parser expects. |
| **Effects** | Code that parses genesis files must handle both schemas or will fail on one of the profiles. The `genesis_schema` gate (`xtask/gates/src/gates/genesis_schema.rs`) should verify every checked-in genesis file parses under one canonical schema. If it does not test `rehearsal-genesis.json`, that file silently diverges. A deployment loading the wrong schema gets a parse error at startup with no recovery path. |
| **Fix** | Either (a) normalize all genesis files to one canonical schema (the simpler `[address, amount]` form for allocations, bare strings for validators), moving the metadata to the topology manifest; or (b) document and test both schemas explicitly. The `genesis_schema` gate should cover every `*-genesis.json` file in `config/`. |

---

## F-1204 — devnet.toml missing explicit RPC authentication fields

| Field | Value |
|---|---|
| **ID** | F-1204 |
| **Location** | `config/devnet.toml:26-31` |
| **What is wrong** | The `[rpc]` section in `devnet.toml` does not set `auth_required`, `operator_auth_required`, `api_key_env`, `allowed_ips`, or `rate_limit_per_minute`. The testnet profile (`config/testnet.toml:49-56`) and mainnet profile (`config/mainnet.toml:65-68`) both set these fields explicitly. The developer persona (`config/personas/developer.toml:30-34`) for the same devnet profile sets `auth_required = true` and `api_key_env`, creating a silent disagreement with the base config. |
| **Why** | The devnet config was written as a minimal configuration and relies on code defaults for security fields. |
| **Effects** | The effective security posture of a devnet node depends on code defaults which are not visible in the config file. If the code default for `auth_required` is `true`, then a developer following `devnet.toml` needs an API key but the config does not tell them which env var to set. If the default is `false`, then the developer persona is silently stricter than the base config, and switching between them changes behavior. Either way the operator cannot determine the RPC security posture by reading the config file alone. |
| **Fix** | Add explicit `auth_required = false`, `operator_auth_required = false`, and `rate_limit_per_minute = 0` (or the intended value) to `devnet.toml` `[rpc]`, matching the pattern of the testnet and mainnet configs. |

---

## F-1205 — Deprecated concept names in wire protocol (proto)

| Field | Value |
|---|---|
| **ID** | F-1205 |
| **Location** | `proto/budlum/network/protocol.proto:66,124,598-601` (Filiz); `proto/budlum/network/protocol.proto:31-35,39-43,175,179,552-568` (Pollen) |
| **What is wrong** | The protobuf wire protocol uses two deprecated concept names: (1) `REGISTER_FILIZ` (enum value 66) and `ProtoFilizRegistration` (message) — per `config/naming/concept_map.toml`, "Filiz" should be "hash_chain_election"; (2) `POLLEN_*` enum values (31-35) and `ProtoPollen*` messages — per concept_map.toml, "Pollen" should be "data_market". The `proto/` directory is excluded from the `typos.toml` scan (`.quality/typos.toml:22`) and the concept_map's `paths` for Filiz do not include `proto/`, so neither the spelling gate nor the concept-name gate flags these. |
| **Why** | Wire protocol names are excluded from automated renaming gates because changing a protobuf field/enum name is a wire-breaking change for existing peers. |
| **Effects** | No runtime impact: proto field names are documentation; the wire carries field numbers. However, the deprecated names propagate into generated code (Rust/Go/JS) as enum variant and struct names, which new code must use. When the concept-name gate enters absolute mode (TLG-24), these names will need resolution — either a migration plan for the wire protocol version or explicit exemptions. |
| **Fix** | Record a decision (as a KR entry) stating that proto enum/message names are frozen for the current wire version and will be renamed in the next wire version bump. Add `proto/` to the concept_map's `global_allow_paths` with a comment citing the decision. |

---

## F-1206 — Kani model-checking mirror has no automated equivalence link to production

| Field | Value |
|---|---|
| **ID** | F-1206 |
| **Location** | `kani/src/lib.rs:19-46` |
| **What is wrong** | The file's own documentation (lines 19-46) explicitly states the risk: `penalty_for` in `kani/src/lib.rs:59` is a manual copy of `core::chain_config::slash_penalty`. The kani package has no dependency on `budlum-core` and vice versa. The only guard is the `kani` gate which checks that every declared harness ran, NOT that the logic equals production. The mirror tests in `budlum-core` (`bond_arithmetic_matches_the_kani_mirror`, `qc_merkle_matches_the_kani_mirror`, `pq_signature_classification_matches_the_kani_mirror`) compare production with a *third* hand-written copy inside the test, not with this file. |
| **Why** | Kani's pinned nightly (rustc 1.93.0) is below `budlum-core`'s MSRV (1.97.1), so `budlum-core` cannot depend on the kani package. The file documents a plan to resolve this with a path dependency but notes it is "tracked as a decision, not done here." |
| **Effects** | If production `slash_penalty` is edited without updating `kani/src/lib.rs`, the Kani proofs pass on stale logic. The proofs would then certify properties about code that is no longer running. The same applies to the Merkle tree and PQ signature length mirrors. This is a verified-but-unlinked-specification risk, not a correctness bug today. |
| **Fix** | (1) Add a `dev-dependency` path from `budlum-core` to the kani package (or extract the mirror functions into a shared `no_std` crate) so a test can call both copies and fail on divergence. (2) Until then, add a `#[test]` in `budlum-core` that reads `kani/src/lib.rs` as text and asserts a content hash over the `penalty_for`, Merkle, and PQ classification functions, so any edit to either copy causes a test failure. |

---

## F-1207 — Three Kani monotonicity harnesses have UNPROVED status

| Field | Value |
|---|---|
| **ID** | F-1207 |
| **Location** | `kani/src/lib.rs:641-649` (`ratio_endpoints_are_exact`), `kani/src/lib.rs:670-694` (`penalty_is_monotonic_in_the_ratio`), `kani/src/lib.rs:697-745` (`penalty_is_monotonic_for_full_stakes`) |
| **What is wrong** | Three Kani harnesses are marked "UNPROVED" in their documentation and "SLOW" in their annotations. They time out on every run because they call `penalty_for` twice with symbolic operands, creating two 128-bit symbolic divisions that CBMC cannot close. The comments at lines 366-463 document the measurement history in detail. The monotonicity of the slash penalty in the ratio is an open claim, not a proof. |
| **Why** | The CBMC bitvector solver cannot close a symbolic 128-bit division in CI time budget. Two such divisions in a single query is beyond the solver's capability at current bit widths. |
| **Effects** | Governance relies on monotonicity when it raises a slash ratio (a higher configured ratio should always produce a higher penalty). If the truncation in `(stake * ratio) / SCALE` is not monotonic for some stake values, a higher configured penalty could produce a smaller actual penalty, an incentive inversion. The property is plausible by inspection but not machine-verified. |
| **Fix** | No immediate fix; the solver limitation is fundamental. (1) Document the monotonicity claim as "UNPROVED" in the TLG report. (2) Consider a randomized property test (proptest/quickcheck) over `u64` operands as a supplement — it cannot prove the property but can falsify it with high probability. (3) Track the upstream Kani toolchain bump that raises the MSRV, which may allow a direct `dev-dependency` and more efficient encoding. |

---

## F-1208 — Fuzz workspace libp2p git rev may diverge from root workspace

| Field | Value |
|---|---|
| **ID** | F-1208 |
| **Location** | `fuzz/Cargo.toml:41` |
| **What is wrong** | The fuzz workspace pins `libp2p` to git rev `38b8a2c0e91bf6955f5357adcdd40d3b6683a0dd`. The root workspace also pins libp2p by git revision (documented in `.quality/deny.toml:126-138`). These are separate Cargo workspaces with separate lockfiles (`fuzz/Cargo.lock` vs root `Cargo.lock`), so there is no mechanism that forces the revisions to agree. |
| **Why** | The fuzz workspace is intentionally separate (libfuzzer needs its own profile), and manually synchronizing git revisions across workspaces is error-prone. |
| **Effects** | If the root workspace updates its libp2p pin (e.g., to close a security advisory) without updating `fuzz/Cargo.toml`, the fuzz targets exercise a different libp2p than production builds. The `reputation` fuzz target directly imports libp2p types, so a rev mismatch could mean the fuzzer tests stale network code. |
| **Fix** | Add a gate in `xtask/gates` (or extend `fuzz_targets_wired.rs`) that extracts the libp2p git rev from both `fuzz/Cargo.toml` and the root `Cargo.toml` and asserts they are equal. |

---

## F-1209 — ops/BACKUP_AND_RESTORE.md placeholder text

| Field | Value |
|---|---|
| **ID** | F-1209 |
| **Location** | `ops/BACKUP_AND_RESTORE.md` |
| **What is wrong** | The document references "the implementation track" as a placeholder phrase in the context of backup procedures, reading like an unfinished cross-reference to a work package or decision that was never filled in. |
| **Why** | The document was drafted as a template and the internal cross-reference was not resolved. |
| **Effects** | An operator following the backup runbook hits an incomplete instruction. TLG-23 requires that operational documentation is complete and accurate. |
| **Fix** | Replace the placeholder with the specific work package (IP-46) or decision reference, or remove the sentence if the context is no longer relevant. |

---

## F-1210 — rehearsal-topology.toml and topology.toml serve different purposes but have confusingly similar names

| Field | Value |
|---|---|
| **ID** | F-1210 |
| **Location** | `ops/rehearsal/topology.toml` (chain_id 45262), `ops/rehearsal/rehearsal-topology.toml` (chain_id 45261) |
| **What is wrong** | Two topology files in the same directory describe different networks: `topology.toml` describes the standalone 72-hour rehearsal network (chain_id 45262, devnet), while `rehearsal-topology.toml` describes the testnet ceremony rehearsal (chain_id 45261, testnet). Their names do not distinguish which is which; "rehearsal-topology" sounds like the more specific rehearsal file but is actually the testnet ceremony version. |
| **Why** | The two files were added at different times for different rehearsal workflows and were not renamed to clarify their scope. |
| **Effects** | An operator or automation script picking the wrong topology file gets the wrong chain_id. Using `topology.toml` (45262) for a testnet ceremony rehearsal would produce a genesis for the wrong network. The confusion is compounded because both files live in `ops/rehearsal/`. |
| **Fix** | Rename to clarify scope: e.g. `ops/rehearsal/standalone-network-topology.toml` (45262) and `ops/rehearsal/ceremony-rehearsal-topology.toml` (45261), or add a `purpose = "..."` field to each file. |

---

## Summary

| Severity | Count | IDs |
|---|---|---|
| Data integrity | 1 | F-1200 |
| Supply chain | 2 | F-1201, F-1208 |
| Naming / migration | 2 | F-1202, F-1205 |
| Schema consistency | 1 | F-1203 |
| Config clarity | 2 | F-1204, F-1210 |
| Verification gap | 2 | F-1206, F-1207 |
| Documentation | 1 | F-1209 |
| **Total** | **11** | F-1200 to F-1210 |
