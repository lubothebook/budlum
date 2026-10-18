# P54 scan: xtask gates 5 (read-only audit, Tier R1)

Base commit: 75ca028. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14, P58, P59), notes/P50-scan.md, notes/P52-scan.md.

## COVERAGE

Read in full (every line, tests included), files 88 to 99 of `ls xtask/gates/src/gates/*.rs`:
- xtask/gates/src/gates/regeneration.rs 1-1610
- xtask/gates/src/gates/rejection_tests.rs 1-468
- xtask/gates/src/gates/relay.rs 1-659
- xtask/gates/src/gates/release_profile_pins.rs 1-239
- xtask/gates/src/gates/repair_fires.rs 1-159
- xtask/gates/src/gates/required_tests.rs 1-320
- xtask/gates/src/gates/rust_literals.rs 1-259
- xtask/gates/src/gates/security_parameters.rs 1-1055
- xtask/gates/src/gates/security_scans_can_fail.rs 1-405
- xtask/gates/src/gates/self_derived_ids_cover_every_field.rs 1-718
- xtask/gates/src/gates/semver.rs 1-689
- xtask/gates/src/gates/serialize_map_keys_are_strings.rs 1-688

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 110-165; notes/P29-scan.md lines 1-30.

Outside the part (rg first, then at most 80 lines): xtask/gates/src/main.rs (registration lines only), .github/semver-exceptions.txt (30 lines), .github/CODEOWNERS (head), .github/workflows/semver.yml (rg), .github/workflows/diverse-double-compiling.yml 118-150, workflow job names (rg), budzero/bud-proof/src/plonky3_prover.rs 78-106, budzero/bud-proof/src/bud_stark/config.rs 138-150, bud_stark/prover.rs and verifier.rs (rg on sample and security_parameters), src/storage/manifest.rs 331-345, src/settlement/proof_market.rs 197-210, impl/struct headers of the six `fn verify_id` files (rg), src/chain/chain_actor.rs (rg), profile tables in Cargo.toml files (rg), ops/scripts (ls).

Not read: the other gate files, the workflows beyond the lines above, src/ bodies. No cargo, no test run. Every claim comes from reading source. No `#[allow]`, `#[ignore]` or shell script was added or found in the 12 files (rg checked): Z3 and Z6 clean for this part. Only `expect` hits are in test code, in writeln to a String, or after a checked condition (security_parameters.rs:490).

Extra-check summary:
- Vacuous passes: required_tests (P54-F04), relay on a missing live report (P54-F05), security_scans MUST_BLOCK with a deleted job (P54-F08), regeneration tree checks when the file is missing (P54-F11).
- Baseline loosening: .github/semver-exceptions.txt (P54-F01, P54-F02, P54-F03). Baseline files `.github/*-baseline.txt` are not touched by any of these gates.
- Regex or parser gaps: P54-F09, F13, F14, F15, F16, F17, F18, F21.
- Swallowed errors: unreadable file turns into "skip" in rejection_tests.rs:233, required_tests.rs:31/87/111, security_scans_can_fail.rs:292, self_derived_ids:416-419, serialize_map_keys:376, regeneration.rs:688/699. Grouped under the finding of each gate.

## FINDINGS

### P54-F01
File: .github/semver-exceptions.txt:20 with xtask/gates/src/gates/semver.rs:129-140, 164-176
Severity: Medium
Axis: invariant(Z3)
Confidence: Certain
Evidence:
```
.github/semver-exceptions.txt:3  # This list must stay EMPTY; an empty list = the cargo-semver-checks gate runs
.github/semver-exceptions.txt:20 AccountState::distribute_block_fees | EIP-1559 fee distribution wiring (step G) - a new public method wired ... | STATUS_ONLINE 2026-07-21 approval
semver.rs:137  .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
semver.rs:165  let entries = has_justified_entries(exc)?;
semver.rs:166-175  if !entries.is_empty() { ... return Ok(msg); }
```
Description: The exceptions file holds one live (non-comment) line. `classify_report` only asks whether any live line exists. It never matches the line against the lints in the report. One entry about one new method turns every breakage in budlum-core into PASS-EXCEPTION.
Impact: The semver gate cannot go red on a real breaking change (removed pub fn, changed signature). The file header says it must stay empty, so the gate is softened against its own policy.
Known: none (the entry is not in any baseline file).

### P54-F02
File: xtask/gates/src/gates/semver.rs:233-238
Severity: Medium
Axis: invariant(Z3)
Confidence: Likely (missing verification step: that semver.yml gives the gate the PR head as `current` and branch protection does not require a reviewer; CODEOWNERS content read, workflow not read in full)
Evidence:
```
semver.rs:233  // The exceptions file belongs to the checkout under test; fall back to
semver.rs:235  let mut exc = current.join(".github/semver-exceptions.txt");
.github/CODEOWNERS:1  # CODEOWNERS disabled to prevent PR deadlock
.github/CODEOWNERS:5  # No owners assigned - file intentionally empty
```
Description: The exception list is read from the checkout that is being judged. A PR that breaks the API can add its own exception line in the same diff. CODEOWNERS is empty, so no owner review guards the file.
Impact: Self-approval of a public API break by the PR author. Combines with P54-F01: even an empty list on main is one added line away from a pass.
Known: none.

### P54-F03
File: xtask/gates/src/gates/semver.rs:212-219
Severity: Low
Axis: correctness
Confidence: Likely (missing step: how semver.yml prepares the baseline dir; a failed checkout that leaves an existing empty directory is the trigger)
Evidence:
```
semver.rs:212  if !baseline.join("Cargo.toml").is_file() {
semver.rs:216      return Ok(String::from("SEMVER GATE: PASS - the baseline is empty (first release), no comparison."));
```
Description: A baseline root that exists but has no Cargo.toml passes. This is fail-open for a broken baseline checkout, a wrong path or a sparse checkout. A non-existent path fails (line 203-205), an empty one passes.
Impact: A CI checkout problem silently disables the gate. Budlum has a base commit, so "first release" is not a real case on this repo.
Known: none.

### P54-F04
File: xtask/gates/src/gates/required_tests.rs:190-243 (registered at xtask/gates/src/main.rs:1034)
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
$ ls -la ops/scripts
-rw-r--r-- 1 root root 0 ... .gitkeep
required_tests.rs:237  if scripts_with_lists == 0 {
required_tests.rs:240      return Ok(String::from("Required-test gate OK: no shell gate declares required_tests=() (all migrated to Rust)."));
```
Description: The gate only reads `ops/scripts/check-*.sh`. That directory holds only `.gitkeep`, so the gate always returns OK without checking a single name. The Rust gates carry their own test lists (named_tests, wallet_core_gate `TESTS`), and this gate does not look at them. It also keeps shell parsing alive (Z6 direction is Rust and BudL only).
Impact: A gate that is green by construction and reports "all migrated" as proof. Required-test names of the Rust gates can lose their `#[test]` without this gate noticing.
Known: none.
Parser defects that would matter once it has input (same file): `required_tests=(a b)` on one line is skipped (line 125-128, the first line is consumed with `continue`); only the first word per line is read (134); names with an upper-case letter or digit-first pattern are dropped (135-139); `#[test] fn x()` on one line is not counted (42-45, `continue` skips the same line); `#[tokio::test(flavor = ...)]` does not match (42); `/target` substring skips any directory whose path contains it (27, 83); a read error gives an empty list (111, `unwrap_or_default`).

### P54-F05
File: xtask/gates/src/gates/relay.rs:428-467
Severity: Medium
Axis: invariant
Confidence: Certain
Evidence:
```
relay.rs:428  //    that fails signature or digest comparison makes the relay red; a
relay.rs:438  if !l.sig_valid || !live_matches || !l.status_ok { live_status_ok = false; }
relay.rs:460  Ok(format!("relay OK: relay-token {} relay-status {} ...", ..., status_word, ...))
.github/workflows/diverse-double-compiling.yml:127  RELAYSTATUS=$(grep -oE "relay-status [a-z]+" /tmp/relay.txt ...)
.github/workflows/diverse-double-compiling.yml:130  test -n "${RELAYSTATUS}"
$ rg 'relay-report' .github/workflows   (no output; only budzero/bud-proof/src/relayer.rs)
```
Description: A bad live report (invalid signature, digest mismatch, status alarm) only changes the word `alarm` in the output. `run` still returns Ok, so the gate exit code is 0, and the comment above says it makes the relay red. The only consumer checks that the status word is not empty, not that it is `ok`. No workflow produces `budzero/target/relay/relay-report.json`, so the live branch (lines 287-299, 326-405) never runs in CI.
Impact: The signature and digest validation of the proof-side report is dead in CI, and where it would run, an alarm cannot fail a job.
Known: none.

### P54-F06
File: xtask/gates/src/gates/relay.rs:215-220, 242, 326, 397-398
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
relay.rs:25   //! ... Keccak-256 over a canonical byte payload (fixed field order), so
relay.rs:27   //! an external party recomputes the payload from the JSON and checks `report_sig` without any key.
relay.rs:242  let sig = keccak256(&payload);
relay.rs:397  let sig_bytes = hex_decode(&sig_hex).unwrap_or_default();
```
Description: `report_sig` is an unkeyed hash. Anyone who can write `target/relay/relay-status.json` or the live report can recompute a valid "signature". The gate binary hash (line 215) is read from the running binary, so it also proves nothing against a party that controls the build. A malformed hex signature is turned into an empty vec and then "invalid" (fail-closed, but the decode error is lost).
Impact: The status file is a checksum, not an attestation. It must not be described or consumed as a signed report.
Known: none.

### P54-F07
File: xtask/gates/src/gates/security_scans_can_fail.rs:166-219
Severity: Medium
Axis: invariant(Z3)
Confidence: Certain
Evidence:
```
security_scans_can_fail.rs:201  if bare.starts_with("continue-on-error:") && bare.contains("true") {
security_scans_can_fail.rs:208  if t.contains("|| true") {
$ rg -n 'set \+e' .github/workflows   (security-hardening.yml:80,94,200,513,572,647,677,846; extra-tooling.yml:297; typos.yml:98,175; supply-chain-extra.yml:59,72,132,161)
docs/AUDIT_PLAN.md:121  also search workflows for continue-on-error, || true, ; true, || exit 0, set +e
```
Description: The gate knows two softener shapes. It misses `set +e`, `; true`, `|| exit 0`, `||true`, `|| :`, `|| echo`, and `continue-on-error: ${{ expr }}` or `True`/`yes` (the test is `contains("true")`, case sensitive and literal). The header says the allow-list reached zero "because every masked exit became an explicit exit-code path", but the gate cannot verify that the 14 `set +e` blocks re-raise their exit code.
Impact: A softened scan written in any other shape passes this gate. Mask style is a trivial rewrite for anyone adding one.
Known: Partly overlaps P58 (workflow review); the gate gap itself is not in FINDINGS.md.

### P54-F08
File: xtask/gates/src/gates/security_scans_can_fail.rs:237-264, 283-294
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
security_scans_can_fail.rs:240  for (file, job, why) in MUST_BLOCK { if found.iter().any(|s| s.file == *file && s.job == *job) { problems.push(...) } }
security_scans_can_fail.rs:266  for a in ALLOWED { if !found.iter().any(...) { problems.push(...dead entry...) } }
security_scans_can_fail.rs:292  let src = std::fs::read_to_string(f).unwrap_or_default();
```
Description: ALLOWED has a dead-entry check. MUST_BLOCK (tsan, zizmor, cargo-hack, mutants, supply-chain, loom) has none: the gate never checks that these jobs still exist. Deleting or renaming the `tsan` job produces no finding. The header text says "a check that inspected nothing must not pass". A workflow file that cannot be read is treated as empty (292). All six jobs exist today (security-hardening.yml:121,287,431,775; security-audit.yml:53,240).
Impact: The most security-relevant jobs can be removed without this gate or any other in the part noticing.
Known: none.

### P54-F09
File: xtask/gates/src/gates/rust_literals.rs:171-177 (with 30-119, 121-150)
Severity: Medium
Axis: correctness
Confidence: Certain (logic); impact Likely (which consumers hit it was not run)
Evidence:
```
rust_literals.rs:171  /// The full scrub ...: literals, then block comments, then line comments.
rust_literals.rs:173  pub fn scrub(text: &str) -> String {
rust_literals.rs:174      let after_literals = strip_rust_literals(text);
rust_literals.rs:175      let after_blocks = strip_block_comments(&after_literals);
rust_literals.rs:176      strip_line_comments(&after_blocks)
$ rg -n '//.*/\*' src   -> src/chain/genesis.rs:23:/// build. The shipped `config/*-genesis.json` files declare ...
```
Description: Line comments are removed last, so a `/*` inside a `//` comment opens a block comment that blanks all code up to the next `*/`. A `"` inside a comment opens a string that blanks code up to the next `"` and flips string and code state for the rest of the file. A `'\u{..}'` char literal is not recognised (line 42-47 skips two chars only). The helpers are shared: rg lists 11 gate files that call the literal or block stripper (no_orphan_source_files, tree_is_english, cross_table_checks, consensus_maps_ordered, untrusted_manifests, uncheckable_proof, zero_tests_witness, air_selectors, forgery_tests, binding_claims, plus security_parameters with its own copy).
Impact: The scanning gates can see less code than exists, and they report "no finding". Doc comments with paths like `config/*-genesis.json` are normal in this tree.
Known: none.

### P54-F10
File: xtask/gates/src/gates/regeneration.rs:77-80, 775-806
Severity: Medium
Axis: invariant
Confidence: Certain
Evidence:
```
regeneration.rs:77  const EMBEDDED_PRIVATE_TRANSFER_SRC: &str = include_str!("../../../../budzero/bud-vm/src/private_transfer.rs");
regeneration.rs:793 if on_disk != part.embedded { return Err(... MODIFIED ...) }
.github/workflows/diverse-double-compiling.yml:123  (cd xtask/gates && cargo run --release --quiet -- relay)
```
Description: The "canonical" copy is `include_str!` of the working tree file, taken when the gate is compiled from the same checkout that is then checked. In CI both sides are the PR head, so `on_disk == embedded` unless the file is deleted. The MODIFIED branch can only fire if the gate binary is built from a different commit than the tree. Nothing in the part or in the workflow line above builds the gate from a trusted base. The only effective behaviour is the deletion path, which rewrites the file (line 784-788) and passes (lines 962-970 add a note).
Impact: The "unauthorized code entry" defence for private_transfer.rs and syscall_context.rs is a self-comparison. The gate also writes into the tree it checks, which hides a deleted file in the CI workspace.
Known: none.

### P54-F11
File: xtask/gates/src/gates/regeneration.rs:812-828, 842-881, 1103-1144
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
regeneration.rs:843  if let Ok(text) = fs::read_to_string(&deal_path) {
regeneration.rs:844      if text.contains("Opcode::VerifyMerkle") {
regeneration.rs:848          let expects_imm_256 = text.contains("imm: 256");
regeneration.rs:849          let expects_regs = text.contains("rd: 1") && text.contains("rs1: 2");
regeneration.rs:817  if !text.contains(pin) {
regeneration.rs:1124 let words = if pin == PINNED_PRIVATE_TRANSFER_PROGRAM_HASH { regenerate_private_transfer_check_program() } ...
```
Description: Three checks are weaker than their text says. (1) The storage-challenge tree check is skipped silently if `storage_deal.rs` is missing or lacks `Opcode::VerifyMerkle`, and when it runs it tests substrings anywhere in a large file ("rd: 1" also matches "rd: 10", comments and tests count). (2) `verify_proof_side_canonical_set` uses `contains` on the raw source, so a pin in a comment satisfies it (relay.rs parses the table properly, the gate does not). (3) `check_pinned_programs` regenerates programs inside the gate and compares them with constants inside the gate; the error text says "the tree's builder no longer reproduces the pinned stream", but no tree builder is called. Drift in `src/ai/execution/guest.rs` or `build_matmul_guest_program` is caught only by that tree's own tests.
Impact: The reproduction story ("rebuild from the ISA and compare with the tree") is mostly self-checking. A tree-side drift of the matmul or storage program does not turn this gate red.
Known: none.

### P54-F12
File: xtask/gates/src/gates/regeneration.rs:664-747, 889-898
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
regeneration.rs:666  s.ends_with(".rs") && !s.contains("/target/") && !s.contains("regeneration.rs")
regeneration.rs:680  for base in ["src", "budzero", "wallet-core"] {
regeneration.rs:709  .position(|l| l.starts_with("#[cfg(test)]"))
regeneration.rs:719  if !(line.contains("Keccak256::new") || line.contains("Sha3_256::new") || line.contains("Keccak::v256"))
regeneration.rs:889  let canonical: Vec<&Producer> = producers.iter().filter(|p| !p.tagged).collect();
```
Description: Discovery holes: root `wallet-core` does not exist (the crate is crates/wallet-core); any file with "regeneration.rs" in its path is exempt; scanning stops at the first column-zero `#[cfg(test)]`, so production code after a test-only item is not seen; only three constructors and a fixed list of loop variable names are matched; read errors skip the file (688, 699). A new producer without a `BDLM_` tag counts as canonical (889) without checking its byte layout, so a producer with a different feed (big-endian, length prefix) passes. The self-test "hidden production point" canary (1288-1306) uses a tagged file, so it does not cover that case.
Impact: The claim "discovers EVERY point producing a program hash" is narrower than documented.
Known: none.

### P54-F13
File: xtask/gates/src/gates/security_parameters.rs:241-265, 744-763, 837-847
Severity: Medium
Axis: test
Confidence: Likely (missing step: a concrete decoy was not run; the logic is read from source)
Evidence:
```
security_parameters.rs:847  if !has_trait_decl(&cfg_src) {          // raw source, comments and strings included
security_parameters.rs:744  let Some(fri_body) = fri_params_body(build_src) else {   // raw source
security_parameters.rs:756  let Some((sec_body_raw, sec_end)) = security_vec(build_src) else {   // raw source
security_parameters.rs:753  let fri_body = strip_comments(&fri_body);   // only the extracted body is stripped
```
Description: `has_trait_decl`, `fri_params_body` and `security_vec` search the unstripped file. The first `let security = vec![` or `fn security_parameters(&self)` found wins, even inside a doc comment, a string or a dead block, while the real code can differ. The canary RAWBUILD (line 978, expect fail) fails for a different reason: its decoy contains a literal vector `vec![1, 2, 3]`, so the "literal entry" check fires. A decoy with a correct-looking vector would be accepted. `has_trait_decl` also accepts a comment containing the signature, and it checks only the declaration, not the impl (config.rs:143) that returns the stored vector.
Impact: Only the absorption check (check_absorption) strips comments and literals; the other two halves of the gate can be satisfied by inert text. Today's tree is correct (plonky3_prover.rs:94-101 builds all six fields from `fri_params`).
Known: none.

### P54-F14
File: xtask/gates/src/gates/security_parameters.rs:461-530, 574-609, 704-739, 864-866
Severity: Low
Axis: test
Confidence: Likely (missing step: no run against a modified prover.rs)
Evidence:
```
security_parameters.rs:468  .filter(|(_, l)| l.contains("sample_algebra_element()"))
security_parameters.rs:478  if !before.contains("security_parameters()") {   // anywhere before the first sample, any function
security_parameters.rs:603  if skip_py_ws(&r[digits_end..]).starts_with(',') {   // only `name: 123,` literals count as FRI fields
security_parameters.rs:864  if checked == 0 {   // unreachable: checked is incremented before (846)
```
Description: The "first challenge" is the first line with `sample_algebra_element()` in the file. `challenger.sample()`, `sample_array`, `sample_bits` and a turbofish call are not challenges for this gate. `security_parameters()` is accepted anywhere before that line, across function boundaries and inside `if` blocks. `observe(binding)` is not tied to the challenger object. `fri_fields` lists only fields written as integer literals, so a field moved to a `const` or an expression leaves the "every numeric field is absorbed" check (790-800) with fewer fields and no failure, and no minimum field count exists. `passes_security` accepts any `new_with_security(.., security, ..)` in the file. Dead check at 864, and the "never observes" message is pushed twice (516-528, Info).
Impact: Defence in depth for Fiat-Shamir parameter binding is looser than documented. Current prover.rs:180 and verifier.rs:429 are correct.
Known: none.

### P54-F15
File: xtask/gates/src/gates/self_derived_ids_cover_every_field.rs:238-276, 297-316, 346-374
Severity: Medium
Axis: test
Confidence: Certain (logic)
Evidence:
```
self_derived_ids...rs:247  if let Some(rest) = stripped.strip_prefix("pub") {
self_derived_ids...rs:248      if !rest.starts_with(|c: char| c.is_ascii_whitespace()) { doc.clear(); continue; }   // pub(crate) skipped
self_derived_ids...rs:299  let needle = format!("self.{field}");   // searched in raw derivation text, comments included
self_derived_ids...rs:416  let raw = std::fs::read(path).map_err(...).unwrap_or_default();
```
Description: Only `pub` fields are examined. `pub(crate)` and private fields of a struct with `verify_id` are never checked, though the header says "every field". A field counts as bound if `self.field` appears anywhere in the derivation text, including a `//` comment or a string. Brace matching (`balanced`, lines 59-77) does not skip strings or comments, so a `"{"` or `'}'` in a body changes the extracted derivation. `id_field_of` takes the first `self.x ==` or `!=` in the text (e.g. `self.version != 1`) and then exempts that field from the check (522). Unreadable files are skipped silently (416).
Impact: A field of ProofTask, ContentManifest, CrossDomainMessage and the other four structs with `verify_id` can be left out of the id without a finding if it is non-pub or merely mentioned in a comment.
Known: none.

### P54-F16
File: xtask/gates/src/gates/self_derived_ids_cover_every_field.rs:45, 421-473
Severity: Low
Axis: test
Confidence: Certain (logic)
Evidence:
```
self_derived_ids...rs:45   const SCAN_ROOTS: &[&str] = &["src", "budzero", "wallet-core"];
self_derived_ids...rs:427  while let Some(rel) = src[cursor..].find("pub struct ") {
self_derived_ids...rs:445  while let Some(rel) = src[impl_cursor..].find("impl ") {
self_derived_ids...rs:463  let Some(fields) = structs.get(&name) else { impl_cursor = pos + 1; continue; };
```
Description: Structs are matched only as `pub struct Name {` (not generic, not `pub(crate)`, not tuple) and impls only as `impl Name {` (not `impl<T> Name<T>`, not `impl Trait for Name`). The struct and its impl must be in the same file. The root `wallet-core` does not exist (crates/wallet-core is the path), so `crates/` is never scanned. The check `out.structs == 0` is global, so removing all but one struct stays green. Six `verify_id` fns exist today (rg: sovereign.rs:186, policy.rs:231, message.rs:107, manifest.rs:696, proof_market.rs:318, types.rs:387); the gate does not report how many it measured against an expected count.
Impact: A new id-bearing type in a different shape is invisible to the gate.
Known: none.

### P54-F17
File: xtask/gates/src/gates/rejection_tests.rs:243-248
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
rejection_tests.rs:245  if !window.iter().any(|w| w.contains("#[test]")) { continue; }
$ rg -A2 '#\[tokio::test' src | rg 'fn \w*(_reject|_refus|_denied|_forbidden)'
src/rpc/server.rs:6972   async fn operator_rejects_foreign_host()
src/rpc/server.rs:7011   async fn operator_rejects_origin_header()
src/rpc/tests.rs:148     async fn bud_identity_resolve_refuses_malformed_and_answers_null_for_absent()
src/rpc/tests.rs:1190    async fn rpc_tests_admin_ban_refused_on_public_listener()
src/cross_domain/evm/adapter.rs:447/483/506   async fn the_offline_adapter_refuses_...
```
Description: A test counts only if `#[test]` is within the six lines above the fn. `#[tokio::test]` tests are skipped, so their bodies are never checked. At least nine rejection-named async tests (RPC and bridge paths) fall outside the gate. Also, `PROMISE_PARTS` need a leading underscore, so names starting with `rejects_` and names such as `_fails`, `_errors`, `_invalid` carry no promise.
Impact: The tests that guard the operator listener and the public RPC boundary are the ones the gate does not read.
Known: none.

### P54-F18
File: xtask/gates/src/gates/rejection_tests.rs:23, 63-148
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
rejection_tests.rs:23   const SCAN_ROOTS: &[&str] = &["src", "budzero", "wallet-core"];
rejection_tests.rs:110  if b.contains(".is_empty()") || b.contains("is_banned(") || b.contains("is_jailed(") {
rejection_tests.rs:130  if b.contains("assert_eq!(") && ["before", "initial", "unchanged", "original", "prior"].iter().any(|k| b.contains(k))
rejection_tests.rs:142  || b.contains("_cap")
```
Description: The evidence test is a substring search over the whole body, comments included. Any `Err(`, `Invalid`, `Failed`, `.is_empty()`, `_cap`, `saturat`, or any `assert_eq!` plus the word "before" in the body counts as a refusal. A positive test that merely builds an `Err(..)` fixture, or mentions "initial", passes. Root `wallet-core` is absent, and `crates/` and `bud/` are not scanned, although crates/bpqs has about 15 tests with `_reject`/`_refus` names (rg count: sign.rs 5, tests_backends.rs 6, merkle.rs 3, epoch.rs 1).
Impact: The gate catches only the case with no negative word at all. This is stated in its header, but the evidence list is wide enough that a wrong test rarely fails it.
Known: none.

### P54-F19
File: xtask/gates/src/gates/release_profile_pins.rs:100-107
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
release_profile_pins.rs:101  let manifest = fs::read_to_string(root.join("Cargo.toml"))
release_profile_pins.rs:54   if current == "profile.release" && ...
$ rg -n '^\[profile' budzero/Cargo.toml crates/*/Cargo.toml bud/Cargo.toml
budzero/Cargo.toml:33, crates/wallet-core/Cargo.toml:67, crates/budscan/Cargo.toml:52, crates/ai-inference/Cargo.toml:35, bud/Cargo.toml:100
```
Description: Only the root manifest table `[profile.release]` is read. The five other workspaces that define their own release profile (budzero holds bud-node and bud-cli) are not pinned. `[profile.release.package.*]` and `[profile.release.build-override]` overrides, which can switch `overflow-checks` off for one crate, are not looked at (the scanner matches the table name exactly). A trailing comment (`strip = true # x`) or `lto = "fat"` fails with a false "hardened value" message. `staged_dir` (line 110-114) uses the predictable `release-profile-pins-<pid>` name with `create_dir_all`, the CWE-377 pattern that `exclusive_scratch_dir` was written to remove.
Impact: Release hardening of the budzero binaries is unguarded; the node binary at the root is guarded.
Known: none.

### P54-F20
File: xtask/gates/src/gates/repair_fires.rs:11-110
Severity: Low
Axis: integration
Confidence: Likely (missing step: whether chain_actor.rs lines 3053 and 3197 sit in a reachable production branch)
Evidence:
```
repair_fires.rs:17   Ok(text.lines().map(|l| l.split("//").next().unwrap_or("").to_string())...
repair_fires.rs:43   actor.contains("objects_below_own_repair_margin"),
repair_fires.rs:75   margin_block.contains("parity_count"),
$ rg -n 'objects_below_own_repair_margin|unrecoverable_objects' src/chain/chain_actor.rs
3053:  .objects_below_own_repair_margin();   3197: .unrecoverable_objects();   4636: unrecoverable: registry.unrecoverable_objects(),
```
Description: Every claim is a substring in a file with `//` comments cut (block comments and strings are not cut, and `//` inside a string such as a URL cuts real code). "Wired" is shown by an identifier appearing anywhere, not by a call from a production branch, and `blockchain.rs` is checked for `open_expiry_reallocation` the same way. The margin check looks at 12 lines after the first `fn repair_margin`. The self-test (115-159) breaks one claim of nine.
Impact: This is the CLAUDE.md section 9 item 1 wiring (repair trigger). The gate gives weak evidence that it is a live call site; the real answer needs the rg call-site check.
Known: none.

### P54-F21
File: xtask/gates/src/gates/serialize_map_keys_are_strings.rs:136-155, 184-228, 290-326, 376
Severity: Low
Axis: test
Confidence: Certain (logic)
Evidence:
```
serialize_map_keys...rs:291  if !derives_serialize(&s.attrs) { continue; }
serialize_map_keys...rs:302  let Fields::Named(named) = &s.fields else { continue; };
serialize_map_keys...rs:165  if meta.path.is_ident("with") || ... "serialize_with" ... "skip" ...
serialize_map_keys...rs:376  let Ok(text) = fs::read_to_string(&path) else { continue; };
```
Description: Not scanned: `enum` variants with maps, tuple structs (`struct T(BTreeMap<[u8;32],_>)`), fields typed through an alias to a map (`type Table = BTreeMap<..>` is followed only for keys, not for the field type), structs inside function bodies or macros, `#[cfg_attr(.., derive(Serialize))]`. Any `serde(with = ..)` is trusted without checking the named module handles map keys. `Address`, `AssetId`, `DomainId` are accepted by name only, so a different type with the same name elsewhere passes. An unreadable file is skipped, not counted as unparsed. rg today finds no tuple-struct map outside tests, so no live miss was shown.
Impact: A future registry type shaped like one of the above reintroduces the snapshot-write failure the gate is named for.
Known: none.

### P54-F22
File: xtask/gates/src/gates/semver.rs:201-204, 171
Severity: Info
Axis: invariant(Z13)
Confidence: Certain
Evidence:
```
semver.rs:201  return Err(format!("current root yok: {current_s}"));
semver.rs:204  return Err(format!("baseline root yok: {baseline_s}"));
semver.rs:171  msg.push_str("  ISTISNA: ");
semver.rs:656  fs::write(&exc, "BDLM-1: onayli\n")
```
Description: Turkish words in gate messages and a test fixture. Z13 asks for English repo text outside README.tr.md and the directive files.
Impact: Language only.
Known: none (a tree-is-english gate and turkish-baseline.txt exist; not checked whether they cover xtask).

### P54-F23
File: xtask/gates/src/gates/semver.rs:320-338; security_parameters.rs:1000-1004, 516-528, 864-866; serialize_map_keys_are_strings.rs:680
Severity: Info
Axis: dead code
Confidence: Certain
Evidence:
```
semver.rs:328   text.lines().map(str::trim).find_map(|line| line.strip_prefix("name") ...)   // first `name =` line in Cargo.toml
security_parameters.rs:995-1004  three stacked "# Errors" doc blocks on the CASES const / Case type
security_parameters.rs:864       if checked == 0 { ... }  // never true
serialize_map_keys...rs:680      "11 canaries behaved."   // the self-test runs 9 numbered cases plus the loops
```
Description: `package_name` takes the first `name =` line, which is a wrong package if `[lib]` or `[[bin]]` precedes `[package]` (a failure then looks like missing rustdoc, fail-closed). The other items are stale doc and a dead check.
Impact: Maintenance only.
Known: none.

## Summary table

Critical 0, High 0, Medium 12 (F01, F02, F04, F05, F07, F08, F09, F10, F11, F13, F15, F17), Low 9 (F03, F06, F12, F14, F16, F18, F19, F20, F21), Info 2 (F22, F23). Total 23.
