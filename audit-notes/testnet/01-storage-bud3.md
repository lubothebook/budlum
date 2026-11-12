# How to build: B.U.D. 3.0 as sole storage system

Testnet index Section 1. Work packages IP-06, IP-07, IP-10, IP-18 through IP-25. TLG gates TLG-03, TLG-04.

---

## Current state

B.U.D. 3.0 is implemented in `src/storage/bud3/` with 30+ submodules. The full pipeline exists: content to payload to BOF-1 to segments to RFC 6330 fountain symbols. The storage registry (`src/storage/bud3/registry.rs`) tracks objects, segments, custody, audits, and repairs. Metrics are emitted from `blockchain.rs` (`emit_bud3_metrics`).

The old storage layer (`budzero/bud-node`) still exists. `src/main.rs` lines 1536-1573 import `bud_node` types for BitSwap and content store. This blocks TLG-03.

## What must be built

1. **Remove budzero/bud-node** (IP-07). Delete the crate. Remove its dependency from root `Cargo.toml`. Remove the import in `src/main.rs` lines 1536-1573. Keep `budzero/bud-vm`, `budzero/bud-compiler`, `budzero/bud-proof`, `budzero/bud-state` (these are BudZero EE, not old storage).

2. **Wire B.U.D. 3.0 into main.rs**. Replace `bud_node::BitSwap` and `bud_node::ContentStore` usage with B.U.D. 3.0 equivalents from `src/storage/bud3/`. The B.U.D. 3.0 content pipeline (`src/storage/bud3/pipeline.rs`) and the content gateway (`src/storage/content_gateway.rs`) are the replacement surfaces.

3. **Persistence layer abstraction** (IP-06). `src/persistence/db.rs` uses `sled::Db` directly. Abstract behind `src/persistence/traits.rs` (exists). Provide sled backend for full nodes and a mobile-compatible backend (redb or flat file) for `mobile/core/`. This unblocks TLG-10 (mobile validators) and resolves F-0044.

4. **B.U.D. 3.0 acceptance tests** (TLG-04). Tests T1-T12, T14-T26, T34-T41 must pass. These are in `src/tests/` and exercise segmentation, fountain coding, BOF-1 format, custody proofs, repair, and the storage deal lifecycle.

5. **Storage fee and Custody Endowment** (IP-25). The fee schedule is in `src/storage/bud3/fees.rs` (3 TESTNET-DEFAULT markers). The Custody Endowment is in `src/storage/bud3/endowment.rs`. Wire the fee into the transaction execution path.

## Key files

- `src/storage/bud3/` (30+ modules): full B.U.D. 3.0 implementation
- `src/storage/mod.rs`: re-export surface
- `src/storage/content_gateway.rs`: HTTP/content serving
- `src/persistence/db.rs`: sled backend (must abstract)
- `src/persistence/traits.rs`: trait definitions (exists)
- `budzero/bud-node/`: old storage (must delete)
- `src/main.rs` lines 1536-1573: bud-node imports (must replace)
- `mobile/core/DB_CHOICE.md`: mobile storage backend decision

## Blocking findings

- F-0001: bud-node still in Cargo.toml
- F-0005: main.rs uses bud-node types
- F-0030: two storage systems coexist
- F-0044: mobile share store in-memory only
- F-0051: sled in persistence layer
