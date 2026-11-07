# Work packages

The directive divides the work into 64 packages and four agents. This file is
the repository half of that registry: one row per package, the lane that owns
it, whether it blocks the Testnet Launch Gate (TLG), what it depends on, the
milestone it lands in, and the open pull requests that currently carry it.

The GitHub half does not exist yet. This repository has issues disabled, so
WP-04 cannot open the 64 tracking issues the directive describes, and a
`CLAIM`, `LOCK-ADD` or `GLOBAL-LOCK` comment has nowhere to go. Until issues are
enabled, the "Open pull requests" column below is the claim record: a package
with no pull request is unclaimed.

A row says which package exists and who is carrying it. It does not say the
package is finished: a package is finished when every job on its card is on
`main`, its tests run in CI, the findings it closes carry a commit SHA, and the
TLG evidence exists. The live state of each finding is in
`docs/audit/AUDIT_FINDINGS.md`; the traceability matrix is in
`docs/audit/TESTNET_TRACEABILITY.md`.

## Reading the table

* **Lane**: the agent that owns the package (Section 15.2). Cross means either
  agent may hold it, as the card says.
* **TLG**: whether the package blocks the launch gate.
* **Depends on**: the packages that must be on `main` before this one starts.
* **Open pull requests**: measured against the 81 open pull requests on
  2026-11-07. A pull request that names two packages appears twice.

| WP | Package | Lane | Layer | TLG | Depends on | Milestone | Open pull requests |
|---|---|---|---|---|---|---|---|
| WP-01 | Live verification: core | A | T-Launch | yes | none | M0 | none |
| WP-02 | Live verification: storage | B | T-Launch | yes | none | M0 | #117 |
| WP-03 | Live verification: execution, BudZero, domains, privacy, AI | C | T-Launch | yes | none | M0 | none |
| WP-04 | Live verification: clients, operations, CI, documents, and the combined state | D | T-Launch | yes | none | M0 | #75, #77, #96, #118 |
| WP-05 | Concept name map and the counter gate | D | T-Launch | yes | WP-04 | M1 | #24, #120 |
| WP-06 | Persistence and state recovery move | B | T-Launch | yes | WP-02 | M1 | #60, #84 |
| WP-07 | Remove the legacy storage systems | B | T-Launch | yes | WP-06 | M1 | #69, #71, #104 |
| WP-08 | Validator classes and the single registry | A | T-Launch | yes | WP-01 | M1 | #40 |
| WP-09 | Execution environment interface | C | T-Launch | yes | WP-03 | M1 | #36, #38, #54, #64, #80 |
| WP-10 | B.U.D. 3.0 skeleton and shared types | B | T-Launch | yes | WP-07 | M2 | #56 |
| WP-11 | Mobile core and the light client | D | T-Launch | yes | WP-10, WP-12, WP-13, WP-15, WP-20 | M4 | #44, #82, #85, #88, #91 |
| WP-12 | Committee based finality, Instant Seat Handover, consensus hardening | A | T-Launch | yes | WP-08 | M2 | #81 |
| WP-13 | Presence protocol and suspension | A | T-Launch | yes | WP-12 | M3 | #45, #70 |
| WP-14 | Instant Duty Handover and disaster mode | A | T-Launch | yes | WP-12, WP-13, WP-20, WP-21 | M4 | none |
| WP-15 | Network: NAT traversal and P2P hardening | A | T-Launch | yes | WP-01 | M2 | #48 |
| WP-16 | Genesis, the ceremony tool, and the economy | A | T-Launch | yes | WP-08 | M3 | #34, #37, #39, #52 |
| WP-17 | State transition and chain hardening | A | T-Launch | yes | WP-01, WP-06 | M2 | #62, #66, #67 |
| WP-18 | Segmentation and the RFC 6330 fountain code | B | T-Launch | yes | WP-10 | M3 | #47 |
| WP-19 | The BOF-1 object format, QR presentation, and the video commitment | B | T-Launch | yes | WP-10, WP-24 | M3 | #45, #55 |
| WP-20 | Class aware placement, custody proof, and repair | B | T-Launch | yes | WP-08, WP-18 | M3 | #13, #59 |
| WP-21 | Upload, view, StorageTx v3, and RPC | B | T-Launch | yes | WP-18, WP-19, WP-20 | M3 | #57, #105 |
| WP-22 | Deduplication, reference, deletion, sealed content, and grants | B | T-Launch | yes | WP-21 | M3 | #45 |
| WP-23 | B.U.D. 3.0 migration of the dependent modules | B | T-Launch | yes | WP-22 | M4 | #36 |
| WP-24 | The A0 codec registry and the access layer | B | T-Launch | yes | WP-10 | M3 | #65 |
| WP-25 | Storage fee, Custody Endowment, and custody rewards | B | T-Launch | yes | WP-16, WP-21 | M4 | #40, #45 |
| WP-26 | The Execution Coordinator and the EE workers | C | T-Launch | yes | WP-07, WP-09 | M2 | #24, #36 |
| WP-27 | The EVM execution environment and the foreign envelope | C | T-Launch | yes | WP-26 | M3 | #78 |
| WP-28 | The Ethereum JSON-RPC gateway | C | T-Launch | yes | WP-27 | M4 | #56, #72, #75, #76 |
| WP-29 | WASM (CosmWasm ABI) and the Cosmos RPC subset | C | T-Launch | yes | WP-26 | M3 | none |
| WP-30 | Universal Call and the Universal Asset Registry | C | T-Launch | yes | WP-27, WP-29 | M4 | none |
| WP-31 | SVM and the Solana RPC subset | C | T-Launch | yes | WP-26 | M4 | #51, #83 |
| WP-32 | Move (Aptos) and the Aptos REST subset | C | T-Launch | yes | WP-26 | M4 | #89 |
| WP-33 | The BudZero execution environment and BudZero soundness | C | T-Launch | yes | WP-26 | M3 | #24 |
| WP-34 | Migration Kit | C | T-Launch | no (beta) | WP-28 | not a condition | #46, #61 |
| WP-35 | Conformance, gas calibration, and cross architecture | C | T-Launch | yes | WP-27, WP-29, WP-31, WP-32, WP-33 | M4 | #24, #36, #79 |
| WP-36 | Android validator application | D | T-Launch | yes | WP-11 | M4 | #92, #97 |
| WP-37 | Mobile device matrix and the field test | D | T-Launch | yes | WP-36 | M4 | #92 |
| WP-38 | iOS opportunistic application | D | T-Launch | no (beta) | WP-11 | not a condition | #100 |
| WP-39 | Wallet core and CLI | D | T-Launch | yes | WP-08, WP-21, WP-27 | M4 | none |
| WP-40 | Explorer and indexer | D | T-Launch | yes | WP-21, WP-26 | M4 | #73 |
| WP-41 | Faucet and operations | D | T-Launch | yes | WP-16 | M4 | #34, #52, #58, #59 |
| WP-42 | CI, gates, and the supply chain | D | T-Launch | yes | WP-04 | M2 | #24, #29, #30, #42, #63, #74, #75, #93, #95, #96, #98, #101, #103, #104, #106, #107, #109, #116, #119, #120 |
| WP-43 | Documents, README, and the TLG report | D | T-Launch | yes | WP-46, WP-47, WP-48 | M6 | #51 |
| WP-44 | Domains, bridge, settlement, and contracts | A | T-Launch | yes | WP-01 | M4 | #14 |
| WP-45 | Privacy, AI, and account abstraction: the testnet boundary | B | T-Launch | yes | WP-22 | M4 | #44, #49 |
| WP-46 | Ceremony rehearsal, the rehearsal network, and measurements | D | T-Launch | yes | M4 | M5 | #92, #99, #102 |
| WP-47 | Independent review 1: core and storage | cross (C or D) | T-Launch | yes | M4 | M5 | #56 |
| WP-48 | Independent review 2: execution, mobile, and clients | cross (A or B) | T-Launch | yes | M4 | M5 | #36, #43, #100 |
| WP-49 | The Universal Object Layer | A | T-Scale | no | M7, WP-30, WP-44 | not a condition | none |
| WP-50 | Move (Sui dialect) execution environment | C | T-Scale | no | WP-35 | not a condition | none |
| WP-51 | Cairo execution environment | C | T-Scale | no | WP-35 | not a condition | none |
| WP-52 | SVM and Move pointers | C | T-Scale | no | WP-30, WP-31, WP-32 | not a condition | none |
| WP-53 | NEAR WASM, PolkaVM, TVM, UTXO script, Plutus | C | T-Scale | no | WP-35 | not a condition | none |
| WP-54 | Shared compression context, the extra codecs, and sealed deduplication | B | T-Scale | no | WP-22, WP-24 (the hidden deduplication part also WP-56) | not a condition | none |
| WP-55 | Live streaming | B | T-Scale | no | WP-21 | not a condition | #86 |
| WP-56 | The BudZero privacy core, predicate proofs, consent, and PTP-1 | B | T-Scale | no | WP-22, WP-33 | not a condition | none |
| WP-57 | The AI Verification Layer | A | T-Scale | no | WP-45, WP-56 | not a condition | none |
| WP-58 | F-Droid and the reproducible Android build | D | T-Scale | no | WP-36 | not a condition | none |
| WP-59 | Parallel execution | C | T-Scale | no | WP-35 | not a condition | none |
| WP-60 | BudZero recursion, RV32, precompiles, domain finality proof, block aggregate proof | C | T-Scale | no | WP-56 | not a condition | none |
| WP-61 | RPC and cryptography hardening | D | T-Launch | yes | WP-01 | M2 | #34, #41, #53 |
| WP-62 | Universal Object Layer: the CosmWasm and Cosmos SDK family | A | T-Scale | no | WP-49, WP-29 | not a condition | none |
| WP-63 | Universal Object Layer: the SVM family | A | T-Scale | no | WP-49, WP-31 | not a condition | none |
| WP-64 | Universal Object Layer: the Move family | A | T-Scale | no | WP-49, WP-32 | not a condition | none |

## What the table shows today

* 44 of 64 packages have at least one open pull request; 20 have none.
* The packages with no open pull request: WP-01, WP-03, WP-14, WP-29, WP-30, WP-39, WP-49, WP-50, WP-51, WP-52, WP-53, WP-54, WP-56, WP-57, WP-58, WP-59, WP-60, WP-62, WP-63, WP-64.
* The most duplicated: WP-42 in 20 pull requests, WP-11 in 5 pull requests, WP-09 in 5 pull requests, WP-41 in 4 pull requests, WP-28 in 4 pull requests. Duplication is a merge conflict, not progress: two pull requests that implement the same package in different files cannot both land, and the second has to be reduced to what the first does not do.

The file collision map, and the order the stack can land in, is in `docs/audit/PR_STACK.md`.
