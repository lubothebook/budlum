# Testnet Items Index

Items that THE_BUDLUM_DIREKTIF.md names as necessary for the testnet (T-Acilis / T-Opening).
Each item maps to one or more work packages (IP) and TLG gates.

---

## 1. Storage: B.U.D. 3.0 as sole storage system

- Remove B.U.D. 1.0, B.U.D. 2.0, `bud/` workspace, `budzero/bud-node`. (IP-07, TLG-03)
- Move persistence layer to `src/persistence/`. (IP-06)
- Build B.U.D. 3.0 skeleton: types, limits, meter, forbidden-phrases gate. (IP-10)
- Segmentation and RFC 6330 fountain code. (IP-18)
- BOF-1 object format, QR presentation, video commitment. (IP-19)
- Class-aware placement, custody proof, repair. (IP-20)
- Upload, view, StorageTx v3, RPC. (IP-21)
- Deduplication, references, deletion, sealed content, permissions. (IP-22)
- Migrate dependent modules (BNS, SocialFi, Data Market, etc.) to B.U.D. 3.0. (IP-23)
- A0 codec registry and access tiering. (IP-24)
- Storage fee, Custody Endowment, custody rewards. (IP-25)
- B.U.D. 3.0 acceptance tests T1-T12, T14-T26, T34-T41 must pass. (TLG-04)

## 2. Validator network

- Two validator classes: Full and Mobile. Single registry. (IP-08, TLG-05)
- Committee-based finality and Instant Seat Handover. (IP-12, TLG-05)
- Presence protocol and suspension. (IP-13)
- Instant Task Handover and disaster mode. (IP-14, TLG-06)
- All Mobile validators depart; finality unchanged; no object lost. (TLG-07)
- 10% of Full validators depart; finality delays at most 2 checkpoints. (TLG-08)
- Rehearsal drills T-3, T-4, T-5, T-7 pass. (TLG-09)
- Real Android phones validate on rehearsal: store, audit, serve, earn. At least 20 phones. (TLG-10)
- Device matrix results: 24h service, battery budget, NAT matrix. (TLG-11)

## 3. Universal Execution Layer (UEL)

- Execution Coordinator and EE workers (separate processes). (IP-26)
- Execution environment interface (`crates/uel-abi`). (IP-09)
- EVM execution environment (revm). (IP-27)
- Ethereum JSON-RPC gateway. (IP-28)
- WASM execution environment (wasmi, CosmWasm ABI) and Cosmos RPC subset. (IP-29)
- Universal Call and Universal Asset Registry. (IP-30)
- SVM execution environment and Solana RPC subset. (IP-31)
- Move (Aptos) execution environment and Aptos REST subset. (IP-32)
- BudZero execution environment (execute-only mode). (IP-33)
- Conformance suites, gas calibration, cross-architecture determinism. (IP-35, TLG-12, TLG-15, TLG-16)
- Tool tests green in nightly: Foundry, Hardhat, viem, ethers, CosmJS, Anchor, solana CLI, aptos CLI. (TLG-13)
- EVM contract calls CosmWasm contract atomically in same transaction. (TLG-14)

## 4. Mobile validator

- Mobile core library (Rust, UniFFI bindings). (IP-11)
- Android validator application (Kotlin, Jetpack Compose). (IP-36)
- Mobile device matrix and field testing. (IP-37)
- iOS opportunistic application (beta, does not block launch). (IP-38)

## 5. Networking

- NAT traversal: relay, DCUtR, AutoNAT. (IP-15, TLG-18)
- P2P hardening: bounded bincode, Kademlia private-address reject, rate limits.

## 6. Consensus and chain

- Genesis, ceremony tool, economy. (IP-16, TLG-17)
- State transition and chain hardening. (IP-17)
- Genesis ceremony rehearsal passes. At least 8 Full validators. Supply check green. (TLG-17)
- Economy: pool splits, storage fee, rewards preserve supply. (TLG-19)

## 7. Domains, bridge, settlement

- Domain registry, bridge, settlement, contracts. (IP-44)
- Bridge is conditional: opens if BLG-043, BLG-057, BLG-105, BLG-194 close.
- Bridge decision and AI layer decision written in TLG report. (TLG-27)

## 8. Privacy, AI, account abstraction

- Privacy, AI, account abstraction: testnet boundary. (IP-45)
- AI on-chain layer conditional: opens at VL-1 if BLG-059 to BLG-062 close. (TLG-27)
- Private transfer closed until PTP-1 merges. (KR-26)

## 9. Clients and tooling

- Wallet core and CLI. (IP-39)
- Explorer and indexer. (IP-40, TLG-21)
- Faucet and operations. (IP-41, TLG-21)
- CI, gates, supply chain. (IP-42, TLG-25)
- RPC and cryptography hardening. (IP-61, TLG-20)

## 10. Naming and documentation

- Concept name map and counter gate. (IP-05, TLG-24)
- Documents, README, TLG report. (IP-43, TLG-23)
- README.md equals Appendix A. README.tr.md equals Appendix B. (TLG-23)
- Concept name gate in absolute mode, green. (TLG-24)

## 11. Rehearsal and review

- Ceremony rehearsal, rehearsal network, measurements. (IP-46)
- Independent review 1: core and storage. (IP-47, TLG-22)
- Independent review 2: execution, mobile, clients. (IP-48, TLG-22)

## 12. Release and CI

- CI green. All nightly workflows green for last 7 nights. (TLG-25)
- Release artifacts reproducible. SBOM and signatures published. (TLG-26)

## 13. Findings

- No open Critical or High findings. Unclosed finding's feature is closed on testnet with a test. (TLG-01)
- Every "Closed" and closing "Conditional" feature has a `<feature>_is_closed_on_testnet` test. (TLG-02)

## 14. Conditional and closed features at launch

Closed features (code exists, rejected on chain ID 45261, proven by test):

- B.U.D. 3.0 cross-owner private dedup. (IP-54)
- Shared compression context, extra codecs (8,12,13,6,7,23,24,25,26). (IP-54)
- Live streaming. (IP-55)
- SVM and Move asset pointers. (IP-52)
- Universal Object Layer. (IP-49)
- Remote App Binding. (post IP-49)
- Shared security. (Sepolia E2E test required)
- Private transfer. (IP-56, PTP-1)
- On-chain AI inference (conditional). (IP-45)

Open at launch:

- PoS, committee finality, instant seat handover.
- Full and Mobile validators.
- B.U.D. 3.0 general content and owner-scoped sealed dedup.
- B.U.D. 2.0 format-specific codecs 14 to 22 (if IP-24 gate passes).
- EVM, WASM, SVM, Move, BudZero execution environments.
- Universal Call (cross-EE).
- Universal Asset Registry (EVM and WASM pointers).
- Migration Kit (beta).
- Consensus domain registry.
- BNS, SocialFi, Data Market.
- Explorer and indexer.
- Faucet.

Conditional at launch:

- Bridge: opens if BLG-043, BLG-057, BLG-105, BLG-194 close.
- AI on-chain layer: opens at VL-1 if BLG-059 to BLG-062 close and VL-1 conditions met.
