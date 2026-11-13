# Storage Module Findings

Files reviewed: src/storage/mod.rs, src/storage/bud3/, src/persistence/

## F-0008: B.U.D. 3.0 module directory structure incomplete

- **Location**: `src/storage/bud3/mod.rs`
- **What is wrong**: The directive (Section 4.3, D-9) requires all B.U.D. 3.0 code to live in `src/storage/bud3/` and `crates/bud-codec/`. The current `src/storage/` directory still contains many modules from B.U.D. 1.0 and 2.0 (erasure.rs, lrc.rs, msr.rs, assignment.rs, lifecycle.rs, etc.) alongside the bud3 subdirectory.
- **Why it is wrong**: The kaldirma plan (Section 5) requires these to be removed after B.U.D. 3.0 is the sole storage system. Their continued presence means the codebase has two storage systems.
- **What it causes**: Confusion about which storage system is authoritative. Dead code that the `no_idle_code` gate should catch. Possible import conflicts.
- **How to fix it**: Execute IP-07 (remove legacy storage) after confirming all functionality has been migrated to B.U.D. 3.0 or `src/persistence/`.
- **Status**: open

## F-0009: Persistence layer migration partially complete

- **Location**: `src/persistence/mod.rs` (24 lines)
- **What is wrong**: The directive (Section 5.2) requires `db.rs`, `traits.rs`, `merkle_trie.rs` (renamed `state_trie.rs`), and `pruning.rs` to move from `src/storage/` to `src/persistence/`. The `persistence` module exists and contains these files, but the old `src/storage/db.rs` still exists at 128KB with the full implementation.
- **Why it is wrong**: If both paths compile, callers may import from either location, creating confusion.
- **What it causes**: The persistence module is the canonical location per the directive, but the old location may still be referenced.
- **How to fix it**: Verify all imports point to `src/persistence/`. If the old files are re-exports, convert them to deprecation warnings. If they are duplicates, remove them.
- **Status**: open

## F-0010: Content ID triple definition

- **Location**: `src/storage/content_id.rs`, `crates/budscan/src/content_id.rs`, and one in bud3
- **What is wrong**: BLG-073 documented three independent definitions of ContentId. The directive requires migration to a single definition in `src/storage/bud3/content_id.rs`. This finding verifies the status.
- **Why it is wrong**: Multiple ContentId definitions that could diverge silently, breaking hash equality.
- **What it causes**: If the definitions diverge, content addressing breaks: a content ID computed by one module would not match the same content computed by another.
- **How to fix it**: Ensure `budlum_types` exports a single `ContentId` type and all three locations re-export or use it.
- **Status**: open (references BLG-073)

## Module Quality Assessment

The persistence layer migration (IP-06) is structurally done. The B.U.D. 3.0 code in `src/storage/bud3/` is production-grade. Legacy storage code needs cleanup per IP-07.
