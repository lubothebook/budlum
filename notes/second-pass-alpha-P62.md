# P62 Repo-wide invariant scan
Base 7c496cd. Files read: scan output only (rg and git), plus spans of 5 to 25 lines around each cited hit. Not read: whole files (repo-wide scan by design).

History note: the clone is grafted. Root commit is f1b8a4a (rev-list --max-parents=0 HEAD) and `git log -S` for every allow and ignore below returns only f1b8a4a, an audit commit. The adding commit of any attribute is not recoverable here. 25ede0b exists, and `git diff --stat 25ede0b HEAD` shows 71 files changed (18 in .github).

## First-pass verdicts
- P62-F01: CONFIRMED | .github/workflows/security-hardening.yml:334 (python gate heredoc), 384-423 (calls), provenance.yml:69 (`python3 - <<'PY'`). `git ls-files | rg '\.(sh|bash|py|ps1)$'` returns no file, so the repo has no script files. Python lives only inside yml. Also semgrep.yml:69 writes a `canary.py` fixture (data, not a gate). Severity note: Low is fairer, because no tracked file breaks Z6 literally.
- P62-F02: CONFIRMED | .github/workflows/determinism.yml:108 `shell: pwsh`. Info stays.
- P62-F03: CONFIRMED | crates/budscan/src/query.rs:110, crates/ai-inference/crates/ai-ops/src/logparse.rs:19, bud/src/bud_format_realcorpus.rs:13, xtask/gates/src/gates/ast_security_gates.rs:297 and 483, budscan_patchset.rs:177. `rg '^\s*#!?\[allow\('` gives 88 outside xtask and 3 inside xtask, the same counts as the first pass. The gate scan roots are `["src","benches","budzero","build.rs"]` (suppressions_are_justified.rs:687), so these six are outside it.
- P62-F04: CONFIRMED | crates/bpqs/src/tests.rs:103, tests_backends.rs:308. `rg 'include-ignored|--ignored' .github xtask` returns only comment text, so no workflow runs them. The doc at tests.rs:100-101 says to run by hand.
- P62-F05: CONFIRMED | src/bin/bud.rs:1, 3, 9, 38. Still present at HEAD.
- P62-F06: CONFIRMED, list incomplete | see SPA-P62-F03 for hits the first pass missed.
- Z7 claim: CONFIRMED | `rg '^\s*(pub )?unsafe (\{|fn|impl|extern|trait)|[=(,;] *unsafe \{'` returns 0 hits. forbid(unsafe_code) is in src/lib.rs:3 and in the budzero crates (bud-vm, bud-proof, bud-isa, verifier-registry, bud-state, bud-node, bud-compiler lib.rs:3). crates/budscan uses Cargo lints (Cargo.toml:46). bud uses Cargo lints (bud/Cargo.toml:54).
- Z8 claim: CONFIRMED for the listed dirs | only chain_actor.rs:331 and 2364 (AI ranking RPC type) plus a comment at consensus/pos.rs:46. See SPA-P62-F02 for the wider scan.
- Z9 claim: CONFIRMED | no private key block, no cloud or VCS token pattern in tracked non-note files (see Checks done).
- LABEL-12.9: CONFIRMED | `rg -n -A15 'fn to_bytes' --type rust | rg unwrap_or_default` returns 0 hits.
- Z3 workflow patterns: CONFIRMED | `continue-on-error`, `|| true`, `; true`, `|| exit 0` appear only in comments. See SPA-P62-F04 for `exit 0` and `set +e` paths.

## New findings
### SPA-P62-F01
File: src/network/node.rs:2572-2573, 2609-2610, 2687-2688
Severity: High
Axis: security
Confidence: Certain for the slice; Likely for reach (missing check: a run with a multi-byte hash string. No earlier filter was found by rg).
Evidence: `&checkpoint_hash[..16.min(checkpoint_hash.len())]` inside `info!` in the Prevote, Precommit and FinalityCert arms. checkpoint_hash is a proto `string` (proto/budlum/network/protocol.proto:560, 568, 576) copied unchanged at proto_conversions.rs:2037. The only checks before the slice are the vote rate limit (node.rs:2567) and the scheme id (node.rs:2603-2608 region for FinalityCert). `rg 'is_canonical_hash' src` shows the shape check only at node.rs:761-762 (blocks) and protocol.rs:201, 224. The fix for the same class on transactions is documented at node.rs:2243-2246 and uses a 64 hex check; node.rs:2261 uses `chars().take(8)`.
What must be refused and what happens today: A vote or certificate whose checkpoint hash is not 64 ASCII hex must be refused before logging. Today a byte slice at 16 can land inside a multi-byte character and panic. Cargo.toml:304 sets `panic = "abort"`, so the whole node stops.
Impact: any peer that passes the vote rate limit, with no stake, can stop a remote release node.
Known: none

### SPA-P62-F02
File: src/network/gossip_dedup.rs:46-86, 223, 305-310; src/network/peer_manager.rs:25-41, 90, 117, 163; src/account_abstraction/quantum_account.rs:156-165; src/ai/types.rs:743-751; src/rpc/server.rs:6405
Severity: Info
Axis: invariant(Z8)
Confidence: Certain
Evidence: f64 peer scores and token buckets in gossip_dedup.rs and peer_manager.rs. `rg 'peer_score\(|\.score\b|mean_peer_score_i64'` outside gossip_dedup.rs shows use only in metrics (node.rs:1953, 2035) and in peer_manager.rs integer scores (:363-433). `storage_cost` (quantum_account.rs:156) has no caller (`rg 'storage_cost\b' src` gives only the definition). reliability_score (ai/types.rs:743) feeds only an RPC reply (server.rs:6404). bud-vm/src/lib.rs:2343 uses f64 inside a test.
What must be refused and what happens today: A float must not reach block validity, state root, fee or reward. Today none does; float stays in peer policy, metrics and RPC output.
Impact: none today. Peer ban and rate state is node-local, not consensus.
Known: partly idle-code-baseline.txt (storage_cost not checked)

### SPA-P62-F03
File: ops/budlum-core.service:10; budzero/LICENSE:95; budzero/verifier-registry/src/lib.rs:6; budzero/bud-node/Cargo.toml:27; src/registry/mod.rs:1; src/privacy/mod.rs:1; fuzz/fuzz_targets/consensus_state_transition.rs:6
Severity: Low
Axis: invariant(Z12)
Confidence: Certain
Evidence: `Description=Budlum Core Layer-1 Node` (ops/budlum-core.service:10); `Licensor Line of Business: Budlum - permissionless multi-consensus L1` (budzero/LICENSE:95, a second copy of LICENSE.md:95). Also 19 files under src with the term in comments (`rg -c -i '\bL1\b|layer[ -]?1' src`; hits include src/rpc/api.rs:434, src/ai/execution/guest.rs:487-489, src/consensus/mod.rs:88, src/chain/chain_actor.rs:2019, src/core/transaction.rs:355, 360) and 12 more in budzero/bud-proof (plonky3_air.rs:418, 1919; adapter.rs:136; verifier.rs:308; plonky3_prover.rs:4939; transfer_verdict.rs:14, 57). Root docs also use it: BUD-AI-KAPSAMLI-DIREKTIF.md:27, MODEL_ROUTING_KURULUM.md:256 (direktif files, owner text). Workflow comments use it too (osv-scanner.yml:11, extra-tooling.yml:44, cargo-vet.yml:9).
What must be refused and what happens today: the term must not appear in public text. Today the service unit, the second license copy and many rustdoc lines carry it. No Z12 text gate covers ops/ or budzero/LICENSE.
Impact: public-text rule only. docs/ARCHITECTURE.md:3246 (`L1[` is a mermaid node id) is a false positive; docs/bud/BUD-KESIF-RAPORU.md:419 quotes the rule.
Known: none

### SPA-P62-F04
File: .github/workflows/ci.yml:528; semver.yml:113, 136; security-hardening.yml:256
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence: ci.yml:520-528: the badge step records `status="${PIPESTATUS[0]}"`, prints a notice if non-zero, then `exit 0`. semver.yml:113 `exit 0` after "NO BASE ... PASS with an info log" (semver.yml:108-113 region) under `set +e`. `rg 'set \+e'` gives 39 uses; each is followed within 25 lines by a status check (`$?`, PIPESTATUS, `set -e` or exit), a loop I ran per hit and which returned 0 unchecked. All `continue-on-error`, `|| true` hits are comments.
What must be refused and what happens today: a red test step must not pass silently. Today the badge copy of `cargo test --lib` cannot fail the job (the comment says the budlum job owns the verdict), and the semver gate passes when the base ref is empty or all zero.
Impact: a gap exists only if the owning job is removed or skipped. Not a softening by itself.
Known: none

### SPA-P62-F05
File: Cargo.toml:265-266; bud/Cargo.toml:77, 80, 83
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence: `unused_qualifications = "allow"`, `deprecated_in_future = "allow"`, and in bud three clippy lints set to allow at workspace level. Each has a rationale comment (Cargo.toml:260-264; bud/Cargo.toml:74-83). The suppression gate reads only attributes in .rs files (suppressions_are_justified.rs:559), so these lines have no budget entry.
What must be refused and what happens today: a lint relaxation should be gated like an attribute. Today a new `= "allow"` line in a Cargo.toml passes the gate.
Impact: maintenance. Z3 text names attributes only.
Known: none

## Lead list (not verified, max 15)
Panic scan of src/network and src/rpc on decode paths:
1. `rg '\.unwrap\(\)|\.expect\(' src/network src/rpc`: every hit sits after the first `#[cfg(test)]` line of its file (proto_conversions.rs:2104, node.rs:3241, mobile.rs:481, privacy.rs:342, sync_codec.rs:99, server.rs:6739, peer_manager.rs:520, gossip_dedup.rs:385) or in src/rpc/tests.rs. No production unwrap or expect.
2. `panic!` in src/network: proto_conversions.rs:2376-2612 only, all in the test module.
3. Indexing guarded: node.rs:2384, 3053, 3092 follow `!blocks.is_empty()` or `!headers.is_empty()`; node.rs:2888 slices a 64 char hex string; rpc/server.rs:843-846 follows a `find(']')`; fixed `[0u8;32]` copies in proto_conversions.rs:745-1233 are outside the scope of this lead.
4. The only new decode-path panic lead is SPA-P62-F01.
Float scan beyond listed dirs: see SPA-P62-F02; no float in src/core, src/execution, src/domain, src/rpc (except the RPC rate reply), src/light_client or src/relayer consensus logic.

## Cross-part notes
- Cargo.toml:304 `panic = "abort"` with overflow-checks (Cargo.toml:303): any panic on a network path stops the node.
- config/testnet-genesis.json:6-17 and devnet-genesis.json:6-11 hold 64 hex values (allocation addresses and validator ids, public data type). No key material by file name or field role.

## Checks done
- Z3: `rg -n '^\s*#!?\[allow\(' --type rust` (88 + 3), `rg '#\[ignore'` (2), `rg 'expect\(' attributes` (0), Cargo lint allows, `rg 'continue-on-error|\|\| *true|; *true|\|\| *exit 0|set \+e' .github` filtered for comments, per-hit status check loop for `set +e`, `rg 'include-ignored|--ignored'`, `git log -S` on every attribute (root commit only).
- Z6: `git ls-files | rg '\.(sh|bash|py|ps1)$'` (0), `rg 'python3?|shell: *pwsh|<<' .github/workflows`.
- Z7: `rg '\bunsafe\b' --type rust` (comments, gate strings only), real-use pattern (0), `forbid(unsafe_code)` roots.
- Z8: listed dirs, then src/core src/execution src/network src/domain src/rpc src/light_client src/relayer src/account_abstraction, float literals in consensus dirs.
- Z9: `BEGIN ... PRIVATE KEY` (0), 64 hex literal count per file (counted, values not printed), key-named hex assignment pattern (0), cloud and VCS token patterns (hits only in redact test data: crates/ai-inference/crates/ai-knowledge/src/redact.rs:53-57, bud/src/bud_format_revolutionary.rs:330, 496-497; type: test data for a redactor), tracked file names with pem, key, env, secret (0). ops/docker-compose.yml:61 and ops/budlum-core.service:37 name a key file path only, no value.
- Z12: the first-pass command (29 hits in 15 files, 3 in docs/audit/FINDINGS.md) plus a tree-wide search outside docs.
- LABEL-12.9: first-pass command (0).
