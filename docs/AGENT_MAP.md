# Agent map

Read this first instead of README.md. It is the short map for every agent (scout, finder, architect, coder, coder-deep, coder-lite). If something you needed is missing here, tell the main session so it can be added. Do not re-explore the layout.

## What Budlum is

Rust workspace for the Budlum Universal Settlement Layer: PoW, PoS, BFT and isolated PoA domains with one verifiable finality layer on top. Also in this tree: BudZero (zk VM, Plonky3 STARK), B.U.D. (content-addressed storage), BNS (names), Pollen (consent data market), AI inference layer, wallet core. Fixed supply 100M BUD with a double burn. Public text must not use the term for layer one (rule Z12).

## Where things are (lines are approximate)

| Path | What | Tier |
|---|---|---|
| src/consensus (5.3k) | PoW, PoS, PoA, BFT engines, QC finality | R3 |
| src/chain (20k) | blockchain.rs (state machine, replay, reorg), chain_actor.rs (command loop), snapshot, genesis | R3 |
| src/crypto (3k) | Ed25519, BLS, ML-DSA, PKCS#11 | R3 |
| src/privacy (0.7k) | note registry, nullifiers | R3 |
| src/tokenomics (1k) | supply, burn, vesting, rewards | R3 |
| src/settlement (5k) | proof market, cold wallet, PQ anchor | R3 |
| src/cross_domain (4k) | bridge, messages, relayer, nonce store, evm/ and external/ | R3 |
| src/registry (8k) | stake registry, slashing, invalid votes, PoA onboarding, identity | R3 |
| src/network/node.rs | libp2p node, gossip handling | R3 |
| budzero/bud-proof | STARK AIR and prover | R3 |
| src/core (11k) | accounts, blocks, transactions, state root (account.rs) | R2 |
| src/execution (4k) | transaction executor (executor.rs) | R2 |
| src/rpc (10k) | JSON-RPC (server.rs), operator and public listeners | R2 |
| src/storage (33k), bud/ | B.U.D. storage, QR video, erasure coding | R2 |
| src/domain (17k) | domain registry, finality adapters, storage deals | R2 |
| src/ai, src/ai_inference, src/account_abstraction | AI registry and execution, AA | R2 |
| src/bns, socialfi, pollen, gateway, relayer, bin, cli | apps and binaries | R1 |
| src/tests (34k, 86 files) | integration tests | R1 |
| xtask/gates (130 files) | CI gate programs | R1 |
| budzero/{bud-vm,bud-isa,bud-compiler,bud-node,bud-cli,bud-state,verifier-registry} | VM, compiler (BudL), node | R2 |
| crates/{bpqs,wallet-core,budscan,ai-inference,note-packing} | standalone crates with own lock files | R1 to R3 |
| contracts/, proto/, config/, ops/, .github/ | Solidity verifier, wire schemas, genesis profiles, deploy, CI | R1 |

Tiers are in MODEL_ROUTING.md section 3. Huge files (never read whole): src/chain/blockchain.rs, src/chain/chain_actor.rs, src/rpc/server.rs, src/network/node.rs, src/core/account.rs, src/execution/executor.rs, src/ai/mod.rs, src/domain/storage_deal.rs, budzero/bud-proof/src/plonky3_prover.rs and plonky3_air.rs, docs/ARCHITECTURE.md.

## Build, test, CI

- Root crate is the package at the repo root. budzero/ and crates/* and bud/ are separate workspaces with their own Cargo.lock.
- Do not run cargo, clippy, fmt, typos, deny, audit, gates, miri or kani locally. GitHub Actions runs them (.github/workflows). Push, then read the result with the GitHub tools; fetch logs only for the red step.
- Files you must not edit: .github/*-baseline.txt (dead-pub-api, idle-code, suppression budgets).
- A public item with no production caller is a finding (idle code). Check with `rg -n '<symbol>' src/ budzero/ crates/`.

## State and docs

- STATUS.md: active work and queues. CLAUDE.md and MODEL_ROUTING.md: rules.
- Audit: docs/AUDIT_PLAN.md (method, 62 parts, prompt templates), docs/AUDIT_PROGRESS.md (state), docs/audit/FINDINGS.md (findings, kept in repo during testnet).
- Design docs only when needed: docs/ARCHITECTURE.md (read `rg -n '^## '` first), docs/SPECIFICATION.md, docs/SECURITY.md, budzero/ARCHITECTURE.md, bud/README.md.

## Words

- Domain: one external consensus network registered with the settlement layer. Finality adapter: checks that domain's finality proof.
- QC: quorum certificate (BLS). Replay: rebuilding state from blocks; live path: applying a new block. Both must give the same state root.
- Registry: stake roles (validator, relayer, prover, storage operator, AI operator) and their slashing.
- Z-rules: CLAUDE.md section 3. Tiers R0 to R3: MODEL_ROUTING.md section 3.
