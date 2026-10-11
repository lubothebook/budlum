# P57 scan: xtask tools (read-only audit, Tier R1)

Base commit: 8081a84. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md, notes/P50-scan.md to notes/P54-scan.md. Part scope: xtask/tools/*, 5873 lines.

## COVERAGE

Read in full (every line, tests included):
- xtask/tools/Cargo.toml 1-20
- xtask/tools/Cargo.lock 1-7 (size seen with wc, content not needed: no dependencies)
- xtask/tools/src/lib.rs 1-276
- xtask/tools/src/support.rs 1-120
- xtask/tools/src/report.rs 1-90
- xtask/tools/src/json.rs 1-362
- xtask/tools/src/rpc.rs 1-254
- xtask/tools/src/bin/budlum-tools.rs 1-163
- xtask/tools/src/prepush.rs 1-228
- xtask/tools/src/devnet.rs 1-243
- xtask/tools/src/seed_corpus.rs 1-209
- xtask/tools/src/backup_drill.rs 1-311
- xtask/tools/src/audit_deps.rs 1-273
- xtask/tools/src/sbom.rs 1-213
- xtask/tools/src/smoke_rpc.rs 1-339
- xtask/tools/src/docker_smoke.rs 1-388
- xtask/tools/src/multinode_smoke.rs 1-432
- xtask/tools/src/clippy_extra_report.rs 1-213
- xtask/tools/src/fsf_fit.rs 1-404
- xtask/tools/src/audit_guard.rs 1-638
- xtask/tools/src/step_reachability.rs 1-717

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 110-165; notes/P29-scan.md lines 1-30.

Outside the part (rg first, at most 80 lines): budzero/bud-isa/src/lib.rs 8-45 (opcode table); src/main.rs 470-500 (check-db output); src/cli/commands.rs (rg on flag names only); .github/workflows/ci.yml 170-205, 240-265, 520-535, 1170-1190; .github/workflows/audit-guard.yml 20-50; .github/workflows/osv-scanner.yml 100-120; .github/workflows/semver.yml 42-58; fuzz/README.md (rg); docs/FSF_PROJECT_FIT.md (rg for bold); rg over .github/workflows for every `xtask/tools` caller.

Not read: docs/FSF_ADAPTATION_PLAN.md beyond one rg line, the node source behind the RPC methods used by the smoke tools, ops/docker-compose*.yml, ops/Dockerfile. No cargo, no test run: every claim comes from reading source.

Extra-check summary:
- Silent success: five cases (P57-F03, F05, F06, F11, F14). None is a hard gate that always passes, each is a weakened check.
- Secrets in output (Z9): none found. smoke_rpc prints the node log tail and the RPC reply only. docker_smoke prints 80 log lines on failure. No key material is read by any tool. The p2p identity file path is passed to the node but never read.
- Path traversal and destructive paths: P57-F04 (SMOKE_DB_PATH wipe), P57-F13 (repo_root fallback to cwd). devnet.rs deletion is guarded and safe.
- Nondeterminism in generated artifacts: fsf_fit output is deterministic. audit_deps and sbom reports carry a wall-clock timestamp but live under target/ only. sbom.cdx.json comes from cargo-cyclonedx (serial number and timestamp come from the tool, not read here).
- Z3: no `#[allow]`, no `#[ignore]`, no `|| true` in the part. Z6: no script files in the part; prepush.rs:156 writes a /bin/sh hook at runtime into .git/hooks (not committed). Z7: `unsafe_code = "forbid"` at Cargo.toml:16 and lib.rs:3, no dependencies. Z13 and Z10: P57-F12.

Summary: 0 Critical, 0 High, 5 Medium, 12 Low, 2 Info (19 total).

## FINDINGS

### P57-F01
File: xtask/tools/src/audit_guard.rs:56-62, 145-152
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
56  fn has_key(text: &str, key: &str) -> bool {
57      text.lines().any(|line| {
58          line.trim_start()
59              .strip_prefix(key)
60              .is_some_and(|rest| rest.trim_start().starts_with(':'))
145 if has_key(text, "pull_request_target") {
148 if has_key(text, "workflow_run") {
```
Description: The forbidden-trigger rule matches only a line that starts with the key and a colon. The inline form `on: [push, pull_request_target]`, the scalar form `on: pull_request_target` and the list form `- pull_request_target` are not matched. The red-team mutant (audit_guard.rs:416-419) uses only the map form, so the self-test cannot see the gap.
Impact: Anyone who edits a workflow can add a privileged trigger in an inline form and the gate named "fail-closed review boundary" stays green.
Trigger: precondition: a workflow change in a PR; entry: workflow_text_audit (audit_guard.rs:133); input: `on: [pull_request_target]`; wrong result: no error is pushed.
Known: none

### P57-F02
File: .github/workflows (no caller), xtask/tools/Cargo.toml:1-20
Severity: Medium
Axis: test
Confidence: Certain
Evidence: `rg -n 'xtask' .github/workflows | rg -v gates` lists only `cargo run` lines (provenance.yml:60, audit-guard.yml:43,47, docker-smoke.yml:106,109,132, ci.yml:196,261,262,526,531,1180,1182). There is no `cargo test`, `cargo clippy` or `cargo fmt` for xtask/tools. For gates the workflow has them (ci.yml:1723,1725).
Description: The crate sets `pedantic = deny` (Cargo.toml:19-20) but clippy never runs on it, and the roughly 150 unit tests of the crate (including `the_committed_document_is_the_generated_one` at fsf_fit.rs:394 and `the_real_workflows_have_no_dangling_references` at step_reachability.rs:707) never run in CI. The bin `--self-test` (budlum-tools.rs:109) is also not called by any workflow.
Impact: Five tools gate CI (audit-guard, step-reachability, audit-deps, generate-sbom, smoke tools). A regression in them is caught only when it breaks the tool run itself, not by its own tests. The lint level is unenforced.
Known: none

### P57-F03
File: xtask/tools/src/smoke_rpc.rs:108-110, 172-194
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
108 pub fn reply_has_result(body: &str) -> bool {
109     body.contains("\"result\"")
173 for attempt in 1..=TRIES {
174     if let Ok(reply) = rpc::call(operator, "bud_chainId", "[]", REQUEST_TIME) {
175         body = Some(reply);
179     if matches!(guard.0.try_wait(), Ok(Some(_))) {
```
Description: The pass test is a substring search for the text `"result"`; the JSON reader in the same crate (json.rs) is not used, and the chain id value is not compared with the network (docker_smoke.rs does compare it). An error reply whose message contains `"result"` also passes. The first successful reply ends the loop before the child exit check at line 179 runs, so a stale listener already bound to the operator port (a leftover node) answers for a node that died on bind.
Impact: The smoke step can report OK for the wrong process or for an RPC error. It runs in docker-smoke.yml:106 as the devnet boot gate.
Known: none

### P57-F04
File: xtask/tools/src/smoke_rpc.rs:56, 133-142, 208-218
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
56  db_path: get("SMOKE_DB_PATH").map(PathBuf::from),
208 fn reset_dir(path: &Path) -> Result<(), String> {
209     if path.as_os_str().is_empty() || path.parent().is_none() {
213         std::fs::remove_dir_all(path)
```
Description: The guard refuses only an empty path and `/`. `SMOKE_DB_PATH=.`, a relative path, `$HOME` or the repository root pass (`Path::new(".").parent()` is `Some("")`) and are removed recursively. The module doc and lib.rs header claim the crate removes the `rm -rf "$VAR/"` class.
Impact: A wrong environment value deletes the operator's working directory or home directory.
Trigger: precondition: SMOKE_DB_PATH set by a person or CI step; entry: smoke_rpc::run (smoke_rpc.rs:128); input: `.` or an existing project path; wrong result: recursive delete before any check of the content (devnet.rs:46-66 shows the content check that is missing here).
Known: none

### P57-F05
File: xtask/tools/src/backup_drill.rs:82-110, 168, 191-200
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
168 let backup = newest_backup(&cfg.backup_dir)?;
191 let check = Command::new(&cfg.binary)
194     .output()
196 let stdout = String::from_utf8_lossy(&check.stdout).into_owned();
198 let verdict = integrity_line(&stdout).map(ToString::to_string);
```
Description: (a) The restored backup is the newest file in the directory by mtime; nothing proves it was written by this run, so a node that exits 0 without writing a backup (or a clock step back) lets an old file pass. (b) The exit status of `--check-db` is ignored; the node binary documents that its exit status carries the verdict (src/main.rs:476-478). (c) The drill never compares restored state (height, state root) with the source. (d) At line 195 the `?` on a failed spawn leaves `restore_parent` in /tmp. (e) The drill has no workflow caller: `rg -n 'backup-drill' .github ops` has no match; only README.md:316 mentions it.
Impact: The backup and restore drill, a mainnet operations check, can pass without proving a fresh backup restores to the same chain. It is also never executed by CI.
Known: none

### P57-F06
File: xtask/tools/src/fsf_fit.rs:284-322; docs/FSF_ADAPTATION_PLAN.md:26
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
294 if self_test {
295     if !errors.is_empty() { ... }
298     return Report::ok(format!("fsf project fit self-test ok: ...
311 if check {
```
Description: `--self-test` returns before `--check` is evaluated, and `--write` wins over `--check`. The plan states `budlum-tools fsf-project-fit --self-test --check` pins the curation file (FSF_ADAPTATION_PLAN.md:26). That command line never compares the document with the generated text. No workflow runs fsf-project-fit (`rg -in fsf .github` is empty), so the only drift guard is a unit test that CI does not run (P57-F02).
Impact: docs/FSF_PROJECT_FIT.md can drift from the tool source with all CI green.
Known: none

### P57-F07
File: xtask/tools/src/audit_guard.rs:121-130, 218-228
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
123 for key in ["contents", "actions", "pull-requests"] {
126     return value == "write" || value == "write-all";
218 let has_pull_request = text.lines().any(|line| line.trim() == "pull_request:");
```
Description: Three gaps in the write-permission rule. The key `permissions: write-all` (scalar form) is not one of the three keys. The scopes `id-token`, `packages`, `security-events`, `checks`, `issues`, `statuses`, `deployments` and `pages` are not checked. A workflow declaring the trigger as `on: [pull_request]` or `on: pull_request` does not set `has_pull_request`, so the whole rule is skipped.
Impact: A pull_request workflow can gain write scopes without a finding.
Known: none

### P57-F08
File: xtask/tools/src/audit_guard.rs:133-238 (scope of rules)
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence: `swallows_failure` and `has_pair(text, "continue-on-error", "true")` run only when `rel` ends with `audit-guard.yml` (lines 159-172). For every other workflow no rule looks for `continue-on-error`, `|| true`, `|| exit 0` or `set +e`. AUDIT_PLAN.md section 5 lists these as the Z3 scan patterns (P62), and the tool describes itself as the gate that catches "a CI configuration that silently stops auditing" (lines 5-8).
Description: The Z3 patterns are checked by hand scan only. Also: the commit-subject scan in the gates job of ci.yml:244-262 runs on a depth-1 checkout (no fetch-depth), so there `git log HEAD` sees one commit while the pass message says "reachable commit subjects ... clean" (audit_guard.rs:533). audit-guard.yml uses fetch-depth 0, so the full scan exists there.
Impact: Z3 regressions in workflows are not machine-checked. The ci.yml copy of the history check is shallow.
Known: none

### P57-F09
File: xtask/tools/src/audit_guard.rs:134-137, 202-216, 230-237
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
136 .map(|line| line.split('#').next().unwrap_or(""))
206 let end = (start + 12).min(lines.len());
207 if !lines[start..end].iter().any(|l| l.contains("persist-credentials: false"))
```
Description: Three weaknesses of the line rules. A `#` inside a string (`echo "a#b"; git push`) cuts the rest of the line, so a push after it is hidden. The checkout credential rule accepts any `persist-credentials: false` within the next 12 lines, which can belong to the next step's checkout. `git push` is matched only as the literal words (`git -c x=y push` and `git push` split by a backslash newline escape it).
Impact: Defense in depth only. A workflow can hide a push or keep credentials with the guard green.
Known: none

### P57-F10
File: xtask/tools/src/step_reachability.rs:186-222, 302-311
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
209 if let Some((id, want)) = outcome_test(e) {
210     return Ok(ids.get(id).is_some_and(|got| got == want));
302 let run = match &s.guard {
303     Some(guard) => match evaluate(guard, &ids, failed) {
```
Description: GitHub Actions wraps an `if:` that contains no status function in `success() &&`. `evaluate` does not. The `--fail` table therefore shows RUN for `if: steps.a.outcome == 'success'` after a red step, while the runner skips it. A step with `continue-on-error` is also treated as failing the job. This is the diagnostic the tool exists for.
Impact: The reachability table can overstate which gates still run after a failure. The hard findings (`check`) are not affected.
Known: none

### P57-F11
File: xtask/tools/src/step_reachability.rs:85-102, 132, 247-287, 374-388
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
132 guard: field(&block, "if").filter(|v| !v.is_empty()),
 85 fn field(block: &str, name: &str) -> Option<String> {
381 .filter(|n| Path::new(n).extension().is_some_and(|x| x == "yml") && !n.starts_with('.'))
```
Description: Latent silent passes. (a) A folded or literal guard (`if: >-`) yields the guard text `>-`, so `steps.<id>` references on the following lines are never checked. One such guard exists today (semver.yml:47) and has no step reference. (b) Any 6-space `- ` list item inside a job (for example `needs:`) is counted as a step. (c) `field()` takes the first `id:`/`if:`/`name:` line anywhere in the block, including `with:` and `run:` bodies. (d) Only `*.yml` files are scanned (line 381); audit_guard.rs:248-251 also scans `*.yaml`. (e) `--fail` as the last argument is ignored and the tool exits 0 (lines 512-513, 532). (f) `self_test` reads unreadable real workflows as empty text (line 456) and counts zero findings.
Impact: A dangling or forward `steps.<id>` guard in a multi-line `if:` or in a `.yaml` workflow is not reported. This is the failure the tool was written to stop.
Known: none

### P57-F12
File: xtask/tools/src/fsf_fit.rs:185-191, 228; xtask/tools/src (Turkish strings)
Severity: Low
Axis: invariant(Z13)
Confidence: Certain
Evidence: The generated, committed docs/FSF_PROJECT_FIT.md has markdown bold at lines 4, 7 and 45 (`**phase 0**`, `**does not**`, `**not**`), produced by the string literals at fsf_fit.rs:188,191,228. Turkish repo text in code: prepush.rs:48,160,169,172,179-184; devnet.rs:71,74,92,96; seed_corpus.rs:79,107,147; lib.rs:165,273; backup_drill.rs:278; budlum-tools.rs:9 and the word `arac` in usage lines 6 and 36 (rg count: 22 lines in 6 files). Z13 requires English repo text and no bold. Z12 check on the part: no match.
Description: Style rule violations, no behavior effect.
Impact: Maintenance only.
Known: none

### P57-F13
File: xtask/tools/src/lib.rs:96-105
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
 98 root_above(&cwd).unwrap_or_else(|| {
 99     option_env!("CARGO_MANIFEST_DIR")
102         .filter(|root| is_repo_root(root))
103         .map_or(cwd, Path::to_path_buf)
```
Description: When neither the working directory nor the build-time path is a checkout, `repo_root` returns the cwd without a marker check and the tool acts there. `devnet` then creates `./data` and deletes `node1.db`, `node2.db` and `validators.json` in an unrelated directory (devnet.rs:67-76). `install-hook` fails on its own check, `seed-corpus` writes `fuzz/corpus/zkvm` under the cwd. The doc comment describes only the panic case.
Impact: A tool started outside the repo writes into the wrong directory instead of failing.
Known: none

### P57-F14
File: xtask/tools/src/sbom.rs:38, 59-87, 135-142; xtask/tools/src/audit_deps.rs:41-46
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
sbom.rs 135  None => "0".to_string(),
sbom.rs 140  .map_or_else(|| "?".to_string(), |items| items.len().to_string()),
sbom.rs 114  ... .contains(CYCLONEDX_VERSION)
audit_deps.rs 41  run_checked("cargo", &["install", "--locked", "cargo-audit"], root)?;
audit_deps.rs 45  let root_exit = audit_exit(root, ROOT_LOCK)?;
```
Description: (a) An SBOM with zero components or a non-array `components` is written and reported as success; the count is printed only. The provenance workflow adds its own emptiness check (provenance.yml:68), the ci.yml SBOM step does not. (b) The SBOM covers the root package only (`budlum-core.cdx.json`), while the report text says it "includes every transitive dependency" for the external audit firm; budzero, bud, fuzz and the crates use separate lockfiles. (c) The version pin test is a substring match: `0.5.90` would satisfy `0.5.9`. (d) audit_deps installs `cargo-audit` unpinned when absent (the extra-tooling.yml gate pins 0.22.2) and audits only Cargo.lock and budzero/Cargo.lock; the other lockfiles are covered by osv-scanner.yml:104-114, which also checks that every lockfile is listed. (e) The verdict comes from the `--json` run while the printed run uses `--deny warnings` and its exit code is dropped (audit_deps.rs:125-126), so the report summary can say "OK NO known security hole" next to printed warnings.
Impact: Weaker supply-chain evidence than the report text claims. Tool version can drift between runs.
Known: none

### P57-F15
File: xtask/tools/src/seed_corpus.rs:31-63; fuzz/README.md:53-60
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: The seeds are labelled `Jmp (0x0A)`, `Poseidon (0x1D)`, `SRead (0x10) + SWrite (0x11)` (seed_corpus.rs:42,55,60). budzero/bud-isa/src/lib.rs:21,30,32-35 defines `Jmp = 0x10`, `Eq = 0x0A`, `Poseidon = 0x19`, `Syscall = 0x1D`, `SRead = 0x1B`, `SWrite = 0x1C`, `Jnz = 0x11`. `ls fuzz/corpus` shows no `zkvm` directory and `git ls-files fuzz/corpus` lists none; the 14 fuzz targets (for example `vm_execute`) read `fuzz/corpus/<target>`, and the nightly workflow caches `fuzz/corpus/${{ matrix.target }}` (fuzz-nightly.yml:85).
Description: Three of five seeds do not encode the opcode they claim, and the output directory `fuzz/corpus/zkvm` is not a corpus of any target. The code comment admits staleness is tolerated. No workflow calls `seed-corpus`.
Impact: The ZKVM fuzzers get no useful seeds from this tool. Dead tooling.
Known: none

### P57-F16
File: xtask/tools/src/prepush.rs:39-50, 126-136
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
126 let code = run_capturing_status("cargo", &["fmt", "--", "--check"], &tmp)?;
129 if code == 0 { return Err("CANARY FELL: ...
136 Ok("pre-push canary OK: a broken format was refused, ...
 44 Ok(out) => lines.push(format!("toolchain: {}", ...
```
Description: The canary treats any non-zero exit as "refused". If rustfmt is missing, or cargo cannot resolve the toolchain, the exit code is also non-zero and the canary reports OK. The `cargo --version` result at line 44 ignores the exit status. `pre-push` never compares the toolchain with `rust-toolchain.toml`; it only prints the version. `install_hook` overwrites an existing `.git/hooks/pre-push` without asking (line 160). Z2 says CI is the single verifier; pre-push runs fmt and clippy locally by design of the tool but conflicts with the directive for agents (humans only).
Impact: A false green canary. Possible loss of a developer's own hook.
Known: none

### P57-F17
File: xtask/tools/src/rpc.rs:62-77, 86-94; xtask/tools/src/multinode_smoke.rs:196, 339-358
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
rpc.rs 62  let mut raw: Vec<u8> = Vec::new();
rpc.rs 74  Ok(n) => raw.extend_from_slice(&chunk[..n]),
rpc.rs 89  if reply.is_ok() { Ok(reply.body) }
multinode 196  node1("bud_netListening").is_ok_and(|b| b.contains("\"result\":true"))
multinode 350  if l > n2 { n2 = l; }
```
Description: (a) `rpc::request` reads without a size cap until close or deadline; local use only, so the effect is a few MB at most. (b) `rpc::call` returns Ok for an HTTP 200 body that holds a JSON-RPC `error`; every caller must check, and smoke_rpc and multinode use substring tests (P57-F03). (c) In step 6 of the multinode smoke the log witness replaces the gauge whenever it is higher, so one broken or lagging witness does not fail the step and one inflated witness passes it. (d) `compose_up` does not tear down an earlier project of the same name, so a stale container set can answer the probes (the workflow does the teardown, multinode_smoke.rs:24). (e) The isolation step passes on any connect error, including all nodes being down.
Impact: Smoke checks pass on weaker proof than their text claims.
Known: none

### P57-F18
File: xtask/tools/src/docker_smoke.rs:36, 177-193
Severity: Info
Axis: test
Confidence: Likely (missing step: which source line prints the marker)
Evidence: `REFUSAL_MARKER = "CRITICAL SECURITY FAILURE"` is the only proof that mainnet refused for the intended reason (no bootnodes, no genesis, no HSM). `judge_mainnet` accepts any non-zero exit whose log has the marker.
Description: A different path that prints the same marker satisfies the test; the text of the reason is not compared.
Impact: Observation only.
Known: none

### P57-F19
File: xtask/tools/src/bin/budlum-tools.rs:25-27, 109-130
Severity: Info
Axis: test
Confidence: Certain
Evidence: The header says `--self-test` is "the canary of every tool". The loop runs four canaries (seed-corpus, devnet, backup-drill, pre-push). audit-guard, step-reachability and fsf-project-fit have their own `--self-test` flags, and the smoke, sbom and audit-deps tools have none.
Description: The usage text and header overstate the coverage of `--self-test`. In `usage()` the lines 44-48 lack the `\n\` continuation used on the other lines, so the output relies on literal newlines and leading spaces (cosmetic).
Impact: None on behavior.
Known: none

## Not findings (checked and clean)

- json.rs: no panic on hostile input; depth capped at 256 (json.rs:22,113), strict trailing-data check, UTF-8 checked. Lone surrogates become U+FFFD silently (json.rs:243-246), acceptable for its readers.
- rpc.rs: connects to 127.0.0.1 only, deadline enforced on every read.
- devnet.rs: the delete guard refuses foreign files and the canary proves it (devnet.rs:46-66, 147-183).
- support.rs `format_utc` and `make_temp_dir` use create_dir, no predictable-name race inside the production path. The tests use fixed names under the system temp directory (for example prepush.rs:194,203, devnet.rs:191,205), which can collide when tests run in parallel with a second checkout; Info only.
- Outside the part, for P58: .github/workflows/provenance.yml:71 contains an inline Python snippet (`json.load`), a Z6 candidate for the P62 repo-wide scan.
