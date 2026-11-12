# Testnet items from the directive

Source: `THE_BUDLUM_DIREKTIF (1).md`, sections 1.5, 15.7, and 18.

The directive defines 64 work packages (WP-01 through WP-64). It defines 27
TLG (Testnet Launch Gate) items. Every TLG item must be green before the
testnet opens.

## Testnet feature matrix (Section 1.5)

Items marked "Open" must work at testnet launch. Items marked "Conditional"
must work if their blocking findings close. Items marked "Closed" are present
in code but rejected on the testnet chain id (45261). Items marked "Beta" do
not block launch.

### Open features (must work)

| Feature | Work packages | Status |
|---|---|---|
| PoS, committee-based finality, instant seat handover | WP-12, WP-13, WP-14 | Open |
| Full validator | WP-08 | Open |
| Mobile validator (Android) | WP-08, WP-20, WP-36 | Open |
| B.U.D. 3.0 general content | WP-18 to WP-25 | Open |
| B.U.D. 3.0 sealed content (same-owner dedup) | WP-22 | Open |
| B.U.D. 2.0 format-specific codecs 14-22 | WP-24 | Open (if gate passes) |
| EVM execution and Ethereum JSON-RPC | WP-27, WP-28 | Open |
| WASM execution (CosmWasm ABI) and Cosmos RPC subset | WP-29 | Open |
| SVM execution and Solana RPC subset | WP-31 | Open |
| Move execution (Aptos dialect) and Aptos REST subset | WP-32 | Open |
| BudZero execution (execution-only mode) | WP-33 | Open |
| Universal Call (cross-execution environment) | WP-30 | Open |
| Universal Asset Registry, EVM and WASM pointers | WP-30 | Open |
| Consensus domains (PoW, PoS, PoA, BFT, ZK) registration | WP-44 | Open |
| BNS, SocialFi, Data Market | WP-23 | Open |
| Explorer and indexer | WP-40 | Open |
| Faucet | WP-41 | Open |

### Conditional features

| Feature | Condition | Work packages |
|---|---|---|
| Bridge (lock, mint, burn, unlock) | BLG-043, BLG-057, BLG-105, BLG-194 close | WP-44 |
| AI inference layer (on-chain) | BLG-059 to BLG-062 close and VL-1 conditions met | WP-45 |

### Beta features (do not block launch)

| Feature | Work packages |
|---|---|
| Mobile validator (iOS, opportunistic class) | WP-38 |
| Migration Kit (EVM verified state transfer) | WP-34 |

### Closed features (must be rejected on testnet chain id)

| Feature | Work packages |
|---|---|
| B.U.D. 3.0 cross-owner secret dedup | WP-54 |
| Shared compression context and extra codecs | WP-54 |
| Live broadcast | WP-55 |
| SVM and Move pointers | WP-52 |
| Universal Object Layer | WP-49 |
| Remote App Binding | WP-49 then separate decision |
| Shared security | Existing, opens after Sepolia E2E test |
| Private transfer | WP-56 (PTP-1) |

## TLG (Testnet Launch Gate) items (Section 18)

Every item must be green. Evidence goes in `docs/audit/TLG_REPORT.md`.

| Code | Item | Source packages |
|---|---|---|
| TLG-01 | No open Critical or High findings. Closed findings have a test. | All |
| TLG-02 | Every "Closed" and closing "Conditional" feature has a `<feature>_is_closed_on_testnet` test. | All |
| TLG-03 | No legacy storage code. | WP-07 |
| TLG-04 | B.U.D. 3.0 acceptance tests green: T1-T12, T14-T26, T34-T41. Nightly tests T3, T4, T6, T7, T17 green for 7 nights. | WP-18 to WP-25 |
| TLG-05 | Committee-based finality and seat handover tests green. | WP-12, WP-13 |
| TLG-06 | Instant Task Handover tests and churn simulation green. | WP-14 |
| TLG-07 | Rehearsal T-1 passed: all Mobile validators left suddenly, finality delay unchanged, no object lost. | WP-46 |
| TLG-08 | Rehearsal T-2 passed: 10% of Full validators left suddenly, finality delayed at most 2 checkpoints. | WP-46 |
| TLG-09 | Rehearsals T-3, T-4, T-5, T-7 passed. | WP-46 |
| TLG-10 | Real Android phones ran as validators in rehearsal: stored content, responded to audits, served symbols, earned rewards. At least 20 phones. | WP-36, WP-46 |
| TLG-11 | Device matrix results exist: 24h service, battery budget, NAT matrix. | WP-37 |
| TLG-12 | EVM, WASM (CosmWasm ABI), SVM, Move (Aptos), BudZero conformance suites pass 100% or every exception is in the limits table. | WP-35 |
| TLG-13 | Tooling tests green in nightly: Foundry, Hardhat, viem, ethers, CosmJS, Anchor, solana CLI, solana/web3.js, aptos CLI, Aptos TypeScript SDK. Code runs unchanged. | WP-28 to WP-32 |
| TLG-14 | An EVM contract calls a CosmWasm contract in the same transaction and the result is atomic. Rehearsal T-6 passed. | WP-30, WP-46 |
| TLG-15 | Cross-architecture root equality green. | WP-35 |
| TLG-16 | Gas calibration benchmark met. | WP-35 |
| TLG-17 | Genesis ceremony rehearsal passed. Supply audit green. At least 8 Full validators. | WP-16, WP-46 |
| TLG-18 | NAT traversal: relay, DCUtR, and AutoNAT nightly tests green. Rehearsal logged relay, hole-punch, and direct connection rates. | WP-15, WP-46 |
| TLG-19 | Economy: pool sharing, storage fee, and rewards preserve supply. `economy_invariants` green. | WP-16, WP-25, WP-26 |
| TLG-20 | RPC: listener rules, method tables, and rate limits green. Rehearsal T-7 passed. | WP-28 to WP-32, WP-61 |
| TLG-21 | Faucet, explorer, dashboards, and alerts ran in rehearsal. | WP-40, WP-41 |
| TLG-22 | Independent reviews done. New Critical and High findings closed. | WP-47, WP-48 |
| TLG-23 | README.md equals Appendix A. README.tr.md equals Appendix B. Definition blocks preserved byte-for-byte. No placeholders. Numbers from CI and rehearsal output. | WP-43 |
| TLG-24 | Concept name gate green in absolute mode. | WP-43 |
| TLG-25 | CI green. All nightly workflows green for 7 nights. | WP-42 |
| TLG-26 | Release artifacts reproducible. SBOM and signatures published. | WP-41, WP-42 |
| TLG-27 | Bridge decision and AI layer decision written in TLG report and proved with tests. | WP-44, WP-45 |

## T-Launch work packages (48 packages)

These must complete before testnet launch.

| WP | Title | Lane | Milestone | Dependencies |
|---|---|---|---|---|
| WP-01 | Live validation: core | A | M0 | None |
| WP-02 | Live validation: storage | B | M0 | None |
| WP-03 | Live validation: execution, BudZero, domains, privacy, AI | C | M0 | None |
| WP-04 | Live validation: clients, ops, CI, docs, and combined status | D | M0 | None |
| WP-05 | Concept name map and counter gate | D | M1 | WP-04 |
| WP-06 | Persistence and state recovery move | B | M1 | WP-02 |
| WP-07 | Remove legacy storage systems | B | M1 | WP-06 |
| WP-08 | Validator classes and unified registry | A | M1 | WP-01 |
| WP-09 | Execution environment interface | C | M1 | WP-03 |
| WP-10 | B.U.D. 3.0 skeleton and common types | B | M2 | WP-07 |
| WP-11 | Mobile core and light client | D | M4 | WP-10, WP-12, WP-13, WP-15, WP-20 |
| WP-12 | Committee-based finality, instant seat handover, consensus hardening | A | M2 | WP-08 |
| WP-13 | Presence protocol and suspension | A | M3 | WP-12 |
| WP-14 | Instant Task Handover and disaster mode | A | M4 | WP-12, WP-13, WP-20, WP-21 |
| WP-15 | Network: NAT traversal and P2P hardening | A | M2 | WP-01 |
| WP-16 | Genesis, ceremony tool, and economics | A | M3 | WP-08 |
| WP-17 | State transition and chain hardening | A | M2 | WP-01, WP-06 |
| WP-18 | Segmentation and RFC 6330 fountain code | B | M3 | WP-10 |
| WP-19 | BOF-1 object format, QR presentation, and video commitment | B | M3 | WP-10, WP-24 |
| WP-20 | Class-aware placement, custody proof, and repair | B | M3 | WP-08, WP-18 |
| WP-21 | Upload, view, StorageTx v3, and RPC | B | M3 | WP-18, WP-19, WP-20 |
| WP-22 | Dedup, reference, delete, sealed content, and permissions | B | M3 | WP-21 |
| WP-23 | Dependent modules B.U.D. 3.0 migration | B | M4 | WP-22 |
| WP-24 | A0 codec registry and access tier | B | M3 | WP-10 |
| WP-25 | Storage fee, Custody Fund, and custody rewards | B | M4 | WP-16, WP-21 |
| WP-26 | Execution Coordinator and EE workers | C | M2 | WP-07, WP-09 |
| WP-27 | EVM execution environment and foreign envelope | C | M3 | WP-26 |
| WP-28 | Ethereum JSON-RPC gateway | C | M4 | WP-27 |
| WP-29 | WASM (CosmWasm ABI) and Cosmos RPC subset | C | M3 | WP-26 |
| WP-30 | Universal Call and Universal Asset Registry | C | M4 | WP-27, WP-29 |
| WP-31 | SVM and Solana RPC subset | C | M4 | WP-26 |
| WP-32 | Move (Aptos) and Aptos REST subset | C | M4 | WP-26 |
| WP-33 | BudZero execution environment and BudZero robustness | C | M3 | WP-26 |
| WP-34 | Migration Kit | C | Not blocking | WP-28 |
| WP-35 | Conformance, gas calibration, and cross-architecture | C | M4 | WP-27, WP-29, WP-31, WP-32, WP-33 |
| WP-36 | Android validator app | D | M4 | WP-11 |
| WP-37 | Mobile device matrix and field test | D | M4 | WP-36 |
| WP-38 | iOS opportunistic app | D | Not blocking | WP-11 |
| WP-39 | Wallet core and CLI | D | M4 | WP-08, WP-21, WP-27 |
| WP-40 | Explorer and indexer | D | M4 | WP-21, WP-26 |
| WP-41 | Faucet and operations | D | M4 | WP-16 |
| WP-42 | CI, gates, and supply chain | D | M2 | WP-04 |
| WP-43 | Docs, README, and TLG report | D | M6 | WP-46, WP-47, WP-48 |
| WP-44 | Domains, bridge, settlement, and contracts | A | M4 | WP-01 |
| WP-45 | Privacy, AI, and account abstraction: testnet boundary | B | M4 | WP-22 |
| WP-46 | Ceremony rehearsal, rehearsal network, and measurements | D | M5 | M4 |
| WP-47 | Independent review 1: core and storage | Cross (C or D) | M5 | M4 |
| WP-48 | Independent review 2: execution, mobile, and clients | Cross (A or B) | M5 | M4 |
| WP-61 | RPC and cryptography hardening | D | M2 | WP-01 |

## T-Expansion work packages (16 packages)

These do not block testnet launch. They merge after launch.

| WP | Title | Lane |
|---|---|---|
| WP-49 | Universal Object Layer | A |
| WP-50 | Move (Sui dialect) execution | C |
| WP-51 | Cairo execution | C |
| WP-52 | SVM and Move pointers | C |
| WP-53 | NEAR WASM, PolkaVM, TVM, UTXO script, Plutus | C |
| WP-54 | Shared compression context, extra codecs, secret dedup | B |
| WP-55 | Live broadcast | B |
| WP-56 | BudZero privacy core, predicate proofs, consent, PTP-1 | B |
| WP-57 | AI Verification Layer | A |
| WP-58 | F-Droid and reproducible Android build | D |
| WP-59 | Parallel execution | C |
| WP-60 | BudZero recursion, RV32, precompiles, domain finality proof, block batch proof | C |
| WP-62 | Universal Object Layer: CosmWasm and Cosmos SDK family | A |
| WP-63 | Universal Object Layer: SVM family | A |
| WP-64 | Universal Object Layer: Move family | A |

## Key decisions (Section 19)

42 decisions are binding. Key ones for testnet:

- KR-01: B.U.D. 3.0 is the only storage system. Legacy storage removed.
- KR-02: Testnet opens with fresh genesis. No migration.
- KR-03: Content custodians are validators. No separate storage operator.
- KR-04: Two validator classes: Full and Mobile. Mobile has Android and iOS opportunistic subclasses.
- KR-06: Committee-based finality: S = min(128, 4 * |E|), quorum = floor(2A/3) + 1 seats.
- KR-08: Non-appearance is not slashing. Only proven double-sign and proven faulty repair.
- KR-10: Stake minimum: Full 1000 BUD, Mobile 10 BUD. Mobile duty txs are free.
- KR-11: Erasure code is RFC 6330 (raptorq). K_MAX 4096.
- KR-14: Storage fee input is physical bytes. P_s = 1 BUD/GiB/year.
- KR-15: Network Security Pool: 10M BUD over 100K epochs. Split: consensus 40%, storage 50%, repair/relay 10%.
- KR-18: Execution environments run in separate worker processes (panic=unwind, UDS, protobuf).
- KR-19: T-Launch environments: BudZero, EVM (revm), WASM (wasmi + CosmWasm ABI), SVM (solana-svm), Move Aptos (aptos-vm).
- KR-24: Common gas unit is BGU. Block limit 60M BGU. Fee is EIP-1559 style.
- KR-34: Testnet params: chain id 45261, magic BDLT, slot 3s, epoch 50 slots, checkpoint every 5 blocks.
