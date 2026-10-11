# P50 scan: xtask gates 1 (read-only audit, Tier R1)

Base commit: 8775ea6. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md (P14, P58, P59).

## COVERAGE

Read in full (every line, tests and fixtures included), all under xtask/gates/src/gates/:
- accumulators_pinned.rs 1-352
- actionlint.rs 1-68
- ai_inference_prompt_is_true.rs 1-963
- ai_inference_reads.rs 1-194
- ai_inference_zk.rs 1-196
- air_selectors.rs 1-285
- ast_security_gates.rs 1-960
- badges_current.rs 1-358
- binding_claims.rs 1-256
- bit_decompositions.rs 1-562
- bns_gate.rs 1-28
- bns_names_are_safe_in_an_address_bar.rs 1-313
- bridge_dual_tree.rs 1-179
- bud_e2e.rs 1-31
- budscan_parity.rs 1-242
- budscan_patchset.rs 1-428
- capability_modules_are_wired.rs 1-1102
- cargo_vet.rs 1-160
- chain_id_is_not_hardcoded.rs 1-262
- ci_workflow_guards.rs 1-293
- clippy_extra.rs 1-164
Total 21 files, 7396 lines. Docs read: docs/AGENT_MAP.md, docs/AUDIT_PLAN.md 110-165, notes/P29-scan.md 1-30.

Outside the part (rg first, then small ranges): xtask/gates/src/gates/exact_named_tests.rs (check_exact_log, rg -A45); xtask/gates/src/main.rs (rg on gate names, --all); xtask/gates/Cargo.toml (head 30); .github/workflows/ci.yml 156-200, 370-415 and rg hits; .github/clippy-extra-baseline.txt, cargo-vet-baseline.txt, unwired-guards-baseline.txt (heads); budzero/bud-proof/src/plonky3_air.rs 905-946 (rg); budzero/bud-isa/src/lib.rs (rg); src/bns/registry.rs 30-75 (rg); crates/budscan/src/patchset.rs 355-360; src/tests/bns.rs, bns_expanded.rs, bud_e2e.rs (rg for the pinned test names, all four sampled exist); src/storage/content_id.rs (rg).

Not read: the rest of xtask/gates (files 22 and up), src/bns/registry.rs beyond the lines above, workflow files other than the ci.yml ranges. No cargo, no test run, no git history beyond one `git log -S`. Claims come from reading source.

Extra-check summary:
- Vacuous passes: most gates fail closed on empty sets (air_selectors, accumulators, budscan_patchset, chain_id have explicit empty-set branches). Passes that are not fail closed: P50-F01 (binding sum never required to be asserted), P50-F04 (both sides change together), P50-F12, P50-F13, P50-F02/F03 (CI guard). cargo_vet returns Ok on a missing tool (P50-F15).
- Swallowed errors: capability_modules_are_wired.rs:631, chain_id_is_not_hardcoded.rs:166, bit_decompositions.rs:316-317 (this one fails closed afterwards). See P50-F23.
- Regex or parser gaps: P50-F05, F06, F09, F10, F11, F12, F13, F21.
- Baseline handling: clippy_extra has two-direction ratchet and is sound; cargo_vet has only one direction (P50-F15). No gate in the part edits or loosens a baseline file. unwired-guards-baseline.txt is read by another gate, not by this part.
- Z3: three `#[allow(clippy::...)]` in the part (P50-F20). No `#[ignore]`, no continue-on-error in the part. Z6: no shell script added; the part contains no .sh/.py file (`git ls-files | rg '\.(sh|bash|py)$'` returned nothing).
- Checked and clean: bns_gate.rs and bud_e2e.rs delegate to exact_named_tests::check_exact_log, which matches the full line `test <name> ... ok` with trim_end (substring cannot satisfy it); sampled names exist in src/tests. The gates are run by `-- --all` at ci.yml:1719-1721.

## FINDINGS

### P50-F01
File: xtask/gates/src/gates/air_selectors.rs:119-141, 181-189
Severity: Medium
Axis: test
Confidence: Certain
Evidence: `parse_binding_sum` only searches the text for `opcode_here.clone() - op(0x`. In the AIR the sum is a `let` at plonky3_air.rs:912 and is constrained at plonky3_air.rs:945 `builder.assert_zero(selector_opcode_binding);`. The gate never looks for line 945. It also pairs each hit with `before.rfind("is_")` (air_selectors.rs:125), the nearest `is_` anywhere before, so a binding written with another variable name is credited to the wrong selector, and `bound.insert` (line 136) lets a later hit overwrite an earlier one.
Description: Deleting the `assert_zero` call (or leaving the sum in a dead local) keeps the gate green, because the text of the sum still exists. A selector that nothing pins lets a prover choose which opcode rules apply to a row.
Impact: ZK soundness regression class (ZK part, AIR constraint per opcode) would not be caught by this gate.
Trigger: precondition: a refactor drops or comments out line 945 (block comment) or splits the sum. Entry: budzero/bud-proof/src/plonky3_air.rs. Expected wrong result: gate prints "AIR selector binding OK".
Known: none

### P50-F02
File: xtask/gates/src/gates/ci_workflow_guards.rs:123-135, 148-151
Severity: Medium
Axis: test
Confidence: Certain
Evidence: line 131-134: `trimmed.contains("cargo run") && trimmed.contains("--") && trimmed.contains(subcommand)`, a plain substring test with no word boundary. Line 150: `invokes_anywhere(r, gate) && r.contains("--self-test")`, where `--self-test` may belong to any command in the same run block.
Description: Subcommand `relay` is satisfied by any line such as `cargo run --release -p budlum-relayer` (contains `relay`). The canary count for one gate is satisfied by another gate's `--self-test` line in the same block. The word-boundary logic of `invokes` (lines 108-118) is bypassed by the fallback.
Impact: The three protected gates (regeneration, relay, tree-pin) can lose their real invocation or canary and the guard still passes. Scope is limited because ci.yml:1719-1721 runs `-- --all`.
Trigger: precondition: workflow edit that removes `-- relay` but keeps another `cargo run ... relay...` line, or merges two gates in one block with one `--self-test`.
Known: none (P58 covers ci.yml itself, not this guard)

### P50-F03
File: xtask/gates/src/gates/ci_workflow_guards.rs:40-101, 137-204
Severity: Medium
Axis: invariant(Z3)
Confidence: Certain
Evidence: `run_blocks` keeps every non-blank line of a block (line 67 `block.push_str(trimmed)`), including YAML comment lines. Nothing reads `continue-on-error`, `if:`, `|| true`, or the job `needs`. Lines starting `- run:` are not matched at all (line 78 `trimmed.strip_prefix("run:")`).
Description: A commented-out line `# cargo run -- gate relay --self-test` inside a run block counts as an invocation. A protected step with `continue-on-error: true` or `if: false` counts as live. The doc header promises that the gates "actually run in CI".
Impact: A softened protected gate (Z3 class) is not detected by the one gate built to detect it. Only 3 gates are in PROTECTED_GATES (line 26).
Trigger: precondition: workflow edit as above. Entry: .github/workflows/ci.yml or diverse-double-compiling.yml. Expected wrong result: guard OK.
Known: none

### P50-F04
File: xtask/gates/src/gates/budscan_parity.rs:160-165, 175-183, 188-197
Severity: Medium
Axis: invariant
Confidence: Certain
Evidence: line 188-190 `chain.contains("pub const EPOCH_LENGTH: u64 = 10;") != browser_lc.contains(...)`; line 175-177 `browser_limit != core_limit` over `contains("10 * 1024 * 1024")`. Line 160 checks the length prefix only in the browser copy (`browser_cid`); `core_cid` is checked only for the tag (line 151).
Description: The comparison is a presence XOR, not a value comparison. If both sides change (chain 20, browser 30) or both drop the literal, both are false and the gate passes. The core ContentId could drop its length prefix and the gate stays green (src/storage/content_id.rs:36 only has a doc line about it). The OK message claims "agree in all three places" but only two files are compared.
Impact: Light client (budscan) takes wrong epoch headers, or verifies a content id that differs from the chain's, with the parity gate green.
Trigger: precondition: a parameter change touching one or both files in a way that breaks the `= 10;` text. Entry: crates/budscan/src/light_client.rs, src/chain/blockchain.rs.
Known: none

### P50-F05
File: xtask/gates/src/gates/chain_id_is_not_hardcoded.rs:46-78
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: line 49-52: `while let Some(pos) = rest.find("#[cfg(test)]") { ... let Some(brace) = after.find('{') ...` then line 60-70 skips to the matching `}` of that first brace.
Description: A `#[cfg(test)]` on an item without a body (a `use`, `const`, `static`, a `fn` declaration with `;`) makes the stripper take the next `{` anywhere after it, which is the next production function, and delete that whole function from the scan. capability_modules_are_wired.rs:74-88 documents the same defect and fixes it with `test_item_end`; this gate still has it. `None => return out` (line 73) also drops the rest of the file when braces do not balance, for example a `'{'` char literal.
Impact: A literal `chain_id: 1,` in a production function that follows a body-less `#[cfg(test)]` item is invisible. This is the wrongly bound public input class the gate exists for.
Trigger: precondition: `#[cfg(test)] use ...;` placed above a production function. Entry: any file under src, budzero, crates.
Known: none

### P50-F06
File: xtask/gates/src/gates/chain_id_is_not_hardcoded.rs:83-110
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: line 89 `trimmed.find("chain_id:")?` takes the first match only; line 92 returns None when the first match is inside `other_chain_id:`. Line 105-107 accepts only an empty tail or `,` or `}` after the digits.
Description: Bypasses: `chain_id: 1u64,` (tail `u64,`), `chain_id: 0x1,`, `chain_id: 1 as u64,`, a value on the next line, and a line with `expected_chain_id: x, chain_id: 1 }` (first hit is the prefixed one, the real one is never seen). Assignments `chain_id = 1;` and `let chain_id = 1;` are not covered.
Impact: Same class as P50-F05, easier to hit by accident (suffix literal).
Known: none

### P50-F07
File: xtask/gates/src/gates/capability_modules_are_wired.rs:55, 575-581, 656, 704-715, 85
Severity: Medium
Axis: integration
Confidence: Certain (logic), Likely (real hit; missing step: list which unwired modules are only reached from other unwired or driver files)
Evidence: `SCAN_ROOTS = ["src", "budzero", "crates/wallet-core"]`; `is_test_path` excludes only `/tests/`, `_tests.rs`, `/tests.rs`, `/benches/`. The evidence pool is every other file in `production`, including files that themselves declare `WIRING: unwired`, `src/sdk/*` developer tools, `src/bin/*`, examples and kani or fuzz drivers if present under budzero (`ls -d budzero/*/benches` shows budzero/bud-proof/benches only, so benches are excluded). strip_test_mods matches only the literal `#[cfg(test)]` (line 85); `#[cfg(all(test, ...))]` and `#[cfg(any(test, ...))]` are not stripped, so test calls count as evidence.
Description: Wiring is any mention from any other non-test file, not a path from node startup. Module A (dead) calling module B makes B wired. The header says drivers are excluded, but the code excludes only benches and test paths. Char literals `'{'` and `'}'` are not handled inside `test_item_end` (line 146-181), so a test module that contains one ends early and its remaining calls leak into the evidence.
Impact: The gate can report "wired" for a module whose only caller is itself unreached. This is the idle-code class of CLAUDE.md section 1.
Known: partly (STATUS.md line 55 lists unwired parts)

### P50-F08
File: xtask/gates/src/gates/capability_modules_are_wired.rs:604-610, 717-731
Severity: Info
Axis: integration
Confidence: Certain
Evidence: `rg -c 'WIRING: unwired' src budzero crates/wallet-core --glob '*.rs'` lists 17 files, among them src/settlement/pq_anchor.rs:3, src/settlement/cold_wallet.rs:740, src/tokenomics/reward_pool.rs:7, src/network/privacy.rs:3, src/relayer/policy.rs:1, src/core/account.rs:156, budzero/bud-proof/src/bud_stark/verifier.rs:3.
Description: A one-line `//! WIRING: unwired` comment turns the gate green for any module. There is no cap and no baseline for the marker count (unwired-guards-baseline.txt counts guard functions, not markers). Most of these are R3 parts.
Impact: Parts that CLAUDE.md section 1 calls open mainnet blockers can be parked with a comment. Observation only.
Known: STATUS.md line 55 (bridge relayer, egress privacy, reward pool, PQ anchor and cold wallet unwired)

### P50-F09
File: xtask/gates/src/gates/ast_security_gates.rs:368-403, 328-357
Severity: Medium
Axis: security
Confidence: Certain (logic); Likely (no current false pass on crates/wallet-core/src/lib.rs; missing step: read the real sign_with_privacy)
Evidence: line 371-379: the guard kind is chosen by `cond.contains("verify_measurement")`, `cond.contains("backend")`, `cond.contains("verify_report_data")`. line 387, 391, 395 assign `self.measurement_guard = ok` (assignment, not OR). The then-branch is checked only for a direct `return Err` (GuardErrCheck, line 249-295, `compact(expr).contains("Err")`).
Description: Polarity is never checked. `if attestation.verify_measurement(x) { return Err(..) }` (inverted) and `if attestation.backend == TeeBackendKind::ClientSgx { return Err(..) }` pass as guards. Any later `if` that mentions the same word overwrites the earlier flag, so a real guard followed by a decoy `if` that contains `backend` but returns Err inside it also passes, while a harmless later `if` makes it fail. `contains("Err")` also matches `return Ok(Error::..)`. Receiver check for verify_quote is `recv.contains("verifier")` only (line 353).
Impact: Defense in depth for the TEE trust boundary (wallet signing) has a regression hole; an inverted attestation check passes the AST gate.
Trigger: precondition: refactor that inverts a guard. Entry: crates/wallet-core/src/lib.rs sign_with_privacy.
Known: none

### P50-F10
File: xtask/gates/src/gates/ast_security_gates.rs:461-473, 540-554, 415-425
Severity: Medium
Axis: security
Confidence: Certain (logic)
Evidence: `closure_tail_is_digest_cmp`: `c.contains("evidence_hash") && c.contains("==")` on the token text of the tail. `visit_expr_if`: `cond.contains("evidence_hash")` then any direct `return true` in the then-block (TopLevelTrue).
Description: A tail `digest == evidence_hash || true` passes the closure check. `if digest != evidence_hash { return true; }` passes the if form (no polarity check). `digest_guards_return` is sticky: once set by one `if`, a later weakened `if` does not clear it. `has_digest_condition` for the closure needs "sha2" in the text (line 480), a string match on a path spelling.
Impact: The gov-slash evidence check (a slashing authority path in src/core/account.rs execute_proposal) can be inverted or made always true without this gate noticing.
Trigger: precondition: refactor of execute_proposal SlashValidator arm. Entry: src/core/account.rs.
Known: none

### P50-F11
File: xtask/gates/src/gates/ast_security_gates.rs:69-123, 254-295, 415-456, 138-147
Severity: Low
Axis: security
Confidence: Certain
Evidence: nesting counters exist for closure, `ItemFn`, `ExprIf`, `ExprMatch`, `ExprLoop` only. There is no override for `ExprWhile`, `ExprForLoop`, async blocks, `ImplItemFn`, or a plain nested block `{ ... }`. `stmt_fails_closed` (line 142) uses `compact(e).contains("Err")`.
Description: `while false { return Err(..) }` or a nested impl method with `Ok(())` counts as a direct member. The module header says these decoy classes are closed.
Impact: Narrow; needs a deliberate decoy. Defense in depth only.
Known: none

### P50-F12
File: xtask/gates/src/gates/bit_decompositions.rs:168-206, 213-300, 95-110, 344-352
Severity: Medium
Axis: security
Confidence: Certain (logic)
Evidence: `bool_z(code)`, `pinned(code)`, `costs(code)` take no operand argument; they search the whole AIR text once (called at lines 277, 283, 291 inside the per-operand loop). `strip_line_comments` blanks only `//` comments, not `/* ... */`. The wrapping_sub test is `ws_seq(&prover_src, &["wrapping_sub(", "0xFFFF_FFFF"])` (line 345), and only when the AIR contains COL_CMP_RS1_HI_INV.
Description: The canonicity rule for rs2 can be removed while rs1's text stays, and both operands report OK. A constraint left inside a block comment still satisfies the match. `wrapping_sub(0xFFFFFFFF)`, `wrapping_sub(HI_MASK)` or lowercase hex passes the prover check. The "reconstitution" check (line 234) only confirms the text `BASE + i]` exists, not the sum.
Impact: Non-canonical bit patterns (second valid bit string for values below 2^32 - 1, the documented bug) could return on comparison opcodes with the gate green.
Trigger: precondition: AIR refactor or edit of the prover. Entry: budzero/bud-proof/src/plonky3_air.rs, plonky3_prover.rs.
Known: none

### P50-F13
File: xtask/gates/src/gates/accumulators_pinned.rs:37-64, 91-93, 251-256, 172-215
Severity: Medium
Axis: security
Confidence: Certain (logic); Likely (effect on the real AIR; missing step: read plonky3_air.rs layout)
Evidence: line 59-60: `let end = stmt_start + "builder".len() + i + 1 + end_rel; out.push(rest[..end].to_string());` where `rest` starts after the previous statement. The pushed "statement" is therefore all text since the previous builder statement, including preceding `let x = cur[COL_GAS_USED].into();` lines. `mentions` (line 91) then matches the column inside that leading text.
Description: A `when_first_row` statement about an unrelated column counts as the pin of an accumulator when a `let` reading that accumulator sits just before it. Same for `when_transition`. A pin passes if any first-row statement mentions the name and is not a literal A-A tautology (line 187, 210); `gas - prover_chosen` passes. Comments are cut at the first `//` even inside strings (line 229), and `src.contains(column)` (line 239) uses the uncommented source.
Impact: A removed or weakened first-row pin on COL_GAS_USED, COL_EVENT_DIGEST_0, COL_MEM_INIT_ACC or COL_REG_INIT_ACC can stay unnoticed; a sliding accumulator lets a prover claim another gas figure or memory image.
Trigger: precondition: AIR edit that removes a pin while a nearby `let` reads the column. Entry: budzero/bud-proof/src/plonky3_air.rs.
Known: none

### P50-F14
File: xtask/gates/src/gates/badges_current.rs:32-49, 134-183, 225-244
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `measured_count` takes the last line (reverse scan) that contains " passed" with digits before it. It is not anchored to `test result:`. `check_license_badge` is called only for README.md (line 223); README.tr.md gets the CI and rust badge checks only (line 239-240), although the header says all four claims are repeated there.
Description: A stray line in the log (a test that prints "... 12 passed", a doc-test summary of 0 passed if the producer ever adds `--doc`) can become the measured count. The Turkish license badge is unchecked.
Impact: Badge claim can drift or be pinned to a wrong number. Low, README only.
Known: none

### P50-F15
File: xtask/gates/src/gates/cargo_vet.rs:80-117
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence: line 83-89 returns `Ok("SKIPPED: cargo-vet is not installed ...")`; line 109-113 `if n < base { return Ok(...IMPROVEMENT...) }`. Baseline file .github/cargo-vet-baseline.txt holds 433.
Description: Two soft spots. (1) A missing tool is a pass; the decision is left to cargo-vet.yml (exists, no continue-on-error found by rg), and nothing in this gate or ci_workflow_guards checks that the job stays. (2) The ratchet has one direction only. clippy_extra.rs:96-103 refuses a baseline more than 10 percent above the measured count; cargo_vet has no such check, so a baseline that sits above the measured count allows new unvetted dependencies up to the old number.
Impact: Supply-chain debt can grow silently by the slack. Low.
Known: none

### P50-F16
File: xtask/gates/src/gates/bns_names_are_safe_in_an_address_bar.rs:172-223 ; src/bns/registry.rs:66-69
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: registry.rs:66-69 `let char_count = name.chars().count(); if !(3..=32).contains(&char_count) { return Err(BnsError::InvalidName); }` and nothing else. `rg -n 'check_name' src crates` finds callers only in crates/budscan (browser copy) and this gate. The gate's `run` passes while the registry accepts `javascript:alert(1)`; it fails only if the registry ADDS a rule.
Description: The gate pins the open defect instead of failing on it. The character set rule exists only in the browser copy and in the gate, not on chain. The module header says the registry change "is being asked separately"; no matching entry in STATUS.md or docs/audit/FINDINGS.md (rg for bns or javascript returned only P58-F07).
Impact: Names with schemes, slashes, NUL, RTL override and uppercase homographs register on chain; consumers other than budscan see them unfiltered. Mainnet blocker for BNS (R1 part, consensus surface once chain-side).
Known: none

### P50-F17
File: xtask/gates/src/gates/bns_names_are_safe_in_an_address_bar.rs:75-79, 160-167
Severity: Low
Axis: security
Confidence: Certain
Evidence: HyphenAtLabelEdge only rejects a label that starts or ends with `-`. `xn--80ak6aa92e.bud` has `x` at the start and `d` at the end and passes the charset (a-z, 0-9, hyphen, dot). The Display text claims "the shape is reserved so punycode's own prefix cannot be forged".
Description: Punycode labels are accepted, so an IDN homograph reaches the address bar as an ASCII name that a browser may display in Unicode. The mixed-script check never sees it. Suffix is not restricted either (`a.b` passes).
Impact: Homograph display risk in budscan or any IDN-aware client.
Known: none

### P50-F18
File: xtask/gates/src/gates/ai_inference_prompt_is_true.rs:458-488, 594-621, 416-441, 322-338
Severity: Low
Axis: test
Confidence: Certain
Evidence: `check_answer_surface` matches only `pub fn` or `pub async fn` whose name EQUALS answer, respond, reply, complete or generate (line 473, `eq_ignore_ascii_case`), and passes when the file contains `redact_` anywhere (line 477), including a comment or an unused import. The core file scan is `read_dir` non-recursive (line 596-611). `check_mask_sites` uses `source.contains(call)` (line 430) over raw source including comments and tests. `defines` (line 322) matches `fn <symbol>` in comments too.
Description: `pub fn answer_request`, `pub(crate) fn`, or a function in a subdirectory module is not seen. A comment saying `redact_model_strings(` satisfies a mask site. The self-test message says "18 canaries" while numbering restarts at 15 twice (lines 647-680 and 864-885); fixture numbering is duplicated.
Impact: The "no key reaches an output" promise has weaker backing than the gate text claims. Low, AI inference layer is not on the consensus path.
Known: none

### P50-F19
File: xtask/gates/src/gates/ai_inference_reads.rs:77-111, 152-163 ; ai_inference_zk.rs:92-114
Severity: Low
Axis: test
Confidence: Certain
Evidence: reads: the generating-variant test is a prefix test (`t.starts_with("Image"|"Video"|"Audio"|"Text")` plus a word list) on perception.rs only; `needs_decoder` check only looks for `Self::Text` within 6 lines, not polarity. zk: the two naming tests are checked by `tier.contains("fn <name>")` only (line 96); a `#[ignore]` or empty body passes (the self-test fixture itself uses empty bodies, line 152). `served_model_name` check is `contains("\"ai_inference-") && contains("self")` over a body with comments included (line 106).
Description: Name-presence checks without body or attribute checks. A literal served name plus the word self in a comment passes. A numeric tier variant check flags any digit on a variant line, including `Light = 1,`.
Impact: Weak guard on tier naming and the no-generation promise. Low.
Known: none

### P50-F20
File: xtask/gates/src/gates/ast_security_gates.rs:297, 483 ; xtask/gates/src/gates/budscan_patchset.rs:177
Severity: Low
Axis: invariant(Z3)
Confidence: Likely (missing verification step: whether these three are inside the CI suppression budget and when they were added; `git log -S` on budscan_patchset.rs returns only the import commit e7e3a45)
Evidence: `#[allow(clippy::struct_excessive_bools)]` twice and `#[allow(clippy::too_many_lines)]` once. rg over .github/*baseline* finds no entry for these lint names. xtask/gates/Cargo.toml sets clippy pedantic to deny, and ci.yml:1723 runs clippy with `-D warnings` on xtask/gates.
Description: Z3 forbids new `#[allow(...)]`. Cannot tell from the tree whether these are old budgeted items.
Impact: Policy only.
Known: none

### P50-F21
File: xtask/gates/src/gates/binding_claims.rs:100-140, 172-192, 37-71 ; air_selectors.rs:38-72
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `scan_bindings(root)` walks the whole repo; `has_mobile` is true for any `.udl` anywhere or any `uniffi::export` text, `has_browser` for any `#[wasm_bindgen` in any `.rs` file (outside target, .git, node_modules). The claim is the first line containing `bindings_are_wired` with `: true` or `: false` (line 174), so a test fixture above the real constant can be read first. `strip_non_code` (both files) treats every `'` as a char literal start and skips to the next `'`, so a lifetime such as `<'a>` swallows code up to the next quote.
Description: "Wired" is satisfied by bindings in any crate, not wallet-core. The lifetime bug can hide text; today budzero/bud-isa/src/lib.rs has the only `'` (line 190, `'_`) after the opcode lines 7-55, so air_selectors is not hit now.
Impact: Latent. A false "bindings wired" claim could pass. Low.
Known: none

### P50-F22
File: xtask/gates/src/gates/budscan_parity.rs:205-242 ; budscan_patchset.rs:40-45, 245-266, 348-428
Severity: Low
Axis: test
Confidence: Certain (canary gap); Likely (brand token; missing step: which foreign project the list names)
Evidence: budscan_parity self_test only proves that a nonexistent tree fails and that constants are non-empty; no fixture shows a divergence failing. budscan_patchset self_test never runs the brand scan or the list-vs-disk check on a fixture (canary 5 is `t.len() < 5`). The brand token list joins `["obs","ide"]` to "obside", which is not a prefix of "obsidian". `allowed_roots` uses `starts_with("browser/")` (line 259), so `browser/../../x` passes. Only `patches/*.patch` at top level are read (line 199-203).
Description: Two gates cannot show they can fail on the thing they guard (AUDIT_PLAN section 5 tools: a gate can really fail). The token spelling looks like a typo of a longer name; cannot confirm which.
Impact: A brand name or a traversal path in a patch could pass.
Known: none

### P50-F23
File: xtask/gates/src/gates/capability_modules_are_wired.rs:631 ; chain_id_is_not_hardcoded.rs:166-169 ; bit_decompositions.rs:316-317
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `std::fs::read_to_string(p).unwrap_or_default()` (capability, line 631); `let Ok(text) = std::fs::read_to_string(f) else { continue; };` (chain_id, line 166); `unwrap_or_default()` on AIR and prover reads (bit_decompositions, fails closed afterwards because the column text is then "gone").
Description: An unreadable file (permission, invalid UTF-8) is treated as empty or skipped. In capability_modules_are_wired it removes the file from both the measured set and the evidence pool; in chain_id it is simply not scanned and still counted in `files`. Not an error.
Impact: Silent narrowing of coverage. Low.
Known: none

### P50-F24
File: xtask/gates/src/gates/budscan_patchset.rs:55, 69, 73-74 ; actionlint.rs:34 ; ast_security_gates.rs:3-30, 362-363 ; clippy_extra.rs:7-9 ; capability_modules_are_wired.rs:937-942 ; bridge_dual_tree.rs:43-46
Severity: Info
Axis: invariant(Z13)
Confidence: Certain
Evidence: Turkish text in repo code: "Yama listesini oku.", "yol bos", "listede iki kez var", "actionlint temiz.", "islenirse asagida yakalanir", "Korumalar", "varyant", "guard'siz". clippy_extra.rs:7-9 says the crate has no dependencies and uses no serde_json, while the code calls `serde_json::from_str` (line 52). A doc comment is attached to the wrong item in bridge_dual_tree.rs (lines 43-46, doc of `run` sits on `report`) and in capability_modules_are_wired.rs (lines 937-942 above `self_test_strip_canaries`).
Description: Z13 says repo text is English. Stale comments mislead the next reader.
Impact: Maintenance only.
Known: .github/turkish-baseline.txt exists (a separate gate counts Turkish text; whether these lines are inside its budget was not checked)

## COUNTS

Critical 0, High 0, Medium 11 (F01, F02, F03, F04, F05, F07, F09, F10, F12, F13, F16), Low 11 (F06, F11, F14, F15, F17, F18, F19, F20, F21, F22, F23), Info 2 (F08, F24). Total 24.
