# Second pass alpha audit

Base: consolidate/audit-62-mainnet at 7c496cd. Method: docs/AUDIT_PLAN.md, parts P01 to P62, each part re-audited from scratch against its first-pass findings. New finding ids: SPA-Pxx-Fnn. Severity scale as in the first pass. Static reading only (rg and git).

## Progress

| Part | Status | New findings |
|---|---|---|
| P62 | covered | H1 L3 I1 |
| P02 | covered | M2 L3 I1 |
| P01 | covered | M1 L6 I3 |

## Parts

## P62 Repo-wide invariant scan

Base 7c496cd. Files read: scan output only (rg and git), plus spans of 5 to 25 lines around each cited hit. Not read: whole files (repo-wide scan by design).

History note: the clone is grafted. Root commit is f1b8a4a (rev-list --max-parents=0 HEAD) and `git log -S` for every allow and ignore below returns only f1b8a4a, an audit commit. The adding commit of any attribute is not recoverable here. 25ede0b exists, and `git diff --stat 25ede0b HEAD` shows 71 files changed (18 in .github).

### First-pass verdicts
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

### New findings
#### SPA-P62-F01
File: src/network/node.rs:2572-2573, 2609-2610, 2687-2688
Severity: High
Axis: security
Confidence: Certain for the slice; Likely for reach (missing check: a run with a multi-byte hash string. No earlier filter was found by rg).
Evidence: `&checkpoint_hash[..16.min(checkpoint_hash.len())]` inside `info!` in the Prevote, Precommit and FinalityCert arms. checkpoint_hash is a proto `string` (proto/budlum/network/protocol.proto:560, 568, 576) copied unchanged at proto_conversions.rs:2037. The only checks before the slice are the vote rate limit (node.rs:2567) and the scheme id (node.rs:2603-2608 region for FinalityCert). `rg 'is_canonical_hash' src` shows the shape check only at node.rs:761-762 (blocks) and protocol.rs:201, 224. The fix for the same class on transactions is documented at node.rs:2243-2246 and uses a 64 hex check; node.rs:2261 uses `chars().take(8)`.
What must be refused and what happens today: A vote or certificate whose checkpoint hash is not 64 ASCII hex must be refused before logging. Today a byte slice at 16 can land inside a multi-byte character and panic. Cargo.toml:304 sets `panic = "abort"`, so the whole node stops.
Impact: any peer that passes the vote rate limit, with no stake, can stop a remote release node.
Known: none

#### SPA-P62-F02
File: src/network/gossip_dedup.rs:46-86, 223, 305-310; src/network/peer_manager.rs:25-41, 90, 117, 163; src/account_abstraction/quantum_account.rs:156-165; src/ai/types.rs:743-751; src/rpc/server.rs:6405
Severity: Info
Axis: invariant(Z8)
Confidence: Certain
Evidence: f64 peer scores and token buckets in gossip_dedup.rs and peer_manager.rs. `rg 'peer_score\(|\.score\b|mean_peer_score_i64'` outside gossip_dedup.rs shows use only in metrics (node.rs:1953, 2035) and in peer_manager.rs integer scores (:363-433). `storage_cost` (quantum_account.rs:156) has no caller (`rg 'storage_cost\b' src` gives only the definition). reliability_score (ai/types.rs:743) feeds only an RPC reply (server.rs:6404). bud-vm/src/lib.rs:2343 uses f64 inside a test.
What must be refused and what happens today: A float must not reach block validity, state root, fee or reward. Today none does; float stays in peer policy, metrics and RPC output.
Impact: none today. Peer ban and rate state is node-local, not consensus.
Known: partly idle-code-baseline.txt (storage_cost not checked)

#### SPA-P62-F03
File: ops/budlum-core.service:10; budzero/LICENSE:95; budzero/verifier-registry/src/lib.rs:6; budzero/bud-node/Cargo.toml:27; src/registry/mod.rs:1; src/privacy/mod.rs:1; fuzz/fuzz_targets/consensus_state_transition.rs:6
Severity: Low
Axis: invariant(Z12)
Confidence: Certain
Evidence: `Description=Budlum Core Layer-1 Node` (ops/budlum-core.service:10); `Licensor Line of Business: Budlum - permissionless multi-consensus L1` (budzero/LICENSE:95, a second copy of LICENSE.md:95). Also 19 files under src with the term in comments (`rg -c -i '\bL1\b|layer[ -]?1' src`; hits include src/rpc/api.rs:434, src/ai/execution/guest.rs:487-489, src/consensus/mod.rs:88, src/chain/chain_actor.rs:2019, src/core/transaction.rs:355, 360) and 12 more in budzero/bud-proof (plonky3_air.rs:418, 1919; adapter.rs:136; verifier.rs:308; plonky3_prover.rs:4939; transfer_verdict.rs:14, 57). Root docs also use it: BUD-AI-KAPSAMLI-DIREKTIF.md:27, MODEL_ROUTING_KURULUM.md:256 (direktif files, owner text). Workflow comments use it too (osv-scanner.yml:11, extra-tooling.yml:44, cargo-vet.yml:9).
What must be refused and what happens today: the term must not appear in public text. Today the service unit, the second license copy and many rustdoc lines carry it. No Z12 text gate covers ops/ or budzero/LICENSE.
Impact: public-text rule only. docs/ARCHITECTURE.md:3246 (`L1[` is a mermaid node id) is a false positive; docs/bud/BUD-KESIF-RAPORU.md:419 quotes the rule.
Known: none

#### SPA-P62-F04
File: .github/workflows/ci.yml:528; semver.yml:113, 136; security-hardening.yml:256
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence: ci.yml:520-528: the badge step records `status="${PIPESTATUS[0]}"`, prints a notice if non-zero, then `exit 0`. semver.yml:113 `exit 0` after "NO BASE ... PASS with an info log" (semver.yml:108-113 region) under `set +e`. `rg 'set \+e'` gives 39 uses; each is followed within 25 lines by a status check (`$?`, PIPESTATUS, `set -e` or exit), a loop I ran per hit and which returned 0 unchecked. All `continue-on-error`, `|| true` hits are comments.
What must be refused and what happens today: a red test step must not pass silently. Today the badge copy of `cargo test --lib` cannot fail the job (the comment says the budlum job owns the verdict), and the semver gate passes when the base ref is empty or all zero.
Impact: a gap exists only if the owning job is removed or skipped. Not a softening by itself.
Known: none

#### SPA-P62-F05
File: Cargo.toml:265-266; bud/Cargo.toml:77, 80, 83
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence: `unused_qualifications = "allow"`, `deprecated_in_future = "allow"`, and in bud three clippy lints set to allow at workspace level. Each has a rationale comment (Cargo.toml:260-264; bud/Cargo.toml:74-83). The suppression gate reads only attributes in .rs files (suppressions_are_justified.rs:559), so these lines have no budget entry.
What must be refused and what happens today: a lint relaxation should be gated like an attribute. Today a new `= "allow"` line in a Cargo.toml passes the gate.
Impact: maintenance. Z3 text names attributes only.
Known: none

### Lead list (not verified, max 15)
Panic scan of src/network and src/rpc on decode paths:
1. `rg '\.unwrap\(\)|\.expect\(' src/network src/rpc`: every hit sits after the first `#[cfg(test)]` line of its file (proto_conversions.rs:2104, node.rs:3241, mobile.rs:481, privacy.rs:342, sync_codec.rs:99, server.rs:6739, peer_manager.rs:520, gossip_dedup.rs:385) or in src/rpc/tests.rs. No production unwrap or expect.
2. `panic!` in src/network: proto_conversions.rs:2376-2612 only, all in the test module.
3. Indexing guarded: node.rs:2384, 3053, 3092 follow `!blocks.is_empty()` or `!headers.is_empty()`; node.rs:2888 slices a 64 char hex string; rpc/server.rs:843-846 follows a `find(']')`; fixed `[0u8;32]` copies in proto_conversions.rs:745-1233 are outside the scope of this lead.
4. The only new decode-path panic lead is SPA-P62-F01.
Float scan beyond listed dirs: see SPA-P62-F02; no float in src/core, src/execution, src/domain, src/rpc (except the RPC rate reply), src/light_client or src/relayer consensus logic.

### Cross-part notes
- Cargo.toml:304 `panic = "abort"` with overflow-checks (Cargo.toml:303): any panic on a network path stops the node.
- config/testnet-genesis.json:6-17 and devnet-genesis.json:6-11 hold 64 hex values (allocation addresses and validator ids, public data type). No key material by file name or field role.

### Checks done
- Z3: `rg -n '^\s*#!?\[allow\(' --type rust` (88 + 3), `rg '#\[ignore'` (2), `rg 'expect\(' attributes` (0), Cargo lint allows, `rg 'continue-on-error|\|\| *true|; *true|\|\| *exit 0|set \+e' .github` filtered for comments, per-hit status check loop for `set +e`, `rg 'include-ignored|--ignored'`, `git log -S` on every attribute (root commit only).
- Z6: `git ls-files | rg '\.(sh|bash|py|ps1)$'` (0), `rg 'python3?|shell: *pwsh|<<' .github/workflows`.
- Z7: `rg '\bunsafe\b' --type rust` (comments, gate strings only), real-use pattern (0), `forbid(unsafe_code)` roots.
- Z8: listed dirs, then src/core src/execution src/network src/domain src/rpc src/light_client src/relayer src/account_abstraction, float literals in consensus dirs.
- Z9: `BEGIN ... PRIVATE KEY` (0), 64 hex literal count per file (counted, values not printed), key-named hex assignment pattern (0), cloud and VCS token patterns (hits only in redact test data: crates/ai-inference/crates/ai-knowledge/src/redact.rs:53-57, bud/src/bud_format_revolutionary.rs:330, 496-497; type: test data for a redactor), tracked file names with pem, key, env, secret (0). ops/docker-compose.yml:61 and ops/budlum-core.service:37 name a key file path only, no value.
- Z12: the first-pass command (29 hits in 15 files, 3 in docs/audit/FINDINGS.md) plus a tree-wide search outside docs.
- LABEL-12.9: first-pass command (0).

## P02 Crypto and bpqs

Base 7c496cd. Files read: src/crypto/primitives.rs 1-1196, pkcs11.rs 1-952, domain_tags.rs 1-364, key_set_preimage.rs 1-278, mainnet_policy.rs 1-231, signer.rs 1-144, mod.rs 1-17; crates/bpqs/src lib.rs, error.rs, params.rs, epoch.rs, hash.rs, merkle.rs, sign.rs, shake256.rs, tests.rs, tests_backends.rs, wots.rs (all lines); poseidon2.rs 1-60 and 205-690 (lines 61-204 are the round constant tables, checked only for shape: 4+4 vectors of 16, 22 internal). Not read: poseidon2.rs 61-204 (data tables; the KAT test at poseidon2.rs:458 and tests_backends.rs:363 pins them).

No history between 25ede0b and HEAD changes this part except unrelated commits (git log shows only 93c9895 and f1b8a4a, neither touches crypto semantics). Line numbers below are HEAD.

### First-pass verdicts
- P02-F01: CONFIRMED | pkcs11.rs:116-117,131-132,554-566 | bls_key and pq_key are created as None and never set; both public key getters return None. Only consumer is pos.rs:854. Mainnet PoS passes keys=None (main.rs:746-753), so a mainnet validator has no BLS public key to register. | none
- P02-F02: CONFIRMED | pkcs11.rs:107-115,157-160 | Key is read and bound with label None; with_key_label runs after. A slot with two Ed25519 pairs is refused before the label applies. A single-key slot with a non-matching label fails every sign call (private_key_handle finds 0 keys, 321). | none
- P02-F03: CONFIRMED, relayed part resolved | pkcs11.rs:221-231 | objects[0] with no count, key type, length or public key check. The vendor result is returned without local verification (blockchain.rs:5948-5954) and node.rs:1578-1588 publishes it and sets last_prevote_height. A bad vendor signature burns the validator's vote for that checkpoint (availability only). | none
- P02-F04: CONFIRMED | .github/workflows (rg bpqs) | Only the fuzz target (ci.yml:1494-1497, fuzz-nightly.yml:51-53) and an OSV scan (osv-scanner.yml:109) mention bpqs. No `cargo test`, clippy or fmt for crates/bpqs, and no run with feature bpqs-research (rg 'bpqs-research' in workflows: none). The KAT and refusal tests in the crate never run in CI. | none
- P02-F05: CONFIRMED | sign.rs:136-179 | sign_at_height is stateless; the quota is the caller count (143). Two different messages at count 0 in one epoch reveal two Winternitz signatures of the same epoch key. Only reachable with feature bpqs-research (settlement/mod.rs:27-28). | none
- P02-F06: CONFIRMED | wots.rs:26-32,137-148; merkle.rs:112-120; sign.rs:186-210 | Verifier ignores chain lanes at index LEN and above, lane bytes N..32, path entries at T_LOG2 and above. For N=24 rows, a non-zero tail reaches the debug_assert in lane() (wots.rs:27) through chain_step and panics under debug assertions. | none
- P02-F07: CONFIRMED | primitives.rs:505-518,677-689 | Both save paths: create, truncate, mode 0o600, no temp file, rename, fsync or create_new. Mode applies only to a new file. Callers main.rs:362,459 (keygen). Any caller can overwrite an existing key file. | none
- P02-F08: CHALLENGED in part | mainnet_policy.rs:102,105,122 | The softhsm backend spelling (raw and canonical) is tested at tests/hardening_h4_locks.rs:54-71 and hsm_mock at :37-42. Still untested: the module path substring rule (mainnet_policy.rs:122) and the "mock" alias (:102). A renamed library still passes the substring rule. | Medium to Low
- P02-F09: CONFIRMED | sign.rs:186-210; tests.rs:264-266 | Line moved: the ignored L5 test is tests.rs:264-266 (and tests_backends.rs:307-309 writer). No test for public key mismatch, params_name mismatch (192-196), epoch at or above 2^T_LOG2 on verify (204), tail tamper, epoch_of wrap, count u32::MAX. | none
- P02-F10: CONFIRMED | primitives.rs:136-141,569-574; 269-288,342-359 | 1 to 127 BLS bytes after the PQ block give bls None; trailing bytes ignored. PQ from_bytes checks lengths only. Scalar::from_bytes accepts zero. | none
- P02-F11: CONFIRMED | primitives.rs:84-95,196-208,491,660; sign.rs:86-91 | No zeroize crate in Cargo.toml (rg zeroize: no hit), no Drop impl, types are Clone. BpqsSigner keeps root_seed and epoch chains on the stack. | none
- P02-F12: CONFIRMED in part | pkcs11.rs:13-17,31,62 | module_path, slot_id, token_pin_env are never read after new. pkcs11_client (31) is a keep-alive for the library, so that allow is justified; the other three are dead fields. _load_software_fallback (62) is unused. primitives.rs:863 is an expect_used allow with a stated reason, not dead_code. | none
- P02-F13: CONFIRMED | signer.rs:10-15 | Defaults hash the message and call sign_block with no label. No production caller: the live vote path is blockchain.rs:5935-5970 and uses bls_sign. | none
- P02-F14: CONFIRMED | pkcs11.rs:281-286,342-345 | Public key value longer than 32 cut to 32; signature longer than 64 cut to 64. Signature is not verified per call. | none
- P02-F15: CONFIRMED | epoch.rs:321 | `(height / window.0) as u32` truncates. The verifier compares the truncated value. | none
- P02-F16: CONFIRMED, refined | sign.rs:146 | `per_epoch_count + 1` overflows at u32::MAX. Root Cargo.toml:303 sets overflow-checks, so a root release build aborts. crates/bpqs has no profile of its own (Cargo.toml read), so a standalone release build wraps and reports attempted 0. | none
- P02-F17: CONFIRMED | primitives.rs:1019-1058; tests.rs:211-218 | Test named "does not leak via println" cannot see stdout. Fixed /tmp path at 1022. Wire size test computes 2660 without the 16-byte randomizer (params.rs:201 and SECURITY-ARGUMENT.md:577 say 2676; SECURITY-ARGUMENT.md:73 says 2660). | none
- P02-F18: CONFIRMED | .github/dead-pub-api-baseline.txt:124-133; idle-code-baseline.txt:1434-1463 | Line numbers moved. Some baseline entries are now stale because main.rs:649 and main.rs:686 call check_mechanism_admissible_for_mainnet and vendor_capabilities. Baselines are not to be edited. | none
- P02-F19: CONFIRMED | wots.rs:52-61 | chain_step hashes only the lane prefix and the chain index. No step index, epoch, member or public seed. Merkle leaf and node (merkle.rs:25-38) carry no height or position either. Multi-target analysis was not redone here. | none
- P02-F20: CONFIRMED | primitives.rs:875-882 | WalletKeyPair::sign falls back to the deterministic variant when the RNG fails. | none
- P02-F21: CONFIRMED | epoch.rs:309 vs params.rs:195 | epoch.rs says q_max = 4; Q_MAX is 1. Bold markers (Z13) at primitives.rs:483,485; pkcs11.rs:379; mainnet_policy.rs:81; hash.rs:371-373. | none

### New findings
#### SPA-P02-F01
File: src/main.rs:686-693; src/crypto/pkcs11.rs:116-134,209-232
Severity: Medium
Axis: security
Confidence: Certain
Evidence: main.rs:687 `if !caps.vendor_native_signing_configured()` tests only that two mechanism ids are set. pkcs11.rs:131-132 sets both mechanisms to None at build; with_vendor_mechanisms (139) only parses strings.
What must be refused and what happens today: A mainnet validator whose HSM lacks the BUD_BLS_KEY or BUD_PQ_KEY private object, or whose vendor mechanism does not match the object, must be refused at start. Today only the Ed25519 key gets a start-up binding test (351-371). BLS and PQ failures show only at the first vote or checkpoint.
Impact: Mainnet validator operator; precondition is a misconfigured HSM; loses votes and liveness, not funds.
Known: none (extends P02-F01 and P02-F03)

#### SPA-P02-F02
File: src/crypto/pkcs11.rs:278-288; src/crypto/pkcs11.rs:107-115
Severity: Medium
Axis: test
Confidence: Likely (missing check: a run against a real token; CI has no PKCS#11 job)
Evidence: `rg -n -i 'softhsm|pkcs11' .github/workflows/*.yml` returns no hit. Public key is read from CKA_VALUE (281); PKCS#11 EC public keys carry the point in CKA_EC_POINT, and `rg 'EC_POINT' src` returns no hit.
What must be refused and what happens today: Nothing is refused wrongly; if a token returns no value or a wrapped value, start-up fails (error at 288 or binding failure at 365-370). No test shows any token works, so the whole mainnet signer is untested end to end.
Impact: Mainnet launch readiness; a token-specific start-up failure is found only in production.
Known: none

#### SPA-P02-F03
File: src/crypto/primitives.rs:525-582 (with 489-499)
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: save writes sig, vrf, then optional PQ then BLS with no header. load picks the PQ block by remaining length only (541-564). A file written by a Dilithium5 build, read by an ML-DSA-65 build, is cut at 1952+32 bytes and is accepted by from_bytes (length check only).
What must be refused and what happens today: A key file from another PQ scheme must be refused. Today the PQ key loads as garbage when length allows, and the BLS tail is then parsed from the wrong offset (usually an error, not always for a file without BLS).
Impact: Operator who switches the PQ feature; registers a PQ public key that never matches the seed. PQ_SCHEME_ID is checked at genesis (chain/genesis.rs:318) but not in the file.
Known: none

#### SPA-P02-F04
File: crates/bpqs/src/merkle.rs:79,112; crates/bpqs/src/params.rs:173-198; crates/bpqs/src/wots.rs:36-48,76-94
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: BpqsParams is an open public trait with no bound on N, LEN, LEN1, T_LOG2. auth_path refuses T_LOG2 above 16 (79) but verify_path uses `T_LOG2.min(16)` (112), so the verifier would accept a shorter path. digits_of slices `msg[..P::N]` and indexes 256-entry arrays.
What must be refused and what happens today: A parameter row with N above 32, LEN above 256 or T_LOG2 above 16 must not compile or must be refused. Today it panics or verifies a truncated path.
Impact: Only a downstream crate that adds a row; unwired. Fix before the slot is opened.
Known: none

#### SPA-P02-F05
File: crates/bpqs/src/hash.rs:414-424
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `(domain.len() as u16)` at 417 and `(part.len() as u32)` at 420. msg is caller data (sign.rs:162,167,207).
What must be refused and what happens today: A domain above 65535 bytes or a part above 4 GiB must be refused. Today the length wraps and the framing stops being injective for such inputs.
Impact: None in practice for anchor digests; defense in depth for the injective framing claim in hash.rs:394.
Known: none

#### SPA-P02-F06
File: src/crypto/primitives.rs:101,630
Severity: Info
Axis: security
Confidence: Certain
Evidence: BLS seed (101) and Ed25519 seed (630) come from `rand::rng()`; VRF key (469) and wallet ML-DSA seed (841) come from OsRng.
What must be refused and what happens today: Nothing refused. Consensus key material uses two randomness sources; the thread RNG is OS-seeded and cryptographically secure, so this is only a consistency point.
Impact: None known.
Known: none

### Cross-part notes
- src/crypto/key_set_preimage.rs:75-78 length-prefixes (u64 le) each key field; sites at core/account.rs:1035,2377, chain/snapshot.rs:91,868, chain/finality.rs:64 call it. State root and validator set digest inputs are injective for these fields.
- src/chain/finality.rs:270-282 PoP message carries tag, chain id, address and public key; BLS verify checks torsion and identity for key and signature (finality.rs:348-395, 402-430). Aggregation code is outside P02 and was not read.
- src/crypto/primitives.rs:731-761 Ed25519 verify checks lengths first and uses verify_strict. Every consensus Ed25519 signature goes through it.
- src/crypto/signer.rs:11-14 trait-default prevote and precommit signatures have no domain label (no production caller).
- main.rs:746-753 mainnet PoS passes no ValidatorKeys, so BLS and PQ keys must come from the HSM; pos.rs:854 then gets None (P02-F01).

### Checks done
- Domain separation: BLS sig and PoP DSTs differ (finality.rs:300-303 region), PoP message has tag; bpqs has 9 tags in lib.rs:48-73 plus length-framed parts; Ed25519 block and vote signing has no tag at primitives level (callers hash first). Binding test message (pkcs11.rs:10) is 32 bytes, the size of a block hash, but cannot be a hash preimage.
- Constant-time compare: only public data is compared (primitives.rs:150, merkle.rs:120). Poseidon2 field code uses cmov (poseidon2.rs:276-331); the loop count in chain walks depends on the public message digest.
- Nonce and randomness: Ed25519 and BLS deterministic; ML-DSA hedged (primitives.rs:378-381, 875-882); bpqs randomizer is a PRF output (sign.rs:156-166).
- Zeroization: none (P02-F11).
- PKCS11 error paths: lock poison, closed session, missing key, short signature all return Err (pkcs11.rs:451-509). Strict mode checks mechanism before software fallback (472-477, 515-520). PIN env is read once and moved into a SecretString (103); the env var stays set for the process lifetime.
- WOTS one-time use: P02-F05 (not enforced by state).
- Signature malleability: Ed25519 closed by verify_strict; bpqs tails open (P02-F06 first pass); ML-DSA decode is done by the library.
- Length checks before slicing: primitives.rs:127-133,527-531,555; pkcs11.rs:282,342; all guarded. bpqs slices use constants (SPA-P02-F04 for open trait).
- BLS rogue key: PoP bound to chain and address (finality.rs:270-282); aggregate verify not in this part.
- ML-DSA parameters: sign context b"" and verify context [] match (primitives.rs:380,403); 65 and 87 lengths fixed (343,349,902-939); signature class lengths distinct (770-773, 766).
- Z7 unsafe, Z8 floats, LABEL-12.9: `rg -n 'unsafe|f32|f64|unwrap_or_default' src/crypto crates/bpqs/src` gave only `#![forbid(unsafe_code)]` (lib.rs:41). Z3: allows at pkcs11.rs:13,15,17,31 and primitives.rs:863; ignores at tests.rs:265 and tests_backends.rs:308 (history is shallow, add dates unknown).
- Z12: domain_tags.rs:164 holds a tag literal containing "L1" (BDLM_L1_NOTE_REGISTRY_V1); it is a wire tag, not public text, so not a finding.
- Integration: `rg -n 'bpqs' src Cargo.toml` shows only the optional feature (Cargo.toml:24,160; settlement/mod.rs:27-28).

## P01 Consensus

Base 7c496cd. Files read: src/consensus/mod.rs 1-346, split_resolver.rs 1-168, merkle_tree.rs 1-461, pos.rs 1-1368, pow.rs 1-793, poa.rs 1-833, qc.rs 1-1374 (all tracked .rs files of the part, 5343 lines). Not read: none. Outside the part, rg first and short ranges only: blockchain.rs (1150-1200, 4640-4700, 4738-4765, 4890-4960, 5100-5262, 5740-5800), node.rs (755-765, 2380-2392, 3040-3100), main.rs (548-560, 725-760, 1026-1036, 1588-1650), account.rs (650-675, 985-1050, 1410-1475, 2235-2265), block.rs (60-200, 335-535), db.rs (1287-1303, 1420-1500).
History: `git log --oneline 25ede0b..HEAD -- src/consensus` shows one commit (f1b8a4a, audit notes only). No consensus code changed since the first pass, so line numbers below are HEAD lines of the same code.

### First-pass verdicts
- P01-F01: CONFIRMED | pos.rs:571, 397-401, 434-452, 607-621 | Slot equals index, seed has no round or retry term, threshold is stake share, so a height can have no leader. Mainnet PoS validator still exits at main.rs:729-733. | no change
- P01-F02: CONFIRMED | poa.rs:383-405 | Only the hash-chosen signer is accepted for a height. No timeout or backup exists in the file. | no change
- P01-F03: CONFIRMED | main.rs:1596, 1647-1650 | The producer loop starts only for PoS and PoW. PoA is reachable only from the stdin console. | no change
- P01-F04: CONFIRMED | pos.rs:983-993, 262-283, 916-917 | Checkpoint is written in record_block at local commit. chain_honours_checkpoint then refuses any candidate without that block, before the tie-break. | no change
- P01-F05: CONFIRMED | pos.rs:285-292, 715-719; blockchain.rs:5169, 5260 | validate_block refuses every block below the node checkpoint, and try_reorg replays from genesis through the same engine. | no change
- P01-F05b: CONFIRMED | node.rs:3053, 2388, 3096; blockchain.rs:5220-5237 | A fetched range (not a genesis-anchored chain) is passed to try_reorg and the result is dropped with `let _ =`. find_fork_point compares from index 0. | no change
- P01-F06: CONFIRMED | pos.rs:970; blockchain.rs:4756, 5057, 5437 | SlashingEvidence::new has one non-test call, inside record_block. record_block runs only for the tip+1 commit paths. try_reorg ends with record_block_with_chain only. | no change
- P01-F07: CONFIRMED | poa.rs:149-160; registry/poa_onboarding.rs:202 | `rg 'add_admin|with_authorities'` finds no non-test caller. poa_is_permissioned is false in production, so any active staker can lead (Z11). | no change
- P01-F08: CONFIRMED | pow.rs:222-226, 309-318 | Work is 2^(4d) and the retarget scales d by a 25..400 percent ratio. Devnet default is PoW (main.rs:728-730), mainnet is PoS. | Medium stands
- P01-F09: CONFIRMED (try_reorg route only) | poa.rs:411-415; main.rs:1033-1034 | Authorities are added to local state after the chain is built, so a genesis replay has an empty set and fails closed. | no change
- P01-F09b: CONFIRMED | main.rs:1033-1034; account.rs:658-662 | add_validator inserts a fresh Validator on every start, which resets stake, flags and jail state of a PoA authority. | no change
- P01-F10: CONFIRMED | poa.rs:228-241, 378-385 | Next leader derives from the previous hash, which covers nonce, timestamp and tx order. See also SPA-P01-F05 (stake term). | no change
- P01-F11: CONFIRMED | pos.rs:147-170, 808-824; blockchain.rs:1181-1194 | Evidence checks index, producer, signatures and hash difference. No chain_id check on either header or against the local chain. | no change
- P01-F12: CONFIRMED | pos.rs:579-586 vs 592-597, 618-621, 633-635 | Engine queue is cloned into the block and cleared before the leader decision. A non-leader tick returns Err and the evidence is gone. | no change
- P01-F13: CONFIRMED | pos.rs:737-739 before 804; node.rs:755-765 | The set hash (BLS PoP checks for every active validator) runs before the Ed25519 check. The gossip filter now also recomputes the block hash, but still checks no signature. | no change
- P01-F14: CONFIRMED | pos.rs:1011-1026; storage/db.rs:1480-1498 | Memory map is pruned to 2 epochs. `rg 'SEEN:' src/storage/db.rs` shows no delete, and load reads every record. | no change
- P01-F14b: CONFIRMED | db.rs:1493 | `parse().unwrap_or(0)` reads a bad height as 0. | no change
- P01-F15: CONFIRMED | blockchain.rs:931-933, 966-968; pos.rs:1055-1091 | Both startup callers only log a load_state error. Checkpoints and seen blocks load before the randomness replay, so a late failure leaves them partly loaded. | no change
- P01-F16: CONFIRMED | pow.rs:285-307; blockchain.rs:4689-4693 | The mining loop has no cancel and runs inside produce_block on the chain actor. | no change
- P01-F16b: CONFIRMED | pow.rs:292-302 | The loop never checks for a new tip. | no change
- P01-F17: CONFIRMED | pow.rs:128-137, 584-632, 687-698 | Retarget tests call a cfg(test) copy. Two tests assert only the 1..32 range. | no change
- P01-F18: CONFIRMED | src/tests/integration.rs:171-196; pos.rs:728-733 | The test block has slot 0 and index 1, so it fails at the slot check (pos.rs:728) before the signature check (pos.rs:804). | no change
- P01-F19: CONFIRMED | (rg) | `rg -i 'fn .*(invalid_vrf|vrf_threshold|wrong_leader|wrong_proposer|unknown_producer|invalid_evidence|poa.*reorg|pos.*reorg)' src` returns no test. Other files (slashing_matrix.rs, poa_isolation.rs) were not read, so partial coverage there is not excluded. | no change
- P01-F20: CONFIRMED | mod.rs:21-23, 139-174 | Only a forward bound and a 1 s minimum step. `rg MAX_PAST_BLOCK_TIME_MS src` shows the definition and a doc comment only. | no change
- P01-F21: CONFIRMED | mod.rs:197-228; pos.rs:461-563 | select_best_chain and can_reorg have no caller outside the trait. The inherent save_state and load_state (POS_STATE) have none. `seed[i] = v as u8` at pos.rs:546 and `unwrap_or(0)` at pos.rs:521 are in that dead path. PoAConfig quorum fields are read only by info() (poa.rs:427). | no change
- P01-F22: CONFIRMED | qc.rs:264-316, 594-598; blockchain.rs:3939 | verify_against_snapshot refuses a blob with any invalid signature, so a stored blob never yields a fault proof that passes verify_against_blob. | no change
- P01-F23: CONFIRMED | pos.rs:754; main.rs:549-552 | validate_block uses the per-node CLI min_stake. | no change
- P01-F24: CONFIRMED | poa.rs:291-295, 345-347; blockchain.rs:4690-4699 | A non-leader PoA node sets the expected producer on its block, leaves it unsigned, and fails its own full_validate. | no change
- P01-F25: CONFIRMED, wider | pos.rs:147-170 vs blockchain.rs:1181-1194; qc.rs:450-492 vs merkle_tree.rs:240-263; mod.rs:88; tests/consensus_expanded.rs:96-110; qc.rs:1132-1180 | Same items as the first pass. More rule hits are in SPA-P01-F07. | no change
- P01-F26: CONFIRMED (still present) | pos.rs:98; qc.rs:389, 438 | Three allow attributes remain. Their add date was not checked (history depth). | no change
- P01-F27: CONFIRMED | qc.rs:106-118; merkle_tree.rs:34-42 | Leaf and node hashes have no domain tags. The leaf preimage length is variable and can be 64 bytes, the node preimage length. A fault proof must match a stored entry, so no known use. | Info stands

### New findings
#### SPA-P01-F01
File: src/consensus/qc.rs:73-88, 655-672; src/chain/blockchain.rs:3885, 5779-5800
Severity: Medium
Axis: integration
Confidence: Likely (missing check: a producer of QC blobs outside src/ is not excluded)
Evidence: `rg -n 'QcBlob::new|pq_signing_message|sign_attestation' src --type rust -g '!src/tests/*' -g '!src/consensus/qc.rs'` returns only light_client/mod.rs:492 and 910-924 (inside `mod tests`, mod tests starts at light_client/mod.rs:485). The only production entry for a blob is the network import (node.rs:2761 to blockchain.rs:3885). `rg -n 'finalized_height = ' src/chain/blockchain.rs` shows only 3837 (invalidate), 5661 and 5703 (snapshot restore) and 5800 (handle_finality_cert).
What must be refused and what happens today: A finality cert must be refused without a verified QC blob (blockchain.rs:5779-5784 does refuse and queues it). Nothing in production signs a PQ attestation or builds a blob, so on a network of production nodes that requirement cannot be met.
Impact: finalized_height can stay at 0, so the chain-layer finality guards (blockchain.rs:4893, 5247) never bind and reorg safety rests on MAX_REORG_DEPTH and the PoS node-local checkpoint. Mainnet blocker for finality.
Known: dead-pub-api-baseline.txt:92 (sign_attestation) and :91 (new_zk_invalid_attestation) cover the names only.

#### SPA-P01-F02
File: src/main.rs:739; src/core/chain_config.rs:240-244; src/chain/blockchain.rs:4325-4328; src/consensus/pos.rs:721, 983
Severity: Low
Axis: invariant (epoch transition)
Confidence: Likely (missing check: a run with `--chain-id` set to a value that is not a known network id)
Evidence: engine epoch length comes from `network_params.epoch_len` (main.rs:739). The chain layer uses `epoch_len_for_chain_id(block.chain_id)` (blockchain.rs:4325), which falls back to `EPOCH_LEN = 100` for an unknown id (chain_config.rs:243). Chain id can be overridden at main.rs:548.
What must be refused and what happens today: The engine and the state machine must use one epoch length. With a custom chain id on devnet (10) or testnet (50), engine epoch, checkpoint cadence and randomness use one value while state epoch advance uses 100. Nothing refuses the mix.
Impact: operator running a non-standard chain id. Epoch bound state (rewards, validator snapshots, QC epoch) and engine epoch drift apart on that network.
Known: none (pos.rs:23-37 documents the default only).

#### SPA-P01-F03
File: src/consensus/pow.rs:208-229, 309-318; src/chain/blockchain.rs:5229-5260
Severity: Low
Axis: security
Confidence: Likely (missing check: reach depends on the F05b fix, which feeds try_reorg a real chain)
Evidence: try_reorg calls `is_better_chain` first (blockchain.rs:5230) and `validate_candidate_chain` later (5260). PoW is_better_chain derives the claimed difficulty path from the candidate own timestamps (pow.rs:310, 245) before any block of the candidate is validated.
What must be refused and what happens today: A candidate whose claimed work comes only from unvalidated timestamps should not buy a full replay from genesis. Today the order is cheap score first, then full replay. Today the exposure is closed by F05b.
Impact: after F05b is fixed, a peer can force one replay from genesis per crafted candidate (depth cap at blockchain.rs:5239-5243 does not bound the replay of the common prefix).
Known: none.

#### SPA-P01-F04
File: src/consensus/pos.rs:147-170; src/core/block.rs:59-80, 151-161, 484-510
Severity: Low
Axis: security
Confidence: Certain for the code; dormant because no writer exists
Evidence: BlockHeader has no producer_public_key field (block.rs:59-80) and BlockHeader::verify_signature checks Ed25519 against the producer bytes only (block.rs:158-160). Block::verify_signature has a second branch for an ML-DSA signed block (block.rs:468-510). `rg -n 'producer_public_key' src -g '!src/tests/*'` finds no assignment of a non-empty value.
What must be refused and what happens today: Double-sign evidence for a block signed with the V5 (ML-DSA) producer path cannot verify, so it would be refused and the producer cannot be slashed. Today no production code sets the V5 field.
Impact: when V5 producers are switched on, equivocation by them is not provable.
Known: none.

#### SPA-P01-F05
File: src/consensus/poa.rs:228-246
Severity: Low
Axis: security
Confidence: Certain (code), Likely for reach (stake tx path read only from the first-pass note)
Evidence: the leader hash input includes each active validator stake: `hasher.update(v.stake.to_le_bytes())` (poa.rs:239).
What must be refused and what happens today: Leader choice for a height should not change because an unrelated account changed stake. Today any change to the stake of an active validator changes the leader for all later heights, and a Stake transaction is open to any funded account (P01-F07).
Impact: an authority (or a staker in an open PoA domain) can steer the next leader with a stake change, in addition to the nonce lever in P01-F10.
Known: none.

#### SPA-P01-F06
File: src/consensus/qc.rs:450-492, 494-539
Severity: Low
Axis: security
Confidence: Likely (missing check: the RPC body limit for bud_submitQcFaultProof was not read)
Evidence: verify_inclusion loops over every element of `merkle_proof` and hashes each one with no length or count cap (qc.rs:462-483). validate() checks emptiness only (qc.rs:513-519). The proof reaches this code from the public RPC named at qc.rs:1062-1063.
What must be refused and what happens today: A proof with more elements than the tree depth (at most 32 for u32 leaf index) or with an element that is not 32 bytes should be refused before hashing. Today it is hashed in full. verify_against_blob later compares the leaf with the stored blob.
Impact: CPU cost proportional to the payload size, bounded only by the transport limit.
Known: none.

#### SPA-P01-F07
File: src/consensus/poa.rs:118, 126; src/consensus/mod.rs:24, 88
Severity: Low
Axis: invariant (Z12, Z13)
Confidence: Certain
Evidence: mod.rs:88 reads "mainnet L1's fact" in a rustdoc comment (Z12). Bold markers are in poa.rs:118, poa.rs:126 and mod.rs:24 (Z13), besides the merkle_tree.rs:8-9 hits already in P01-F25.
What must be refused and what happens today: Public text must use the approved name from rule Z12 and plain text. The comments carry the banned term and bold markers.
Impact: style rule only.
Known: partly P01-F25 (mod.rs:88, merkle_tree.rs:8-9).

#### SPA-P01-F08
File: src/consensus/pos.rs:373-377
Severity: Info
Axis: security
Confidence: Certain
Evidence: for target epochs 0 and 1 the randomness is the genesis bootstrap hash with no VRF term (`hasher.update(b"genesis_bootstrap")`).
What must be refused and what happens today: Nothing is refused. The leader schedule of the first two epochs (20 blocks devnet, 100 testnet, 200 mainnet by the pos.rs:31 comment) is known from genesis alone.
Impact: leader prediction in the bootstrap epochs.
Known: none.

#### SPA-P01-F09
File: src/consensus/poa.rs:353-417; src/consensus/pow.rs:376-415
Severity: Info
Axis: correctness
Confidence: Certain
Evidence: PoS requires `block.epoch == index / epoch_length` and `block.slot == index` (pos.rs:721-733). PoA and PoW validate neither field. `rg -n '(block|header)\.(epoch|slot)\b' src -g '!src/tests/*' -g '!src/consensus/*'` finds only copy sites (block.rs:70-71, proto_conversions.rs).
What must be refused and what happens today: Nothing reads the fields for PoA and PoW, so a free value does no harm today. A later reader would inherit unchecked data.
Impact: none today.
Known: none.

#### SPA-P01-F10
File: src/consensus/pow.rs:244-245, 309-318
Severity: Info
Axis: correctness
Confidence: Certain
Evidence: the window runs from block `boundary + 1 - interval` to `boundary`, so it spans `interval - 1` block gaps, while `expected_time = target_block_time * interval`.
What must be refused and what happens today: The measured span and the expected span should cover the same number of gaps. Today the ratio is biased by about 1 percent at interval 100, toward a higher difficulty.
Impact: small drift, no halt by itself.
Known: none.

### Cross-part notes
- Leader eligibility reads live state, not an epoch snapshot: stake and total stake at pos.rs:797 and validator set hash at pos.rs:737. total stake counts validators below min_stake (account.rs:992-997).
- PoS block validation depends on node-local mutable data (checkpoints, pos.rs:715) in addition to state. Two nodes with equal state can disagree, and a node can refuse its own history (blockchain.rs:5130 is_valid).
- PoS randomness indexes the chain slice by height (pos.rs:343-353) and requires the genesis block at slice position 0 (pos.rs:308-315). A pruned or suffix chain cannot validate PoS blocks. Pruning deletes old block records on disk (blockchain.rs:5112-5116); the restart path was not traced.
- Finality irreversibility is enforced at the chain layer, not in the engines: live path blockchain.rs:4893-4906, reorg path blockchain.rs:5247-5258. Engine fork choice (mod.rs:245-251, pos.rs:886-933, pow.rs:471-490, poa.rs:435-452) ignores finality. See SPA-P01-F01 for how finalized_height advances.
- Fork choice order is deterministic and arrival independent: split_resolver.rs:74-105 orders by weight, height, lower hash, lower proposer. PoW work is exact in 256 bits (pow.rs:23-77); the accumulated work walk and the per-prefix walk fold the same boundaries (pow.rs:213-219, 320-332).
- Block hash covers epoch, slot, VRF fields, validator set hash and the evidence bytes (block.rs:99-147, 339-360). Evidence apply is idempotent per validator through the slashed flag (account.rs:1425-1436).
- QC threshold math is not in P01. qc.rs enforces entry count at most validator count (qc.rs:255-262), unique signers (qc.rs:292-297) and optional required signers (qc.rs:318-326). The quorum comparison sits in chain/finality.rs (quorum_stake at finality.rs:119, quorum_count at 129), which belongs to another part.
- Slashing evidence reaches state only through block.slashing_evidence (blockchain.rs:4310-4321). Evidence has no height or age bound at apply time. The engine queue is in memory only (pos.rs:102) and is not persisted by the live path.
- Serialization determinism: consensus size check uses serde_json::to_vec and returns an error on failure (mod.rs:185-187). Validator set is a BTreeMap and get_active_validators sorts by address (account.rs:259, 999-1007), so leader selection does not depend on HashMap order. seen_blocks is a HashMap but is never iterated for output (pos.rs:1017 retain only).

### Checks done
- Read every line of the 7 files. Compared each first-pass finding with HEAD lines.
- Z8: `rg -n '\bf(32|64)\b|as f64|as f32' src/consensus` returns a comment line only (pos.rs:46). No float in the part. Threshold math uses u128 (pos.rs:441-445).
- Z3, Z7, LABEL-12.9: `rg -n '\bunsafe\b|#\[ignore|#\[allow|unwrap_or_default' src/consensus` gives three allow attributes (known P01-F26), no unsafe, no ignore. unwrap_or_default sites: mod.rs:150 (clock), pos.rs:946 (signature), merkle_tree.rs:138 (not inside to_bytes). No LABEL-12.9 hit.
- Z11: PoA reads the shared permissionless validator set (poa.rs:150) and narrows only through an admin that production never sets (rg add_admin, with_authorities). See P01-F07.
- Integration: `rg -n 'select_best_chain|\.can_reorg\(|save_state\(|calculate_seed\(|get_checkpoints\(|get_slashing_evidence\(|\.is_validator\(' src -g '!src/tests/*'` outside src/consensus returns nothing. record_block call sites: blockchain.rs:4756, 5057. `rg 'QcBlob::new|pq_signing_message'` for SPA-P01-F01.
- Double vote: record_block keys (producer, index) and keeps the first header (pos.rs:949-981). A third conflicting block yields one more evidence entry each time (no dedupe in the engine queue; blockchain.rs:1154-1161 dedupes pending only by the same ordered pair). Not a finding by itself.
- Replay and live agreement: validate_block is the same function for live (blockchain.rs:4951) and replay (blockchain.rs:5169). Differences are the node-local checkpoint input and the wall clock bound in validate_timestamp (mod.rs:148-157), which only rejects future times.
- Epoch transition: PoS has no transition code of its own. Epoch advance is in blockchain.rs:4325-4328. Mismatch risk in SPA-P01-F02.
- Leader election determinism: PoS seed from chain data only (pos.rs:299-403). PoA from index, previous hash, sorted set, stake (poa.rs:215-246). Malformed previous hash maps to a fixed sentinel (poa.rs:23-29).
- Not checked: tests/slashing_matrix.rs, tests/poa_isolation.rs, tests/fork_choice_locks.rs bodies (outside the part). Account.rs calculate_state_root (P01-F09c) was outside the part: UNCHECKED.
