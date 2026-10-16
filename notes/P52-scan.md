# P52 scan: xtask gates 3 (read-only audit, Tier R1, CI gate programs)

Base commit: 5f24faa. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14, P58, P59).

## COVERAGE

Read in full (every line, tests and canaries included), files 45 to 69 of `ls xtask/gates/src/gates/*.rs`, 7577 lines:
- xtask/gates/src/gates/gates_do_not_panic.rs 1-419
- xtask/gates/src/gates/gating_flags.rs 1-168
- xtask/gates/src/gates/geiger.rs 1-196
- xtask/gates/src/gates/generated_content.rs 1-191
- xtask/gates/src/gates/genesis_schema.rs 1-215
- xtask/gates/src/gates/git_deps_audited.rs 1-161
- xtask/gates/src/gates/gov_slash_evidence_is_validator_only.rs 1-454
- xtask/gates/src/gates/governance_invariants.rs 1-26
- xtask/gates/src/gates/guards_reachable.rs 1-293
- xtask/gates/src/gates/hash_inputs_are_length_prefixed.rs 1-813
- xtask/gates/src/gates/indexing_is_not_new.rs 1-390
- xtask/gates/src/gates/kani.rs 1-479
- xtask/gates/src/gates/license_consistency.rs 1-438
- xtask/gates/src/gates/lock_failures.rs 1-210
- xtask/gates/src/gates/logup_multipliers.rs 1-274
- xtask/gates/src/gates/master_derivation.rs 1-101
- xtask/gates/src/gates/mempool_mev_free.rs 1-200
- xtask/gates/src/gates/mermaid.rs 1-546
- xtask/gates/src/gates/metrics_are_written.rs 1-830
- xtask/gates/src/gates/minting_paths_are_counted.rs 1-201
- xtask/gates/src/gates/module_coverage.rs 1-317
- xtask/gates/src/gates/multi_ratio_consensus.rs 1-239
- xtask/gates/src/gates/named_tests.rs 1-83
- xtask/gates/src/gates/network_hardening_gate.rs 1-42
- xtask/gates/src/gates/no_conflict_markers.rs 1-291

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 141-165 (section 6); notes/P29-scan.md lines 1-45 (format).

Outside the part (rg or Grep first, short ranges): .github/workflows/kani-slow.yml 62-107; .github/workflows (rg on module-coverage and geiger); src/core/account.rs 1949-1952 (try_mint_balance); try_add_balance call sites (rg count); src/lib.rs 45-75; src/rpc/tests.rs and src/tests/mod.rs (rg on cfg(test) shape only); bud/src/bud_format.rs and bud_format_bft.rs (rg on ratio only); Cargo.toml dependency lines (rg); crates/*/Cargo.toml names (rg); `ls .github` and `ls` of crate roots.

Not read: the gate dispatcher (xtask/gates/src/main.rs), xtask/gates/src/gates/rust_literals.rs, regeneration.rs, other parts of the gates crate, the baseline file contents (.github/*-baseline.txt are not edited and were only listed), .github/git-dep-audit.toml content, Cargo.lock (read denied by the sandbox, so git sources in lock files were not verified). No cargo, no test run. Every claim is from reading source.

Extra-check summary:
- Vacuous passes: found in module_coverage (SKIP forever, P52-F03), logup_multipliers (dead vacuity guard, P52-F06), no_conflict_markers (no file floor, P52-F19), guards_reachable (returns Ok("0") without src, P52-F04), gates_do_not_panic and lock_failures (unreadable file skipped, P52-F13/F10).
- Swallowed errors: `let Ok(..) else continue` on file reads in gates_do_not_panic.rs:246, indexing_is_not_new.rs:267, git_deps_audited.rs:32, no_conflict_markers.rs:48.
- Regex/parser gaps: named_tests (P52-F07), hash_inputs (P52-F09), indexing (P52-F08), minting (P52-F01), lock_failures (P52-F10).
- Baseline loosening: guards_reachable marker exemption is uncounted (P52-F04); indexing baseline is per file (P52-F08); module coverage has no floors (P52-F03).
- Z3: no `#[allow(`, `#[ignore]` or `cfg_attr` in the 25 files (the one `allow` hit is a canary string, indexing_is_not_new.rs:351). Z6: no shell script added; the `.sh` hits are doc comments about ports. Z7: no `unsafe` in the 25 files. Z9: no secrets.
- Named test existence: sampled 7 of the 28 names in governance_invariants and network_hardening_gate with rg; all exist as `#[test]` fns (src/core/governance.rs, src/network/peer_manager.rs, src/tests/hardening_h5_h7_locks.rs). No `#[ignore]` found on the sampled ones.

---

### P52-F01
File: xtask/gates/src/gates/minting_paths_are_counted.rs:28-32, 90-100, 117-121, 149-154
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
minting_paths_are_counted.rs:96  if in_tests || t.starts_with("//") || t.starts_with("pub fn") || t.starts_with("fn ") {
minting_paths_are_counted.rs:90  if line.starts_with("mod tests") || line.starts_with("pub mod tests") {
minting_paths_are_counted.rs:119 .filter(|(_, l)| l.contains(MOVE_FN) && !l.contains(MINT_FN))
src/core/account.rs:1949  pub fn try_mint_balance(&mut self, public_key: &Address, amount: u64) -> Result<(), String> {
src/core/account.rs:1950      self.ensure_mint_headroom(amount)?;
```
Description: The gate drops every line that starts with `fn` or `pub fn`, so a one-line `pub fn pay(..) { s.try_add_balance(..)?; }` is never counted. It sets `in_tests` once and never clears it, so production code after a column-zero `mod tests` is skipped. Only three files are scanned, and nothing checks that `try_mint_balance` still calls `ensure_mint_headroom`.
Impact: A new supply-creating call that fits one of these shapes passes the "every path counted" gate. The cap then rests on review only. Removing the `ensure_mint_headroom` line also passes while any `try_mint_balance` call remains.
Trigger: precondition: a future change adds a one-line fn or a call after `mod tests`, or a call in a fourth file (today `rg try_add_balance src` shows hits only in executor.rs, account.rs, blockchain.rs, plus a test string in src/relayer/policy.rs:954). Entry: run(), line 107. Expected wrong result: OK.
Known: none

### P52-F02
File: xtask/gates/src/gates/lock_failures.rs:11; xtask/gates/src/gates/hash_inputs_are_length_prefixed.rs:60
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
lock_failures.rs:11  const SCAN_ROOTS: &[&str] = &["src", "budzero", "wallet-core"];
hash_inputs_are_length_prefixed.rs:60  const SCAN_ROOTS: &[&str] = &["src", "budzero", "wallet-core"];
ls: cannot access 'wallet-core'   (the crate is at crates/wallet-core)
crates/bpqs, crates/budscan, crates/note-packing, crates/ai-inference, bud/ also exist
```
Description: `wallet-core` is not a root directory; the crate lives at `crates/wallet-core`. Missing roots are skipped silently (`if !base.is_dir() { continue; }` at lock_failures.rs:68; `read_dir` error returns at hash_inputs:171). `crates/*` and `bud/` are never scanned by either gate.
Impact: The lock fail-open rule and the length-prefix rule are not enforced in crates/bpqs (PQ signatures), crates/wallet-core, crates/note-packing, or bud/. A digest in those crates with two variable fields back to back passes.
Known: none

### P52-F03
File: xtask/gates/src/gates/module_coverage.rs:25-43, 185-196; .github/workflows/ci.yml:1624, 1636
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
module_coverage.rs:185  if !baselines.is_file() { ... "SKIP: {BASELINES} is absent (report mode)..." return Ok(msg);
.github  (ls): no module-coverage-baselines.json
ci.yml:1624  -- module-coverage coverage/cov.json
ci.yml:1636  -- module-coverage coverage/budzero-cov.json
module_coverage.rs:38  ("budzero:vm", "budzero/src/"),     (no budzero/src directory)
module_coverage.rs:35-37  node_di, cli, docs   (src/node_di and src/docs do not exist)
```
Description: The baselines file is absent, so both CI calls pass with SKIP and assert nothing. The prefix table has no entry for src/settlement, src/registry, src/cross_domain, src/privacy, src/ai, src/mempool, and the budzero entries `budzero/src/` and the missing dirs never match. Those R3 modules fall into `__other__` and cannot get a floor.
Impact: The coverage step is a green no-op. When floors are added, R3 modules named above cannot be protected because they are not modules in the table.
Known: none

### P52-F04
File: xtask/gates/src/gates/guards_reachable.rs:128-147, 175-178, 186-203, 230-249
Severity: Medium
Axis: invariant(Z3)
Confidence: Certain
Evidence:
```
guards_reachable.rs:183  if has_file_level_unwired_marker(text) { continue; }
guards_reachable.rs:231  if count > baseline {
guards_reachable.rs:242  if count < baseline { return Err(...)
guards_reachable.rs:189  let Some(rest) = t.strip_prefix("pub ") else { continue; };
guards_reachable.rs:176  if !root.join("src").is_dir() { return Ok(String::from("0")); }
rg '^//! WIRING: unwired' src : 12 files (sdk/mod.rs, domain/regen_health.rs, relayer/policy.rs, ai_inference/i18n.rs, cross_domain/bridge_relayer.rs, network/privacy.rs, settlement/pq_anchor.rs, account_abstraction/tee_attestation.rs, account_abstraction/private_transfer_auth.rs, storage/msr.rs, storage/three_regime.rs, tokenomics/reward_pool.rs)
```
Description: Four weaknesses. (1) A one-line `//! WIRING: unwired - x` exempts the whole file from the ratchet and the exemption is not counted or baselined, so any new file can opt out. (2) The ratchet compares counts only, so removing one unwired guard and adding another passes. (3) Only lines starting `pub fn` or `pub async fn` are guards; `pub(crate) fn`, `pub const fn` and `pub unsafe fn` are not. (4) A call from any file counts as wiring, including from the exempt unwired modules, so an unwired module calling another unwired module's guard hides it. A missing `src` returns Ok("0").
Impact: The "no refusal that nothing calls" rule can be satisfied by marker text. Two of the exempt modules are R3 (settlement/pq_anchor.rs, tokenomics/reward_pool.rs), which are mainnet blockers under CLAUDE.md section 1.
Known: none

### P52-F05
File: xtask/gates/src/gates/guards_reachable.rs:57; xtask/gates/src/gates/hash_inputs_are_length_prefixed.rs:166; xtask/gates/src/gates/lock_failures.rs:56
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
guards_reachable.rs:57   if rel.contains("/tests/") || rel.ends_with("_tests.rs") { continue; }
hash_inputs...rs:166     s.contains("/tests/") || s.ends_with("_tests.rs") || s.ends_with("/tests.rs")
lock_failures.rs:56      rel.contains("/tests/") || rel.ends_with("_tests.rs") || rel.ends_with("/tests.rs")
src/rpc/tests.rs  exists and contains #[cfg(test)]
```
Description: The test-file filter differs between gates. guards_reachable does not exclude `tests.rs`, so `src/rpc/tests.rs` is read as production. A call to a guard from a test file then counts as production wiring (is_called at line 154 scans all of `prod`). The `code()` filter strips `#[cfg(test)] mod` blocks, but a file that is wholly a test module with no inner attribute is not stripped.
Impact: A guard called only by src/rpc/tests.rs is counted as wired. Real wiring is overstated.
Trigger: precondition: a guard whose only caller is in a `tests.rs` file; confirm by `rg 'fn check_|fn verify_' src/rpc` against `rg` calls in src/rpc/tests.rs (not run).
Known: none

### P52-F06
File: xtask/gates/src/gates/logup_multipliers.rs:30-39, 88-131, 193-202
Severity: Medium
Axis: invariant
Confidence: Certain
Evidence:
```
logup_multipliers.rs:30-33  let needle = format!("let {term}"); let start = src.find(&needle)?;   (src is air_src, comments not stripped)
logup_multipliers.rs:92     if !air_src.contains(&inv) { continue; }
logup_multipliers.rs:106    if !air_src.contains(&format!("assert_bool({idx}_z")) {
logup_multipliers.rs:193    checked += 1;
logup_multipliers.rs:202    checked += 1;
logup_multipliers.rs:204    if checked == 0 { return Err(String::from("gate checked nothing")); }
```
Description: (1) `checked` is raised by a constant at lines 193 and 202, so the "gate checked nothing" guard at 204 can never fire. (2) find_def and the witness checks search `air_src` with comments still in it, so a commented `// let is_real_mem_op = ...` or `// assert_bool(rs1_idx_z)` is found first or satisfies the check. (3) If a `*_inv` name is removed from the AIR the whole witness check is skipped by `continue` with no problem and no count. (4) Only the four named locals are tested; an activity term built from `cur[COL_RS1]` directly is never seen.
Impact: A soundness edit to the AIR (R3, bud-proof) that moves the register-index logic to another name, or leaves a decoy comment, keeps the gate green.
Known: none

### P52-F07
File: xtask/gates/src/gates/named_tests.rs:17-22, 35-39 (used by governance_invariants.rs:21, network_hardening_gate.rs:37)
Severity: Medium
Axis: test
Confidence: Likely (missing step: the real CI log shape, which was not read; the weakness is in the matcher itself)
Evidence:
```
named_tests.rs:17  fn line_matches(line: &str, name: &str) -> bool {
named_tests.rs:19      Some(pos) => line[pos + name.len()..].contains("ok"),
```
Description: A line matches when the name occurs anywhere and the text after it contains the two letters `ok` anywhere (for example in `token`, `broken`, `look`). It does not require the `test ` prefix, the ` ... ok` suffix, or an exact name boundary (a longer test name containing the required name also matches). An ignored test with a reason, `test x ... ignored, token`, matches.
Impact: A required test that is ignored with a reason containing "ok", renamed to a longer name, or only mentioned in log text satisfies the gate. This covers 6 governance and 22 network hardening names.
Trigger: precondition: any log line with the test name followed later by `ok`; entry check_log (named_tests.rs:29). Expected wrong result: "named tests observed".
Known: none

### P52-F08
File: xtask/gates/src/gates/indexing_is_not_new.rs:53-120, 129-152, 176-211, 264-266
Severity: Medium
Axis: invariant(Z3)
Confidence: Certain
Evidence:
```
indexing_is_not_new.rs:264  if BASELINE.contains(&rel.as_str()) { continue; }
indexing_is_not_new.rs:208  if inner.chars().all(|c| c.is_ascii_digit() || c == '_') { continue; }
indexing_is_not_new.rs:197  let Some(close) = close else { continue };
indexing_is_not_new.rs:129  let Some(brace) = after.find('{') else { break; };
```
Description: (1) The baseline is a file list, not a count. The 66 listed files, including src/chain/blockchain.rs, src/consensus/pos.rs, src/crypto/primitives.rs and src/core/account.rs, may add any number of new panicking indexes; the docs claim "the debt cannot grow". There is also no check that a baselined file still has any index, so the list never shrinks by itself. (2) Any literal index passes, including `payload[0]` on a Vec or slice, which panics when empty; the doc limits this to fixed arrays. (3) An index split across lines (`x[\n i ]`) has no close bracket on the line and is skipped. (4) `#[cfg(test)]` followed by a bodyless item (`mod tests;`, `use`) strips the next `{ ... }` block of the file, which can be production code. Today only src/lib.rs has `#[cfg(test)] pub mod tests;` (src/lib.rs:51-52) and the next block there is itself a cfg(test) mod, so there is no live miss.
Impact: New panic points in R3 files are invisible. Release builds use panic=abort, so each is a remote crash if the index is input-controlled.
Known: none (clippy::indexing_slicing is separate and covers `--lib` only)

### P52-F09
File: xtask/gates/src/gates/hash_inputs_are_length_prefixed.rs:63, 219-240, 267-295, 381-403
Severity: Medium
Axis: invariant
Confidence: Certain
Evidence:
```
hash_inputs...rs:63   const VARIABLE_TYPES: &[&str] = &["Vec<u8>", "String", "BoundedBytes", "Vec<String>"];
hash_inputs...rs:267  if let Some(rest) = stripped.strip_prefix("pub") {
hash_inputs...rs:296  if stripped.starts_with("#[") { continue; }
hash_inputs...rs:401  let window = lines[cur.line.saturating_sub(1)..=cur.line].join("\n");
hash_inputs...rs:402  if window.contains("len()") { continue; }
```
Description: Only `pub` fields of `pub struct` with one of four exact types are variable-length. Private fields, `pub(crate)` fields, `Option<Vec<u8>>`, `Vec<Vec<u8>>`, `&[u8]`, `Bytes` and newtypes are never flagged. A field on the same line as `#[serde(default)]` is skipped. Only `.update(` with a plain path argument counts (`.clone().as_bytes()` resolves the field to `clone`); only the first `.update(` on a line is read; the pair test is "no `.update(` between", and any `len()` on the line or the line above (such as `if v.len() > 3`) clears it. The scan roots are also wrong (P52-F02).
Impact: The consensus-digest injectivity rule (four earlier bugs, per the module doc) is checked for a fraction of the digests. A new unprefixed pair using any of the shapes above passes.
Known: none

### P52-F10
File: xtask/gates/src/gates/lock_failures.rs:24-26, 28, 59-62, 140-148
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
lock_failures.rs:24  let Some(brace_rel) = head.find('{') else { return out; };   (out.push_str(&rest[..at]) happens later, at line 33)
lock_failures.rs:28  if !between.contains("mod") {
lock_failures.rs:61  t.contains("unwrap_or(true)") || t.contains("unwrap_or_else(|_| true)")
lock_failures.rs:140 let window_end = (i + 6).min(lines.len());
lock_failures.rs:148 let declared = exempt_from.iter().any(|e| *e <= i && i - *e <= 30);
lock_failures.rs:122 let raw = std::fs::read_to_string(path).map_err(...)
```
Description: (1) strip_test_mods returns `out` before pushing the text preceding the attribute, so a file whose last `#[cfg(test)]` has no `{` after it (for example `#[cfg(test)] mod tests;` at the end) loses all code since the last cut and is not scanned. (2) `between.contains("mod")` treats any text with "mod" (such as `model`) as a module and can strip a production block. (3) The fail-open test is two literal spellings within 6 lines of `.lock()`; `|e| true`, `|_|true`, `map_or(true, ..)`, `match .. { Err(_) => true }` and `RwLock` `.read()` and `.write()` are not seen. (4) One `FAILOPEN: allowed` marker exempts any lock within the next 30 lines, not one function. The marker is not used anywhere in the repo today (rg).
Impact: No live miss today (only src/lib.rs and src/tests/mod.rs use bodyless cfg(test) mods, and neither has `.lock()`). The gate narrows silently as code changes.
Known: none

### P52-F11
File: xtask/gates/src/gates/gov_slash_evidence_is_validator_only.rs:67-134, 193-209
Severity: Low
Axis: security
Confidence: Likely (missing step: a build of the stripper against src/core/account.rs; rg found no `'"'` literal there today)
Evidence:
```
gov_slash...rs:114  if bytes[pos] == b'"' {         (no char literal handling, `'"'` opens a string)
gov_slash...rs:193  if !block.contains("record.report.role != crate::registry::role::roles::VALIDATOR") {
gov_slash...rs:199  if !block.contains("bincode::serialize(&record.report)") {
```
Description: The role check and the digest serialization are substring tests inside the arm; the gate does not check they control flow (the comparison to `evidence_hash` gets a structured check, the role check does not). `let _ = record.report.role != ..VALIDATOR;` satisfies it. The stripper does not know char literals, so a `'"'` (or `'\"'`) in account.rs before or inside the arm flips string state for the rest of the file.
Impact: The CWE-345 fix (any-role evidence for SlashValidator) can be regressed to an inert role check with the gate green. No `'"'` is present in src/core/account.rs today.
Known: none

### P52-F12
File: xtask/gates/src/gates/gating_flags.rs:12, 49-63, 78-125
Severity: Low
Axis: invariant
Confidence: Certain
Evidence:
```
gating_flags.rs:12  const FLAGS: &[(&str, &str)] = &[("COL_REG_SAME", "r_same"), ("COL_MEM_SAME", "m_same")];
gating_flags.rs:54  code.contains(&format!("assert_bool({local}"))
gating_flags.rs:58  code.contains(&format!("one.clone() - {local}.clone()"))
gating_flags.rs:62  code.contains(&format!("assert_eq({local}.clone()"))
```
Description: Two hardcoded flags; any new gating column is unchecked. Each of boolean, negated and pinned is a whole-file substring. `assert_bool(r_same` anywhere in plonky3_air.rs (any function, any builder) and `assert_eq(r_same.clone(), anything)` satisfy the gate; the pin does not need to relate to the gated condition. String literals are not stripped (only comments) and `//` inside strings cuts the line.
Impact: A weakened constraint on a SAME flag that keeps one of these substrings elsewhere stays green. Soundness of the memory and register argument rests on review.
Known: none

### P52-F13
File: xtask/gates/src/gates/gates_do_not_panic.rs:97-106, 133-134, 153-158, 246-248
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
gates_do_not_panic.rs:105  t.contains(".unwrap()") || t.contains(".unwrap_err()") || t.contains(".expect(\"")
gates_do_not_panic.rs:134  text.truncate(120);
gates_do_not_panic.rs:157  is_fn && (trimmed.contains("self_test") || trimmed.contains("canar"))
gates_do_not_panic.rs:246  let Ok(text) = std::fs::read_to_string(path) else { continue; };
```
Description: (1) `panic!`, `unreachable!`, `todo!`, `.expect(&format!(..))`, `.expect(msg)` and direct indexing are not panic points. (2) `String::truncate(120)` panics when byte 120 is inside a multi-byte character; the gate that forbids panics can panic. (3) Any fn whose signature contains `canar` or `self_test` is skipped to its end, so a helper named `canary_x` hides its body. (4) A file that cannot be read is skipped but still counted toward the 40-file floor. (5) A line that starts with `write!`/`writeln!` exempts everything on that line, including a second `.unwrap()`.
Impact: Gate code can panic and hide the findings after it. Low, because gate code is not shipped.
Known: none

### P52-F14
File: xtask/gates/src/gates/git_deps_audited.rs:28, 32, 35-43
Severity: Low
Axis: security
Confidence: Likely (missing step: Cargo.lock reading was denied; whether any lock has a `branch=` or `tag=` git source was not checked)
Evidence:
```
git_deps_audited.rs:35  if let Some(rest) = t.strip_prefix("source = \"git+") {
git_deps_audited.rs:36      if let Some(rev) = rest.find("rev=") {
git_deps_audited.rs:28  if !s.contains("/target") {
Cargo.toml:68  libp2p = { git = "https://github.com/libp2p/rust-libp2p", rev = "38b8a2c...", features = [
```
Description: Only lock sources with `?rev=<40hex>` are collected. A git dependency pinned by `branch` or `tag` has a lock source `git+URL?branch=x#<40hex>` and is not collected, so it needs no audit record. The directory filter skips any path containing the substring `/target` (for example `/targets/`, `/targeted`), and unreadable lock files are skipped (line 32).
Impact: An unaudited git dependency by branch or tag passes this gate. The cargo-deny `unknown-git` rule (Cargo.toml:64) is the other line of defence.
Known: none

### P52-F15
File: xtask/gates/src/gates/multi_ratio_consensus.rs:13-15, 86-108, 125, 139, 164-185
Severity: Low
Axis: invariant
Confidence: Certain
Evidence:
```
multi_ratio_consensus.rs:14-15  //! ... its non-zero `pipe_id`, and a finite `ratio` above one.
multi_ratio_consensus.rs:180     if c.contains("ratio:") {
multi_ratio_consensus.rs:125     let arms = body.matches("=> vec![").count();
multi_ratio_consensus.rs:139     if c.contains("pipe_name: \"default\"") && c.contains("ratio: 1.0") {
bud/src/bud_format.rs:618        pub ratio: f64,
```
Description: The doc says the gate checks a finite ratio above one; the code only checks the text `ratio:` is present, so `ratio: 0.0` or `f64::NAN` passes. Duplicate `pipe_id` values across candidates are not detected. `=> vec![` also matches an empty arm `=> vec![]`, so four empty arms count as four format classes. Candidates built by a helper (not the literal `RatioCandidate {`) are not inspected at all. The forbidden list is six name fragments. The gate fixes `ratio: 1.0` f64 as the identity sentinel; ratios are `f64` and compared by `to_bits` in the signed certificate (bud/src/bud_format_bft.rs:155-161), which touches Z8 if that certificate is a consensus path.
Impact: The generative-pipeline exclusion is easy to route around. The float ratio in a signed vote is a determinism question for architect review (not decided here).
Known: none

### P52-F16
File: xtask/gates/src/gates/metrics_are_written.rs:107-176, 295-322, 340
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
metrics_are_written.rs:340  if !matches!(ty, "IntCounter" | "IntGauge" | "Histogram") {
metrics_are_written.rs:296  let anchor = format!(".{field}");
metrics_are_written.rs:148  b';' => break,
```
Description: Fields of type `IntCounterVec`, `IntGaugeVec`, `HistogramVec`, `Gauge` and `Counter` are not parsed, so they are never checked (rg found none in src/core/metrics.rs today, 37 matching fields). A write is `.field.set(` on any receiver in any production file, so another struct with the same field name counts. strip_cfg_test treats a `;` before the first `{` as a bodyless item; `#[cfg(test)] fn f(a: [u8; 4]) { .. }` leaves the body in the blob as production. Braces inside strings and comments are counted before the comment stripper runs.
Impact: A new vec-type metric that is never written is not reported. Currently no such field.
Known: none

### P52-F17
File: xtask/gates/src/gates/generated_content.rs:27, 54-56, 90; xtask/gates/src/gates/master_derivation.rs:36-58; xtask/gates/src/gates/mempool_mev_free.rs:50-60, 84, 113, 123
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
generated_content.rs:54   let end = rest.find("}\n").map_or(rest.len(), |e| e + 2);
generated_content.rs:90   if body.is_none_or(|b| !b.contains("meter.charge")) {
master_derivation.rs:56   if !code.contains(&format!("fn {t}")) {
mempool_mev_free.rs:54    if name.starts_with("by_") && (name.contains("priority") || name.contains("boost") || name.contains("bid"))
mempool_mev_free.rs:123   if src.contains("fn test_same_fee_canonical_order_by_hash") {
```
Description: These three are presence-by-substring gates over whole files with comments left in. generated_content: the id-check body ends at the first `}\n` (usually an inner block), and `meter.charge` or `IdMismatch` in a comment satisfies the check. master_derivation: the five regression tests count if `fn name` occurs anywhere, including a comment, a non-`#[test]` fn, or a fn under `#[ignore]`; `MasterStillDerived` and the other variants match in comments. mempool_mev_free: the `BTreeMap` field text and `by_fee.iter().rev()` can sit in a comment, the tie-break test is only a name match, and a bidding index is recognised only when the field name starts `by_` and contains priority, boost or bid (`tips`, `fast_lane` or a `BinaryHeap` pass).
Impact: Each invariant can be regressed while its text stays in the file. These are R2 and R3 storage and mempool surfaces.
Known: none

### P52-F18
File: xtask/gates/src/gates/license_consistency.rs:60, 77-83, 134-136, 153-182, 199
Severity: Low
Axis: invariant
Confidence: Certain
Evidence:
```
license_consistency.rs:153  if lic.len() < CANONICAL_LENGTH || hex32(&keccak256(&lic[..CANONICAL_LENGTH])) != CANONICAL_KECCAK {   (only LICENSE.md)
license_consistency.rs:164  if !text.contains("PolyForm Shield License 1.0.0") {      (budzero/LICENSE: title and "## section" substrings only)
license_consistency.rs:199  let is_package = text.lines().any(|l| l.trim() == "[package]");
license_consistency.rs:134  if kind.is_symlink() { continue; }
```
Description: budzero/LICENSE is not hash-pinned, only checked for headings, so its body can be edited without a failure. A manifest whose header is `[package] # comment` is not a package and is skipped. Symlinked directories are not walked. Solidity SPDX headers in contracts/ and non-Cargo manifests are outside the check.
Impact: The legal text of the BudZero copy can drift. Low.
Known: none

### P52-F19
File: xtask/gates/src/gates/no_conflict_markers.rs:26, 37-39, 45-50, 108-111
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
no_conflict_markers.rs:38  line.starts_with("<<<<<<< ") || line.starts_with(">>>>>>> ") || line == "======="
no_conflict_markers.rs:57  for (lineno, line) in text.split('\n').enumerate() {
no_conflict_markers.rs:26  const SKIP_DIRS: &[&str] = &[".git", "target"];
no_conflict_markers.rs:110 if found.is_empty() { return Ok(...) }
```
Description: The input is split on `\n` only, so in a CRLF file the separator line is `=======\r` and is missed (the open and close markers are still caught). Any directory named `target` at any depth is skipped. Files that cannot be read are skipped. There is no minimum file count, so a wrong root reports OK on an empty walk.
Impact: Low, since open and close markers are still found. A wrong root is a vacuous pass.
Known: none

### P52-F20
File: xtask/gates/src/gates/kani.rs:63-92, 279-292; .github/workflows/kani-slow.yml:87-104
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
kani.rs:63   fn declared_count(path: &Path, scope_all: bool) -> Result<usize, String> {
kani.rs:285      let declared = declared_count(&proofs_path(root), false)?;
kani-slow.yml:99   elif grep -q '^VERIFICATION:- SUCCESSFUL' "/tmp/$h.log"; then
kani.rs:95   fn fn_name(trimmed: &str) -> Option<&str> {     (name chars: lowercase, digit, underscore only)
```
Description: The all-scope count exists only for tests; no command-line mode calls it. The three slow harnesses are checked only by the workflow's own grep, which treats TIMEOUT as non-failing by design, and the count gate never sees them. A harness name with an uppercase letter is invisible to `--fast-names` and `--slow-names`, so the run would then fail the count rather than run it.
Impact: The three slow bond-arithmetic proofs can time out indefinitely with a green job. Documented behaviour; filed for the mainnet record.
Known: none

### P52-F21
File: xtask/gates/src/gates/genesis_schema.rs:68-73, 100-134, 143
Severity: Low
Axis: invariant
Confidence: Certain
Evidence:
```
genesis_schema.rs:143  const GENESIS_FILES: &[&str] = &["config/mainnet-genesis.json"];
genesis_schema.rs:121  for key in ["allocations", "validators"] { if !map.get(key).is_some_and(Value::is_array) {
```
Description: Only `config/mainnet-genesis.json` is read. `allocations` and `validators` need only be arrays (empty passes). Ratios such as `annual_burn_ratio_fixed` may be zero. There is no check that allocations sum to the 100M cap, or that addresses and validator keys are unique and well formed. Other genesis profiles under config/ are not validated.
Impact: A malformed testnet or profile genesis, or a mainnet file with an empty validator list, passes the schema gate. The Rust lock test mentioned in the doc is the only other check.
Known: none

### P52-F22
File: xtask/gates/src/gates/geiger.rs:27-41
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
geiger.rs:39  (*name == "budlum-core" || *name == "verifier-registry" || name.starts_with("bud-"))
Cargo.toml:160  budlum-bpqs = { path = "crates/bpqs", optional = true }
crates/*/Cargo.toml names: budlum-bpqs, budlum-wallet-core, budlum-note-packing, budscan, budlum-kani
```
Description: First-party means `budlum-core`, `verifier-registry` or `bud-*`. `budlum-bpqs` is a first-party path dependency of the root crate (optional) and would be reported as a third-party row, informational only.
Impact: Unsafe usage in budlum-bpqs would not fail the gate. crates/bpqs/src/lib.rs has `forbid(unsafe_code)`, so no live miss.
Known: none

### P52-F23
File: xtask/gates/src/gates/mermaid.rs:312, 363-366; geiger.rs:36-38; guards_reachable.rs:273; xtask/gates/src/gates/geiger.rs:111-133
Severity: Info
Axis: LABEL-12.9
Confidence: Certain
Evidence:
```
mermaid.rs:312  let _ = is_edge(line);                       (is_edge result unused)
mermaid.rs:365  ("crates/ai-inference/ARCHITECTURE.md", 0),   (floor 0, but a missing file still fails at read_to_string)
geiger.rs:36-38  // `verifier-registry` budzero'nun sekizinci uyesi ve isim kalibina ... sayilirdi.
guards_reachable.rs:273  "canary: uncalled guard sayilmadi"
geiger.rs:111-133  scratch files temiz.txt, kirli.txt, bos.txt
```
Description: mermaid.rs has dead code (is_edge) and only flowchart or graph diagrams are judged; a fence that is not exactly "```mermaid" is not seen. Turkish words remain in comments and strings in the part (Z13 says repo text is English); .github/turkish-baseline.txt may already cover these (not read).
Impact: None on correctness.
Known: possibly .github/turkish-baseline.txt (not read)
