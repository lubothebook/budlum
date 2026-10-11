# P56 scan: xtask gates last (read-only audit, Tier R1)

Base commit: aca9897. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md, notes/P50-scan.md to notes/P53-scan.md.

## COVERAGE

Read in full (every line, tests and canaries included):
- xtask/gates/src/main.rs 1-1399
- xtask/gates/src/gates/untrusted_manifests.rs 1-635
- xtask/gates/src/gates/value_transfers.rs 1-604
- xtask/gates/src/gates/wallet_core_gate.rs 1-58
- xtask/gates/src/gates/wallet_node_address_agree.rs 1-356
- xtask/gates/src/gates/wire_fields_are_signed.rs 1-618
- xtask/gates/src/gates/workflows_produce_jobs.rs 1-655
- xtask/gates/src/gates/zero_address_sender_is_verified.rs 1-362
- xtask/gates/src/gates/zero_storage_frozen.rs 1-205
- xtask/gates/src/gates/zero_tests_witness.rs 1-291
- xtask/gates/src/gates/zizmor.rs 1-221
- xtask/gates/src/gates/zkvm_immune_regeneration.rs 1-248

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md 110-165; notes/P29-scan.md 1-30.

Outside the part (rg first, at most 80 lines): xtask/gates/src/gates/named_tests.rs 1-45; xtask/gates/src/gates/rust_literals.rs 165-215; .github/workflows/ci.yml 1015-1045 and 1700-1730; .github/workflows/diverse-double-compiling.yml 92-150; src/core/transaction.rs enum TransactionType (awk extract, 149 lines) and rg on encode_transaction_type_payload; src/chain/chain_actor.rs 4229-4245; src/storage/manifest.rs 725-760; src/domain/deal_open.rs 59-125; src/domain/storage_tx.rs 375-385; src/execution/zkvm.rs 530-546; crates/wallet-core/src/lib.rs 1070-1090; src/crypto/primitives.rs 896-925; rg on src/registry/params.rs, src/core/account.rs, src/storage/generated.rs, budzero/bud-proof/src/plonky3_air.rs, budzero/bud-vm/src/lib.rs.

Not read: the other 119 gate files, xtask/tools, the production files above beyond the ranges. No cargo, no gate run: every claim comes from reading source and rg output.

Extra-check summary:
- Registration (main.rs): 130 files in gates/ match 130 `pub mod` lines exactly (no orphan file, no missing file). 127 GATES entries; the 3 modules without an entry are helpers (exact_named_tests, named_tests, rust_literals). No duplicate gate name. kani and semver have `run` placeholders and `run_args` only. Every one of the 127 gate names is either run by `--all` (ci.yml:1721) with its canary by `--all --self-test` (ci.yml:1719), or has its own CI step with a path argument (rg loop over .github/workflows). tree-pin is excluded from `--all` (run_args set) and runs at ci.yml:2140-2143 and diverse-double-compiling.yml:137. No wiring finding.
- Vacuous pass: no gate in the part passes on an empty input by itself (all have a "checked == 0" or a missing-file error). The weak points are text-presence checks, see F02 to F13.
- Swallowed errors: `unwrap_or_default` on file reads at untrusted_manifests.rs:408-411 and value_transfers.rs:363-366 fail closed (empty text makes the strong check "gone"). The same call fails open at zkvm_immune_regeneration.rs:241 and wire_fields_are_signed.rs:337 (F03, F06).
- Baseline handling: no gate in the part reads or writes `.github/*-baseline.txt`.
- Z3: `rg '#\[allow|#\[ignore|continue-on-error|\|\| true'` on the 12 files: no hit. Z6: no shell file added; zizmor.rs shells out to curl, sha256sum, tar (not repo scripts). Z7/Z8: no unsafe, no float.

### P56-F01
File: xtask/gates/src/gates/named_tests.rs:19-24 (outside the part; used by wallet_core_gate.rs:52-54)
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
named_tests.rs:19: fn line_matches(line: &str, name: &str) -> bool {
named_tests.rs:20:     match line.find(name) {
named_tests.rs:21:         Some(pos) => line[pos + name.len()..].contains("ok"),
```
Description: A required test counts as passed when its name is followed anywhere on the same line by the letters "ok". libtest prints `test NAME ... ignored, <reason>` for an ignored test, and a reason such as "token" or "blocked" contains "ok". The match is also a prefix match, so `NAME_extra ... ok` satisfies `NAME`.
Impact: An `#[ignore = "..."]` on a required wallet-core test (and on every other gate built on named_tests) can pass the gate. This is a Z3 bypass that the gate is meant to stop. The CI step has `set -euo pipefail` so a FAILED test still stops earlier; ignored tests do not.
Trigger: precondition: a required test gets `#[ignore = "token expiry"]`; entry point wallet_core_gate.rs:53; input line `test multisig_policy_validates_threshold ... ignored, token expiry`; expected wrong result: gate prints "wallet-core gate OK".
Known: none (other named-test gates are in other parts; likely repeated in P5x scans)

### P56-F02
File: xtask/gates/src/gates/zero_tests_witness.rs:132, 97-101, 189-291
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
zero_tests_witness.rs:132: if !air_src.contains(witness) {
zero_tests_witness.rs:98:  statements(air_code).iter().any(|s| {
zero_tests_witness.rs:99:      s.contains(&format!("when(is_{snake}")) && s.contains("assert_one(") && s.contains(air_reg)
```
rg -c on budzero/bud-proof/src/plonky3_air.rs: COL_ASSERT_INV 3, COL_DIV_INV 2, COL_INV_ZERO 5, COL_JNZ_COND_INV 2 (one of each is the `pub const` definition, e.g. line 538).
Description: The positive check runs on the raw AIR text (`air_src`, comments not stripped) and only asks that the constant name appears. The constant's own definition satisfies it, so no constraint has to use the witness. The only real check is negative and matches one spelling: `when(is_<snake>` plus `assert_one(` plus the register name in one statement. `assert_zero`, `assert_eq`, `when_transition`, or a renamed selector (not `is_<snake>`) are not seen. Only Assert has a direct-form canary; Div, Inv, Jnz, Not have none.
Impact: A soundness regression in the zero tests of five opcodes (field non-zero stated without an inverse witness) can pass the gate. The AIR is R3 (budzero/bud-proof).
Trigger: precondition: a change to plonky3_air.rs that constrains `rs2_val` for Div through `assert_zero(is_div * (rs2_val - one))`; entry point zero_tests_witness.rs:106; expected wrong result: "zero-test gate OK".
Known: none

### P56-F03
File: xtask/gates/src/gates/zkvm_immune_regeneration.rs:227-248
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
zkvm_immune_regeneration.rs:229: for dir in ["src/execution", "src/rpc"] {
zkvm_immune_regeneration.rs:230:     let Ok(rd) = std::fs::read_dir(root.join(dir)) else {
zkvm_immune_regeneration.rs:241:     let body = std::fs::read_to_string(&path).unwrap_or_default();
```
rg -n execute_bytecode_ungated src budzero crates: src/execution/zkvm.rs:101 (definition) and src/execution/zkvm.rs:542 (inside the cfg(test) module that starts at line 356).
Description: The header says "the ungated path may only be named by the file that defines it" and the run message says "no production path reaches the un-gated executor". The scan reads only the top level of two directories (no recursion, no src/chain, src/network, src/domain, src/ai, src/cli, src/bin, budzero). A read error becomes an empty body, which counts as "no caller". The only canary puts the caller in src/rpc/server.rs.
Impact: A call from any other directory (for example chain_actor.rs) bypasses gas and mainnet gating without a red gate. Today the rg above shows no such caller, so this is a coverage gap, not a live hole.
Trigger: precondition: a new call to `ZkVmExecutor::execute_bytecode_ungated` in src/chain/ or src/domain/; entry point zkvm_immune_regeneration.rs:72; expected wrong result: "ZKVM regeneration containment OK".
Known: none

### P56-F04
File: xtask/gates/src/gates/zkvm_immune_regeneration.rs:36-48, 83-109
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
zkvm_immune_regeneration.rs:42: if src.get(start..end).is_some_and(|w| w.contains(code)) {
zkvm_immune_regeneration.rs:83: if exec.contains("proof_bytes.len() > crate::execution::proof_verifier::MAX_PROOF_BYTES") {
zkvm_immune_regeneration.rs:97: if line.contains("1 << 20") {
```
Description: All three checks run on raw text with comments and tests included. A commented-out `// BudlumError::validation("ai_exec_stark", ...)`, a string in a test, or a dead function satisfies the rejection check; nothing ties it to the proof path. The doc says the bound "has to be compared ... before the envelope is decoded", but only presence of the text is tested; the canary puts the bound last. `contains("1 << 20")` also accepts `11 << 20` or `(1 << 20) * 64`.
Impact: The five fail-closed proof checks and the 1 MiB ceiling can be removed or raised while the gate stays green. AI execution proofs are a wire input.
Trigger: precondition: executor.rs keeps the strings in comments only; entry point zkvm_immune_regeneration.rs:51; expected wrong result: gate reports all five rejections present.
Known: none

### P56-F05
File: xtask/gates/src/gates/wire_fields_are_signed.rs:258-277, 404-440
Severity: Medium
Axis: security
Confidence: Likely (missing step: run the gate with a debug print of the resolved struct set; the counts below come from rg)
Evidence: TransactionType in src/core/transaction.rs has inline named variant fields, extracted with awk (enum body, 149 lines): `nft_id: u64`, `delta_mcd: i64`, `tag: String`, `price: u64`, `name: String`, `website_url: String`, `manifest_id: Option<ContentId>`, `domain_height: u64`, `state_updates: Vec<(Address, u64)>`, `request_id`, `verifier`. The gate only inspects `pub struct X {` bodies named in the enum. rg `struct <Name>\b` for the 123 capitalised words in the enum: resolved to a braced struct: AccessGrant, AiAgentPayment, AiExecutionProof, AiInferenceRequest, AiInferenceResult, AiModelSpec, ConsensusKeyRegistration, DataAsset, ExternalTransaction, PrivateTransferSubmit, RelayerExternalResult, SaleAuthorization, Storage. Not resolved: StorageTx (enum), newtype ids, Escrow (private struct elsewhere).
Description: The header says "for every TransactionType variant". Inline variant fields, enum payloads (StorageTx), tuple structs and nested struct fields (for example the `Option<CrossDomainMessage>` inside ExternalTransaction) are never measured. A carried name that has no struct is skipped silently (`continue` at line 406). The variant name `Storage` also resolves to the unrelated `pub struct Storage` in src/storage/db.rs:117, so that struct's fields are measured against the preimage by accident. Coverage is by `.field` name anywhere in the encoder text, comments included.
Impact: An unsigned wire field in an inline variant or a storage payload passes the gate; a relaying node can rewrite it and the signature still verifies. In my spot check every inline field name does appear in `encode_transaction_type_payload` (src/core/transaction.rs:1840-2500, grep by name), so no live hole was found.
Trigger: precondition: a new inline field added to a TransactionType variant and not appended in the encoder; entry point wire_fields_are_signed.rs:453; expected wrong result: "wire-field signing gate OK".
Known: partial, the file header "Known limits" (lines 26-30) admits coverage by name only.

### P56-F06
File: xtask/gates/src/gates/wire_fields_are_signed.rs:39, 148-168, 186-205, 337-358
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
wire_fields_are_signed.rs:39:  const SCAN_ROOTS: &[&str] = &["src", "wallet-core"];
wire_fields_are_signed.rs:337: let raw = std::fs::read(path).unwrap_or_default();
wire_fields_are_signed.rs:350: structs.entry(name.clone()).or_insert_with(|| {
```
ls: there is no `wallet-core` directory at the repo root (it is `crates/wallet-core`).
Description: (1) The second scan root does not exist, so `collect_rs` returns silently. (2) The first struct found by name wins, in `read_dir` order, which is not sorted, so two structs with one name give an order-dependent result. (3) Generic structs (`pub struct X<T> {`) and `pub(crate)` fields are skipped without a message; private fields that serde still sends are not looked at. (4) An unreadable file becomes an empty file (fail open). (5) The self-test classification `msg.contains("TransactionType") && msg.contains("no")` (line 539) matches the word "not" in a real finding.
Impact: Maintenance risk; the gate can change verdict by directory order or by a new same-name struct.
Known: none

### P56-F07
File: xtask/gates/src/gates/untrusted_manifests.rs:292-323; value_transfers.rs:134-166; zero_storage_frozen.rs:50-67; zero_address_sender_is_verified.rs:270-273
Severity: Medium
Axis: test
Confidence: Certain
Evidence:
```
untrusted_manifests.rs:298: while rest.starts_with("#[") {   // any attribute is skipped, #[ignore] included
untrusted_manifests.rs:382: if !has_test_fn(deal_src, test) {   // deal_src is the raw file, comments kept
zero_storage_frozen.rs:67:  let Some(start) = code.find(FROZEN_TEST_FN) else {
```
Description: The "required regression test exists" checks accept `#[test] #[ignore] fn NAME(`, and search raw text, so a test inside a block comment or a string also counts. zero_storage_frozen.rs does not even test for `#[test]`. zero_address_sender_is_verified.rs only needs `#[test]` on the line above a line that contains the name; the self-test body is empty `{}`, so an empty test passes.
Impact: A required regression test can be ignored, commented out, or emptied and the gate stays green. This is the Z3 pattern (`#[ignore]`) at the gate layer, on the storage-manifest door, the value-pricing path, the frozen generated bytes, and the zero-address bypass.
Trigger: precondition: `#[ignore]` added under `#[test]` on `a_deal_open_refuses_a_manifest_claiming_parity_it_does_not_have` (src/domain/storage_deal.rs:4862); entry point untrusted_manifests.rs:394; expected wrong result: "untrusted manifest gate OK".
Known: none

### P56-F08
File: xtask/gates/src/gates/untrusted_manifests.rs:164-179, 265-289, 480-488, 558-635
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
untrusted_manifests.rs:283: let region: String = actor_code[abs..].chars().take(2000).collect();
untrusted_manifests.rs:480: const CHAIN_ACTOR: &str = "pub async fn register_storage_manifest(&self) {\n\
untrusted_manifests.rs:486: ChainCommand::RegisterStorageManifest { manifest, response } => {\n\
untrusted_manifests.rs:487:     if let Err(e) = manifest.validate_untrusted() { return; }\n\
```
Description: (1) The handler "region" is the next 2000 characters, not the brace-matched arm, so a `manifest.validate_untrusted()` in the next match arm satisfies it. In the real file the call is about 11 lines below the arm start (src/chain/chain_actor.rs:4229 and 4240). (2) Every canary uses the same strong CHAIN_ACTOR text, so the handler door has no failing canary (weak handler, missing handler). (3) `strip_comments` cuts `//` before it blanks strings, so a string such as "http://x" blanks the rest of the line and leaves an unterminated quote; `strip_quoted` then pairs quotes across lines and can hide real code or expose inert text. (4) `has_standalone_n` and `contains("erasure.k")` accept `erasure.kind` or any variable named `n`.
Impact: The chain-actor door can lose its validation without a red gate.
Trigger: precondition: validation removed from the RegisterStorageManifest arm while another arm within 2000 characters calls `validate_untrusted()`; expected wrong result: gate OK.
Known: none

### P56-F09
File: xtask/gates/src/gates/value_transfers.rs:46-60, 192-209, 304-330
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
value_transfers.rs:197: if !validate.contains(rate) {
value_transfers.rs:219: if !body.contains("amount") {
value_transfers.rs:307: if !gov_code.contains(&format!("\"{rate}\"")) {
```
Description: Only `//` comments are stripped (no block comments, no strings). The checks are substring presence: `validate` only has to mention the rate, `proportional_fee` only has to mention `amount`, and the whitelist check accepts the quoted key anywhere in governance.rs (a test or a message is enough). `body_of` takes the first `pub fn validate` in params.rs, not the one in `RegistryParams`.
Impact: Fee-pricing regressions that keep the words in place pass. The bait shape is the one the module header describes.
Known: none

### P56-F10
File: xtask/gates/src/gates/zero_address_sender_is_verified.rs:139-150, 108-124, 192, 211-263
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
zero_address_sender_is_verified.rs:143: if compact.contains("![Ok(())") || compact.contains("[Ok(()),") {
zero_address_sender_is_verified.rs:144:     return false;
zero_address_sender_is_verified.rs:259: if has_ok_success && !guarded_success {
```
Description: `ok_success_in` returns false for the whole block when the text contains `![Ok(())` anywhere. A zero-address block with `return Ok(());` outside the verify branch plus one decoy `let _d = vec![Ok(())];` gives `has_ok_success == false`, so no problem is pushed. This is the CWE-306 bypass the gate was written for. Also: `strip_rust_noise` has no char literal handling, so `'"'` opens a string and blanks code until the next quote; only the exact text `if tx.from == Address::zero() {` is judged, so an earlier `tx.from.is_zero()` early return, or a flag set without `Ok(())`, is not seen; and `src.contains("zero-address sender is only valid ...")` (line 164) is satisfied by a comment or a test string.
Impact: The zero-address validation skip (HIGH finding already fixed) can return without a red gate.
Trigger: precondition: a PR re-adds an unguarded `return Ok(());` in the zero-address branch and adds the decoy vec; entry point zero_address_sender_is_verified.rs:283; expected wrong result: "zero-address gate OK".
Known: none

### P56-F11
File: xtask/gates/src/gates/zero_storage_frozen.rs:23-41, 74-160
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
zero_storage_frozen.rs:33: if let Some(name) = t.strip_suffix(',') {
zero_storage_frozen.rs:75: let table_end = rest.find("\n    }").unwrap_or(rest.len());
zero_storage_frozen.rs:86: if lit.chars().all(|c| c.is_ascii_hexdigit()) && lit.len() >= 8 {
```
Description: The header promises "two 64-hex digests per variant". The code counts `GeneratorId::X,` occurrences and checks that every hex-looking string is 64 long. A vector whose digest is not hex ("todo", "") is ignored, not counted; digests need not be distinct or one per vector. Comments are not stripped, so a commented table counts. A last enum variant without a trailing comma is dropped from the variant list. The table ends at the first "\n    }", which depends on indentation. The second number after the id is taken as the length; a changed tuple layout silently measures the seed.
Impact: A frozen-vector table can be hollowed (placeholder digests, duplicate digests) and still pass.
Known: none

### P56-F12
File: xtask/gates/src/gates/wallet_node_address_agree.rs:26-37, 96-137
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
wallet_node_address_agree.rs:34: //!   * a 32-byte output on both sides.
wallet_node_address_agree.rs:97: let hasher = body
wallet_node_address_agree.rs:98:     .find("::new()")
```
Description: The module doc lists a 32-byte output check; no code compares the output length or type. The hasher name is taken from the first `::new()` in the body, which can be `Vec::new()` or any constructor, so a hash change on one side can be hidden. The key absorb is the first `update(` that is not a byte-string literal. Comments are not stripped. There is no known-answer vector run on both sides.
Impact: Funds sent to a wallet address the chain never credits (the failure the header describes) are only caught for edits that touch the visible separator, hasher name, and order. Both real functions currently match: crates/wallet-core/src/lib.rs:1075-1081 and src/crypto/primitives.rs:903-911 use Sha3_256 with `update(b"BUDLUM_ADDRESS_V2")` first.
Known: none

### P56-F13
File: xtask/gates/src/gates/workflows_produce_jobs.rs:113-133, 142-167, 198-206, 329-336
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
workflows_produce_jobs.rs:329: if !u.job_level && !is_sha_pinned(&u.value) {
workflows_produce_jobs.rs:199: if value.starts_with('.') || value.starts_with("docker://") {
workflows_produce_jobs.rs:470: let legal = "...  uses: slsa-framework/slsa-github-generator/.github/workflows/generator.yml@v1.9.0
```
Description: (1) Job-level `uses:` (reusable workflow) is never checked for a SHA pin, and the self-test canary at line 470 requires a tag reference there to produce zero findings. (2) `docker://` references count as pinned, tag or not. (3) The reader is line based, not YAML aware: a job id line with a trailing comment (`build: # note`) is not recognised as a job, so an empty job after it is neither counted nor flagged; any line inside a `run: |` block that starts with `uses:` or `steps:` is read as structure. (4) A job with `if: false` or `steps: []` counts as having a body. (5) Composite actions are not scanned (no .github/actions directory exists now).
Impact: A tag-pinned reusable workflow or a docker tag can enter CI without this gate objecting. rg '^\s{4}uses:' on .github/workflows found no job-level `uses:` today.
Known: none

### P56-F14
File: xtask/gates/src/gates/zizmor.rs:10-11, 40-45, 143-157, 163-221
Severity: Low
Axis: security
Confidence: Likely (missing verification: run `zizmor 1.27.0 --help` and read the token and offline behavior; ci.yml has no GH_TOKEN, rg found none)
Evidence:
```
zizmor.rs:145: let out = std::process::Command::new(&bin.path)
zizmor.rs:146:     .arg(".")
zizmor.rs:153:     String::from_utf8_lossy(&o.stdout)
```
Description: The gate runs `zizmor .` with no token and no explicit `--offline`, so online audits (impostor commit, ref confusion, known vulnerable actions) are probably not run in this gate; security-audit.yml:165 runs zizmor-action separately. Error output uses only stdout, so tool errors (exit 3) lose stderr. `ZIZMOR_BIN` skips the version and sha256 pin. The gate pins 1.27.0 while security-audit.yml:110 installs 1.29.0. The canary downloads the release from github.com on every `--all --self-test`, so the canary step needs network. The hardening comments (planted /tmp binary) protect nothing against a PR, because the same job runs the PR's own xtask/gates source first (comment at lines 46-51).
Impact: Possible reduced audit coverage and version drift between two zizmor checks.
Known: none

### P56-F15
File: xtask/gates/src/gates/zizmor.rs:83,85; xtask/gates/src/gates/zkvm_immune_regeneration.rs:79-81
Severity: Info
Axis: invariant(Z13)
Confidence: Certain
Evidence: `format!("zizmor indirilemedi (curl): {e}")`, `String::from("zizmor indirilemedi (curl exit != 0)")`, `let (bulunan, reddi) = rejections(&exec);`
Description: Turkish error text and identifiers in repo code; Z13 says repo text is English. The tree-is-english gate did not flag them (no special letters).
Impact: Language rule only.
Known: none

### P56-F16
File: xtask/gates/src/main.rs:1316-1334, 1361-1364
Severity: Info
Axis: correctness
Confidence: Certain
Evidence:
```
main.rs:1318: /// `BUDLUM_ROOT` wins ... The fallback walks up from the binary's own directory
main.rs:1325: let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
main.rs:1331:     return PathBuf::from(".");
```
Description: The doc says the walk starts at the binary directory; the code starts at the current directory. When no ancestor holds docs/ARCHITECTURE.md and Cargo.toml, the root silently becomes ".". Gates mostly report a missing source file, so the result is a red run, not a pass. `--all` drops every gate with `run_log` or `run_args`; the registry has no check that such a gate has a CI step, so wiring depends on the rg result in the summary above (all 127 are wired today).
Impact: Maintenance only.
Known: none
