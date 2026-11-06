# Lane C — Live verification ledger (execution, BudZero, domains, privacy, AI)

Status: OPEN (WP-03 in progress on branch arena/eba94906-budlum).

This file tracks the live verification status for findings whose closing package is in the execution / BudZero / domain / privacy / AI lanes (packages WP-26 through WP-35, WP-44, and WP-49 through WP-60, per THE_BUDLUM_DIREKTIF.md Section 15.8, card WP-03).

## Rules

Every row must have: finding id, area, English title, live status (`Verified`, `Pre-corrected`, `Invalid`, `Waits for removal`), test name, evidence file, closing package recommendation, importance recommendation.

Live verification uses the same nine-step protocol as WP-01 (audit/repro-c branch):
1. Record the branch, last commit SHA, and CI status in this file header.
2. Read the original Finding Ledger (external to the repo, source record) for each row's description and evidence.
3. Locate the reference by content, not by line number (`Bölüm 0.4`, live branch confirmation).
4. Assign a live status.
5. For `Verified`: reproduce with a red-output test on `audit/repro-c` (not merged; the closing package carries it into its own PR with red CI output and run number — `S-3`).
6. For `Pre-corrected`: write a green regression test that stays in `src/tests/audit_lane_c.rs` (`Bölüm 14.1` madde 7).
7. Note any divergence from the analysis base (`budlum-testnet-quantum.zip`, 2026-11-02).
8. New issues use `BLG-268` and onward numbering (`Bölüm 14.6`).

Special audits required by the WP-03 card:
- Engine inventory (`docs/audit/live/engine-inventory.md`): `revm`, `wasmi`, `solana-svm`, `solana-sbpf`, `aptos-vm` — version, license, `cargo-deny` pre-check result.
- `src/execution/executor.rs` `ContractCall` path (finding BLG-248).
- BudZero `Pcs::ZK` value and 64-bit commitment paths (BLG-223, BLG-224).
- Ethereum sync committee module production call site (BLG-235).
- Bridge findings BLG-043, BLG-057, BLG-105, BLG-194 — these four determine the bridge decision (`Bölüm 1.5`).

## Header (updated at start of work)

- Agent: arena agent (single agent mode, Section 15.1).
- Branch: arena/eba94906-budlum (current session branch).
- Base: main (cccf1da1f4bcfe97503a3a00cfba24b172b49b31).
- Start commit SHA: cccf1da1f4bcfe97503a3a00cfba24b172b49b31.
- CI status at start: unknown (no CI runner accessible in this sandbox; `S-1` applies only when a red result is observed).
- Package: WP-03 (Lane C), T-Launch, TLG required.
- Lock paths: `docs/audit/live/lane-c.md`, `docs/audit/live/engine-inventory.md`, `src/tests/audit_lane_c.rs`, `budzero/tests/audit_lane_c.rs`.

## Inventory (placeholder — filled as verification proceeds)

| Finding id | Area | English title | Live status | Test name | Evidence | Closing package | Importance |
|---|---|---|---|---|---|---|---|
