# P51 scan: xtask gates 2 (files 22 to 44, read-only audit, Tier R1)

Base commit: e7f75d6. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14, P58, P59).

## COVERAGE

Read in full (every line, tests and self-tests included), all under xtask/gates/src/gates/:
- coding_audit_samples_the_relationship.rs 1-789
- consensus_maps_ordered.rs 1-741
- consensus_state_only_changes_in_blocks.rs 1-306
- containment_defaults.rs 1-123
- content_encryption_is_declared_and_bound.rs 1-788
- coverage.rs 1-117
- cross_table_checks.rs 1-155
- dead_pub_api.rs 1-466
- derived_content.rs 1-170
- docker_toolchain.rs 1-316
- domain_tags.rs 1-323
- economy_invariants.rs 1-46
- every_fuzz_target_is_run.rs 1-366
- every_opcode_forgery.rs 1-308
- evidence_provenance.rs 1-478
- exact_named_tests.rs 1-90
- fixture_integrity.rs 1-132
- forgery_tests.rs 1-303
- fork_choice_gate.rs 1-34
- fsf_import_boundaries.rs 1-466
- fuzz_targets_wired.rs 1-185
- gas_calibration.rs 1-291
- gates_are_wired.rs 1-544
(23 files, 7537 lines.)

Docs read: docs/AGENT_MAP.md 1-60; docs/AUDIT_PLAN.md 110-165; notes/P29-scan.md 1-30.

Outside the part (rg first, at most 80 lines): xtask/gates/src/gates/named_tests.rs 1-83 (used by economy_invariants.rs and fork_choice_gate.rs); xtask/gates/src/gates/rust_literals.rs exclusive_scratch_dir (rg -A25); xtask/gates/src/gates/guards_reachable.rs strip_test_mods 70-107; xtask/gates/src/gates/module_coverage.rs (rg only, lines 95-102); xtask/gates/src/main.rs 1336-1395 (runner); budzero/bud-vm/src/lib.rs gas_cost (45 lines); src/execution/zkvm.rs 466-490; src/storage/manifest.rs 93-135 (and rg); src/core/account.rs 2300-2335; src/core/transaction.rs 1-5; src/lib.rs (rg); .github/workflows/ci.yml 1575-1625, 1700-1723 (rg and short ranges); .github/workflows toolchain lines (rg); ops/Dockerfile FROM lines (rg); ops/scripts (ls); .github/coverage-baseline.txt.

Not read: the real sources the gates inspect (blockchain.rs, executor.rs, plonky3_prover.rs, erasure.rs, storage_deal.rs, registry files) beyond the rg hits above. No cargo, no test run, no git history. CI gate output was not fetched, so claims about real-tree behaviour come from reading source only.

Extra-check summary:
- Vacuous passes: found in F01, F02, F03, F04, F05, F09, F10, F22. Gates fail closed on missing files in most cases; they pass open on unreadable or non-UTF-8 files (F22, F14, F17).
- Regex or parser gaps: F03, F07, F08, F09, F16, F18, F19.
- Baseline handling: dead_pub_api tightens correctly (stale entries fail). coverage baseline has no lowering guard and accepts NaN (F12). No gate in this part compares a baseline against the base branch.
- Z3 softening: no `#[allow]`, `#[ignore]`, `|| true` or `continue-on-error` in the 23 files. Gaps that let a Z3 violation through: F09 (ignored required test), F07, F21 (`|| true` counted as wired).
- Z6: `git ls-files | rg '\.(sh|bash|py)$'` returns nothing. ops/scripts holds only .gitkeep, so gates_are_wired.rs is a no-op today (F21).
- Panics in non-test code: one, containment_defaults.rs:38 (F19). The `unwrap` hits are in fixtures and #[cfg(test)] only.

## FINDINGS

### P51-F01
File: xtask/gates/src/gates/forgery_tests.rs:130-137
Severity: Medium
Axis: test
Confidence: Certain
Evidence: `let test_before_fn = blob.find(&format!("fn {name}(")).is_some_and(|fn_pos| blob[..fn_pos].rfind("#[test]").is_some());` where `blob` is every .rs file under budzero/ concatenated (collect_sources 37-67).
Description: Any `#[test]` anywhere earlier in the concatenated blob satisfies the "is a real test" check for every required name. The canary (205-213) only works because its fixture has no earlier `#[test]`.
Impact: In the real tree a required forgery test can lose `#[test]` or become a plain fn and the gate stays green. The 23 required R3 ZK tests are then not guaranteed to run.
Known: none

### P51-F02
File: xtask/gates/src/gates/containment_defaults.rs:77-82
Severity: Medium
Axis: invariant (VerifyMerkle gate stays closed)
Confidence: Certain
Evidence: `if !exec_code.contains("verify_merkle_enabled") && !exec_code.contains("merkle_enabled")` over src/execution/zkvm.rs. `rg -n 'verify_merkle_enabled' src/execution/zkvm.rs` gives lines 471 and 478 only: 471 is a doc comment (stripped), 478 is `assert!(!d.verify_merkle_enabled, ...)` inside `#[test] fn the_staged_rollout_defaults_are_the_ones_in_force` (zkvm.rs:476).
Description: Check 3 says the execution layer consults the flag at run time. The only mention is a unit test. The second operand `merkle_enabled` is a substring of the first, so it adds nothing.
Impact: The gate passes with no production consultation of the flag. Containment rests only on bud-vm decoding with default(). Checks 1 and 2 cover that, but check 3 gives false assurance and would stay green if the decode site were replaced by another path.
Known: none

### P51-F03
File: xtask/gates/src/gates/evidence_provenance.rs:171 and 209
Severity: Medium
Axis: security (slashing provenance)
Confidence: Certain
Evidence: `let prod = text.split("#[cfg(test)]").next().unwrap_or(&text);`. src/core/transaction.rs line 2 is `#[cfg(test)]` on `fn test_addr_from_byte` (one item, 2818 lines in the file). `rg -U` finds single-item `#[cfg(test)]` before production code also in cross_domain/external/ethereum.rs:558 and cross_domain/message_registry.rs:74.
Description: The production text is cut at the first `#[cfg(test)]`, which also marks single items, not only the test module. Everything after it is not scanned for `.slash(`. In transaction.rs the gate sees 1 line of 2818. Also the detector needs a leading dot, so `PermissionlessRegistry::slash(&mut r, ...)` is not seen. File read errors become empty text (`unwrap_or_default`, 170, 184, 206-207), so an unreadable file is "clean".
Impact: A new caller of the bare `slash` in any file with an early single-item cfg(test) bypasses check 3, and a stake cut can then skip `is_actionable`.
Known: none

### P51-F04
File: xtask/gates/src/gates/dead_pub_api.rs:211-215 (doc claim at 15-21)
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: `let scan_text = if has_rs_extension(&rel) { strip_test_mods(&text) } else { text.clone() };` strip_test_mods (guards_reachable.rs:70-107) removes only inline `#[cfg(test)]` blocks and `#[cfg(test)] x;` lines. src/lib.rs:51-52 is `#[cfg(test)] pub mod tests;` (out of line, 86 files in src/tests).
Description: References from src/tests/*.rs, `*_tests.rs`, budzero and crates test files count as live references. The doc says a name used only from a test is dead.
Impact: A pub fn called only from the 34k lines of integration tests is "wired" for the ratchet. This is the exact "tested is not wired" confusion the gate is meant to stop (AGENT_MAP: call only from a test is a finding). The dead-code baseline undercounts.
Known: none (baseline files not edited)

### P51-F05
File: xtask/gates/src/gates/economy_invariants.rs:41 and fork_choice_gate.rs:29 (shared code: named_tests.rs:17-20, outside P51)
Severity: Medium
Axis: test
Confidence: Certain (logic); Likely for the panic-line case (log content not fetched)
Evidence: `match line.find(name) { Some(pos) => line[pos + name.len()..].contains("ok"), None => false }`
Description: A line matches if the name appears and the two letters "ok" appear anywhere after it. The name is matched as a substring, so a longer test name also matches. A libtest failure line `thread 'tests::x::<name>' panicked at src/tokenomics/...` contains the name followed by "ok" (in "tokenomics"). A removed test is hidden by any test whose name starts with it, for example `<name>_v2 ... ok`.
Impact: 26 economy invariant tests and 11 fork-choice tests can be deleted, renamed with a suffix or failing and the gate stays green. Contrast exact_named_tests.rs:26-27 which compares the whole line.
Known: none

### P51-F06
File: xtask/gates/src/gates/coverage.rs:32-43
Severity: Medium
Axis: correctness
Confidence: Likely. Missing verification: read coverage/cov.json from a CI run and compare the percent the gate prints with `totals.lines.percent`.
Evidence: `let lines_block = text.find("\"lines\":{")?;` then the first `"percent":` after it. CI writes the file with `cargo llvm-cov nextest --lib --json` (ci.yml:1619). llvm-cov export emits `{"data":[{"files":[{"filename":..,"summary":{..,"lines":{..,"percent":..}}}],"functions":[..],"totals":{..}}]}`; files come before totals. module_coverage.rs:95-102 reads `f["summary"]["lines"]` per file with serde_json, so it knows the per-file shape.
Description: The first `"lines":{` in that layout belongs to the first file, not to the totals. The self-test fixture (77) contains only a `totals` object, so it cannot see this.
Impact: The line-coverage ratchet compares the first file's percent with 64.30. The gate can go red or green for reasons unrelated to total coverage.
Known: none

### P51-F07
File: xtask/gates/src/gates/every_opcode_forgery.rs:150, 215-227, 245
Severity: Medium
Axis: test (ZK soundness coverage)
Confidence: Certain
Evidence: `let isa_src = strip_comments(...)` but `let prover_src = std::fs::read_to_string(&prover_path)...` (150) is not stripped. An opcode counts as attacked when `Opcode::X,` or `Opcode::X)` appears anywhere in the body of any `rejects_*` fn whose body contains `COL_` or `prove_fails_after_tamper` (217-226). Refusal needs only `is_err()`, `"accepted"` and similar substrings anywhere in the body (32-45).
Description: Three weaknesses. 1) Comments count: `// assert!(r.is_err())` or `// COL_X` satisfies the checks. 2) Setup counts: an opcode in the honest program of another opcode's forgery test is "attacked" (the doc at 193-213 names this problem for Add and the fix does not tie the opcode to the tampered step). 3) `#[ignore]` is not looked at; `fn rejects_` is found in any text, test or not. Opcodes written without `= value` in the enum are not enumerated (165-168).
Impact: "every opcode attacked" is a weaker claim than measured. A deleted AIR constraint for an opcode can keep a green gate. R3 path (budzero/bud-proof).
Known: none

### P51-F08
File: xtask/gates/src/gates/consensus_maps_ordered.rs:26-33, 185-216, 431-437, 467-492
Severity: Medium
Axis: invariant (deterministic state root)
Confidence: Certain (logic); no live violation was searched for
Evidence: 1) `declared.insert((scope, name), kind)` is keyed by (module, field name) only, last write wins. `field_kind_any` also matches struct-literal and call lines like `entries: other_map,` (type "other_map") and replaces the real declaration. 2) iter_field accepts `for .. in [&]self.F[.values()|.keys()|.iter()]` followed by `{` or `&` only. 3) HASHING_FNS is six names. 4) `has_own_slot_write(&body)` exempts the whole 40-line window if any `x[i] = ..` appears.
Description: 1) Two structs in one module with the same field name (for example `entries`) and different types: the later one decides for both. A renamed local assigned to the field masks a HashMap. 2) Missed shapes: `.iter().filter(..)`, `.iter().map(..)`, `.into_iter()`, `.values_mut()`, `.drain()`, and any local or parameter map (account.rs:2302 `affected_indices: HashSet` is a real local that the gate cannot see). 3) `merkle_root` (identity.rs:472, merkle_tree.rs:146, commitment_tree.rs:25), `commitment` (three_nft.rs:75) and any other name are never examined. 4) A hasher fold plus an unrelated indexed write in the same window is exempt. `has_fold` also needs `.update(` or `.push(`; `.extend(`, `.write(`, `.absorb(` and helper calls are not folds. `use .. as Map` aliases and `FxHashMap` or `Option<HashMap<..>>` types are not in UNORDERED.
Impact: A HashMap iteration folded into a commitment can pass in several shapes. Current tree: no live violation found by reading (account.rs:2304 is order-independent: it writes `cached_leaves[pos]`).
Known: none

### P51-F09
File: xtask/gates/src/gates/coding_audit_samples_the_relationship.rs:119-156 and content_encryption_is_declared_and_bound.rs:187-224
Severity: Medium
Axis: test / invariant(Z3)
Confidence: Certain
Evidence: Both `test_is_present` walk forward from a `#[test]` marker and skip following attributes. The accepted order `#[ignore]` before `#[test]` is never examined; content_encryption also skips `#[ignore]` after `#[test]` (198-203). The locks file is not comment-stripped, so `// #[test]` followed by `fn name(` matches. The fixtures that must PASS use empty bodies: coding_audit:708 and content_encryption:672 `writeln!(locks, "#[test]\nfn {n}() {{}}\n")`. In coding_audit the attribute loop tests `locks[j..].starts_with('[')` (130) but the next attribute starts with `#`, so any attribute after `#[test]` makes the test "missing".
Description: Twelve required regression tests for storage commitments (src/tests/manifest_commitment_locks.rs) only need a name. They may be ignored, empty, commented-out `#[test]` or compiled out. The mod is compiled via src/tests/mod.rs:40 only under cfg(test).
Impact: The manifest id binding and coding-audit regressions can be hollowed out with a green gate. R3 storage (commitment binding).
Known: none

### P51-F10
File: xtask/gates/src/gates/docker_toolchain.rs:88-103
Severity: Medium
Axis: integration (reproducible build claim)
Confidence: Certain
Evidence: `let v: String = rest.trim().trim_matches('"').chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect(); if !v.is_empty() ...`. `rg -n 'toolchain:' .github/workflows` shows `toolchain: stable` (rust.yml:60) and `toolchain: nightly` (ci.yml:1324, fuzz-nightly.yml:62, supply-chain-extra.yml:39); all give an empty version and are skipped. Lines 88-90: any workflow file containing the text `DELIBERATE-TOOLCHAIN-DIVERGENCE` is skipped whole (today only diverse-double-compiling.yml).
Description: Floating `stable` passes as if pinned, and the doc claim "every workflow toolchain agrees" is false. The marker exempts a whole file, even if it appears in a comment, and no reason text is checked. `dtolnay/rust-toolchain@<ver>` refs and `rustup` lines in run blocks are not read. Only the first `FROM rust:` is compared (digest check accepts any rust FROM line).
Impact: A main workflow builds with a moving compiler while the gate is green. Low blast radius for the chain, but it breaks the determinism claim in the gate header.
Known: none

### P51-F11
File: xtask/gates/src/gates/dead_pub_api.rs:216-218, 248-259
Severity: Low
Axis: dead code
Confidence: Certain
Evidence: `references` counts every identifier token in every corpus file (strings, comments and doc comments included), globally by name. `if counted > declared { continue; }` compares the global count with the declarations in the same file. The exemption window is `body[start..index]` (14 lines above) with no check that the line is a doc comment of this item.
Description: Two dead pub fns with the same name in two files make each other live. A common name (`new`, `len`) is always live. A doc line of a neighbouring fn that says `Convenience:` exempts every pub fn within the next 14 lines. The gate does not stop a baseline growth in a PR; only convention does (AGENT_MAP says never edit the baseline).
Impact: The ratchet undercounts dead API. Maintenance risk.
Known: none

### P51-F12
File: xtask/gates/src/gates/coverage.rs:10-24, 61
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence: `.parse::<f64>()` accepts `nan`, `inf`, `-5`. `if pct < base` is false for a NaN baseline.
Description: A baseline file of `nan` (or any very low number) makes the gate always green. The gate has no memory of the previous baseline, so lowering 64.30 is not detected here (the header says lowering is a violation, but only review enforces it).
Impact: Silent loosening of the coverage ratchet through one baseline edit.
Known: none

### P51-F13
File: xtask/gates/src/gates/consensus_state_only_changes_in_blocks.rs:27-33, 75-125
Severity: Low
Axis: invariant (state changes only in blocks)
Confidence: Certain
Evidence: `code_of` returns raw text; every check is `contains` on it. `bc.matches("account.nonce = *new_nonce").count() != 0` is one exact spelling. `slice_of(ex, "TransactionType::StateUpdate", "Ok(())")` uses the first occurrence of both markers.
Description: Needles in comments, strings or test modules satisfy presence checks (`final_hasher.update(self.bridge_root)`, `MAX_STATE_UPDATES`, `save_cross_domain_message(`). A renamed variable (`n`, `nonce`) writes nonces out of block unseen. The first `TransactionType::StateUpdate` in executor.rs may not be the arm.
Impact: Weak detection of regression. The self-test breaks the real tree five ways, which is good evidence for those five shapes only.
Known: none

### P51-F14
File: xtask/gates/src/gates/domain_tags.rs:44-78, 91-119, 137
Severity: Low
Axis: invariant (domain separation)
Confidence: Certain
Evidence: `scan_dir` returns on `read_dir` error and skips non-UTF-8 files. The inventory is excluded by file name only: `path.file_name().is_some_and(|n| n == ex)` with ex = "domain_tags.rs". Tags are found only as one quoted literal `"BDLM_[A-Z0-9_]+"` or `"BUDLUM_.."`.
Description: No check that two constants use the same tag value or that one tag is a prefix of another (the header names the cross-tree collision as the case the inventory exists for, but the gate sees a set). A missing `bud/` or `crates/` dir passes silently (the exact blind spot named in the header). Any other file called domain_tags.rs (xtask/gates/src/gates/domain_tags.rs exists, outside the scan roots today) would be hidden if it moved under a root. Tags from `concat!`, `format!` or lowercase bodies are not seen. Roots exclude xtask, fuzz and contracts. Strings in tests and comments count as "used".
Impact: A reused or prefix-colliding tag, or a stale inventory entry kept alive by a test string, passes.
Known: none

### P51-F15
File: xtask/gates/src/gates/every_fuzz_target_is_run.rs:111-121, 169-184 and fuzz_targets_wired.rs:16-35, 110-112
Severity: Low
Axis: integration
Confidence: Certain
Evidence: every_fuzz_target_is_run counts a corpus dir as seeded if `read_dir(..).filter(is_file).count()` is above zero (a `.gitkeep` counts). names_target accepts a bare line equal to the name in the whole of ci.yml, in any job, also in a disabled job or a `run: |` body. fuzz_targets_wired parses any line starting with `name` or `path` (including `[package] name`), and accepts a name token anywhere in all workflow text, comments included.
Description: The two gates overlap and the older one (fuzz_targets_wired) is weaker: a comment saying "run abc" is a schedule. Neither checks that the job runs for non-zero time or is not skipped.
Impact: A target can be listed but never executed with green gates.
Known: none

### P51-F16
File: xtask/gates/src/gates/gas_calibration.rs:63-70, 73-93, 30-44
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `first_integer(after)` takes `skip_while(!digit)` over the text after `=>`, including comments and the next arms. Named constants are not resolved. `default` is set when `pending.is_empty() && lhs.contains('_')`, which any guard arm such as `op if op.is_alu() => 2` also satisfies. EXPANDED and HEAVY are fixed lists.
Description: `Opcode::Div => DIV_GAS, // 64 rows` reads 64. `Opcode::Div => COST,` followed by `Opcode::SWrite => 12` reads 12. The list of multi-row opcodes is not derived from the AIR. Today gas_cost (bud-vm lib.rs, 45 lines read) uses literals, so the gate reads it correctly.
Impact: A refactor to named constants can make the calibration check lie in either direction.
Known: none

### P51-F17
File: xtask/gates/src/gates/fsf_import_boundaries.rs:120-129, 159-160, 232-244
Severity: Low
Axis: invariant(Z10)
Confidence: Certain
Evidence: SKIP_DIRS (`build`, `dist`, `coverage`, `.cargo`, `target`, ..) is matched by directory name at every depth (165). Symlinks are skipped (159). `has_boundary_note` walks up to the root and accepts a `BUDLUM_IMPORT_BOUNDARY.md` of any content (even empty) at any level including the repo root. Non-UTF-8 files are skipped without a count.
Description: Copied copyleft source under `vendor/x/build/..` or `third_party/dist/..` is never read. One empty note file at the root disables the copyleft check for all import surfaces. Copyleft markers are checked only under vendor, third_party, external and upstream paths, so a copy placed under `crates/foo/` is not flagged (by design, but it leaves the Z10 rule to review).
Impact: Defense-in-depth gap for the clean-room rule.
Known: none

### P51-F18
File: xtask/gates/src/gates/content_encryption_is_declared_and_bound.rs:155-185, 377-401 and coding_audit_samples_the_relationship.rs:228-381
Severity: Low
Axis: invariant
Confidence: Certain
Evidence: key_material_in matches a banned word only at a word start followed by `:`, so `master_key:`, `data_key:`, `kek:`, `password:` pass (prev char `_` fails prev_ok), and a tuple variant `ClientSide(ContentCipher, [u8; 32])` has no `name:` at all. Only `pub enum ContentEncryption` is scanned; ContentCipher (manifest.rs:120) is not. check_call_sites matches only the text `manifest_id_from_parts(&self.shards` and counts `checked = 1` even if there are no matches. check_binding accepts `commitment_tag` and `BDLM_MANIFEST_V4` anywhere in the body, even as an unused `let _ =`. coding_audit checks are `contains('%')`, `contains('^')`, `contains("self.k")` on whole bodies.
Description: Presence-of-token checks, not data-flow. A declaration with key material in another shape, or a call that passes a literal through a different first argument, is not seen.
Impact: The key-material and binding invariants hold on the current text only; modest protection against a real regression.
Known: none

### P51-F19
File: xtask/gates/src/gates/containment_defaults.rs:16-21, 35-45; derived_content.rs:23-28, 60-76; content_encryption..:63-81; coding_audit..:64-79; evidence_provenance.rs:49-62; every_opcode_forgery.rs:11-19
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: These six gates strip comments with `split("//").next()` (or find("//")) and do not handle block comments or `//` inside strings. Three other gates in the part already use rust_literals::scrub (consensus_maps_ordered.rs:414, cross_table_checks.rs:62, forgery_tests.rs:111). containment_defaults.rs:37-39: `let end = rest.find('}').unwrap_or(rest.len()); &rest[..=end]` panics if no `}` follows the Default impl.
Description: A `/* MainnetActivation::default() */` block comment satisfies the presence check at containment_defaults.rs:62. A string such as "http://x" hides the rest of the line from the scan (for example a float literal after it in derived_content.rs, or a `full()` call in bud-vm). The Default block ends at the first `}`; lines after it (for example `s.verify_merkle_enabled = true;`) are not read, and the flag check is `starts_with("flag:") && contains("false")` per line.
Impact: Known-class CWE-184 gap left in six gates, closed in three. Low because the real sources do not use these shapes today.
Known: none

### P51-F20
File: xtask/gates/src/gates/cross_table_checks.rs:19-48; derived_content.rs:84-120; fixture_integrity.rs:50-65
Severity: Low
Axis: test
Confidence: Certain
Evidence: cross_table ACCUMULATORS is two hard-coded names (`rg 'COL_[A-Z_]*_ACC'` in plonky3_air.rs finds only those two today) and binding_blocks only sees brace blocks without nested braces. derived_content passes if the text `DerivationChain` and `fn check_master_is_stored` exist (check_master_is_stored is called at storage_deal.rs:1587, so it is wired today). fixture_integrity needs the key names as substrings and 1 KiB to 64 KiB size; `"btc_merkle_blocks":[]` with padding passes.
Description: Presence-only gates. A new COL_*_ACC column, a binding inside a nested block, a dead check fn, or an empty fixture with padding is not caught.
Impact: Low. These are schema canaries by their own header and the tests carry the real checks.
Known: none

### P51-F21
File: xtask/gates/src/gates/consensus_maps_ordered.rs:376-382, 386, 516-519
Severity: Low
Axis: test (vacuous pass)
Confidence: Certain
Evidence: `let Ok(rd) = read_dir(&dir) else { continue };` and `let Ok(text) = read_to_string(path) else { return Vec::new() };`. `run` returns OK when `findings.is_empty()` with no count of scanned files. Directories are skipped when the full path contains `/target` or `/.git` (a substring of the whole path, including the root's parents).
Description: An unreadable root or an unreadable file gives "OK". A checkout under a path containing `/target` scans nothing.
Impact: Gate can pass with zero files scanned.
Known: none

### P51-F22
File: xtask/gates/src/gates/gates_are_wired.rs:396-430, 337-384; docker_toolchain.rs:79-80; coverage.rs:50
Severity: Info
Axis: invariant(Z6) / language
Confidence: Certain
Evidence: ops/scripts contains only `.gitkeep` (`ls -la ops/scripts`), so run() takes the "no scripts remain" branch and returns OK. is_wired counts a line such as `bash ./ops/scripts/check-x.sh || true` or one in a job with `if: false` as wired. exact_named_tests.rs:25 passes with an empty `tests` slice; named_tests::self_test slices `&tests[1..]` (panics on an empty list). Non-English text in repo files: docker_toolchain.rs:79-80 ("denetlenmez"), coverage.rs:50 ("coverage raporu yok").
Description: gates_are_wired is a no-op by design after the migration. If a shell gate returns, `|| true` is counted as wired (Z3 shape). Turkish words are outside the Z13 rule (repo text is English).
Impact: Observation only.
Known: none
