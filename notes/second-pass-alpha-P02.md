# P02 Crypto and bpqs
Base 7c496cd. Files read: src/crypto/primitives.rs 1-1196, pkcs11.rs 1-952, domain_tags.rs 1-364, key_set_preimage.rs 1-278, mainnet_policy.rs 1-231, signer.rs 1-144, mod.rs 1-17; crates/bpqs/src lib.rs, error.rs, params.rs, epoch.rs, hash.rs, merkle.rs, sign.rs, shake256.rs, tests.rs, tests_backends.rs, wots.rs (all lines); poseidon2.rs 1-60 and 205-690 (lines 61-204 are the round constant tables, checked only for shape: 4+4 vectors of 16, 22 internal). Not read: poseidon2.rs 61-204 (data tables; the KAT test at poseidon2.rs:458 and tests_backends.rs:363 pins them).

No history between 25ede0b and HEAD changes this part except unrelated commits (git log shows only 93c9895 and f1b8a4a, neither touches crypto semantics). Line numbers below are HEAD.

## First-pass verdicts
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

## New findings
### SPA-P02-F01
File: src/main.rs:686-693; src/crypto/pkcs11.rs:116-134,209-232
Severity: Medium
Axis: security
Confidence: Certain
Evidence: main.rs:687 `if !caps.vendor_native_signing_configured()` tests only that two mechanism ids are set. pkcs11.rs:131-132 sets both mechanisms to None at build; with_vendor_mechanisms (139) only parses strings.
What must be refused and what happens today: A mainnet validator whose HSM lacks the BUD_BLS_KEY or BUD_PQ_KEY private object, or whose vendor mechanism does not match the object, must be refused at start. Today only the Ed25519 key gets a start-up binding test (351-371). BLS and PQ failures show only at the first vote or checkpoint.
Impact: Mainnet validator operator; precondition is a misconfigured HSM; loses votes and liveness, not funds.
Known: none (extends P02-F01 and P02-F03)

### SPA-P02-F02
File: src/crypto/pkcs11.rs:278-288; src/crypto/pkcs11.rs:107-115
Severity: Medium
Axis: test
Confidence: Likely (missing check: a run against a real token; CI has no PKCS#11 job)
Evidence: `rg -n -i 'softhsm|pkcs11' .github/workflows/*.yml` returns no hit. Public key is read from CKA_VALUE (281); PKCS#11 EC public keys carry the point in CKA_EC_POINT, and `rg 'EC_POINT' src` returns no hit.
What must be refused and what happens today: Nothing is refused wrongly; if a token returns no value or a wrapped value, start-up fails (error at 288 or binding failure at 365-370). No test shows any token works, so the whole mainnet signer is untested end to end.
Impact: Mainnet launch readiness; a token-specific start-up failure is found only in production.
Known: none

### SPA-P02-F03
File: src/crypto/primitives.rs:525-582 (with 489-499)
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: save writes sig, vrf, then optional PQ then BLS with no header. load picks the PQ block by remaining length only (541-564). A file written by a Dilithium5 build, read by an ML-DSA-65 build, is cut at 1952+32 bytes and is accepted by from_bytes (length check only).
What must be refused and what happens today: A key file from another PQ scheme must be refused. Today the PQ key loads as garbage when length allows, and the BLS tail is then parsed from the wrong offset (usually an error, not always for a file without BLS).
Impact: Operator who switches the PQ feature; registers a PQ public key that never matches the seed. PQ_SCHEME_ID is checked at genesis (chain/genesis.rs:318) but not in the file.
Known: none

### SPA-P02-F04
File: crates/bpqs/src/merkle.rs:79,112; crates/bpqs/src/params.rs:173-198; crates/bpqs/src/wots.rs:36-48,76-94
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: BpqsParams is an open public trait with no bound on N, LEN, LEN1, T_LOG2. auth_path refuses T_LOG2 above 16 (79) but verify_path uses `T_LOG2.min(16)` (112), so the verifier would accept a shorter path. digits_of slices `msg[..P::N]` and indexes 256-entry arrays.
What must be refused and what happens today: A parameter row with N above 32, LEN above 256 or T_LOG2 above 16 must not compile or must be refused. Today it panics or verifies a truncated path.
Impact: Only a downstream crate that adds a row; unwired. Fix before the slot is opened.
Known: none

### SPA-P02-F05
File: crates/bpqs/src/hash.rs:414-424
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `(domain.len() as u16)` at 417 and `(part.len() as u32)` at 420. msg is caller data (sign.rs:162,167,207).
What must be refused and what happens today: A domain above 65535 bytes or a part above 4 GiB must be refused. Today the length wraps and the framing stops being injective for such inputs.
Impact: None in practice for anchor digests; defense in depth for the injective framing claim in hash.rs:394.
Known: none

### SPA-P02-F06
File: src/crypto/primitives.rs:101,630
Severity: Info
Axis: security
Confidence: Certain
Evidence: BLS seed (101) and Ed25519 seed (630) come from `rand::rng()`; VRF key (469) and wallet ML-DSA seed (841) come from OsRng.
What must be refused and what happens today: Nothing refused. Consensus key material uses two randomness sources; the thread RNG is OS-seeded and cryptographically secure, so this is only a consistency point.
Impact: None known.
Known: none

## Cross-part notes
- src/crypto/key_set_preimage.rs:75-78 length-prefixes (u64 le) each key field; sites at core/account.rs:1035,2377, chain/snapshot.rs:91,868, chain/finality.rs:64 call it. State root and validator set digest inputs are injective for these fields.
- src/chain/finality.rs:270-282 PoP message carries tag, chain id, address and public key; BLS verify checks torsion and identity for key and signature (finality.rs:348-395, 402-430). Aggregation code is outside P02 and was not read.
- src/crypto/primitives.rs:731-761 Ed25519 verify checks lengths first and uses verify_strict. Every consensus Ed25519 signature goes through it.
- src/crypto/signer.rs:11-14 trait-default prevote and precommit signatures have no domain label (no production caller).
- main.rs:746-753 mainnet PoS passes no ValidatorKeys, so BLS and PQ keys must come from the HSM; pos.rs:854 then gets None (P02-F01).

## Checks done
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
