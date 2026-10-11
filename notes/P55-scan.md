# P55 scan: xtask gates 6 (read-only audit, Tier R1)

Base commit: b11a22f. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14, P58, P59), notes/P50-scan.md, notes/P51-scan.md, notes/P52-scan.md.

## COVERAGE

Read in full (every line, tests and self-tests included), files 100 to 119 of `ls xtask/gates/src/gates/*.rs`:
- shard_placement.rs 1-609
- slash_expression.rs 1-498
- source_reading.rs 1-222
- storage_penalties.rs 1-531
- storage_priced.rs 1-251
- storage_proof_boundary.rs 1-114
- storage_provider_gate.rs 1-30
- suppressions_are_justified.rs 1-832
- tee_trust_boundary_is_structural.rs 1-530
- test_modules_can_see_what_they_test.rs 1-339
- the_image_builds_what_the_manifest_declares.rs 1-233
- three_is_recipe_only.rs 1-113
- threshold_rates.rs 1-607
- timing_safe.rs 1-208
- transcript_mirrors.rs 1-203
- tree_is_english.rs 1-1252 (two reads: 1-650, 650-1252)
- tree_pin.rs 1-382
- typos.rs 1-76
- udeps.rs 1-168
- uncheckable_proof.rs 1-170

Total 7368 lines. Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 141-165 (section 6); notes/P29-scan.md lines 1-30.

Outside the part (rg first, then at most 80 lines): xtask/gates/src/main.rs (rg on gate names, lines 1340-1399); xtask/gates/src/gates/named_tests.rs (rg, check_log); .github/workflows/ci.yml (rg, lines 2136-2146); .github/workflows/supply-chain-extra.yml 50-70; .github/*-baseline.txt (ls, rg); .github/gate-panics-baseline.txt (head, rg count); src/storage/living_threshold.rs (rg, lines 620-645); src/chain/blockchain.rs 6120-6132; crates/wallet-core/src/lib.rs 1268-1280 (rg); crates/wallet-core/src (ls, rg); src/domain/storage_deal.rs (rg on test-mock-proof); src/account_abstraction/quantum_account.rs 150-168; ops/Dockerfile and Cargo.toml (rg); budzero (find by extension); xtask/gates/pins/budzero-tree.pins (count only).

Not read: rust_literals.rs (scrub, exclusive_scratch_dir), regeneration.rs (keccak256), the rest of named_tests.rs, the rest of main.rs. No cargo, no test run, no git write. Every claim comes from reading source or from the rg and find output named in the evidence.

Extra-check summary:
- Vacuous pass, empty match set: timing_safe (P55-F09), test_modules (P55-F18), image gate (P55-F17), udeps (P55-F01, P55-F02). Floors exist in slash_expression (50 files), tree_is_english (50 files), tree_pin (empty pin file), suppressions (no files).
- Missing baseline file: udeps passes with SKIP and the baseline has never existed in git history (P55-F01). tree_is_english treats a missing baseline as empty, which fails closed.
- Swallowed errors: unwrap_or_default on file read in shard_placement.rs:303 (fails closed by a later check) and suppressions_are_justified.rs:708 (hides the file, P55-F04).
- Regex or parser gaps: P55-F04, P55-F05, P55-F06, P55-F12, P55-F18, P55-F19.
- Baseline loosening: none found in the part. No gate here edits a baseline. tree_pin `--pin` rewrites pins; CI runs it without the flag (ci.yml:2140-2143).
- Z3: no new `#[allow]`, `#[ignore]` or commit pin in the part. The required-test checks do not detect `#[ignore]` (P55-F08). Z6: no shell script in the part. Z8: the only float in the part scope is a budgeted allow (P55-F23).
- Wiring: all 20 gates are reached by `--all` and `--all --self-test` (ci.yml:1719, 1721); gates that need a log or arguments have direct steps.

## FINDINGS

### P55-F01
File: xtask/gates/src/gates/udeps.rs:73-80
Severity: Medium
Axis: test
Confidence: Certain
Evidence: `let baseline_path = root.join(".github/udeps-baseline.txt"); if !baseline_path.is_file() { return Ok(format!("SKIP: {} is absent - first measurement (step 1); findings:\n{}", ...)) }`. Command `ls .github/udeps-baseline.txt` returns "No such file or directory". Command `git log --oneline --all -- .github/udeps-baseline.txt` prints nothing. The workflow calls the gate as a ratchet (supply-chain-extra.yml:55 "a finding outside the baseline = FAIL").
Description: The ratchet has never had a baseline file, so every parsed unused dependency returns Ok. Deleting a baseline would also turn the gate off.
Impact: A new unused dependency (or an unused path or git dependency that widens the supply chain) never fails CI. The gate name and job title claim the opposite.
Known: none

### P55-F02
File: xtask/gates/src/gates/udeps.rs:48-72
Severity: Low
Axis: test
Confidence: Likely (missing step: run the gate on an empty file and on a half-written report)
Evidence: `if !out.is_file() { return Err("the udeps output is missing/empty") }` checks existence only. An empty file parses to zero findings and returns `Ok("OK: no unused dependencies (parsed).")`. The tool-died test (`starts_with("error:")`, `contains("\nerror:")`, `contains("failed to")`) runs only inside `if found.is_empty()`. A report with some packages followed by an `error:` line for another member is accepted if the parsed lines are on the baseline. No positive sentinel ("All deps seem to have been used.") is required.
Description: A clean verdict is accepted without proof that the tool finished. The CI wrapper (supply-chain-extra.yml:64) covers the case of a non-zero exit with no `unused dependencies:` header, not an exit 0 with empty output.
Impact: A truncated or empty report reads as clean.
Known: none

### P55-F03
File: xtask/gates/src/gates/suppressions_are_justified.rs:687
Severity: Medium
Axis: invariant(Z3)
Confidence: Certain
Evidence: `const SCANNED: &[&str] = &["src", "benches", "budzero", "build.rs"];`. Command `rg -n '#!?\[\s*(allow|expect)\s*\(' crates bud xtask fuzz tests` shows unbudgeted suppressions the gate cannot see: crates/ai-inference/crates/ai-ops/src/logparse.rs:19 `#![allow(dead_code)]`; crates/budscan/src/query.rs:110 `#[allow(clippy::too_many_lines)]`; bud/src/bud_format_realcorpus.rs:13 `#[allow(dead_code)]`; xtask/gates/src/gates/ast_security_gates.rs:297 and :483 `#[allow(clippy::struct_excessive_bools)]`; xtask/gates/src/gates/budscan_patchset.rs:177 `#[allow(clippy::too_many_lines)]`.
Description: The gate says "Every `#[allow(...)]` in the tree is listed here" and the success line says "across N files". It never walks crates/ (bpqs, wallet-core, budscan, ai-inference, note-packing), bud/, xtask/ or fuzz/. The NEVER entry `unsafe_code` is also blind there.
Impact: Z3 softening (a new allow, including `allow(unsafe_code)`) in crates/bpqs or crates/wallet-core, which hold key and signature code, passes this gate.
Known: none

### P55-F04
File: xtask/gates/src/gates/suppressions_are_justified.rs:554-603, 708
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence: `allows_in` matches only the exact text `#[allow(` or `#![allow(` (after an optional `!`). It does not match `#[expect(`, `#[cfg_attr(test, allow(...))]`, `#[ allow(` or `#[allow (`. Line 708: `let src = std::fs::read_to_string(f).unwrap_or_default();` turns a non-UTF-8 or unreadable .rs file into an empty file. Budgets are counts per file and lint, so moving an allow to another item, or swapping one for another of the same lint in the same file, passes. `NEVER` (line 539) lists only `unsafe_code`.
Description: Several spellings of a suppression and unreadable files are not counted.
Impact: A suppression written as `cfg_attr(..., allow(...))` or `#[expect(...)]` is invisible to the Z3 counter.
Known: none

### P55-F05
File: xtask/gates/src/gates/slash_expression.rs:147-151, 134
Severity: Medium
Axis: correctness
Confidence: Certain (by reading; no run)
Evidence: `let compares = cond.contains("stake") && cond.contains(['<', '>']); compares && ((yes == "stake" && narrowed(no)) || (no == "stake" && narrowed(yes)))` and `let narrowed = |v: &str| resolve(v, &bindings).contains("asu64");`. The self-test fixture `branch` (line 433) uses `if narrow > stake { stake } else { narrow }`, which is a minimum. The same shape `if narrow<stake{stake}else{narrow}` normalizes to cond `narrow<stake`, yes `stake`, no `narrow` and passes, but it returns the larger of the two. `(0 as u64).min(stake)` also passes the `.min(stake)` arm because `narrowed` only looks for the text `asu64`.
Description: The cap check accepts any condition that names the stake and one comparison sign, in either direction. The comparison direction and the narrowed operand are not checked.
Impact: R3 slashing clamp. If both `slash_penalty` homes (src/core/chain_config.rs and budzero/verifier-registry/src/params.rs) drift to the wrong direction together, the gate stays green and a slash can exceed the bond (the B35 class the gate exists for). Today's bodies were not diffed here.
Known: none

### P55-F06
File: xtask/gates/src/gates/slash_expression.rs:310, 346-347; source_reading.rs:13
Severity: Low
Axis: test
Confidence: Certain
Evidence: slash_expression.rs:310 `["src", "budzero", "wallet-core"]` and source_reading.rs:13 `SCAN_ROOTS = ["src", "budzero", "wallet-core"]`. Command `ls -d wallet-core` returns "No such file or directory"; the crate lives at crates/wallet-core. Both gates skip a missing root silently (`!base.is_dir() => continue`, `read_dir ... else continue`). crates/, bud/ and xtask/ are not walked. `is_slash_expr` (line 18) works on one line and needs the literal `fixed_point_scale`, `u128`, `stake` or `bond`, `as u64`, `*` and `/` on that same line. Command `rg -l FIXED_POINT_SCALE crates bud` finds nothing today.
Description: A stale root name hides that a whole crate tree is out of scope. A wrapped or renamed expression is not found.
Impact: A second copy of the unclamped slash arithmetic in crates/ or in a multi-line form is not found. No such copy exists today.
Known: none

### P55-F07
File: xtask/gates/src/gates/threshold_rates.rs:83-96, 186-202
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: `has_pinned_literal(src, "disk_picodollars_per_byte_epoch: 403")` accepts the first line containing the key. Command `rg -n 'picodollars_per_(byte_epoch|nano): *(403|694)' src/storage/living_threshold.rs` prints only lines 640, 642, 861, 866, 966. `#[cfg(test)]` starts at line 624. Command `rg -n 'OperatorRates\b' src | rg -v living_threshold.rs` prints only the re-export at src/storage/mod.rs:97.
Description: The pinned 403 and 694 exist only in the test fixture `rates()`. No production code builds an `OperatorRates`, so the "measured rates at one scale" claim is a check of test data. The gate would stay green if a production rate source used another scale.
Impact: Mainnet part not wired: the living-threshold decision has no production rate source, and the gate hides that by passing on fixtures.
Known: none (idle-code-baseline lists only the constants of this module)

### P55-F08
File: storage_penalties.rs:347-349; uncheckable_proof.rs:29-31; threshold_rates.rs:206,257,305,335,342; storage_priced.rs:100-111; shard_placement.rs:120-151
Severity: Medium
Axis: invariant(Z3)
Confidence: Certain
Evidence: storage_penalties `has_test`: `deal_src.contains(&format!("fn {name}("))` on the raw source, including comments, strings and non-test functions; its message says "is missing or is not a `#[test]`" but never checks `#[test]`. uncheckable_proof `has_fn`: `code.contains(&format!("fn {name}("))`. threshold_rates: `src.contains("fn each_lever_has_its_own_crossing_point")` and six more. storage_priced `has_test` accepts `#[test]` followed by `#[ignore]` (the rule is "last `#[test]` within 3 lines before"). shard_placement `has_test_fn` skips every attribute between `#[test]` and `fn`, so `#[ignore]` and `#[cfg(any())]` pass.
Description: The "required regression test exists" checks accept an ignored, disabled, commented-out or non-test function with the right name.
Impact: Z3. A required regression test can be turned off with `#[ignore]` and these gates stay green; storage penalties, storage pricing, uncheckable-proof containment, threshold rates and shard placement are covered.
Known: none

### P55-F09
File: xtask/gates/src/gates/timing_safe.rs:39, 53-85, 99-102, 150-171
Severity: Medium
Axis: test
Confidence: Certain
Evidence: `self_test` does not call `collect`, `collect_dir` or `is_filtered`; it re-implements the test inline: `if has_secret(line) && has_plain_eq(line) && !line.contains("ct_eq") && !line.contains("constant_time")`. `collect_dir` returns on `read_dir` error (`let Ok(rd) = fs::read_dir(dir) else { return; }`) and `run` returns `CLEAN` with no count of files scanned. `is_filtered` drops a line that contains `.len()`, `assert`, `expect(`, `ct_eq`, `REPLACE_TOKEN` or `#[cfg(test)]` anywhere, for example `if token.len() == 5 || token == supplied`. Stems (lines 23-36) omit hmac, mac, signature, nonce, seed, mnemonic, pin, otp, digest.
Description: The canary cannot detect a broken scan path, a renamed `src/rpc` or `src/crypto`, or a wrong filter. The scan covers only src/rpc and src/crypto, one line at a time.
Impact: The "static gate WORKS" message is not evidence. A plain `==` on a secret that has a non-listed name, sits on a line with `.len()`, or spans two lines is not reported; src/network, src/relayer, src/gateway, src/bin and crates/wallet-core are out of scope.
Known: none

### P55-F10
File: xtask/gates/src/gates/storage_proof_boundary.rs:27-59
Severity: Low
Axis: test
Confidence: Certain
Evidence: `let root_bound = src.contains("storage_root") && src.contains("proof_bytes") || src.contains("storage_root.is_some");` where `proof_bytes` is already required at line 36, so the test reduces to `contains("storage_root")`. All checks are `src.contains(..)` over the raw file (comments, strings, test modules). The mock guard check requires the guarded text `cfg!(test) && proof_bytes == b"test-mock-proof"` to be present; it does not check that no unguarded accept exists. Command `rg -n 'test-mock-proof' src` shows one non-test site: src/domain/storage_deal.rs:3025, guarded. `actor.contains("apply_storage_bond_slash")` is any mention in chain_actor.rs, not a call on the `Mismatched` path.
Description: Presence checks stand in for the boundary claims "STARK verification mandatory" and "a failed proof slashes the bond".
Impact: A second, unguarded mock-proof acceptance, or a slash call moved off the `Mismatched` path, passes. Today the tree matches the claims.
Known: none

### P55-F11
File: xtask/gates/src/gates/storage_penalties.rs:17-63, 113-117, 157, 293, 323-345
Severity: Low
Axis: test
Confidence: Likely (missing step: a fixture where the first match is another type)
Evidence: Check 3 passes on `chain_code.contains("prune_expired_cooldowns")` anywhere in src/chain/blockchain.rs, test module included; the message says "no production path calls it". `body_of` returns the first `pub fn root(` and `pub fn open_deal(` in storage_deal.rs, which may belong to another type than `StorageRegistry`. `strip_comments_and_literals` treats `'` as a char literal opener; two lifetimes on one line (`<'a, 'b>`) blank the text between them. Command `rg -n prune_expired_cooldowns src --glob '!src/tests/*'` shows a production call at blockchain.rs:6130, so the claim is true today.
Description: The caller and anchor checks are text presence, not production call or the right type.
Impact: Moving the only call into a `#[cfg(test)]` block would keep the gate green.
Known: none

### P55-F12
File: xtask/gates/src/gates/tee_trust_boundary_is_structural.rs:16-21, 302-312, 386-397, 160-224
Severity: Low
Axis: test
Confidence: Certain
Evidence: Checks 1 and 2 run on the raw text: `src.contains("trait TeeQuoter: TeeRuntime")`, `src.contains("trait TeeQuoteVerifier")`, `src.contains("UnavailableTeeQuoteVerifier")`, so a comment or a test counts. Only `check_live_verifier_call` uses `strip_rust_noise`. The guard check looks at the first occurrence of `if !attestation.verify_measurement(` and never reads the argument; the self-test fixture (line 433) passes `&[0u8; 32]`. `if attestation.backend !=` is matched without its right-hand side. `tee_files` lists only crates/wallet-core/src/lib.rs and tee.rs; the directory also holds privacy_crypto.rs and privacy_transfer.rs. `strip_rust_noise` has no char-literal rule, so `'"'` opens a string. Messages at lines 305, 388, 394, 412, 527 contain a stray `\\` before the line break. The real code is consistent: crates/wallet-core/src/lib.rs:1276-1286 compares against the enrolled `trusted` measurement.
Description: The split is name-locked. The values compared are not checked, and the unavailable verifier's reject-all behaviour is a name.
Impact: Self-attesting variants such as `verify_measurement(&attestation.measurement)` pass. A new `-> Result<TeeAttestation>` method in another wallet-core file is not seen.
Known: none

### P55-F13
File: xtask/gates/src/gates/transcript_mirrors.rs:53-103, 36-37
Severity: Low
Axis: test
Confidence: Likely (missing step: a fixture with a test-only observe)
Evidence: `absorptions` counts every line containing `challenger.observe` in the whole file, including `#[cfg(test)]` code and helper functions, and `classify` returns "commitment" for anything that is not a scalar, security-params or public-values. A call split over two lines gives an empty argument and is classed "commitment". An alias (`ch.observe`), `observe_ext_element`-style calls, loops and conditional absorptions count once. Command `rg -c 'challenger\.observe' budzero/bud-proof/src` shows 10 in each of prover.rs and verifier.rs and none elsewhere. Z13: lines 28, 140 and 185 are Turkish ("Ne yakalamaz", "emilim ayrisiyor", "3 emilim beklenirdi"), and line 25 has the garbled "saniyeler surer".
Description: The mirror check compares file-wide counts and coarse kinds, not the per-function transcript.
Impact: A test-only observe on one side balances a missing production observe on the other; swapping two commitments of the same kind goes unseen. Fiat-Shamir binding is R3 (bud-proof).
Known: none

### P55-F14
File: xtask/gates/src/gates/tree_pin.rs:75-80, 83
Severity: Low
Axis: security
Confidence: Likely (missing step: a PR that adds a file symlink under budzero/)
Evidence: `if kind.is_symlink() { continue; }` applies to files as well as directories; `if path.file_name().is_some_and(|n| n == "target") { continue; }` skips any directory named `target` at any depth. Command `find budzero -type l -not -path '*/target/*'` prints nothing today. Pins: `grep -vc '^#' xtask/gates/pins/budzero-tree.pins` gives 79 and a find over the pinned extensions gives 79.
Trigger (security only): precondition: commit rights; entry point: a PR adding `budzero/<crate>/src/x.rs` as a symlink to content outside budzero/ plus a `mod x;` line (the pinned lib.rs changes, which is re-pinned); input shape: symlink target; expected wrong result: compiled code that no pin covers and the gate does not report.
Description: Symlinks are skipped silently, so the content a symlinked .rs file points to is neither pinned nor reported as a new file.
Impact: A source file can enter the budzero build without a pin line or a gate message.
Known: none

### P55-F15
File: xtask/gates/src/gates/three_is_recipe_only.rs:28-62
Severity: Low
Axis: test
Confidence: Certain
Evidence: Eleven `contains` checks. `gen.contains("admits_body")`, `deal_code.contains("admits_body()")` and `deal.contains("Three admits no storage deal")` look at the whole file; the error text is checked on `deal`, which is not comment-stripped (only `deal_code` is). Nothing ties `admits_body()` to `open_deal` or to a returned `Err`. Z13: line 3 contains the Turkish word "tarif".
Description: The claim "opening a deal on Three is refused" is tested by the presence of three strings.
Impact: A comment holding the error text plus an unrelated `admits_body()` call keeps the gate green after the refusal is removed.
Known: none

### P55-F16
File: xtask/gates/src/gates/uncheckable_proof.rs:45-55, 76-81, 98
Severity: Low
Axis: test
Confidence: Certain
Evidence: The flag is "a plain constant false" if the first non-empty line after the definition line trims to `false`: `.find(|l| !l.trim().is_empty()).map(|l| l.trim() == "false")`. A body `false\n    || enabled()` passes. A one-line definition `fn f() -> bool { false }` reads the next line and reports "now reports true". The first definition in the file wins. The guard is found by an exact text `if !Self::{flag}() => Ok(())`, and ordering is judged by the first `DefaultAdapter::verify` text position in the file, not by control flow. `(Some(_), None) => Err(` is any match in the file.
Description: Text-order and first-line checks stand in for control flow.
Impact: A configurable flag whose first line is `false` keeps the "cannot slash on an unverifiable proof" gate green; a harmless reformat turns it red.
Known: none

### P55-F17
File: xtask/gates/src/gates/the_image_builds_what_the_manifest_declares.rs:38-64, 71-103, 108-140
Severity: Low
Axis: test
Confidence: Certain
Evidence: `copied_dirs` inserts every token of every `COPY` line, destination included (line 78 comment: "a false extra entry here can only make the gate more permissive about a path that is genuinely copied"). A line `COPY other/ ./crates/bpqs/` makes `crates/bpqs` covered. `ADD`, JSON-form COPY and continuation lines are not parsed; a COPY in the runtime stage counts. Only the root Cargo.toml is read, so path dependencies declared in crates/*/Cargo.toml or budzero/*/Cargo.toml are not examined. A missing or empty `declared` set returns OK with "0 path dependenc(ies)". No `.dockerignore` check (command `ls ops/.dockerignore .dockerignore` finds none today). Current Dockerfile lines 41-53 do copy src, benches, proto, budzero, crates/note-packing and crates/bpqs.
Description: The permissive destination rule contradicts its own comment: a destination is not a source.
Impact: Image build failures that the gate exists to catch earlier stay possible; no present miss was found.
Known: none

### P55-F18
File: xtask/gates/src/gates/test_modules_can_see_what_they_test.rs:38-65, 77-140, 143-173, 233-254, 322-339
Severity: Low
Axis: test
Confidence: Certain
Evidence: `parent_functions` accepts only lines that start (column 0) with `pub fn `, `fn ` or `pub(crate) fn `, so `const fn`, `pub const fn`, `pub(super) fn` and `async fn` are missed. `test_modules` ends a module at the first line equal to `}`; a multi-line `use super::{` list yields an empty import set for the following lines; `#[cfg(test)]` followed by `mod tests;` takes the next column-0 `}` as the body end. `run` skips a missing root (`if p.exists()`) and returns OK with no minimum module count. `walk` enters `target/` directories under crates and budzero. The symlink test at line 324 uses a predictable `std::env::temp_dir().join(format!("gate-walk-{}", pid))` instead of `exclusive_scratch_dir`.
Description: The parser covers the common layout only; the gate reports success without evidence that it found any test module.
Impact: The CI-only compile failure the gate targets can still happen for `const fn` helpers or reformatted imports.
Known: none

### P55-F19
File: xtask/gates/src/gates/source_reading.rs:62-79, 82-101, 142-158
Severity: Low
Axis: dead code
Confidence: Certain
Evidence: Line 158 `if !narrows(&src) || !lit.is_empty()` follows a `continue` when `lit.is_empty()`, so the condition is always true and `narrows` (lines 82-101) is dead. Only a same-line `let <x> = include_str!("<own file name>")`, followed by `<x>.contains("<lit of 4 or more chars>")`, is seen; a wrapped `let x =\n include_str!(..)`, `x.find`, `x.matches` and `.contains(&needle)` are not. Line 75 `rest = &after[lit_end + 1..];` panics when the quote is never closed (`lit_end == after.len()`). Scan roots: see P55-F06.
Description: The gate is stricter than its doc about narrowing but narrower than its doc about what it reads.
Impact: A self-reading test written in a wrapped form passes. A malformed source file makes the gate panic instead of report.
Known: none

### P55-F20
File: xtask/gates/src/gates/typos.rs:22, 69-74, 39
Severity: Low
Axis: test
Confidence: Certain
Evidence: self-test: `Ok(o) if o.status.success() => Err(..), Ok(_) => Ok("self-test: typos catches the planted word.")`. Any non-zero exit (unknown flag, crash, bad config) counts as "catches". `bin()` takes `TYPOS_BIN` from the environment with no version pin. Line 39 returns the Turkish text "typos temiz." (Z13).
Description: The canary does not check the exit code (2 for findings) or that the report names the planted word.
Impact: A typos binary that fails for another reason passes the canary; the run mode stays fail-closed.
Known: none

### P55-F21
File: xtask/gates/src/gates/tree_is_english.rs:85-107, 120, 161-319, 438-447, 531-534
Severity: Low
Axis: invariant(Z13)
Confidence: Certain
Evidence: Turkish text sits in gate files this gate reports clean: transcript_mirrors.rs:28 and :140 and :185, udeps.rs:117 ("BOZUK PARSE: beklenen=.. gelen=.."), :124 ("bozuk.txt"), :131 ("temiz.txt"), typos.rs:39 ("typos temiz."), three_is_recipe_only.rs:3 ("tarif"). Command `rg -n 'xtask/gates' .github/turkish-baseline.txt` prints nothing, so none of these lines is counted. The 157-word list lacks emilim, beklen, bozuk, temiz, tarif and yakalamaz. `is_allowed` matches the file name anywhere in the tree (`typos.toml`, `CLAUDE.md`, `MODEL_ROUTING.md`, `tree_is_english.rs` in any directory) and `ALLOWED_DIRS` by `contains` on the full path string including the checkout prefix. Non-UTF-8 files are skipped (line 531), so Turkish in cp1254 is not read; directory names `target` and `corpus` are skipped at any depth.
Description: The vocabulary gate has the limit its own doc warns about, and the exemptions are wider than "exact file name".
Impact: Z13 (repo text is English) is not enforced for xtask sources; a file named CLAUDE.md in any directory is exempt.
Known: none

### P55-F22
File: shard_placement.rs:315,327; storage_penalties.rs:416; threshold_rates.rs:147,211-217
Severity: Info
Axis: dead code
Confidence: Certain
Evidence: shard_placement.rs:315 `code.split("#[cfg(test)]").next()` cuts production text at the first `#[cfg(test)]` anywhere, which would make the float check (line 192) pass over the cut part. The `checked == 0` branches (shard_placement.rs:327, storage_penalties.rs:416) cannot fire because `checked` is incremented unconditionally before any condition. threshold_rates.rs:147 `disk_line == Some(i - 1)` underflows (usize) when `cpu_picodollars_per_nano:` is on the first line of the file, and the pair check applies only to a disk line directly above a cpu line. threshold_rates.rs `sed_range(.., "    }")` ends at the first line equal to four spaces and `}`, or at end of file, so `assert!` or `Decision::Hold` anywhere later in the file can satisfy the test-body check. Doc comment at threshold_rates.rs:168-170 has an empty `# Errors` on a private fn.
Description: Dead guards and fragile range rules that look like checks.
Impact: Low; the shipped inputs satisfy the rules today.
Known: none

### P55-F23
File: xtask/gates/src/gates/suppressions_are_justified.rs:59-64 (budget); src/account_abstraction/quantum_account.rs:156-167
Severity: Info
Axis: dead code
Confidence: Certain
Evidence: The budget accepts `clippy::cast_precision_loss` with the reason "a storage cost in dollars per TB-month is an estimate ... displayed, not settled on". The function is `pub fn storage_cost(&self) -> f64` and uses `as f64` and float literals (`tb * 0.23342 / 16.68`). Command `rg -n '\.storage_cost\(\)' src budzero crates --glob '*.rs'` finds no caller.
Description: A public float function with no caller is shielded by a budget line. If it is wired into a fee path it breaks Z8.
Impact: Dead public API now; a consensus float risk if called from execution later.
Known: none (check .github/dead-pub-api-baseline.txt before editing)

## SUMMARY

Counts: Critical 0, High 0, Medium 6 (P55-F01, F03, F05, F07, F08, F09), Low 15 (F02, F04, F06, F10 to F21), Info 2 (F22, F23). Total 23.
