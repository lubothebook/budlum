# P53 scan: xtask gates 4 (read-only audit, Tier R1)

Base commit: b76703e. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14, P58, P59), notes/P50-scan.md.

## COVERAGE

Read in full (every line, tests and canaries included), all under xtask/gates/src/gates/:
- no_idle_code.rs 1-1737 (two reads: 1-900, 900-1737)
- no_new_shell_gates.rs 1-203
- no_orphan_source_files.rs 1-862
- no_unicode_dashes.rs 1-310
- no_upstream_brands.rs 1-369
- node_classification_gate.rs 1-26
- one_house_guards.rs 1-452
- paid_content.rs 1-596
- parallel_execution.rs 1-365
- pinned_downloads.rs 1-245
- poa_compliance_gate.rs 1-30
- proof_deps_are_exactly_pinned.rs 1-152
- qr_is_derivative_only.rs 1-123
- quantum_account.rs 1-180
- readme_no_deny.rs 1-123
- reduction_claims.rs 1-151
- refusals_no_mutate.rs 1-527
- regen_health_door.rs 1-250

That is files 70 to 87 of the sorted list, 6701 lines. Docs read: docs/AGENT_MAP.md, docs/AUDIT_PLAN.md lines 110-165, notes/P29-scan.md lines 1-30.

Outside the part (rg first, small ranges): xtask/gates/src/gates/named_tests.rs 1-45; xtask/gates/src/main.rs 1335-1400 (runner); .github/workflows/ci.yml 508-528, 975-1000, 1715-1722 and rg on curl/gate names; src/domain/storage_deal.rs struct StorageRegistry and root() (about 1181-1299); src/domain/regen_health.rs rg only; src/gateway/service.rs 228-250, 318-335; src/pollen (rg only); src/registry/identity.rs 465-490; src/lib.rs 49-62; budzero/bud-proof/src/lib.rs 15-40; src/rpc/server.rs (rg only); Cargo.toml files (rg p3-); .github/idle-code-baseline.txt (wc only).

Not read: the gated source files beyond the ranges above, src/tests, rust_literals.rs. No cargo, no test run: every claim comes from reading source or rg output.

Extra-check summary:
- Wiring: all 18 gates are registered in main.rs and run in CI through `--all` and `--all --self-test` (ci.yml:1719-1721). Not a finding.
- Vacuous passes: found in 11 gates, listed per finding (missing evidence file counted as checked, absent-everywhere guard passes, skipped RPC check, ordering pin that compares against test code).
- Parser gaps: textual needles that also match comments and strings (one_house_guards, paid_content, quantum_account, regen_health_door, parallel_execution); chained or continued lines (pinned_downloads, refusals_no_mutate).
- Baseline handling: no_idle_code is the only baseline user. Baseline itself is read fail-closed. The loosening path is the WIRING exemption (P53-F01).
- Z3: no `#[allow]`, no `#[ignore]`, no `|| true` in the 18 files. The WIRING exemption and the PARTIAL: allowed doc line act as soft allow lists.
- Z6: no shell script in `git ls-files`. no_new_shell_gates only watches ops/scripts/check-*.sh (P53-F06).

## FINDINGS

### P53-F01
File: xtask/gates/src/gates/no_idle_code.rs:117-128, 728-788, 986-993
Severity: Medium
Axis: invariant(Z3)
Confidence: Certain
Evidence: `const EXEMPT_TOKENS: &[&str] = &["WIRING:", "Convenience:", "exposed for"];` (line 123). A token in the `//!` header sets `module = true` and exempts every idle item of the file (line 924: `if ex.module || ex.names.contains(&item.name)`). Item notes use a 14 line window above any `pub` line (779-782). rg: 104 lines carry a token, 50 files carry it in the module header. Header text `//! WIRING: unwired - this is a file-format schema` (src/sdk/mod.rs:51) exempts that whole module.
Description: One header line silences the ratchet for a whole file, with no cap and no baseline entry. The text after the token is not read, so "WIRING: unwired" counts as a documented door. One note also covers the next `pub` items within 14 lines.
Impact: A new idle item avoids the gate by adding a comment. The OK message only prints a count, so the loosening is not a red result. regen_health_door.rs:221 treats "WIRING: unwired" as a failure, so the two gates disagree about the same token.
Known: none

### P53-F02
File: xtask/gates/src/gates/no_idle_code.rs:93-102, 161-174
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `src[i..].find("#[cfg(test)]")` is the only test marker. rg: `#[cfg(test)]\npub mod tests;` (src/lib.rs:51-52), `pub mod trace_layout_tests;` (budzero/bud-proof/src/lib.rs:21-22), `mod tests; mod tests_backends;` (crates/bpqs/src/lib.rs:92-95), `#[cfg(all(test, feature = "bud3"))]` (bud/src/lib.rs:215), `#[cfg(all(test, feature = "wallet-ml-dsa"))]` (src/account_abstraction/threshold_mldsa.rs:267, tee_attestation.rs:248, private_transfer_auth.rs:207).
Description: Only the literal `#[cfg(test)]` is stripped. A test file declared by `#[cfg(test)] mod x;` is scanned as production (SKIP_DIRS skips a directory named `tests`, not a file named tests.rs). `cfg(all(test, ...))` and `cfg(any(test, ...))` blocks stay in the text.
Impact: Calls inside those files count as production callers, so an item used only by tests is reported as reached. The gate's own header says test callers must not count.
Known: none

### P53-F03
File: xtask/gates/src/gates/no_idle_code.rs:906-908, 112
Severity: Low
Axis: correctness
Confidence: Likely (missing step: no sample of a real false negative was run)
Evidence: `ids.contains(name)` over every identifier of every other production file; `const VACUITY_FLOOR: usize = 200;` against a stated 4600 items (line 22).
Description: Reach is by bare identifier. A common name such as `new` or `process` that is defined once is reached by any unrelated mention in another crate. A name used only in another idle file also counts. The floor of 200 fires only when 95 percent of the walk is lost.
Impact: The gate undercounts idle items and a half-broken walker still passes.
Known: none

### P53-F04
File: xtask/gates/src/gates/no_orphan_source_files.rs:64, 577-583
Severity: Medium
Axis: invariant (tools: wrong file pattern)
Confidence: Certain
Evidence: `const SCAN_ROOTS: &[&str] = &["src", "budzero", "wallet-core"];` then `if !base.is_dir() { continue; }`. `ls wallet-core` fails, the crate is `crates/wallet-core`. ci.yml:517 repeats the stale root list.
Description: The wallet-core root never exists, and a missing root is skipped silently. crates/* (bpqs, budscan, ai-inference, note-packing, wallet-core), bud/ and xtask are never checked for orphan or unreachable files.
Impact: An undeclared or unreachable .rs file in those crates (the exact failure the header describes) passes. The module doc says `crates/wallet-core/`.
Known: none

### P53-F05
File: xtask/gates/src/gates/no_orphan_source_files.rs:439-463, 465-488
Severity: Low
Axis: test
Confidence: Likely (missing step: no undeclared test file exists today, all 86 files in src/tests are declared; the gap is latent)
Evidence: `if matches!(p, "tests" | "benches" | "examples") { return true; }` inside a loop over every path component.
Description: A `tests`, `benches` or `examples` directory at any depth is treated as Cargo auto-discovered and as a crate root. Cargo discovers only `<crate>/tests` at the crate root. `src/tests/` is a module tree declared by `#[cfg(test)] pub mod tests;`.
Impact: A new file in src/tests that no `mod` declares is exempt and counted as a root, so it never compiles and its tests never run, with a green gate.
Known: none

### P53-F06
File: xtask/gates/src/gates/no_new_shell_gates.rs:32-52
Severity: Low
Axis: invariant(Z6)
Confidence: Certain
Evidence: `const SHELL_GATES_AT_MIGRATION_START: &[&str] = &[];` and `dir = root.join("ops/scripts")`, accepted names `name.starts_with("check-") && is_shell`. ops/scripts holds only `.gitkeep`. The doc says `scripts/check-*.sh`.
Description: The gate looks at one directory, non-recursive, for `check-*.sh`. A shell gate named otherwise, kept elsewhere, or a .bash, .py or .ps1 file passes. The path in the doc and the path in the code differ.
Impact: The gate gives no Z6 guarantee for the repo. A repo-wide check is only in the P62 manual command list. If `.gitkeep` is removed, `read_dir` fails and the gate turns red for the wrong reason.
Known: none

### P53-F07
File: xtask/gates/src/gates/no_unicode_dashes.rs:24-39, 99-103
Severity: Low
Axis: invariant(Z13)
Confidence: Certain
Evidence: `DASHES: [(char, &str); 8]` lists U+2010 to U+2015, U+2212, U+00AD. `SKIP_FILES = ["Cargo.lock", "flake.lock", "imports.lock", "LICENSE.md"]` matched by basename at any depth. `let Ok(text) = fs::read_to_string(&path) else { continue; };`.
Description: Missing long dashes: U+2E3A, U+2E3B, U+FE58, U+FE63, U+FF0D. A non-UTF-8 file is skipped whole, even when it holds an em dash. Any file named LICENSE.md, and any directory named `target` or `.cargo`, is skipped at every depth.
Impact: Z13 (no long dash) can be broken through those characters or files without a red gate.
Known: none

### P53-F08
File: xtask/gates/src/gates/no_upstream_brands.rs:174-199, 211-212, 61-95
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `.is_none_or(|c| !c.is_alphanumeric())` on the lowercased line, so `ColibriEngine` becomes `colibriengine` and is not a hit. `shown.truncate(120);` on a String. `ATTRIBUTION_STEMS` match by file stem in any directory.
Description: CamelCase use of a forbidden name (`ColibriEngine`, `JcodeMemory`) passes; the doc only promises snake_case. `String::truncate(120)` panics when byte 120 is inside a multi-byte character (the repo has Turkish text). Any file named LICENSE, NOTICE, THIRD-PARTY or PROVENANCE_NOTES in any directory is exempt. Extensions svg and lock are not scanned.
Impact: The brand rule leaks in Rust type names, and a hit on a non-ASCII line crashes the gate instead of reporting it.
Known: none

### P53-F09
File: xtask/gates/src/gates/named_tests.rs:15-27 (shared helper outside the part), used by node_classification_gate.rs:389-391 and poa_compliance_gate.rs:419-421
Severity: Low
Axis: test
Confidence: Likely (missing step: no CI log was inspected)
Evidence: `Some(pos) => line[pos + name.len()..].contains("ok")`. ci.yml:904 `cargo test --lib node`, ci.yml:987 `cargo test --lib poa_compliance`.
Description: A line passes when the test name appears anywhere in it and the letters "ok" appear after it. A longer test name that starts with the required name passes. A line `... ignored, reason: broken` contains "ok". The check does not see `#[ignore]` as a failure.
Impact: A required test that is ignored with a reason containing "ok", or replaced by a longer-named test, still satisfies the gate.
Known: none

### P53-F10
File: xtask/gates/src/gates/one_house_guards.rs:125-160
Severity: Medium
Axis: invariant (R3 registry, Merkle)
Confidence: Certain
Evidence: family "merkle-delegation" says "every committed Merkle root and proof must come from the one tree primitive". Not in the family: src/registry/identity.rs:472-485 `pub fn merkle_root(leaves: &[[u8; 32]])` with `let right = chunk.get(1).copied().unwrap_or(left);` (odd tail paired with itself) and `field_pair_hash`. The header of the gate names this shape (F-6, "the third duplicated it").
Description: The sibling list is hand written and does not include the registry identity root, which duplicates an odd tail. No gate or guard checks it.
Impact: The registry identity commitment can have the second-preimage shape the family was made to prevent (an odd leaf list and the same list with the last leaf repeated give the same root). Whether this root is consensus-bound was not traced.
Known: none

### P53-F11
File: xtask/gates/src/gates/one_house_guards.rs:241-267, 270-281, 293-312
Severity: Medium
Axis: invariant (supply ceiling, Z-class)
Confidence: Certain
Evidence: `if with.is_empty() { continue; }` (guard absent everywhere is not a finding). `body.contains(needle)` on raw text, including comments and strings. Guard "promotion binding" uses the needle `"promote"`. `src.find(start)` takes the first occurrence only. `body_of` counts raw braces (comment says strings are not special; char literals `'{'` and `'}'` are not handled either).
Description: If all siblings drop `ensure_mint_headroom` or `split_bridge_fee(`, the gate stays green. A comment containing the needle satisfies a guard. `promote` also matches any "promoted" or "promotes" in prose.
Impact: The 100M supply ceiling guard on the bridge mint paths can be removed from every path at once, or kept only in a comment, without a red gate. The canary (self_test lines 366-383) only removes the guard from siblings that share the file.
Known: none

### P53-F12
File: xtask/gates/src/gates/paid_content.rs:316-326, 361-372
Severity: Medium
Axis: integration
Confidence: Likely (missing step: whether gateway bytes can belong to protected content was not traced)
Evidence: rg -l `authorize_content_read` in src (not tests): only src/pollen/offers.rs (definition at 734). `authorize_read` callers: only src/pollen/content_gate.rs unit tests (324-362). src/gateway/service.rs:236 `self.chain.get_storage_manifest(cid).await` and :326 render the manifest bytes; rg `pollen` in that file: no match.
Description: The gate proves that the refusal functions exist (step 1, 4) but not that any read path calls them. Step 7 scans only src/ai. The gateway is a reach to storage without Pollen and is outside every step.
Impact: The headline claim "paid content cannot be read for free" is not proven. The refusal is unwired code today, a mainnet blocker by CLAUDE.md section 1.
Known: none

### P53-F13
File: xtask/gates/src/gates/paid_content.rs:43-61, 73-89, 146-188, 220-245, 279-358
Severity: Medium
Axis: invariant (vacuous pass, parser gaps)
Confidence: Certain
Evidence: steps 1, 2, 4 use `count_in_range(&gate_src, ...)` on text that is not comment stripped; the range ends at the first line equal to `"    }"` and runs to the end of file when none exists (line 59). Step 6: `if rpc.is_file() { ... }` (a moved file skips it) and `strip_prefix("    async fn ")` (rg: `pub async fn` appears in src/rpc/server.rs:358; src/rpc/api.rs is not scanned). Step 3 matches the single-line text `bindings.remove` (a rustfmt-wrapped `self.bindings\n.remove(` does not match) and reads only content_gate.rs. Step 7: `mentions_pollen = lower.contains("pollen")` exempts a whole file.
Description: A comment saying `GrantRequired` satisfies step 1. A comment saying `pub fn authorize_content_read` satisfies step 4. A file that mentions Pollen anywhere may read storage directly. `pollen_asset_for_content` anywhere in an endpoint body (even unused) marks it guarded. `"shardId":` is the only emission key.
Impact: Each control can be removed while keeping a mention, with a green gate. Endpoint split across files or a renamed key leaves the shard id exposure unchecked.
Known: none

### P53-F14
File: xtask/gates/src/gates/parallel_execution.rs:117-136, 193-209
Severity: Medium
Axis: invariant (state root)
Confidence: Certain
Evidence: `if (rest.contains("BTreeMap<") || rest.contains("BTreeSet<"))` is the only field filter and the type must sit on the same line. src/domain/storage_deal.rs: `pub dictionaries: crate::storage::dictionary::DictionaryRegistry` (line 726, a struct holding a BTreeMap), `pub view_grants: ...ViewGrantRegistry`, and `pub self_host_policies:\n BTreeMap<...>` (type on the next line, skipped). rg `dictionar` between lines 1181 and 1299 (root()): no match. Dictionary state is read in registration (`self.dictionaries.has_dictionary`, line 1624).
Description: The gate lists only fields declared with a BTreeMap or BTreeSet on one line. `dictionaries` is consensus state (reference counts decide manifest acceptance and deletion) and is not folded into `StorageRegistry::root()`. The gate cannot see it.
Impact: Two nodes, or a snapshot provider, can differ in dictionary state and still agree on the registry root, which feeds the account state root (src/core/account.rs:2462). The state root does not authenticate that field.
Known: none

### P53-F15
File: xtask/gates/src/gates/parallel_execution.rs:214-216, 160-172
Severity: Medium
Axis: invariant (vacuous pass)
Confidence: Certain
Evidence: `if body.contains(name.as_str()) { checked += 1; continue; }` on the raw body text (comments included). Field names: `deals` and `deals_by_shard` are both fields (storage_deal.rs struct). Unordered check: `body.lines().filter(|l| l.contains("HashMap") || l.contains("HashSet"))`.
Description: A field counts as folded if its name is a substring anywhere in the body: `deals` is satisfied by `self.deals_by_shard`, and any name is satisfied by a comment (root() has comments naming operator_cooldowns and others at 1186-1190). The HashMap check looks for the type name in the fold body, where a field iteration (`self.field.iter()`) never writes the type.
Impact: Removing the fold of `deals` while `deals_by_shard` stays passes. A new HashMap field that is folded in iteration order is not detected.
Known: none

### P53-F16
File: xtask/gates/src/gates/parallel_execution.rs:223-238, 41-110
Severity: Low
Axis: correctness
Confidence: Likely (missing step: no field with a second writer through another method was found)
Evidence: single-writer test patterns are `.entry(`, `.insert(`, `.remove(`, `.clear()` on `self.access_events`. `depth -= 1` on a `usize` in `span_of`. A lifetime tick (`&'a`) starts `in_char` and swallows code up to the next quote.
Description: Writes through `.get_mut(`, `.retain(`, `.extend(`, `.append(`, `.values_mut(` or a `&mut self.access_events` alias do not count as writes. The brace scanner treats a lifetime as a char literal.
Impact: The access_events exception can grow a second writer unseen. A body with a lifetime can mislocate the end of root(); a stray `}` underflows (panic in debug builds).
Known: none

### P53-F17
File: xtask/gates/src/gates/pinned_downloads.rs:458-465, 471-502, 499
Severity: Medium
Axis: invariant (supply chain, parser gap)
Confidence: Certain
Evidence: `if !(t.contains("curl") || t.contains("wget")) { return false; }` per line. The repo style puts the URL on the continuation line: extra-tooling.yml:150-151 `curl -sSfL -o /tmp/machete.tar.gz \` then `https://github.com/.../cargo-machete-v0.8.0-...tar.gz`. Same shape in grype.yml:59, typos.yml:45, ci.yml:269 and others.
Description: The URL line of a continued command contains no `curl`, so a checksum URL there is never tested. URLs built from variables (`$BASE/$NAME.sha256`) have no `https://` token. Only `.github/workflows/*.yml` is read (not composite actions or ops/). The "verifying" test is `text.contains("sha256sum -c")` anywhere in any workflow. A download with no checksum at all passes.
Impact: A remotely fetched checksum, written in the repo's normal two line style, passes. Today all 19 `curl -sSfL` calls have a matching `sha256sum -c` (grep count per file), so this is a latent gap.
Known: none

### P53-F18
File: xtask/gates/src/gates/proof_deps_are_exactly_pinned.rs:27, 44-66, 90
Severity: Medium
Axis: invariant (supply chain, soundness boundary)
Confidence: Likely (missing step: cargo semantics of a partial version after `=` not run; Cargo treats `=0.7` as `>=0.7.0, <0.8.0`)
Evidence: `if !version.starts_with('=')` only. `rest.split("version").nth(1)?` returns None for `{ workspace = true }` and `{ git = ... }`, and the caller does `continue`. `MANIFESTS = ["budzero/bud-proof/Cargo.toml"]`. rg p3-: only bud-proof/Cargo.toml lines 18-32 today, all `"=0.7.0"`.
Description: `p3-fri = "=0.7"` passes though the doc says `=x.y.z`. Inherited (`workspace = true`), git and `[dependencies.p3-x]` table forms are skipped silently. A p3 dependency added to another manifest (bud-vm, bud-node, root) is never checked. The self-test only exercises `dependency()`, never `run()`.
Impact: A patch release of a Plonky3 crate can change the proof system silently, which the gate exists to prevent (CVE-2026-46654 in the header).
Known: none

### P53-F19
File: xtask/gates/src/gates/qr_is_derivative_only.rs:186-199, 208-217
Severity: Low
Axis: invariant
Confidence: Certain
Evidence: `render.contains("`QrStream` is deliberately not reachable from the RPC")` and `contains("not a proof of storage")` are checks on prose. `strip_line_comments` is `l.split("//").next()`. Forbidden names are four literal strings in src/domain/storage_deal.rs only.
Description: The RPC unreachability claim is enforced as a sentence in render.rs, not on src/rpc/server.rs. Other deal surfaces (src/domain/deal_open.rs, storage_tx.rs) and other spellings (`QrFrame`, `qr_`) are not scanned.
Impact: QrStream can be wired into an RPC method or a bond path with a green gate while the sentence stays.
Known: none

### P53-F20
File: xtask/gates/src/gates/quantum_account.rs:305-324, 335-394
Severity: Low
Axis: invariant
Confidence: Certain
Evidence: `reg.matches(".validate_all()").count()` with `calls >= 2` over the whole registry.rs (tests and comments included). `seed.contains("SeedError") && seed.contains("entropy.len()")` on raw body text. `depth -= 1` in body_of without string handling. Current code: validate_all calls at registry.rs:136 and :253.
Description: The check is a count, not a binding to the insert and replace functions. Needles are matched in comments and strings. The gate never reads `validate_all` itself, so a validate_all that returns Ok(()) passes.
Impact: Removing the call from `replace` while keeping two mentions elsewhere (a test, a doc line) passes.
Known: none

### P53-F21
File: xtask/gates/src/gates/readme_no_deny.rs:518-522, 465-483, 485-500
Severity: Low
Axis: invariant (vacuous pass)
Confidence: Certain
Evidence: `if !evidence.is_file() { checked += 1; continue; }` (a missing evidence file counts as a checked pair). Denial match is a lowercase substring of the whole README.
Description: A moved or renamed evidence file turns the pair into a pass. A denial wrapped over two lines, or reworded, does not match. Only two pairs exist, both for src/storage/README.md.
Impact: README drift on other modules is not watched. The gate can report "2 pairs agree" while checking nothing.
Known: none

### P53-F22
File: xtask/gates/src/gates/reduction_claims.rs:12-31, 88-102
Severity: Low
Axis: invariant (vacuous pass)
Confidence: Certain
Evidence: `lower.contains("files") && lower.contains('%')` and `lower.contains("bytes") && lower.contains('%')` over the whole document; `text.contains("447")`.
Description: The document level test passes when the words "files", "bytes" and any percent sign appear anywhere, not per claim. "447" matches any number containing it (for example 1447). Only the JSON table row is cross-checked.
Impact: A single-unit claim in a new paragraph is not found. The gate also hard-codes one research figure, so a corrected corpus table needs a code change.
Known: none

### P53-F23
File: xtask/gates/src/gates/refusals_no_mutate.rs:267-274, 281-338, 387-421
Severity: Medium
Axis: invariant (partial write)
Confidence: Likely (missing step: no concrete unflagged function was run through the gate)
Evidence: `l.contains("return Err(")` is the only later-error test; no `?`. Receiver must be exactly `self.<ident>.remove(`. Restore test is any `.insert(` between the first remove and the error line (`lines[i].contains(".insert(")`). `t.contains("-> Result<")` (so `io::Result<`, `anyhow::Result<`, aliases do not match). Only `.remove(`; no `take`, `pop`, `drain`, `retain`, `mem::take`, `swap_remove`, `clear`. rg: 81 production lines match `self.<ident>.remove(`.
Description: The usual Rust refusal after a mutation is `?`, which the gate does not see. Any unrelated `.insert(` on any collection counts as a restore. Receivers like `state.x.remove(` (local binding) are out of scope by design. `PARTIAL: allowed` needs no reason text.
Impact: Functions that remove an entry and then fail through `?` leave a partial write with a green gate. The 81 remove sites are the surface that is partly covered.
Known: none

### P53-F24
File: xtask/gates/src/gates/refusals_no_mutate.rs:12, 196-246
Severity: Medium
Axis: invariant (wrong file pattern)
Confidence: Certain
Evidence: `const SCAN_ROOTS: &[&str] = &["src", "budzero", "wallet-core"];` with `if !base.is_dir() { continue; }`. `is_test_path`: `rel.contains("/tests/") || rel.ends_with("_tests.rs") || rel.ends_with("/tests.rs")`.
Description: Same stale root as P53-F04: crates/* (including crates/wallet-core) and bud/ are never scanned, and a missing root is skipped silently. Files are excluded by name, so a production file that is named `*_tests.rs` is skipped whole.
Impact: Partial-write refusals in the standalone crates are unchecked.
Known: none

### P53-F25
File: xtask/gates/src/gates/refusals_no_mutate.rs:162-193, 429
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `let between = &after["#[cfg(test)]".len()..brace_rel]; if !between.contains("mod")` where `brace_rel` is the next `{` anywhere. `src[..brace].matches('\n').count()` runs on the stripped text.
Description: `#[cfg(test)] pub mod tests;` (src/lib.rs:51-52) has no brace, so the strip takes the next brace block in the file and drops it; any `#[cfg(test)] use ...model...;` also matches the substring "mod". The next item (possibly production) is then not scanned. Reported line numbers are computed on the stripped text and drift after any removed block.
Impact: Production functions that follow such an attribute can be hidden from the gate. In src/lib.rs and budzero/bud-proof/src/lib.rs no production fn follows today.
Known: none

### P53-F26
File: xtask/gates/src/gates/regen_health_door.rs:245-275
Severity: Medium
Axis: invariant (vacuous pass)
Confidence: Certain
Evidence: `let body = &health[start..];` takes the file from `pub fn assess(` to the end, and the last verdict is `body.rfind("HealthVerdict::RevertTo")`. rg in src/domain/regen_health.rs: `pub fn assess(` at 196, RevertTo at 244, 344, 411, 561, `#[cfg(test)]` at 355. The doc comment says "Each is a code token (not a comment)", but the text is not stripped.
Description: rfind lands on line 561, in the test module, so "the final verdict is last" is always true. `body.find` for the other tokens takes the first occurrence in any comment or later function. The ordering pin (audit trail, canonicality, economics, verdict) is therefore not bound to `assess`.
Impact: Reordering the decision ladder, which the header calls the substance of the module, can pass. Canary 4 removes a token instead of testing order alone.
Known: none

### P53-F27
File: xtask/gates/src/gates/regen_health_door.rs:276-286, 215-220
Severity: Low
Axis: test
Confidence: Certain
Evidence: `chaos.contains("RECOVERY_WINDOW_BLOCKS")` and `chaos.contains(name)` for four test names. src/tests/regeneration_recovery.rs:27 `const RECOVERY_WINDOW_BLOCKS: u64 = 64;`. `mod_rs.contains("pub mod regen_health;")`.
Description: Presence of a name is all that is pinned. The constant value is free, a test name in a comment or an `#[ignore]` test keeps the gate green, and `pub mod regen_health;` may sit in a comment or behind a cfg.
Impact: The recovery window can be raised to any value, or a chaos test disabled, without a red gate (the generic ignore scan is P62).
Known: none

## SUMMARY

Findings: 27. Critical 0, High 0, Medium 13 (F01, F04, F10, F11, F12, F13, F14, F15, F17, F18, F23, F24, F26), Low 14, Info 0.

Questions for the main session (not findings): none needed to continue.
