# Audit findings, first session

Base commit: 25ede0b. Date: 2026-10-09. Written to the repository by owner decision: the network is in the testnet phase (see CLAUDE.md section 3, rule Z14). Before mainnet, move any open item to the private channel in docs/SECURITY.md and remove this file.

Status words: Confirmed means a new Opus (architect) call read the code and agreed. Raw means a Sonnet agent reported it and nobody verified it yet. Severity of a raw finding is the Sonnet value and may change at verification. Static reading only: no cargo run, no test run, no exploit run.

Parts covered: P62 (repo-wide invariant scan), P01 (consensus), P02 (crypto and bpqs), P03 (tokenomics, privacy, settlement), P04 (registry), P05 (cross-domain core, Part 7). All other parts are not started. See docs/AUDIT_PLAN.md and docs/AUDIT_PROGRESS.md.

## Part 1. Confirmed findings (P01 F01 to F08)

### P01-F01 PoS leader election can stall the chain. Critical. Confirmed.
Files: src/consensus/pos.rs:299-465, 565-636, 695-802; src/core/chain_config.rs:262-263; src/chain/blockchain.rs:4613-4630; src/main.rs:1581-1615.
Evidence: slot is the block index (pos.rs:571). The VRF threshold is stake times u64::MAX divided by total stake (pos.rs:441-445) and VRF_BASE_PROB equals FIXED_POINT_SCALE, so the extra factor is 1. The seed uses only a tag, chain_id, epoch, slot and epoch randomness (pos.rs:397-401); it has no time, round or retry term. Epoch E uses the VRF outputs of epoch E-2 (pos.rs:293-298, 317-370). The producer loop retries every tick (main.rs:1605-1608) and gets the same answer each time.
Impact: at one height the chance that no validator is leader is (1-1/N)^N for equal stakes: N=2 25 percent, N=3 29.6 percent, N=4 31.64 percent, limit 36.8 percent. Validators below min_stake count in the total (account.rs:992-997) but cannot lead (pos.rs:607-609), so the chance is higher. A height with no leader never gets a block. Single-validator tests hide this (src/tests/integration.rs:371).
Exposure today: the mainnet PoS validator exits at start (main.rs:729-733), so the exposure is devnet and testnet. It becomes unconditional when the HSM VRF path exists.
Known: STATUS.md:102 lists the proposer liveness design as an owner decision.

### P01-F02 PoA leader failure stalls the chain. High. Confirmed.
Files: src/consensus/poa.rs:194-296, 374-416.
Evidence: one leader per height, computed from index, previous hash and the active set (poa.rs:200-201, 230-243). Only that signer is accepted (poa.rs:398). No timeout, backup or round (rg "timeout|backup|fallback" is empty).
Impact: if the chosen leader is offline the height never fills. Chance per height with one offline node is 1/N.
Known: STATUS.md:102.

### P01-F03 PoA has no production loop in daemon mode. High. Confirmed (integration gap).
Files: src/main.rs:1595, 1647-1649, 1672-1674.
Evidence: the producer loop starts only for PoS and PoW (main.rs:1595). For PoA, produce_block is reachable only by the stdin command "block" or "mine" (main.rs:1674). When stdin closes, the console waits forever.
Impact: a PoA daemon stays at genesis.

### P01-F04 Checkpoint is written at local commit time with no finality. Critical. Confirmed.
Files: src/consensus/pos.rs:197-292, 886-1053; src/chain/blockchain.rs:4718-4730, 5015-5032.
Evidence: pos.rs:984 and 993 write the checkpoint when the block at an epoch boundary is committed locally. add_checkpoint has one non-test caller (pos.rs:993). It has no link to QC or finalized height. After that, chain_honours_checkpoint (pos.rs:916-917) rejects any candidate that lacks the block; this check runs before the tie breaker (pos.rs:925-930).
Impact: two leaders at a boundary height are normal (about 26 percent for N=4 equal stakes). Each producer commits its own block and each peer commits the first block it sees. The nodes then hold different checkpoints and reject each other chains at any length. This is a permanent split with no attacker. A two-node run was not done; the result comes from the code trace.
Known: STATUS.md:102 lists the checkpoint and reorg rule as an owner decision.

### P01-F05 Reorg to a checkpointed chain always fails. High. Confirmed, plus an extra root cause.
Files: src/consensus/pos.rs:288, 715-719; src/chain/blockchain.rs:5099, 5121, 5138, 5198-5229; src/network/node.rs:2375-2390, 3040-3097.
Evidence: PoS does not override full_validate, so the default calls validate_block. validate_block rejects any block below the local checkpoint (pos.rs:715, 288). try_reorg validates the candidate from block 1 with the same engine object (blockchain.rs:5121, 5138). After the first local checkpoint, block 1 of any candidate is rejected and try_reorg returns an error. The same check makes is_valid() (blockchain.rs:5099) reject the node own chain.
Extra root cause P01-F05b (PoW and PoS): node.rs:3053 takes from = headers[0].index, the height after the local suffix. node.rs:2388 and 3096 pass that suffix to try_reorg. The suffix does not start at genesis, so find_fork_point (blockchain.rs:5188-5196) returns Some(0) and try_reorg fails with "replace configured genesis" at 5207, or is_better_chain (5199) returns false for a short suffix. Both call sites drop the result with `let _ =`. No reorg can succeed over the network.
Impact: a minority-fork node stays stuck. F05b must be fixed before a fix for F04 can be observed.

### P01-F06 Double-sign evidence is not produced on the receive path. High. Confirmed.
Files: src/consensus/pos.rs:935-1045; src/chain/blockchain.rs:4716-4728, 5018-5030, 5398-5410; src/network/node.rs:1596-1600, 2281-2290.
Evidence: SlashingEvidence::new has one non-test call (pos.rs:970), inside record_block. record_block is called only at blockchain.rs:4725 and 5026, both for the tip+1 commit. The end of try_reorg calls record_block_with_chain only (5405-5406). node.rs:2282 only accepts and relays evidence from a peer. A gossiped block gets no producer double-sign check (rg for double sign and seen_blocks in node.rs and chain_actor.rs is empty). Because of F05b a rival block never enters the chain either.
Impact: block-level double-sign proof is practically never created. The test at pos.rs:1173 calls record_block twice directly and does not use the production path. Evidence should hook the receive path, not the commit path.

### P01-F07 Anyone with stake can become a PoA leader candidate. High. Confirmed; one sub-claim was wrong.
Files: src/consensus/poa.rs:44-56, 95-112, 140-180, 252-272; src/main.rs:545-560, 995-1035; src/execution/executor.rs:216-285; src/registry/poa_onboarding.rs:25-31.
Evidence: poa.rs:151 narrows the leader set only when the domain has an admin (poa_is_permissioned needs has_any_admin). add_admin has no non-test caller (poa.rs:626, 659 and poa_onboarding.rs:470 are inside cfg(test)). PoAEngine::new leaves authorities empty and nothing calls with_authorities. Stake (executor.rs:216-262) and RegisterConsensusKeys (271) have no domain or PoA gate. min_stake is 1 outside chain ids 45260-45262 (executor.rs:237-238, chain_config.rs:101). Leader choice ignores stake weight (poa.rs:257-262), so a Sybil raises its slot share.
Wrong sub-claim of the raw report: "validators_file is never read" is false. main.rs:558-559 reads the file and main.rs:1033-1035 adds those addresses to state with stake 1. What is unused is the engine own validators_file field.
Exposure: the default mainnet consensus is PoS (main.rs:553), but the strict rules in commands.rs do not restrict consensus, so `--consensus poa` is open on mainnet. The PoA domain add-on on a PoS node is frozen (main.rs:970-974), so the impact is in an isolated PoA network. Precondition: a funded account on the PoA chain. This breaks rule Z11 in practice.

### P01-F08 PoW retarget scales the exponent linearly. Medium (raw High). Confirmed, structural.
Files: src/consensus/pow.rs:169-345, 700-775; src/main.rs:712-722.
Evidence: work is 2^(4d) (pow.rs:222-226) but the retarget multiplies d by a ratio clamped to 25..400 percent (313-315). The right update is d plus log16(r). Examples: d=2, r=400 gives 8 (work x16^6); d=2, r=150 gives 3; d=1 with r under 200 does not change.
Why lowered: production uses the calibrated engine (main.rs:719); the measured stall in the test comment belongs to the uncalibrated engine. The test at pow.rs:722-755 only checks arithmetic and does not mine.
Remaining scenario: two or more miners, MIN_BLOCK_INTERVAL 1 second (mod.rs:166-167), testnet slot 3 seconds, r about 200 to 300, path 2 to 4 to 12. At d=12 each block needs 16^12 about 2.8e14 hashes, a lasting halt. Missing proof: no real multi-miner run. Mainnet default is PoS.

Root causes merged:
- K1: leader election depends only on height and state, with no round, timeout or backup leader (F01, F02). Owner decision (STATUS.md:102).
- K2: no PoA production loop (F03).
- K3: checkpoint without finality, used as an absolute lower bound (F04, F05). Owner decision.
- K4: network sync gives try_reorg a suffix without genesis (F05b). A plain wiring bug.
- K5: double-sign detection is tied to the commit path (F06).
- K6: PoA permission opens only when an admin exists, and no production path assigns one (F07).
- K7: retarget scales the exponent linearly (F08).
Handoff advice: F05 and F06 get separate handoffs; fix K4 before K3.

## Part 2. P01 F09 to F27 (F09 to F16 verified in V04, F17 to F27 raw)

- F09 Medium. PARTLY confirmed (V04). Only the try_reorg route fails (validate_candidate_chain starts from genesis_config.build_state, blockchain.rs:5115; PoA then fails with No active PoA authorities, poa.rs:411-414). Restart and rebuild_state do not call consensus validation. Genesis validators without consensus keys are inactive (genesis.rs:581-603), so listing them in genesis does not fix it. Raw text: src/consensus/poa.rs:353-417; main.rs:1033-1035; genesis.rs:581-583; blockchain.rs:5106-5139. PoA authorities are added to local state after the chain object is built (main.rs:1033). Genesis state does not contain them. validate_candidate_chain replays from genesis_config.build_state() where the PoA set is empty, so it fails with "No active PoA authorities". try_reorg and full sync from genesis fail. Likely; missing step: a run with validators.json.
- F10 Medium. CONFIRMED (V04). Timestamp and tx choice can also be ground; about N tries let one authority keep the chain. Raw text: poa.rs:215-246, 378-385. The next leader derives from the previous block hash (poa.rs:234). The hash includes nonce, timestamp and tx order; validation does not constrain the nonce. The current leader can try nonces to choose the next leader (about N tries): censorship or steering the chain to an offline leader.
- F11 Medium. CONFIRMED (V04), re-rated from forged to replayed real signatures; needs the same key on two chains. Raw text: pos.rs:147-170, 579-586, 808-824; blockchain.rs:1181-1194. Slashing evidence checks same index, same producer, different hash. It does not check chain_id (BlockHeader.chain_id, block.rs:31). A key used on two chains allows forged double-sign proof from public blocks and a wrongful slash (account.rs:1410). Known: STATUS.md:101 "slashing evidence chain id binding".
- F12 Medium. CONFIRMED (V04). Raw text: pos.rs:565-636. Evidence in the engine queue is moved into the block before the leader check and the queue is cleared (579-583). If the node is not leader the block is dropped and the evidence is lost. Known: STATUS.md:101.
- F13 Medium. CONFIRMED (V04). Gossip filter checks only chain_id and hash (node.rs:759-764). Honest blocks also pay N uncached BLS PoP verifies per block. Raw text: pos.rs:695-836; account.rs:1009-1042. validate_block calls consensus_validator_set_hash before the signature check; that does BLS PoP and PQ key checks for every active validator. The cheap Ed25519 check is last (804). An unauthenticated block message costs N times BLS work. Magnitude not measured.
- F14 Medium. CONFIRMED (V04). Raw text: pos.rs:1011-1026; storage/db.rs:1454-1500. Memory seen_blocks is pruned to 2 epochs but the SEEN: records on disk are never deleted and are all loaded at start.
- F15 Medium. CONFIRMED (V04). One bad SEEN record fails load_state before checkpoints load. Raw text: pos.rs:197-239, 935-1030; blockchain.rs:930-934, 4725-4728. Persistence errors are only logged by callers. If load_state fails at start the node runs with empty checkpoints and seen blocks, so checkpoint enforcement and double-sign records are silently off.
- F16 Medium (PoW only, devnet default). CONFIRMED (V04). Raw text: pow.rs:285-307, 364-374; chain_actor.rs:3398-3399. Mining is a synchronous loop inside the chain actor and cannot be cancelled. At high difficulty the actor handles no RPC, validation or sync.
- New from V04: F09b Medium: main.rs:1033 add_validator on every start resets a PoA authority to stake 1, active, not slashed, not jailed (account.rs:658-660). F09c Info: calculate_state_root hashes only balance and nonce (account.rs:2266-2276), so a different local validator set never shows as a root mismatch. F14b Low: db.rs:1492 parse().unwrap_or(0) reads a bad height as 0. F16b Low: PoW mining never checks for a new tip.
- F17 Medium (test). pow.rs:128-137, 309-318, 584-632, 687-698. Retarget tests call a cfg(test) copy (adjusted_difficulty_for_test) and two tests only check the range 1..32.
- F18 Medium (test). src/tests/integration.rs:171-196; pos.rs:728-733. The test block has slot 0 and index 1, so validate_block fails on "slot mismatch" before the signature check; the test passes for the wrong reason.
- F19 Medium (test). Missing negative tests: invalid VRF, VRF threshold, unknown or inactive producer, wrong validator_set_hash, invalid slashing evidence, wrong PoA leader, PoS and PoA try_reorg, multi-validator PoS production. F01, F04, F05, F06 and F07 are not caught by tests.
- F20 Low. consensus/mod.rs:136-172 (constant at line 22). Only a forward time bound (+15 s) and a 1 s minimum step exist. MAX_PAST_BLOCK_TIME_MS is defined and never used. A producer can slow chain time by staying 1 s ahead of the parent.
- F21 Low (dead code). mod.rs:197-228; pos.rs:863-884, 467-563, 147-170, 405-432, 461-465. can_reorg and select_best_chain have no production caller; the old POS_STATE persistence path is unused (with `seed[i] = v as u8` at 546 and `unwrap_or(0)` at 521); several PoSConfig and PoAConfig fields are never read (slot_duration, annual_reward_rate, slashing_penalty, double_sign_penalty, unbonding_epochs, block_period, epoch_length, quorum fields); is_validator, calculate_seed, get_checkpoints, get_slashing_evidence have no caller. Partly known: idle-code-baseline.txt:1278-1297, dead-pub-api-baseline.txt:84-91.
- F22 Low. qc.rs:340-383, 450-492, 541-637; blockchain.rs:3735-3760. Every stored QC blob has only valid signatures, so the InvalidDilithiumV1 fault proof always fails with "targets a valid signature"; the ZK variant always returns an error (qc.rs:597, 617). The path is reachable from network and RPC but decides nothing.
- F23 Low. pos.rs:754; main.rs:550-552. validate_block uses the CLI --min-stake as a consensus rule while the executor uses the network parameter (executor.rs:276-278). A node with another value splits off.
- F24 Low. poa.rs:252-351; blockchain.rs:4668-4672. A non-leader PoA node still applies state and computes the root, then fails local validation and logs an error every tick.
- F25 Low. pos.rs:147-170 and blockchain.rs:1181-1194 duplicate the evidence check; qc.rs:450-492 rewrites merkle_root_from_proof; consensus/mod.rs:88 uses the banned term in rustdoc (Z12); bold markers in merkle_tree.rs:8-9 (Z13); consensus_expanded.rs:96-195 repeats one test 40 times; qc.rs:1133-1180 is a timing test that can flake.
- F26 Info. pos.rs:98, qc.rs:389, qc.rs:438: three allow attributes whose add date could not be found in the shallow history.
- F27 Info. qc.rs:106-118, 450-492; merkle_tree.rs:34-42. Leaf and inner node hashes have no domain tags. Not exploitable today because fault proofs must match a stored entry.

## Part 3. Raw findings P02 (crypto and bpqs)

Scope read: all 19 files. Clean checks: no unsafe in the part (bpqs forbids it), no float, no secrets, no to_bytes unwrap_or_default, Ed25519 uses verify_strict (primitives.rs:726-762), key_set_preimage is wired at 5 sites, bpqs framing is injective, Poseidon2 and SHAKE256 have vector tests. bpqs is not wired to production (only cfg feature bpqs-research).

- F01 Medium. CONFIRMED (V06). Fails closed: non-strict bls_sign returns No BLS key in HSM (pkcs11.rs:504-508). The only consumer is pos.rs:854. Raw text: src/crypto/pkcs11.rs:116-134, 417-418, 554-566. bls_key and pq_key are set to None and nothing ever writes them. bls_public_key and pq_public_key always return None. Only consumer: consensus/pos.rs:854. The public half of an HSM BLS or PQ key is unknown, so the mainnet HSM path is not wired end to end for BLS and PQ. STATUS.md:55 mentions only the VRF gap.
- F02 Low. CONFIRMED (V06). Runs on every PKCS#11 start; fails closed, the feature is dead, not unsafe. Raw text: pkcs11.rs:107-115, 155-160; main.rs:673-677. The public key read and the binding check run with key_label None; the label is set later by with_key_label. A slot with several Ed25519 keys is refused before the label applies (require_single_key, 276, 321), so the label feature cannot work. It fails closed.
- F03 Medium. CONFIRMED (V06); the relayed part is UNCLEAR. session.sign takes objects[0] with no count, length or public key check (pkcs11.rs:218-231). Strict mainnet path (main.rs:678-679); callers blockchain.rs:5926, 5958. Impact is an invalid own vote (availability), not forgery. Not checked: whether the vote is verified before broadcast. Raw text: pkcs11.rs:209-232, 478-509, 521-552. The vendor sign path takes objects[0] when several objects match the label (the Ed25519 path says never guess, 274-276). Signature length and public key are not checked. The software path hashes to curve; the vendor path sends the raw message. A wrong object or a different DST could produce an invalid BLS or PQ vote that is relayed. Precondition: several keys with one label, or a vendor scheme mismatch. Entry: pkcs11.rs:464, 511.
- F04 Medium. PARTLY confirmed (V06): bpqs has no cargo test gate (only the fuzz target, ci.yml:1367-1370, a cargo hack check in security-audit.yml:340 and an osv scan). Blocks mainnet only if bpqs is wired. Raw text: .github/workflows/ci.yml; crates/bpqs/Cargo.toml. crates/bpqs is a separate package with its own lock file and the root has no workspace. In the workflows read, bpqs has only the fuzz target (ci.yml:1367, 1370) and no cargo test, clippy, fmt or cargo-deny. Missing step: check other workflows and xtask gates.
- F05 Low now, High if wired as is. CONFIRMED (V06). Property that breaks: the one-time property of WOTS. Two messages in one epoch are signed with the same key; the verifier cannot see it; EUF-CMA fails for that epoch. Safety rests on a caller-held counter that nothing persists. Reachable only with feature bpqs-research (settlement/mod.rs:27-28). Fix before wiring. Raw text: crates/bpqs/src/sign.rs:131-179, 186-210. sign_at_height is stateless; quota comes from the caller count (143). Two messages with count 0 in one epoch reuse the same WOTS key and the verifier cannot tell. Not wired to production (pq_anchor.rs:70 is reserved).
- F06 Low. CONFIRMED (V06). Property that breaks: strong unforgeability (non-malleability) only; EUF-CMA holds. Tail bytes N..32 (L3), lanes LEN..256 and root and path tails can change without failing verification. In debug and fuzz builds a non-zero tail can panic in lane() (release has no debug assertions). Not reachable in production (research feature). Raw text: crates/bpqs/src/wots.rs:26-32, 63-69, 137-148; merkle.rs:104-125. The verifier ignores the tail bytes 24..32 of each lane (level L3), unused chains and unused path entries, so signatures are malleable. With debug assertions on, a non-zero tail panics in lane() (wots.rs:27). Fuzz covers only the N=32 row. Malleability is documented (SECURITY-ARGUMENT.md:41-42); the debug panic is not. Not wired to production.
- F07 Low-Medium. CONFIRMED (V06). Two functions: KeyPair::save (677-689) and ValidatorKeys::save (505-518), both create+truncate+mode 0o600, no chmod, temp file, rename or fsync. Callers main.rs:362 and main.rs:459. New F07-a: keygen has no existence check and no create_new, so a run into an existing path destroys the old key silently (main.rs:341-362, 452-459). Raw text: src/crypto/primitives.rs:505-518, 677-689. KeyPair::save opens with create, truncate and mode 0o600. The mode applies only to a new file, so an old file with wider rights keeps them. Write is truncate then write with no temp file, rename or fsync, so a crash can leave an empty key file. The test (hardening.rs:261-310) covers only a new file. Entry: main.rs:459, 362.
- F08 Medium (test). src/crypto/mainnet_policy.rs:119-124, 90-136. The SoftHSM module-path rejection (substring match on "softhsm") and the "mock" alias have no test. A renamed or symlinked library passes the substring check.
- F09 Medium (test). bpqs sign.rs:186-210, tests. Missing negative tests: wrong public key, params_name mismatch (192-196), epoch at or above 2^T_LOG2 (204-206), tail tamper, epoch_of overflow, count u32::MAX. KAT covers only the test-fast row; the L5 test is ignored (tests.rs:103).
- F10 Low. primitives.rs:525-582, 342-359. A key file with 1 to 127 BLS bytes silently loses its BLS key; extra trailing bytes are ignored. The PQ from_bytes checks only lengths, not that public and secret halves match. A zero BLS scalar is accepted (verify_pop rejects the identity point, finality.rs:418).
- F11 Low. primitives.rs:84-95, 204-208, 480-499; pkcs11.rs:22-23; bpqs sign.rs:86-91. No zeroize or Drop anywhere; secrets and the save buffer stay in memory; the types are Clone.
- F12 Low (Likely). pkcs11.rs:13, 15, 17, 31, 58-63; primitives.rs:863. dead_code allows on fields that are written and never read; `_load_software_fallback` is unused and new() equals new_without_software_fallback(). Add date of the attributes unknown (shallow history).
- F13 Low. src/crypto/signer.rs:10-15. The default sign_prevote and sign_precommit have no domain label and sign the same hash as sign_block with the same Ed25519 key. No production caller today (production uses BLS and the domain tags).
- F14 Low. pkcs11.rs:278-288, 333-346. A public key longer than 32 bytes is cut to 32; a signature longer than 64 is cut. The start-up binding check catches a wrong key, but signatures are not verified per call.
- F15 Low. bpqs epoch.rs:26-31, 39-41. `(height / window) as u32` truncates; a wrapped epoch below 2^16 is accepted. Needs height/W of at least 2^32.
- F16 Low. bpqs sign.rs:143-148. count == u32::MAX makes `+ 1` overflow; release has overflow-checks, so the process aborts, against the "never panics" statement.
- F17 Low (test). primitives.rs:1019-1058; bpqs tests.rs:48-56. A test named "does not leak via println" does not capture stdout; a fixed /tmp path is shared; the wire size test computes from constants (2660) while SECURITY-ARGUMENT.md says 2676.
- F18 Info. primitives.rs:795-821; pkcs11.rs:234-255, 399-410; signer.rs:62-69. Several public items have no production caller. Known: dead-pub-api-baseline.txt:125-134, idle-code-baseline.txt:1429-1459.
- F19 Info (Likely). bpqs wots.rs:52-61. The chain step hash takes only the chain number (no epoch, member or step index, no public seed). Multi-target analysis not read.
- F20 Info. primitives.rs:875-882. WalletKeyPair::sign falls back silently to deterministic signing when the RNG fails; the verifier path returns an error instead.
- F21 Info. Bold markers in comments (Z13); epoch.rs says q_max 4 while params.rs:36 sets Q_MAX 1.

## Part 4. Raw findings P03 (tokenomics, privacy, settlement)

Scope: all files in the part read; cold_wallet.rs lines 950-1004 and 1056-1159 (tests only) not read. Clean: no unsafe or float, no HashMap (BTreeSet and Vec used), supply balance checked at genesis (mod.rs:355-373, genesis.rs:344), vesting uses saturating math, cold_quorum uses verify_strict.

- F01 Medium. CONFIRMED (V05), reachable on Devnet and Testnet only, blocks mainnet privacy. Stronger form S1: even with proofs wired, the proof is a free input; derive_nullifier(c, p) is SHA-256 of c and p (note_registry.rs:227-233), so one commitment gets a fresh valid nullifier per p. insert_note keeps no record of spent commitments (79-96). Raw text: src/privacy/note_registry.rs:104-111, 163-173; execution/executor.rs:2023-2085. The nullifier binding check runs only when proofs are passed (163) and production passes none (executor.rs:2077). A nullifier is chosen by the spender and not bound to the commitment. A spent commitment can be made live again with PrivacyNoteInsert and spent again. Precondition: Devnet or Testnet only (allowlist executor.rs:63-80). Notes are not tied to balances, so no mint. Known: executor.rs:63-80, STATUS 4.7, idle-code-baseline.txt:1743, 1745.
- F02 Medium. CONFIRMED (V05), DoS on testnet, gated on mainnet. apply_transfer_with_proofs inserts outputs with no cap (207-209); worst case about 3.9M live notes. Raw text: note_registry.rs:79-96, 192-207, 243-247; core/account.rs:2456-2458. MAX_LIVE_COMMITMENTS applies only to insert_note; transfers add outputs without a limit (1 input, 16 outputs gives +15 notes). The spent nullifier set cannot shrink (prune_spent_nullifiers is empty); when full, the feature is closed for good. state_root rehashes the whole set. No test for the transfer path limit.
- F03 Low. note_registry.rs:136-141, 200-204; submit.rs:33-86. No duplicate check inside the nullifier list: [n, n] spends two notes and records one nullifier.
- F04 Low. note_registry.rs:83-85, 113-115, 225-233; submit.rs:57-64. An empty defensive if; doc says Poseidon but the code uses SHA-256; its output would fail the field-element shape check.
- F05 Low today, Medium once wired with escrow. CONFIRMED (V05), latent (only close_epoch is called outside settlement). Raw text: src/settlement/proof_market.rs:296-316, 634-650, 695-697. task_id has no nonce; duplicate check covers active tasks only; a completed task is removed and the same fields are accepted again, giving a second receipt. No production caller yet (F07).
- New from V05: S2 cold wallet: the trusted device set is passed in by the caller, not stored in policy or state (cold_wallet.rs:703, pq_anchor.rs:641); any caller can present its own identities and pass the quorum. Latent, Medium when wired. S3: a rotation only bumps key_epoch (cold_wallet.rs:752-756); no key material changes and the anchor keybook stays as it was.
- F06 Low. proof_market.rs:331-368, 634-650. add_task accepts a task already in Assigned state (skips the min_prover_stake check) and silently drops Completed or Expired tasks with Ok.
- F07 Medium (integration). proof_market.rs:594-785; core/account.rs:1570. The only production trace is close_epoch. add_task, assign, complete_task and pay_receipt have no caller; pay_receipt does not touch balances and there is no escrow. Known: idle-code-baseline.txt:1834-1853.
- F08 Low. proof_market.rs:702-715. mark_paid() runs before a checked_add that can fail; the receipt stays Paid while the total does not change.
- F09 Low. proof_market.rs:559-568, 681-683, 728-732, 776-784. complete_task refuses at the cap, so the drop-oldest branch is unreachable; unpaid and revoked receipts are never cleared; at 10000 receipts the market is jammed.
- F10 Low. proof_market.rs:787-813. verified_epoch is not in the state root.
- F11 Medium, latent. CONFIRMED (V05). sign_with_quorum signs the epoch so an honest quorum limits it; the public sign() path (F14) does not. Raw text: src/settlement/cold_wallet.rs:498-513, 614-619, 635-653. request.epoch comes from the request; the only rule is not to go backwards. A request that raises the epoch resets the epoch budget. Precondition: the online side is compromised (the module own threat model). Entry: cold_wallet.rs:699 and 485. Not wired to production.
- F12 Low, latent. CONFIRMED with narrower impact (V05): same-chain replay is blocked by key_epoch; cross-chain needs shared device keys and equal key_epoch; effect is a forced epoch bump. Raw text: src/settlement/pq_anchor.rs:639-668. The rotation payload has no chain_id (647-653); the cold wallet settlement payload has one (cold_wallet.rs:451). With the same committee devices on two networks, a rotation certificate could replay. Missing step: confirm the committees are shared.
- F13 Medium, latent. CONFIRMED (V05). Raw text: pq_anchor.rs:117-134, 426-493, 664-667. The key book has no epoch; verify_anchor_full never compares key_epoch with the current epoch; rotation does not remove old keys from the key book. After a rotation an anchor signed by an old key still verifies until the key book is edited by hand. Not wired to production.
- F14 Low. cold_wallet.rs:485-536, 742-766. sign and rotate_key are public and ask for no proof; only the quorum paths do.
- F15 Low. cold_wallet.rs:677-718. On an epoch mismatch the refusal is recorded before any signature is checked, so unsigned calls can push real refusals out of the fixed-size log (doc 681-686 says they must not).
- F16 Low. src/settlement/cold_quorum.rs:282-355. Two device ids with the same verifying key count as two devices; identities.len() is not compared with device_count.
- F17 Low. cold_quorum.rs:46-59, 148-170; settlement/mod.rs:9-12. dev_fixture_device_signers is public and not cfg(test); its seed derives from a visible constant pattern. verify_quorum does not reject those identities.
- F18 Low. pq_anchor.rs:426-458. The signature list is unbounded; a repeated valid signature costs N ML-DSA-87 checks.
- F19 Medium (integration). cold_wallet.rs, cold_quorum.rs, pq_anchor.rs have no production caller outside the settlement module. Known: STATUS 4.7.
- F20 Medium. CONFIRMED (V05), reachable through operator genesis. validate_tokenomics_supply (genesis.rs:342-374) checks no ratio against FIXED_POINT_SCALE; a ratio above the scale makes executor.rs:2546-2549 return fee_underflow and every block with a tx is refused (halt). Ratio is set only at genesis (genesis.rs:617). Supply invariant holds (fee burn at most the fee; timed burn clipped to the reserve, account.rs:2087). mainnet_genesis values are in range (genesis.rs:695-708). Raw text: src/tokenomics/mod.rs:279-290; chain/genesis.rs:342-373; execution/executor.rs:2546-2552. tx_fee_burn_ratio_fixed, annual_burn_ratio_fixed and validator_annual_yield_ratio_fixed have no upper bound at FIXED_POINT_SCALE (validate_tokenomics_supply does not check). A ratio above the scale makes burn larger than fee, and the executor rejects every block with transactions with fee_underflow (executor.rs:2547): a network halt. `as u64` also truncates (mod.rs:286-289). Trigger: the operator genesis, not an attacker. Compare slash_penalty, which has a cap (chain_config.rs:265-300).
- F21 Low. tokenomics/mod.rs:243-277; reward_pool.rs:1-116. calculate_epoch_reward returns u64::MAX on overflow; epochs_per_year of 0 has three different meanings; the reward pool is unwired. Known: baselines and STATUS 4.7.
- F22 Info. reward_pool.rs:72-110. A duplicate validator address gets two shares; the total stays within budget.
- F23 Low (test). src/tests/tokenomics_proptest.rs:27-122. The supply test cannot fail with its input range; the burn test does not check the amount; parameters are not swept; nothing catches F20.
- F24 Low. src/settlement/proof_verifier.rs:67-117; cross_domain/event_tree.rs:46-62. leaf_hash covers only message_id; VerifiedDomainEvent does not check that the body matches the id (bridge.rs:437 does); expected_block_hash None skips the block hash check; the only production call passes Some.
- F25 Info. Roots in this part are pure and deterministic; live and replay share one function (blockchain.rs:1915-1918); the five entry points in STATUS 4.3 were not re-checked.

## Part 5. Raw findings P04 (registry)

Scope: all 14 files read in full. Clean: no unsafe, float or secrets; registry maps are BTreeMap or BTreeSet except poa_compliance.rs:116-117; slash_penalty uses u128.

- F01 Critical. CONFIRMED by a new architect call (V03, 2026-10-09). src/registry/invalid_vote.rs:66-107; chain/blockchain.rs:5789-5813, 5840-5860; network/node.rs:2566; core/account.rs:2500-2502; blockchain.rs:4955-4960. An invalid BLS vote is counted against the voter_id in the message. voter_id and epoch come from gossip and the sender is not authenticated. Anyone can make an invalid vote for a victim. The counters and registry changes are made outside blocks, and the invalid_votes root is part of state_root (account.rs:2500), so one fake vote makes that node reject honest blocks with "State root mismatch". Above the threshold (default 20) the victim registry entry is cut by 100 percent and the account relayer, prover, storage operator and AI operator bonds are zeroed (permissionless.rs:659-688). Entry: node.rs:2566, then chain_actor.rs:3504, then blockchain.rs:5789. A per-peer vote rate limit exists (peer_manager.rs:399); burst size not measured. Known class: STATUS 4.4. Needs Opus verification.
  V03 notes: the BLS check runs against the victim key, so a garbage signature counts against the victim (blockchain.rs:5796-5812, same gate in handle_precommit 5840-5860). The counter ticks only if the victim is in state.validators with a non-empty BLS key. Threshold default 20 (params.rs:336); spam maps to MaliciousBehaviour with ratio 100 percent (evidence.rs:117, params.rs:308). Correction: the validators map stake is not cut, only registry entries. An invalid vote does not lower the peer score (node.rs:2587-2589). Open (S4): not checked whether a registry-only Slashed status removes the validator from quorum.
- F02 High. CONFIRMED (V03). src/registry/permissionless.rs:480-515, 690-737; blockchain.rs:3464-3560; executor.rs:401-452. Report-based slashing (finality equivocation, invalid vote, liveness) cuts only the PermissionlessRegistry entry. It does not touch the validators map that consensus reads. Unstake does not check slashed or jailed. A later Stake or Unstake overwrites the cut stake with the uncut value (490-492). The only consistent penalty paths are governance SlashValidator (account.rs:1698) and the QC fault path (blockchain.rs:3869).
- F03 High. CONFIRMED (V03). No src path (tests excluded) slashes unbonding_queue.
  (raw text:) permissionless.rs:494-496; executor.rs:401-441; account.rs:1422-1440. After a full Unstake the validator stake is zero, the registry entry is removed and the amount moves to the unbonding queue. A later equivocation report returns NotRegistered (permissionless.rs:715); slash_validator cuts only validator.stake. A validator can unstake fully before the proof arrives and leave unpunished.
- F04 Medium (was High). PARTLY confirmed (V03). Overwrite and debit-without-entry hold. bond_storage_operator has no production caller (only rpc/tests.rs:1146); reachable paths are the relayer RPC (server.rs:2941) and prover RPC (server.rs:2972). Needs operator rights.
  (raw text:) permissionless.rs:480-515; account.rs:683-781; chain_actor.rs:3714-3722; rpc/server.rs:2933-2946. bond_relayer, bond_prover and bond_storage_operator subtract the balance first (account.rs:705) and pass only the added amount to upsert_stake (707-710). An existing entry gets its stake overwritten with the new amount (the old bond vanishes). With no entry and an amount below min_stake, no entry is made but the balance is still debited. Precondition: operator RPC rights (server.rs:2941). Entry: bud_registryBondRelayer and the prover and storage commands.
- New from V03 (not yet traced): S1 bond RPCs change balance and registry outside any block (chain_actor.rs:3714-3722), same root-divergence class as F01. S2 bond RPC takes any address with no signature (server.rs:2941-2946). S3 Stake sets v.active without checking jailed (executor.rs:245); get_active_validators filters only !slashed (account.rs:1003).
- F05 Medium. permissionless.rs:531-589, 644-688; params.rs:15-29; account.rs:1589-1596; blockchain.rs:2856-2860, 4405-4425. No exit from the Slashed state: begin_unbonding needs Active, withdraw needs Unbonding, re-register gives AlreadyRegistered; rg "slashed = false" is empty. The liveness penalty sets jail_until and also slashed=true, and the release loop needs !slashed, so the documented timed jail does not work. Remaining stake is locked for good.
- F06 Medium. invalid_vote.rs:80-88. The epoch of a vote has no upper bound (blockchain.rs:5809). epoch=u64::MAX moves current_epoch forward; later real epochs count as old and are ignored; counters are cleared. The spam guard is closed for good.
- F07 Medium. poa_onboarding.rs:200-365; poa_membership.rs:121-217; consensus/poa.rs:149-158. The write side (add_admin, submit_application, approve, reject, revoke, renew_kyc) has no production caller (only poa.rs:626 inside cfg(test)). No domain can get an admin, so poa_is_permissioned is always false. Same root as P01-F07. Known: idle-code-baseline.txt:1795-1806.
- F08 Medium. core/account.rs:2256-2532; chain/snapshot.rs:896-966, 2121-2125. poa_onboarding is consensus gate state but is in neither the state root nor the snapshot digest. snapshot.rs:2121 notes a field added with serde default and no version bump. Hidden today; opens when the write path is wired.
- F09 Medium. registry/poa_compliance.rs:114-264; blockchain.rs:1069-1135. freeze_poa_account takes a bool for authority (no admin address or signature) and has no production caller (only src/tests/poa_isolation.rs). is_frozen is always false; the record is not persisted, not in the root and has no unfreeze; maps are HashMap. The frozen-operator rejection at blockchain.rs:1088 is dead.
- F10 Medium. permissionless.rs:364-448, 807-821; blockchain.rs:1771-1785. Roles VERIFIER, MASTER_VERIFIER, AI_VERIFIER, ATTESTER and CONTENT_VALIDATOR have no production registration path. ATTESTER total stake is always 0, so the StorageAttestation authority check (blockchain.rs:1775-1776) never runs in production.
- F11 Medium. poa_onboarding.rs:319-365; poa_membership.rs:147-172. renew_kyc checks only that the commitment is non-zero and writes it nowhere; submit_application returns AlreadyApplied for a member, so the commitment cannot be updated. Known: idle-code-baseline.txt:1803.
- F12 Low. poa_onboarding.rs:239-260, 403-429; poa_membership.rs:124-126. logged_expiry is never cleared (no second expiry entry after renewal); validity_horizon has no upper bound; there is no admin removal path.
- F13 Medium. registry/identity.rs:571-622 (601-619), 966-974, 774. Issue needs only sender equal to issuer and no consent from the subject. Any registered DID can overwrite credential_root in another record; is_credential_valid accepts only the latest root. issuer_known looks at the subject own method key_id, so an unregistered issuer passes. Entry: executor.rs:2426.
- F14 Medium. identity.rs:713-745, 786-793. Only the credential id enters the state root (root, issuer, issued_at). Subject, schema and expires_at are outside it, yet validity reads them (762).
- F15 Medium. registry/identity_fill.rs:333-381, 508-551; chain_actor.rs:4872-4893. The presentation receipt is unsigned. requester, document_digest and epoch can be changed to make a new valid receipt; epoch comes from the receipt and sets the expiry check. PresentationReceipt::digest is never used.
- F16 Medium (Likely). identity_fill.rs:523-549; chain_actor.rs:4872-4893. entries is unbounded and credential.root() is computed twice per entry inside the chain actor. Body limit is 10 MB (main.rs:1287). Missing step: RPC load test.
- F17 Medium (test). identity.rs:1427-1474; src/tests/identity_tx_door.rs:167-215; permissionless.rs:1364-1386; params.rs:441-487. No success test for Recover with a valid ML-DSA signature; no tests for third-party Issue (F13), bond on bond (F04), equivocation after Unstake (F03), bad epoch (F06) or a second expiry after renewal (F12); identity.rs:1473 discards its result; two tests scan their own source file as text.
- F18 Low. registry/evidence.rs:122-135, 228-251, 431-486. validate_shape has no byte limits (the sibling budzero/verifier-registry/src/evidence.rs:90-93 has them). Provenance is a public field, so ConsensusVerified can be stamped by any caller.
- F19 Low. registry/params.rs:242-287. min_stake, bridge_relayer_min_fee and ai_model_register_fee have no upper bound; a governance proposal can set min_stake to u64::MAX.
- F20 Low. params.rs:36-53. Persisted structures have no version field (accepted in the docs); the comment at params.rs:371 says 15 u64 fields, the real count is 16.
- F21 Low. identity.rs:722-725; identity_fill.rs:375-376. usize.to_le_bytes() goes into a hash input, so the width depends on the target.
- F22 Low. identity.rs, identity_fill.rs, evidence.rs. Wallet helpers (build_presentation, fill_template), credential_*_digest and consensus_invalid_relay_* have no production caller. Partly known: idle-code-baseline.txt:1756-1762.
- F23 Low. registry/quarantine_ledger.rs:53-116; blockchain.rs:1496-1509. The ledger is never trimmed and the whole ledger is rewritten on each event.
- F24 Info. registry/evidence.rs:23-48; registry/mod.rs:1; rpc/server.rs:3136; blockchain.rs:3490. The RPC bud_submitSlashingReport forces provenance to Unverified and submit_registry_slashing_report refuses Unverified, so the endpoint always fails. Docs describe a promotion flow that is test-only. mod.rs:1 uses the banned term (Z12).
- F25 Info. evidence.rs:254, poa_onboarding.rs:464: two allow attributes whose add date is unknown (shallow history).

## Part 6. P62 repo-wide invariant scan (raw, then summarized)

- Z3: 104 raw hits of allow or ignore. Real attributes outside xtask: 88 allow and 2 ignore; inside xtask: 3 allow. Every attribute inside the gate scan area (src, benches, budzero, build.rs) matches a budget entry. No CI softening pattern found (continue-on-error, `|| true`, `; true`, `|| exit 0`, `set +e` outside comments: 0).
- P62-F01 Medium (Z6). .github/workflows/security-hardening.yml:334-377 and its calls at 384-423 (a Python gate written through a heredoc), and provenance.yml:69-83 (python3 heredoc). Python code lives inside yml.
- P62-F02 Info (Z6). determinism.yml:108 uses `shell: pwsh`; 219 multi-line bash blocks. Whether Z6 covers workflow yml is not stated in the directive.
- P62-F03 Medium (Z3). Six allow attributes outside the area the suppression gate scans (it scans src, benches, budzero, build.rs): crates/budscan/src/query.rs:110; crates/ai-inference/crates/ai-ops/src/logparse.rs:19 (crate-wide dead_code allow; known partly in idle-code-baseline.txt:948-955); bud/src/bud_format_realcorpus.rs:13; xtask/gates/src/gates/ast_security_gates.rs:297 and 483; budscan_patchset.rs:177.
- P62-F04 Low (Z3). crates/bpqs/src/tests.rs:103 and tests_backends.rs:308 carry ignore attributes that no workflow runs.
- P62-F05 Medium (Z12). src/bin/bud.rs:38 (and lines 1, 3, 9): CLI help text uses the banned term.
- P62-F06 Medium (Z12). Public docs use the banned term: docs/CONTRIBUTING.md:5, docs/SECURITY.md:3, docs/NOTICE:9, LICENSE.md:95, crates/ai-inference/README.md:4, budzero/README.md (19, 29, 86, 139), budzero/ARCHITECTURE.md:3, 20 lines in 9 files of budzero/docs, budzero/verifier-registry/Cargo.toml:7, and about 20 rustdoc comments (for example src/rpc/api.rs:434). Changing NOTICE or LICENSE text needs an owner decision.
- Z7: no real unsafe use (src/lib.rs:3 has forbid(unsafe_code)). Z8: no float on consensus paths (f64 only in the AI ranking RPC type, chain_actor.rs:331, 2364). Z9: no secret found (token-like strings are test data). Label 12.9: no hit.
- Dead code notes (not verified): src/chain/blockchain.rs:946 and 972, src/tests/bench_performance.rs:14-160, src/account_abstraction/quantum_account.rs:156 (no caller found). src/network/node.rs:1611 has an allow(single_match) on a let statement.
- Method note: the clone is shallow (50 commits), so the commit that added each attribute could not be found.

## Part 7. Raw findings P05 (cross-domain core and EVM parsing)

Scope: all 15 files read in full (6078 lines), plus small call-site spans in other parts. Scan by coder-deep (Sonnet). Not verified. Clean checks: replay store (nonce.rs) is monotonic and BTreeMap only; message id binds all fields; mint payload and status gates hold; RLP length arithmetic and depth cap, MPT depth cap, header checked_add have no panic or overflow path. No f32/f64, unsafe or L1 term in these files. Sync committee, adapter.rs, verify.rs and external/* are P06.

- F01 High (Likely). bridge.rs:529-585 (burn_with_event); blockchain.rs:2322-2349; executor.rs:1191, 1265. Mint credits the recipient with new BUD (try_mint_balance, executor.rs:1191). Burn only sets status Burned (bridge.rs:581-583) and debits nothing. Unlock credits the owner with try_add_balance (executor.rs:1265). Per lock, mint, burn, unlock cycle total balances grow by about one transfer amount, which breaks the 100M supply and the double burn invariant. Operator gated (server.rs:2770). Missing step: a supply-conservation test; no debit was found by reading. src/tests/bridge_lifecycle.rs:80 has no supply assertion.
- F02 High (Certain). bridge.rs:529-585, 721-749; relayer.rs:296. A Burned transfer leaves Burned only through unlock, which needs a relayed burn message. If that message expires unrelayed, process_relay refuses it for good; sweep refunds only Locked (bridge.rs:736). The source-side lock stays locked forever and the row is never pruned. burn_with_event takes expiry_height from the caller with no check against domain height (bridge.rs:541).
- F03 Medium (Likely). evm/mpt.rs:181-194; evm/verify.rs:266. verify always walks keccak256(key) as the path (secure trie). Ethereum receiptsRoot is a plain trie keyed by rlp(tx_index). Real receipt proofs cannot verify (fails closed). The test fixture matches the code, not a real trie (real_chain_fixtures.rs checks only header hashes). Missing step: run mpt::verify on a real receipts trie.
- F04 Medium (High if an on-chain path calls verify_evm_receipt). evm/header.rs:117-158; adapter.rs:253; verify.rs:288-312. verify_chain checks only parent hash links and number+1: no proof of work, no signature, no trusted checkpoint. The sync committee check is optional and the adapter passes None. The only caller chain is relayer/worker.rs:754 (off-chain self check); the on-chain gate is executor.rs:1072 (state.external_roots) and does not use this module.
- F05 Medium (Certain). relayer.rs:333-352; blockchain.rs:3270-3320. process_relay writes the ledger record before the message presence and id checks, so an error leaves the record and the pending entry; every retry returns AlreadyRelayed. The caller persists relayer state (blockchain.rs:3280) before the bridge mint or unlock and does not roll back when fee split, mint headroom or mint fails (3315). A relay can be consumed with no mint. Known class: STATUS 4.7.
- F06 Medium (Certain). message.rs:44-59; message_registry.rs:20-32; blockchain.rs:2878-2883. The registry authenticates only that the id matches the fields. The relayer gate checks the self-declared sender field, not a signer (server.rs:2670 and node.rs:2866 feed an unauthenticated message into it). Anyone can name an active relayer as sender and register arbitrary messages, or pre-register a predictable lock message id. Known class: 4.7 and Q2; the sender-spoof detail is not recorded there.
- F07 Medium (Certain). message_registry.rs:8-67; message.rs:14, 35-39. Messages are never removed; root() hashes all of them and is called at blockchain.rs:867, 1916, 1950; Custom(Vec<u8>) has no length bound. Per-block cost and state grow without limit. Known class: 4.7; no pruning recorded.
- F08 Medium (Certain, integration). relayer.rs:1-15, 176-197, 372-378. Slashing for a missed relay and the config fields relay_window_blocks, min_relayer_stake, slash_ratio_invalid, slash_ratio_expired have no use outside relayer.rs and tests. Blockchain::expired_relays (blockchain.rs:3426) has no caller. Known: idle-code-baseline.txt:1424-1428 (the config fields are not listed).
- F09 Medium (Certain for not wired). evm/bud_to_eth.rs:25-137. The claim takes recipient_eth as a free parameter; the burn message carries no Ethereum address, so nothing binds the recipient to the burn. The same Burned transfer can also be unlocked on the source side (see F01). No caller outside the file. The Ethereum contract is not in the repo. Known: dead-pub-api-baseline.txt:120; idle-code-baseline.txt:1402-1405.
- F10 Medium (Certain, integration). bridge_relayer.rs:81-335. The file says WIRING unwired; BridgeRelayerPipeline is only re-exported (mod.rs:13). Production does the same steps in blockchain.rs without the rollback. lock() and burn() clone the whole BridgeState per call (lines 144, 250). Known: STATUS 4.7; idle-code-baseline.txt:1395-1397.
- F11 Low (Certain for the order). blockchain.rs:2252-2267, 2333-2348. The production lock path debits and mutates bridge state first and registers the message after (2258, 2266); on a registry refusal the debit, row, nonce and expiry entry stay and no relay is queued. The pipeline version rolls back (bridge_relayer.rs:137-143). With F06 an attacker may force the refusal. Burn path has the same order.
- F12 Low (Certain). relayer.rs:296 against bridge.rs:727. At height == expiry both relay and refund are legal (relay valid while height <= expiry; sweep uses ..=height). Mint never reads message.expiry_height (bridge.rs:432-502). Two nodes can order them differently. No double credit (status gated).
- F13 Low (Certain, documented design). nonce.rs:29-34, 99-116; bridge.rs:458-485. A relayer can relay a later lock first; the earlier lock can then never mint and waits for expiry (BridgeState::lock allows expiry 0, then it never expires; production forces expiry > source height, blockchain.rs:2223).
- F14 Low (Certain). message.rs:88-105. correlation_id None and Some([0;32]) give the same message id (unwrap_or zero); same class as the header fix in core/hash.rs:22-28. No exploit found.
- F15 Low (Certain). evm/rlp.rs:162-166, 258-282. Non-canonical forms are accepted: a one byte string below 0x80 in 0x81 form, and integer zero as 0x00; the doc (rlp.rs:19-21) says they are refused. Hash binding means no forgery.
- F16 Low (Likely). evm/rlp.rs:138-236. No cap on item count or total nodes in decode(); each input byte can become an Item plus a heap Vec. The only limit is the caller's (adapter.rs:47, 1 MiB). Memory amplification. Missing step: measure peak memory.
- F17 Low (Certain). evm/mpt.rs:150-171, 249-268, 27-47. hp_decode does not check the low nibble of an even path; any non-32 byte string is decoded as an inline node; the doc overstates the reachability of the empty-extension cycle (node_map keys are keccak of node bytes, so a hash cycle is not reachable through verify).
- F18 Low (Certain). evm/receipt.rs:70-76, 97-104, 148-155. Type byte never checked; any nonzero status byte is success; find_log returns only the first match and no log index, so (tx, log index) cannot be bound as a deposit id. Known: idle-code-baseline.txt:1412 (EthLog).
- F19 Low (Medium for R3 paths) (test). mpt.rs:556-572; bud_to_eth.rs:144-188; rlp.rs:407-447; receipt.rs:158-328; bridge.rs:824-1141. Missing negative tests (rlp NestingTooDeep and non-canonical forms, receipt trailing bytes and bad status, bridge-level out-of-order nonce, mint after expiry, supply conservation, relayer half-state). mpt garbage test has no assert and ends at MissingNode for every case; bud_to_eth tests FinalityProofMissing, NotBurned and AmountExceedsCap are never exercised.
- F20 Low (dead code). Variants nothing builds: MptError::InvalidNodeRef, BudToEthError::InvalidRecipient, RelayerError::NotActiveRelayer, RelayerError::InvalidTransferState, DomainEventKind::MessageEmitted and BridgeUnlocked. Other public items are in the baselines (dead-pub-api-baseline.txt:120-124, idle-code-baseline.txt:1395-1428).
- F21 Info. Allow attributes: bridge.rs:349, bridge_relayer.rs:123, bud_to_eth.rs:1 and 91 (add date unknown, shallow history). Z13: non-English text (receipt.rs:21, 71; mpt.rs:126 and others; rlp.rs:102, 449) and bold markers (21 matches). relayer.rs:6 names Blockchain::submit_relay, which does not exist (only submit_relay_proof).
- F22 Info. Merkle proof index high bits are ignored (merkle_tree.rs:255-260), so one proof verifies with many index values and ledger proof_hash (relayer.rs:333-335) depends on which variant a node got. enqueue_relay stores source_event without checking it equals the message (relayer.rs:244-252). Default verify_observation takes its root from the same result (chain_adapter.rs:142).

## Verification V07 (P05 F01 to F04, main session, 2026-10-11)

Method: main session read the cited spans. No agent. Not an Opus call; a second check by a fresh architect is still open for F01.

- P05-F01 CONFIRMED, High. blockchain.rs:2252-2260 debits the owner at lock. executor.rs:1191-1198 mints the recipient at mint (lock and mint are balanced). bridge.rs:529-585 burn only sets Burned, no debit. executor.rs:1265 and the RPC unlock path credit the owner again. Net per cycle: plus one transfer amount. Test settlement_prod.rs:1840-1870 runs lock, burn, unlock and asserts no supply. Fix direction: debit the recipient at burn (balance check first) or hold the minted amount in escrow; test first: total supply before equals after a full cycle. Needs an owner decision on the model (STATUS 5).
- P05-F02 CONFIRMED, High. bridge.rs:736 sweep releases only Locked. A Burned row whose burn message expires unrelayed has no exit.
- P05-F03 CONFIRMED, Medium. mpt.rs:187 uses keccak256(key) as path for every proof. Ethereum receiptsRoot is a plain trie keyed by rlp(index). Fails closed.
- P05-F04 CONFIRMED, Medium. header.rs:131-158 checks parent hash and number only. Severity stays Medium while no on-chain path calls it.

## Verification V08 (P05 F05 to F12, architect agent, 2026-10-11)

Full text: notes/V08-P05-F05-F12.md. F05, F06, F07, F08, F09, F10, F12 CONFIRMED. F11 CONFIRMED for the order, PARTLY reachable (only the burn half runs in production, operator RPC).
Reachable today: F05 (operator RPC), F06 and F07 (public RPC and gossip, no gate), F11 burn half. F08, F09, F10 are idle code.

- P05-F06a High (Likely, main session checked the missing step). Any peer can push messages into the message registry outside block execution. The registry root feeds the state root (account.rs:2450, message_root). Block validation compares the state root (blockchain.rs:4960, 5179, 5389). Nodes with different registry content then reject the same block: a consensus split. Known class: STATUS 4.5 (consensus state written outside blocks), now with a concrete entry point.
- P05-F05a Medium. The relay path changes balances outside blocks on one node.
- P05-F11a Low. Bridge state is saved to disk before the message is accepted.

## Part 8. Raw findings P06 (EVM verification, external proof)

Scan by general-purpose agent (Sonnet, medium), 2026-10-11. Not verified. About 1100 lines of hex test data in sync_committee.rs skimmed by shape only. Full text copied from notes/P06-scan.md.

# P06 scan (cross_domain evm adapter, sync_committee, verify; external domain_bridge, ethereum, evm_hybrid, prover, zkvm_proof, profile)

Scope read in full: src/cross_domain/evm/{adapter.rs 921, sync_committee.rs 1418, verify.rs 982}, src/cross_domain/external/{domain_bridge.rs 317, ethereum.rs 594, evm_hybrid.rs 545, prover.rs 249, zkvm_proof.rs 589, profile.rs 220}. The 1100 lines of hex test constants in sync_committee.rs (known-answer data) were skimmed by shape only.
Read-only. No cargo run. Excluded as already known: FINDINGS.md Part 7 P05-F03 (mpt.rs keccak path), P05-F04 (header.rs parent hash and number only), STATUS.md 4.5.
Counts: Critical 0, High 2, Medium 9, Low 4, Info 1 (16 findings, P06-F01 to P06-F16).

### P06-F01
File: src/cross_domain/external/ethereum.rs:393-406 (verify), :88-145 (layout); src/cross_domain/external/intake.rs:64-131 (PairingBls)
Severity: High
Axis: security
Confidence: Certain (code behaviour); the admission gate that decides who may register an EthereumSync domain is outside P06 and sets the precondition
Evidence: ethereum.rs:399-406
    verifier.verify_aggregate(&self.signing_root, &update.aggregate_pubkey, &update.signature, signers)
intake.rs:121-125 (PairingBls)
    let h_msg = crate::chain::finality::hash_to_g2(signing_root);
    ... multi_miller_loop(&[(&pk, &h_msg.into()), (&g1_gen_neg, &sig.into())])
ethereum.rs:141-143 `NEXT_COMMITTEE_ROOT` is parsed into `SyncCommitteeUpdate.next_committee_root` and read nowhere (rg -n next_committee_root src: only ethereum.rs parse and struct).
Description: The aggregate public key that the signature is checked against is a field of the submitted payload (layout::AGGREGATE_PUBKEY). Nothing ties it to a known sync committee (no committee root, no stored period state, no Merkle proof of membership). `participants` is only a count and is never used to rebuild the key from a committee, and PairingBls comments say committee membership "belongs to the adapter's layout", but the adapter does not check it.
Impact: Whoever can submit external evidence for an admitted EthereumSync domain generates their own BLS key pair, sets 342 or more bits in the bitmap, signs the adapter's fixed signing root with their own key and puts their own public key in the payload. The check passes. The attestation then carries any height and state_root. The path is submit_verified_domain_commitment -> verify_domain_commitment_finality (blockchain.rs:1582, 1660 ff) -> external-root anchor write (AR-GE-6). A forged anchor feeds the executor relayer gate.
Trigger: Precondition: a domain admitted with AdapterSpec::EthereumSync. Entry: Blockchain::submit_verified_domain_commitment (blockchain.rs:1576). Input: 376-byte payload with attacker key at bytes 88..136, attacker signature, 43 or more bytes of 0xff in the bitmap, chosen state_root at 344..376 and the same value in declared_root. Expected wrong result: FinalityStatus::Finalized for a state the Ethereum chain never had.
Known: none (STATUS.md 4.5 lists "three of four mainnet domains cannot finalize" only as a class, no detail)

### P06-F02
File: src/cross_domain/external/ethereum.rs:255-257, 299-306, 344-376, 399-425
Severity: High
Axis: security
Confidence: Certain
Evidence: ethereum.rs:254-257
    /// The signing root is supplied per call by the caller that owns the
    /// beacon chain's fork version and genesis validators root.
    signing_root: [u8; 32],
The adapter is built with `EthereumSyncAdapter::new(network, *signing_root, bls)` (intake.rs:177) from a constant stored in AdapterSpec::EthereumSync. `verify` passes `&self.signing_root` to the verifier, never a value derived from the update.
Description: The signed message is one constant per adapter. It is not derived from finalized_root, attested_slot, finalized_slot or state_root. No finality branch (finalized header inside the attested header's state) and no branch from finalized_root to state_root exist, so state_root (the value Budlum commits to) is a free payload field. `depth = attested_slot - finalized_slot` is also payload data and is not a depth of anything.
Impact: Even with a bound committee key (F01 fixed), one genuine signature over the constant signing root, once observed, validates every later payload with any state_root, slot and bitmap. Replaying a genuine Ethereum signature is enough. The comparison with verify.rs shows the right design: sync_committee.rs rebuilds the signing root from the beacon header fields and walks a branch (verify_execution_block_finality), the external adapter does neither.
Trigger: Precondition as F01 plus one signature observed (the mainnet data are public). Entry ethereum.rs:313 (verify). Input: payload with own state_root and slots, signature and key copied from the earlier valid update, bitmap with 342 set bits. Expected wrong result: attestation with attacker-chosen height and state_root.
Known: none

### P06-F03
File: src/cross_domain/external/ethereum.rs:361-425; src/cross_domain/external/spec.rs:315-324 (field), :335-342
Severity: Medium
Axis: security
Confidence: Certain for require_slashable and for the stored `now`; Likely for the age effect (missing step: confirm the value of `now` the admission path stores in entry.policy)
Evidence: rg -n 'require_slashable' src --glob '!src/tests/**' gives only spec.rs:324,341,354,363 (declaration and constructors). rg -n 'max_age' shows the only use is ethereum.rs:417-419. blockchain.rs builds the bridge with `entry.policy.clone()`, a stored value, and `VerificationPolicy::strict(now)` fixes `now` at construction.
Description: policy.require_slashable is never checked by any adapter or by the bridge, so a policy that refuses unslashable backing still accepts the sync-committee set (`slashable: false`, ethereum.rs:445). policy.now is a number frozen in the stored policy while the consensus path verifies at later heights, so `age = now - finalized_slot` (saturating) does not measure staleness at verification time, and max_age defaults to 0 in both constructors (check off).
Impact: A domain or operator relying on require_slashable or max_age gets no protection; an old but once-valid update is accepted.
Known: none

### P06-F04
File: src/cross_domain/external/domain_bridge.rs:108-160
Severity: Medium
Axis: security
Confidence: Likely (missing step: confirm in accept_domain_commitment and the consumers of event_root, tx_root and domain_block_hash that none of them re-binds these fields)
Evidence: domain_bridge.rs:138-148 compares only `attestation.height != commitment.domain_height` and `attestation.state_root != commitment.state_root`; line 154 compares the proof hash. For comparison finality_adapter.rs:252 `|| target.event_root != commitment.event_root` is used by the in-tree adapters.
Description: For an external domain only height and state_root are bound to the attestation. domain_block_hash, parent_domain_block_hash, tx_root, event_root, validator_set_hash, sequence, timestamp_ms, producer and state_updates of the commitment are free. blockchain.rs:1566 stores `com.domain_block_hash` as `last_committed_hash`. The ZK payload carries event_digest and state_writes_digest (inputs) that zkvm_proof.rs does not expose in the attestation and the bridge never compares.
Impact: A submitter who holds one valid evidence for (height, state_root) can attach any block hash, event_root and tx_root to it. Later message proofs or chain-link checks that rely on those fields rest on unattested data.
Trigger: Precondition: valid external evidence for the height. Entry domain_bridge.rs:108. Input: commitment with matching height and state_root and different domain_block_hash/event_root. Expected wrong result: Finalized.
Known: none

### P06-F05
File: src/cross_domain/external/zkvm_proof.rs:202-263
Severity: Medium
Axis: security
Confidence: Likely (missing step: whether bud-proof's canonical program set constrains block_height, final_state_root and chain_id so that a prover cannot choose them; that is P12 to P16 scope)
Evidence: zkvm_proof.rs:209-216 checks only declared_height == inputs.block_height and declared_root == inputs.final_state_root, then `bud_proof::DefaultAdapter::verify(&decoded.envelope, &decoded.inputs, &decoded.program)`. The attestation takes `height: decoded.inputs.block_height, state_root: decoded.inputs.final_state_root`. No use of inputs.chain_id, initial_state_root, sender, nonce, event_digest, state_writes_digest or program_hash against an expected program.
Description: The module header calls the three-way binding "the whole guarantee", but no field ties the proof to this network, to a pinned program, or to the previous attested root. Any canonical-set program run with chosen public inputs attests whatever height and root those inputs name. Continuity (initial_state_root equals the last attested root of this domain) is not checked.
Impact: If the AIR allows the public inputs to be chosen by the prover, the Trustless domain is forgeable by anyone, and an old height with a different root can be re-attested. If the AIR constrains them to the execution of a domain-specific program, this is only a missing program pin. Either way the code gives no domain-specific anchor.
Known: none

### P06-F06
File: src/cross_domain/evm/adapter.rs:153-172, 328-357, 221-272; src/relayer/worker.rs:735-754; src/execution/executor.rs:1027-1037
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: adapter.rs:166-171 `generate_receipt_proof` always returns Err; :332 `submit_transaction` always returns Err(SubmissionFailed); :343-348 `wait_for_confirmation` calls generate_receipt_proof and propagates the Err. worker.rs:735-754 calls `wait_for_confirmation` and only then `adapter.verify_observation(&result)`. executor.rs:1036 refuses Ethereum relay results with "relayer_evm_package_required". .github/idle-code-baseline.txt:1401 lists `verify_deposit` as idle; .github/unwired-guards-baseline.txt:43 lists it.
Description: With EvmChainAdapter registered through --evm-bridge-address, the worker cannot get past submit_transaction/wait_for_confirmation, so the verify_observation override, verify_deposit and verify_evm_receipt run only in tests. The consensus side refuses Ethereum results. The Ethereum deposit path therefore has no live end to end route.
Impact: The bridge leg Ethereum to Budlum does not operate; a reader of the comments ("the real safe path", "the production path") would believe it does. Mainnet blocker class.
Known: idle-code-baseline.txt (verify_deposit, SyncAttestation, test_default, DEFAULT_DEPOSIT_TOPIC0); unwired-guards-baseline.txt:43. The end-to-end unreachability is not stated in the baselines.

### P06-F07
File: src/cross_domain/evm/adapter.rs:244-254; src/cross_domain/evm/verify.rs:177, 192-204, 293-301
Severity: Medium
Axis: security
Confidence: Certain
Evidence: adapter.rs:253 `sync_attestation: None,` is the only construction of EvmDepositProof outside tests (rg -n 'EvmDepositProof \{' src). DepositProofPackage has no attestation field (verify.rs:193-204). rg -n 'SyncCommitteeState' src outside evm/ and tests gives nothing; `rotate_period` has no caller (dead-pub-api-baseline.txt:123).
Description: Header authenticity is never checked in the production verifier: the target and confirmation headers are accepted on parent hash and number alone (P05-F04 covers header.rs; this finding covers that the strong path that would repair it, verify_execution_block_finality, cannot be reached because the adapter hard-codes None and no state source exists). verify.rs:5-6 and the module text still say the chain is verified.
Impact: A relayer can fabricate an entire header chain of the required length with any receiptsRoot and a receipt MPT built under it; verify_observation returns Ok. Today closed by F06 (executor refusal), so Medium; it becomes High when the EVM leg is opened.
Trigger: Precondition: EvmChainAdapter registered and the executor refusal lifted. Entry adapter.rs:221. Input: DepositProofPackage with synthetic headers. Expected wrong result: a deposit accepted that never happened on Ethereum.
Known: related to P05-F04 (header.rs), not the same item.

### P06-F08
File: src/cross_domain/evm/sync_committee.rs:20-23, 410-439, 441-446
Severity: Medium
Axis: correctness
Confidence: Likely (missing step: check against the Ethereum consensus-spec light-client sync protocol, where finality requires finality_branch from the attested header's state to the finalized header; this repo carries no such branch)
Evidence: sync_committee.rs:20-21 "the sync committee ... gives real PoS finality"; verify_execution_block_finality checks only that the signed beacon header's body holds the execution block hash (branch gindex 812) and a pairing over that header. BeaconBinding has no finality branch and no check that the header is at or below the finalized checkpoint. rotate_period (:443) swaps `next_sync_committee` in with no proof that it is the chain's next committee.
Description: A sync aggregate signs the head of the chain; two thirds signing a head block does not make it final. The module treats a signed head as finalized. Committee trust root and rotation are also unproven: SyncCommitteeState is an unauthenticated input, and next_sync_committee is never verified by a branch to the finalized state.
Impact: A deposit block that is later reorged away can still carry a valid sync aggregate. Weaker than the "PoS finality" the documents and VerifyError::SyncCommittee text claim.
Known: none

### P06-F09
File: src/cross_domain/evm/sync_committee.rs:355-358, 430-436, 281-292
Severity: Low
Axis: correctness
Confidence: Likely (missing step: confirm with the Altair spec that the committee is chosen by the signature slot's period, which is the attested slot plus one)
Evidence: sync_committee.rs:356 `self.slot / SLOTS_PER_SYNC_COMMITTEE_PERIOD` with binding.slot being the signed (attested) header slot; :431 compares it to state.current_period.
Description: The committee that signs is the one in force at the signature slot (the block after the signed header). For the last slot of a period the code selects the wrong period and refuses a genuine aggregate. BeaconChainParams has one fork_version; the version for the signed slot changes at each fork, so a fixed value refuses every aggregate after a fork until the config is changed.
Impact: Fail closed, liveness only, at period and fork boundaries.
Known: none

### P06-F10
File: src/cross_domain/evm/verify.rs:245-314; src/cross_domain/evm/adapter.rs:255-272; src/cross_domain/evm/receipt.rs:151-155 (find_log)
Severity: Medium
Axis: correctness
Confidence: Certain for the code facts; the impact depends on the future consumer of VerifiedDeposit (none exists, see F06)
Evidence: rg -n 'PayloadMismatch|deposit_log_data' src: the variant is declared (verify.rs:59) and displayed, never constructed; deposit_log_data is set (verify.rs:310) and read only by tests. adapter.rs:255-272 uses `verified.receipts_root` only and drops the rest. find_log (receipt.rs:151) is `.iter().find(...)`, first match only. VerifiedDeposit identity is (block_hash, receipt_key) without a log index.
Description: A successful verification proves "some log from the bridge address with topic0 exists in some successful receipt". The observation is not tied to the transaction the worker broadcast: tx_hash is compared only against two relayer-supplied strings (adapter.rs:233, worker.rs:738), and the log data (amount, recipient) is compared with nothing (ExternalTransaction.payload is never compared). A receipt with two deposit logs yields only the first, and a replay set keyed on (block_hash, receipt_key) conflates both logs.
Impact: A relayer can present any older bridge deposit receipt as the observation of the transaction it broadcast. A later mint consumer keyed on (block_hash, receipt_key) would pay one of two deposits, or pay the same log under another tx_hash intent.
Known: none

### P06-F11
File: src/cross_domain/external/evm_hybrid.rs:183-202, 210-250; contracts/external-domain/BudlumFinalityVerifier.sol:92-112, 423-445
Severity: Medium
Axis: correctness
Confidence: Likely (missing step: run the pairing equation of the Solidity _verifyBls in P60 scope; here it is read from the comments and field types)
Evidence: evm_hybrid.rs fields `g1_hashed_message` (128), `g2_aggregate_pubkey` (256), `g1_generator` (128), `g2_negated_signature` (256). Solidity: "e(H(m), agg_pubkey) * e(-signature, g1_generator) == 1" with H(m) in G1, pubkey in G2, signature in G2. Rust side (sync_committee.rs:6-10, intake.rs:55-63): public keys in G1, signatures and hash-to-curve in G2.
Description: The group assignment of the EVM proof does not describe either BLS variant. With H(m) in G1 and the key in G2 the signature must be in G1 and pair with the G2 generator; here the signature is in G2 and pairs with the G1 generator. Ethereum sync aggregates (key G1, signature G2, H(m) in G2) cannot be put in this struct. The structure also supplies the hashed message point, the generator and the key from the caller (the planner's `MessageBinding` is a flag the caller states).
Impact: Either no genuine attestation can verify (liveness), or, because the caller provides H(m), the key and the signature, an equation of the form e(H, PK) = e(G1, SIG) can be satisfied with chosen points (for example H = G1 and SIG = PK) when the contract does not pin the key and the message point. P60 must decide which. In the planner this makes FullCryptographic a statement about declared flags.
Known: none

### P06-F12
File: src/cross_domain/external/prover.rs (whole file); rg -n 'ProverBond|DomainEconomics|honesty_is_cheaper' src
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: rg for ProverBond, DomainEconomics, ProverFee, ChallengeReward, required_bond_atoms, honesty_is_cheaper outside prover.rs finds only the re-export list (external/mod.rs:107-108) and mod.rs unit tests (mod.rs:674-780). No call in chain/, execution/, registry/ or rpc/. No code applies slash(), pays a ProverFee or accepts a fraud proof.
Description: The module describes bonds, slashing by fraud proof, fees and unbonding, and the module header says the bond "is slashed when an accepted attestation is contradicted". None of it is wired. Domain admission (intake.rs) takes bond_atoms as data and does not call DomainEconomics::admits.
Impact: The economic backing shown in the DomainProfile (bond_atoms, bond_posters) is not enforced or even produced by a prover role; a domain operator reading the documents would expect a deterrent that does not exist. Mainnet blocker class under CLAUDE.md section 1 (public item without production caller).
Known: none (not in dead-pub-api-baseline.txt or idle-code-baseline.txt: unreachable only through the re-export and tests)

### P06-F13
File: src/cross_domain/external/prover.rs:92-111, 185-192, 227-249
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: prover.rs:189-190 `let part = ... / BPS_DEN; part.max(self.floor_atoms)`; slash() takes `challenge_reward_atoms` as an argument and stores it with no relation to `taken`; prover.rs:245-248 `let gain = economics.fee.for_value(value_atoms); let loss = economics.penalty_for(value_atoms, economics.required_bond_atoms()); gain < loss`.
Description: (1) The floor can make the reward larger than the slashed amount, contrary to the header text that the reward "comes out of the slashed bond rather than from nowhere"; slash() does not cap it. (2) honesty_is_cheaper compares the prover's fee with the penalty, but the profit of a false attestation is the value it diverts (bounded by value_atoms), not the fee. The bond is 10 percent of the ceiling, so for value above the bond the penalty is the bond while the attacker's gain is the value. The function returns true for exactly the case it is meant to catch.
Impact: Wrong economic safety signal in any future caller. Unwired today (F12), so Low.
Known: none

### P06-F14
File: src/cross_domain/external/zkvm_proof.rs:86-108; src/cross_domain/evm/adapter.rs:222-232
Severity: Low
Axis: security
Confidence: Likely (missing step: confirm the bincode version's `deserialize` default accepts trailing bytes; the zkvm decode uses the top-level `bincode::deserialize`, while the adapter uses `bincode::options()`)
Evidence: zkvm_proof.rs:96 `bincode::deserialize(payload)` (the top-level function accepts trailing input), after only a length cap of 12 MiB (:62). adapter.rs:222 uses `bincode::options().with_fixint_encoding().with_limit(...)`, which rejects trailing bytes.
Description: ZkFinalityEvidence payloads with arbitrary trailing bytes decode to the same value but hash to a different evidence.digest() and a different finality_proof_hash. A valid proof can be re-wrapped into many distinct commitments' proof hashes. The 12 MiB cap also has no bincode limit, so memory is bounded only by the cap.
Impact: Duplicate attestation records for the same proof and evidence_digest-based dedup (if added) can be bypassed; resource use per verify up to 12 MiB decode plus the STARK verify.
Known: none

### P06-F15
File: src/cross_domain/evm/adapter.rs:1; src/cross_domain/evm/sync_committee.rs (test fn mainnet_kat_state, attribute allow large_stack_arrays); src/cross_domain/evm/adapter.rs:742-786
Severity: Low
Axis: invariant(Z3) and test
Confidence: Certain
Evidence: adapter.rs:1 `#![allow(clippy::pedantic, clippy::nursery)]` (a module-wide suppression with no reason text, whole file including production code); sync_committee.rs test helper `#[allow(clippy::large_stack_arrays)]` with a comment as reason. adapter.rs:744-745 `include_str!("adapter.rs")` and `include_str!("../../relayer/worker.rs")`, then `contains("verify_observation(&result)")`. verify.rs `verify_does_not_panic_on_garbage` (verify.rs:675-690) and sync_committee `garbage_aggregate_does_not_panic` end in `let _ = ...` with no assertion on the result.
Description: The file-level allow turns off two lint groups for the production code of the EVM adapter; the module test `the_relayer_reaches_verify_deposit` checks source text instead of behaviour (it passes if the string occurs in a comment-stripped line, even when the branch is unreachable, see F06); two garbage tests would still pass if verification returned Ok.
Impact: Maintenance risk and false assurance on an R3 path. Remaining negative tests (wrong emitter, topic, floor, root, tx) are real.
Known: none checked against P62 output (FINDINGS.md has no entry for these files)

### P06-F16
File: src/cross_domain/evm/adapter.rs:343-357
Severity: Info
Axis: LABEL-12.9
Confidence: Certain
Evidence: adapter.rs:354 `receipt_proof: bincode::serialize(&proof).unwrap_or_default(),`
Description: unwrap_or_default on a serialize result in a result constructor. Unreachable today because generate_receipt_proof returns Err first (line 348).
Impact: none now. Label only.
Known: none

## Part 9. Raw findings P07 (external intake and quorum)

Scan by general-purpose agent (Sonnet, medium), 2026-10-11. Not verified. P07-F02 overlaps P06-F01 and F02 (same BLS key and signing root issue); domain_bridge.rs belongs to P06. Full text copied from notes/P07-scan.md.

# P07 scan: src/cross_domain/external/{mod,intake,intake_quorum,quorum,registry,selftest,spec,versioning}.rs

Scope read in full: the eight files above (8064-line directory, these eight about 5300 lines). Also read for call paths only: ethereum.rs:250-440, rpc/server.rs:1640-1960, chain/blockchain.rs:2905-3215, storage/db.rs:1064-1087. Not repeated: STATUS.md section 4.5 (consensus state written outside blocks, external roots not in header, etc.).
Count: High 3, Medium 8, Low 8, Info 2. Total 21.

### P07-F01
File: src/rpc/server.rs:1712-1722 (entry); src/cross_domain/external/registry.rs:638-701 (check); src/cross_domain/external/intake.rs:379-405
Severity: High
Axis: security
Confidence: Certain (call chain read); Likely for public-listener exposure (missing step: check which listener serves bud_submitExternalEvidence)
Evidence: server.rs:1712 `async fn submit_external_evidence(&self, evidence: serde_json::Value)` has no `self.require_operator(...)` line, unlike every sibling (1654, 1806, 1825, 1840, 1888, 1920). The identity used for bond, vote and slashing is a plain field: spec.rs:125 `pub submitter: Address` inside the unsigned `RawConsensusEvidence`. registry.rs:692 `let Some(bond) = prover else { return Err(UnknownProver ..)}` looks up `reg.provers.get(&evidence.submitter)`.
Description: Anyone who can reach the endpoint can submit evidence naming any bonded address (poster addresses are public via the domain registration RPC) as submitter. Nothing proves the caller controls that address.
Impact: One caller can impersonate every bonded prover, so "one bond, one voice" in intake_quorum is void: a single actor reaches any quorum threshold alone, or fills a round with refusal votes under other provers' names. Attestations are recorded with the victim as carrier (registry.rs:430, 540), and the carrier is the slash target (registry.rs:785-795). Each call also rebuilds the adapter and persists plus flushes the whole intake (blockchain.rs:2951, db.rs:1068-1070), a cheap amplification.
Trigger: precondition: one registered domain with a bonded prover address A. Entry: server.rs:1712. Input: RawConsensusEvidence with submitter = A and a payload that is valid (for a vote-based domain) or garbage (to cast a refusal). Expected wrong result: the evidence is processed as A's own, counters and rounds move under A.
Known: none (related class only: STATUS.md 4.5 "consensus state written outside blocks")

### P07-F02
File: src/cross_domain/external/intake.rs:64-131 (PairingBls), src/cross_domain/external/ethereum.rs:393-406
Severity: High
Axis: security
Confidence: Certain for the code facts; Likely for end impact (missing step: read domain_bridge.rs and its consumers to see what an attestation unlocks)
Evidence: ethereum.rs:399-405 `verifier.verify_aggregate(&self.signing_root, &update.aggregate_pubkey, &update.signature, signers)` where `signing_root` is the constant held by the adapter (set at registration, intake.rs:142-145) and the key and signature are parsed from the payload. intake.rs:121-128 checks only `e(pk, H(signing_root)) == e(g1, sig)` with subgroup checks. The doc at intake.rs:56-63 says committee membership "belongs to the adapter's layout"; grep shows `next_committee_root` is parsed (ethereum.rs:237) and never compared to anything.
Description: The pairing check is over a fixed message and a key the prover supplies, and nothing ties the key to a known sync committee, the participation bits to the key, or the signed message to the finalized header or state root. Anyone can pick a secret scalar s, set pk = s*G1 and sig = s*H(signing_root), set the bitvector to all ones, and put any state_root in the payload.
Impact: A forged Ethereum finality attestation (arbitrary height and state root, SignatureSet with 512 signers) passes registry.check and is stored in the attestation book and committed to the digest. Any consumer that treats the attestation as finality is deceived. Severity could be Critical if value routing depends on it.
Trigger: precondition: a registered EthereumSync domain and a submitter address with a bond (see F01, spoofable). Entry: intake.rs:379. Input: 376-byte payload with self-made key and signature as above. Expected wrong result: attestation accepted.
Known: none

### P07-F03
File: src/cross_domain/external/registry.rs:452-471
Severity: High
Axis: correctness
Confidence: Certain
Evidence: registry.rs:452-466
`Err(err) => { ... if !already_faulted && reg.record.state.serves() { ... reg.record.state = DomainState::Faulted; ...} ... Err(err)`
Description: In the single-submission path every refusal of any kind faults the domain: UnknownProver (an address with no bond), DuplicateAttestation (a replay of accepted evidence), version-gate refusals, Stale, InsufficientDepth, malformed payload. The fault is a property of the caller's bad input, not of the domain.
Impact: Any caller (F01 removes the need for a bond) can move any Admitted or Active domain to Faulted with one malformed request. intake.rs test submitted_evidence_reaches_the_framework_registry_and_its_rules shows the refusal counter moves for an unbonded submitter. Recovery is blocked by F05.
Trigger: precondition: a domain in Admitted or Active state without a quorum policy. Entry: server.rs:1712. Input: any evidence that is refused, e.g. a one-byte payload. Expected wrong result: domain state Faulted, history entry with the error text.
Known: none

### P07-F04
File: src/cross_domain/external/registry.rs:51-52, 638-754; src/cross_domain/external/intake.rs:403, 461
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: `rg -n 'NotServing|\.serves\(\)' src/` returns registry.rs:52 (definition), registry.rs:457 and 622 (only state transitions), mod.rs:665 (test). `check` (registry.rs:638) never reads `reg.record.state`.
Description: The Faulted state does not stop anything. submit, evaluate and commit_attestation keep accepting attestations for a Faulted domain; `NotServing` is never constructed. The docs (registry.rs:1-17, quorum.rs:10-11, intake_quorum.rs:34-36) say the domain "must stop serving" after a dispute.
Impact: After a quorum dispute (two bonded provers with different valid finalized headers) the domain stays writable, and a later round or single submission can still commit an attestation. The only effect of the fault is the label seen by readers.
Trigger (security): precondition: a Faulted domain. Entry: intake.rs:379. Input: valid evidence at a fresh height. Expected wrong result: attestation committed while state is Faulted.
Known: none

### P07-F05
File: src/cross_domain/external/registry.rs:337-371; src/chain/blockchain.rs:3022-3048
Severity: Medium
Axis: correctness
Confidence: Likely (missing step: run readmit_external_domain on a registered domain in a test; the derivation below says it always fails)
Evidence: registry.rs:354-360 `let digest = admission.digest(); if digest == reg.admission_digest { return Err(RegistryError::AdmissionDigestMismatch {..}) }`. blockchain.rs:3033-3037 rebuilds the adapter from the stored spec and golden and calls `admit(adapter.as_ref(), &entry.policy)`, the same inputs as registration (intake.rs:329-330).
Description: Admission is deterministic (compiled-in probes, same golden, same policy), so the re-run report has the same digest as the registered one and readmit refuses it. The error name is also inverted: a digest that is equal is reported as "does not match".
Impact: The only exit from Faulted (F03, quorum Disputed) is unreachable until the binary's probe set changes. Combined with F01 and F03, a domain can be killed permanently by an anonymous request. No test calls readmit (F12).
Known: none

### P07-F06
File: src/cross_domain/external/intake.rs:521-545; src/cross_domain/external/quorum.rs:131-155, 231-282
Severity: Medium
Axis: security
Confidence: Certain
Evidence: set_quorum_policy only refuses the two AcceptMostCommon behaviours (intake.rs:535-542). `strict(agreement_threshold: usize, max_participants: usize)` and the struct fields take any value. quorum.rs:245 `if valid < policy.agreement_threshold` and :259 `g.count >= policy.agreement_threshold` with threshold 0 make every non-empty group a winner.
Description: No validation of threshold against max_participants or against the number of bonded provers. Threshold 0 or 1 lets one prover decide; threshold at or below half of max_participants lets the first group to reach it win a race even when a larger honest group would form (intake_quorum.rs:300-302 decides on every submission); threshold above max_participants or above the bonded count can never decide and ends Disputed (intake_quorum.rs:347); max_participants 0 yields RoundFull for everyone; max_participants of usize::MAX voids the "memory bound" claimed in quorum.rs:136. Policies loaded from storage are not rechecked.
Impact: A mistaken operator call (or a malicious one, the call is operator-only) turns "quorum" into single-prover trust or a permanently dead domain without any warning.
Known: none

### P07-F07
File: src/cross_domain/external/intake_quorum.rs:251-271, 357-363; src/cross_domain/external/intake.rs:424-444
Severity: Medium
Axis: security
Confidence: Certain
Evidence: intake_quorum.rs:261-271 `let height = evidence.declared_height; let round = self.rounds.entry(RoundKey { domain, height }).or_insert_with(...)`. Adapter refusals are folded in as answers (intake_quorum.rs:291, 389-397), including DeclarationMismatch and Malformed.
Description: Rounds are keyed by a prover-chosen number and a refusal is a vote, so a bonded prover opens a new retained round for every garbage height it submits, each kept for ROUND_RETENTION_BLOCKS (10,000) and included in every state_digest and every persist. Also, a refusal quorum (AgreedRefusal) for a height blocks that height for all later honest evidence (`RoundClosed`, line 273) until the sweep, although the refusal only says that those particular bytes were bad. With F01 two spoofed identities are enough.
Impact: Unbounded state growth bounded only by call rate, and height-level censorship of an attestation for up to 10,000 local blocks, repeatable after each sweep.
Trigger: precondition: quorum policy with threshold 2 and two bonded addresses (spoofable, F01). Entry: server.rs:1712. Input: two evidence values with the true external height and corrupt payloads. Expected wrong result: round AgreedRefusal, honest evidence for that height answered with RoundClosed.
Known: none

### P07-F08
File: src/cross_domain/external/intake.rs:391-403 (policy source); src/cross_domain/external/spec.rs:317-321, ethereum.rs:417-425
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: intake.rs:392 `let policy = entry.policy.clone();` is the policy stored at registration (intake.rs:350-355). spec.rs:318-321 `now: u64` "supplied by the caller". `rg -n '\.now' src/cross_domain/external/intake.rs src/chain/blockchain.rs` finds no update. ethereum.rs:418 `let age = policy.now.saturating_sub(update.finalized_slot);`
Description: `now` is frozen at the value given at registration, so `max_age` is measured against a stale clock forever. For newer evidence the subtraction saturates to 0 and the Stale check can never fire; max_age is dead in practice.
Impact: A registrant who sets max_age to demand fresh evidence gets no freshness: old (but valid) updates stay acceptable and fresh ones are indistinguishable.
Known: none

### P07-F09
File: src/cross_domain/external/intake.rs:240-246; src/cross_domain/external/registry.rs:97-98; src/storage/db.rs:94-97; src/chain/blockchain.rs:844-846
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: intake.rs:244-246 `/// ... default keeps old serialized states readable ... #[serde(default)] pub quorum: QuorumRounds`; db.rs:95 `bincode::serialize(value)`; blockchain.rs:844 `if let Ok(Some(stored_external_intake)) = store.load_external_intake() { ... }`.
Description: Storage is bincode, which is not self-describing, so `serde(default)` on a missing trailing field does not apply; a state written before `quorum` or `attestation_provers` existed fails to decode. The load site swallows the error and keeps the empty intake.
Impact: After an upgrade a node can silently start with no external domains and no bonds, then disagree with peers on `bud_getExternalIntakeDigest`. The doc comment gives false assurance. (Load site is in blockchain.rs, outside P07; flagged here because the claim sits in intake.rs.)
Known: none

### P07-F10
File: src/cross_domain/external/registry.rs:173-293; src/rpc/server.rs:1651-1653
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: server.rs:1651-1652 `// Operator-only: the bond figure in this build is a declared number, not a signed stake transfer`. registry.rs:191-192 `bond_atoms: u128, poster: Address` are stored as given; bond_prover (registry.rs:573-605) likewise. slash (registry.rs:796-802) only edits the stored ProverBond.
Description: No balance is locked, moved or burned by registration, bonding or slashing. The economics that the "honesty_is_cheaper" gate (intake.rs:326) reasons about are bookkeeping.
Impact: Mainnet blocker class: the security argument of the whole external-domain design (bond scaled to ceiling, challenger reward from the slash) has no money behind it. The challenge reward is a number nobody receives.
Known: none (adjacent to STATUS.md 4.5 "stake bonding" only in a different subsystem)

### P07-F11
File: src/cross_domain/external/registry.rs:764-803
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: registry.rs:776-801 checks that the digest was accepted and who carried it, then `bond.slash(height, evidence_digest, penalty, ...)` with no check of `bond.slashings` for the same digest; the accepted attestation stays in `reg.attestations`.
Description: The same evidence digest can be slashed repeatedly until the bond is empty, and a slashed attestation keeps being returned by latest_attestation (registry.rs:113, used at server.rs:2226).
Impact: Operator-only call, so the risk is a repeated or mistaken slash and a contradicted attestation that stays served as valid.
Known: none

### P07-F12
File: src/cross_domain/external/registry.rs (whole file, 823 lines, no `#[cfg(test)]`)
Severity: Medium
Axis: test
Confidence: Certain
Evidence: `grep -c '#\[test\]'` gives registry.rs 0, selftest.rs 0, spec.rs 0 (covered only through mod.rs). `rg -n 'slash\(|readmit|mark_faulted|DuplicateAttestation|ProverUnderbonded|AttestationProverMismatch|set_version_policy|schedule_external_fork' src/tests tests src/cross_domain/external/*.rs` finds no test that calls them in the external tree.
Description: The R3 registry has no direct tests for slash and its carrier check, readmit, fault transitions, the duplicate-slot rule, the underbonded gate, set_version_policy, or a persistence round trip of IntakeState (bincode). The intake tests cover only register, unknown prover, bonding and the happy quorum path. The defects in F03, F05 and F11 would have been found by any of these tests.
Impact: Missing negative tests on an R3 path.
Known: none

### P07-F13
File: src/cross_domain/external/intake.rs:1122-1161
Severity: Low
Axis: test
Confidence: Certain
Evidence: test name `a_split_that_cannot_recover_freezes_the_domain_faulted`; its comment (1146-1152) admits "the round agrees ... To force the dispute, the policy would need threshold 3"; the last assert is only `!matches!(round.state, RoundState::Open)`.
Description: The test named for the dispute-freezes-domain path does not reach Disputed. The intake branch that calls mark_faulted (intake.rs:464-487) and the AgreedRefusal branch have no test through IntakeState; intake_quorum tests cover the round only.
Impact: The dispute-to-fault wiring is untested and the test name misstates coverage.
Known: none

### P07-F14
File: src/cross_domain/external/mod.rs:381-392, 665
Severity: Low
Axis: test
Confidence: Certain
Evidence: mod.rs:381-391 `for (name, _) in &report.probes { let mut corrupted = Tiny.golden_evidence()...; corrupted.payload.clear(); assert!(Tiny.verify(&corrupted, ...).is_err(), ...) }` never uses the probe. mod.rs:665 `assert!(!p.state.serves() || p.state == DomainState::Admitted);` with the fixture state Admitted is always true.
Description: Two assertions that cannot fail: the loop re-checks the same emptied payload four times and the comment claims it checks each probe's corrupted evidence; the staleness test's last assert holds for its own fixture by construction.
Impact: False coverage in the admission and profile tests.
Known: none

### P07-F15
File: src/cross_domain/external/intake.rs:441-489
Severity: Low
Axis: correctness
Confidence: Likely (missing step: check whether the chain actor rolls back or persists after a returned Err; blockchain.rs:2944-2951 persists on both arms)
Evidence: intake.rs:442-444 `self.quorum.submit(...)?` mutates the round, then 461 `self.registry.commit_attestation(&winner, carrier)?;` and 479 `self.registry.mark_faulted(...)?;` can still return Err.
Description: The round is already AgreedClaim or Disputed when a later step fails, and the error path persists the half-applied state. The commit can fail with DuplicateAttestation if the slot was filled by another path; the round then says agreed with nothing committed and refuses all later votes (RoundClosed).
Impact: Stuck round with an inconsistent book until the 10,000-block sweep.
Known: none

### P07-F16
File: src/cross_domain/external/versioning.rs:138-200, 244-265; src/cross_domain/external/registry.rs:314-335
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: schedule_fork never compares new_version with the adapter's `accepted_evidence_versions`; set_version_policy (registry.rs:331-332) overwrites `reg.record.accepted_evidence_versions` with the window list. is_consistent doc (versioning.rs:244-245) says "at least one open window" but the code only checks non-empty, duplicates and sunset >= valid_from.
Description: An operator can schedule a version the adapter does not accept and sunset the one it does, leaving a domain with no usable version; a registration whose only window already sunset is accepted. The profile then advertises versions the adapter refuses.
Impact: Operator-only misconfiguration bricks a domain; with F03 each refusal also faults it.
Known: none

### P07-F17
File: src/cross_domain/external/selftest.rs:214-222, 326-330
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `&self.probes.len().to_le_bytes()` and `&probes.len().to_le_bytes()` hash a `usize`.
Description: usize is 4 bytes on 32-bit targets and 8 on 64-bit, so the admission digest and probe commitment differ by platform. The digest is compared at readmit and stored in consensus state.
Impact: A 32-bit build would compute different digests for the same admission. Fix is `as u64`.
Known: none

### P07-F18
File: src/cross_domain/external/quorum.rs:112-129, 245-265, 305-314; src/cross_domain/external/registry.rs:51-52
Severity: Low
Axis: dead code
Confidence: Certain
Evidence: `rg -n 'AcceptMostCommon' src/` shows production use only in the refusal at intake.rs:535-539; the behaviours are constructed otherwise only in tests (quorum.rs:496-497, intake.rs:997-1001). `RegistryError::NotServing` is never constructed (F04).
Description: The permissive resolution branches (winner_of, both AcceptMostCommon arms) cannot be reached in production because the only installer refuses them, and `QuorumRounds::set_policy` (pub) does not enforce that refusal itself.
Impact: Maintenance risk and a pub path (`QuorumRounds::set_policy`) that bypasses the guard.
Known: none (baseline files not checked for these symbols; grep of both baselines found no match)

### P07-F19
File: src/cross_domain/external/intake_quorum.rs:357-363; src/cross_domain/external/intake.rs:460
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: intake_quorum.rs:360 `self.rounds.retain(|_, round| local_height.saturating_sub(round.opened_at) <= ROUND_RETENTION_BLOCKS);` applies to every state; intake.rs:460 `.unwrap_or(evidence.submitter)`.
Description: A Disputed round (the evidence for the "challenge game", intake_quorum.rs:99-101) is swept after 10,000 blocks like any other, while the domain stays Faulted. The carrier lookup falls back to the current submitter if the entry is missing, which would credit the closer; the doc says the first carrier.
Impact: The record of who disputed what disappears before anything resolves it; the silent fallback hides a broken invariant instead of failing.
Known: none

### P07-F20
File: src/cross_domain/external/registry.rs:295-337
Severity: Info
Axis: correctness
Confidence: Certain
Evidence: the doc block "Re-admits a domain under a new probe set ..." (295-302) sits above and is merged with the doc of `set_version_policy` (303-313); `pub fn readmit` (337) has no doc; the `# Errors` list under it describes readmit.
Description: Misplaced doc comments, so rustdoc shows readmit's description on set_version_policy.
Impact: Misleading docs only.
Known: none

### P07-F21
File: src/cross_domain/external/mod.rs:39-60; src/cross_domain/external/intake.rs:16-25
Severity: Info
Axis: integration
Confidence: Certain
Evidence: mod.rs:50 "the `GlobalBlockHeader` gains a root when the first real external domain earns one"; intake.rs:584-587 state_digest is "the commitment consumers compare until a header root is earned".
Description: The module headline "Wired through intake" is true for call sites but the attestation is not part of any block or header; the digest is an RPC value only. This is the same class as STATUS.md 4.5 and is noted so the WIRING claim is read with that limit.
Impact: None beyond the known blocker class.
Known: STATUS.md section 4.5 (external roots)

## Verification V09 (P06 F01 to F08, architect agent, 2026-10-11)

Result written by the main session (the agent has no write tool). Static reading. Admission gate for F01, F02, F05: only the node operator can admit an EthereumSync or ZkVm domain, on their own node, with two calls on the loopback operator RPC (bud_registerExternalDomain server.rs:1654 to intake.rs:305-358; bud_registerConsensusDomain server.rs:1625 to blockchain.rs:1712-1757). No genesis or P2P path fills external_intake (blockchain.rs:709, 845). The admit() self-test runs on operator supplied samples, so it cannot tell a real Ethereum committee from a self-made key. After admission, bud_submitVerifiedDomainCommitment is on the public RPC with no operator gate (server.rs:2616-2633).

- P06-F01 CONFIRMED, High. Verifier uses the pubkey from the payload (ethereum.rs:399-406); next_committee_root parsed (:237) and never used; the adapter never does the membership check (intake.rs:60-63, 121-127). The bridge calls adapter.verify directly (domain_bridge.rs:121-126), skipping the prover bond check. Anyone reaching the public RPC can forge the anchored state_root of an admitted domain.
- P06-F02 CONFIRMED, High. One constant signing root per adapter (ethereum.rs:257, 401; intake.rs:177). No finality branch, no link to state_root; depth from payload (:410).
- P06-F03 CONFIRMED, Medium. require_slashable declared (spec.rs:324), read nowhere; now fixed at policy build (spec.rs:335-342); max_age default 0. Config weakness.
- P06-F04 PARTLY, Medium. Bridge binds only height and state_root (domain_bridge.rs:138-148); validator_set_hash and parent hash are bound elsewhere (blockchain.rs:1329-1336, 1542).
- P06-F05 CONFIRMED, raised to High. Program comes from the payload (zkvm_proof.rs:242-246); no program pin, chain_id or continuity check; bud-proof adapter.rs:303-305 says plain verify accepts any well formed program. Reachable once an operator admits a ZkVm domain.
- P06-F06 CONFIRMED, Medium. All three EVM adapter stubs return Err (evm/adapter.rs:166-171, 332-336, 348); no live end to end path (worker.rs:733-736, executor.rs:1034-1039).
- P06-F07 CONFIRMED, Medium. Production always builds the proof with sync_attestation None (evm/adapter.rs:253); DepositProofPackage has no attestation field (verify.rs:193-204). Not reachable today.
- P06-F08 CONFIRMED, Medium. verify_execution_block_finality has no finality branch; rotate_period swaps committees with no proof (sync_committee.rs:420-446); docs overclaim (:20-22). Not reachable today.

New sub-findings:
- P06-F04a Medium. domain_block_hash is free, so the first submitter at a height can make a later honest commitment look like equivocation and freeze the domain (blockchain.rs:1338-1350).
- P06-F01a High if confirmed. submit_verified_domain_commitment writes state.anchor_external_root from an RPC call, outside block execution (blockchain.rs:1603). Open: is external_roots part of the state root? Same class as P05-F06a.
- P06-F01b Medium. The external domain bridge path skips the registry prover bond and refusal counter checks of submit_external_evidence (domain_bridge.rs:121 vs intake.rs:403).

## Part 10. Raw findings P60 (Solidity verifier, proto, buf)

Scan by finder agent (Opus, medium), 2026-10-11, written by the main session from the agent reply. Not verified. Contract: contracts/external-domain/BudlumFinalityVerifier.sol (BFV). Proto and buf.yaml: no defect; ProtoBlockHeader matches Rust BlockHeader (src/core/block.rs:25-46).

- P60-F01 Critical (Certain, verify first). BFV:100-111, 438-471. No trusted key or message is fixed in the contract. The caller supplies all four pairing points including the generator, and the ML-DSA public key (111). root is unused inside _verifyBls (456-462). Trigger shape: in verify (302) pass H as the G1 generator, PK as any G2 point P, negSig as -P, plus a self-made ML-DSA key signing signingRoot(a). e(H,P)*e(G,-P) = 1, so blsOk and pqOk are true and any stateRoot is accepted with no window (339-341). Settles the open question in P06-F11.
- P60-F02 High (Certain). BFV:339-357, 397-411. With precompiles present and verification failed, verify opens a challenge window instead of reverting; settle accepts the digest after the window unless a fraud proof arrives. Contradicts the doc at 293.
- P60-F03 Medium (Likely; missing step: run the probe on an EIP-2537 node). BFV:237, 509-515. Pairing probe has 5000 gas but EIP-2537 charges 70300 for k = 1, so hasBlsPrecompile stays false on real chains and every attestation takes the optimistic path.
- P60-F04 Medium (Certain). BFV:267-288. signingRoot hashes g1HashedMessage and g2NegSignature, so the BLS signature cannot sign the root. Only ML-DSA binds height, stateRoot and epoch.
- P60-F05 Medium integration (Certain). No Rust encoder exists for the Solidity encodings (BFV:248-260, 270-285); evm_hybrid.rs builds no attestationDigest or signingRoot preimage. The parity check in the plan cannot pass. Pair layout (443, 459-461) matches neither BLS variant.
- P60-F06 Medium (Certain). BFV:185, 302-358, 415. accepted is keyed by digest only, verify has no access control, no one-root-per-height rule. Two roots for one height can both settle.
- P60-F07 Medium test (Certain). The contract is never compiled or tested in CI (only xtask/tools/src/audit_guard.rs:43 names the path). No negative test for F01 to F06.
- P60-F08 Low dead code (Certain). BFV:142-145, 147-148, 156, 166-168: ten items declared and unused (PRECOMPILE_BLS_G1ADD, G1MSM, G2ADD, G2MSM, MAP_FP_TO_G1, MAP_FP2_TO_G2, PRECOMPILE_P256, GAS_MLDSA_VERIFY, GAS_P256_VERIFY, GAS_P256_SOLIDITY). The comment at 153-155 promises a P256 probe that does not exist. ChallengeWindowOpen(digest, 0) serves as the error for both a closed window (375) and re-verify (318).
- P60-F09 Info (Certain on proto3 semantics). protocol.proto:200, 222, 268 and buf.yaml:14-20: enum zero values carry meaning (ETHEREUM, BRIDGE_LOCK, SOCIAL_FI); documented and intended. Rust handling not checked.

## Verification V10 (P07 F01 to F08, architect agent, 2026-10-11)

Written by the main session from the agent reply. Static reading. F01, F02, F04, F07 CONFIRMED. F03, F05, F06, F08 PARTLY.

- P07-F01 CONFIRMED, High, reachable. rpc/server.rs:1712-1730 has no require_operator call; every sibling has one (1654, 1806, 1920; check at 443). main.rs:1348-1357 starts the Public listener with the same method set. submitter is a plain unsigned field (spec.rs:123-125); the bond is looked up by it (registry.rs:411, 692). blockchain.rs:2951 persists on every call (db.rs:1068-1070). Precondition: an operator registered a domain.
- P07-F02 CONFIRMED, High (code facts). ethereum.rs:399-405 checks a fixed signing root against key and signature from the payload; intake.rs:121-128 single pairing; next_committee_root (237) and finalized_root (233) parsed and unused. Same root cause as P06-F01 and F02.
- P07-F03 PARTLY, lowered to Medium. registry.rs:452-466: any refusal moves an Admitted or Active domain to Faulted; an unbonded caller is enough (UnknownProver, 692). Faulted blocks nothing (F04), so today the effect is a false label in the RPC profile and digest, made permanent by F05.
- P07-F04 CONFIRMED, Medium. NotServing (registry.rs:52) is never built; check() (638-754) never reads the state; nothing honours Faulted. "Must stop serving" is at registry.rs:611.
- P07-F05 PARTLY, Medium. registry.rs:354-360 refuses an equal digest and admit() is deterministic (selftest.rs:214-231, 377-417), so readmit always fails on the same binary. registry.rs:83-85 suggests a new probe set is the intended exit. No test calls readmit.
- P07-F06 PARTLY, lowered to Low, operator only. RPC refuses threshold 0 and max < threshold (server.rs:1954-1959). Still accepted: threshold 1, threshold at or below half of max (first group wins, intake_quorum.rs:300-302), threshold above bonded count (ends Disputed, 342-347), max = usize::MAX. Stored policies are not re-checked.
- P07-F07 CONFIRMED, Medium, reachable (with F01 one attacker is enough). Rounds keyed by prover chosen declared_height (intake_quorum.rs:261-271); refusals count as votes (291, 389-397); AgreedRefusal makes honest evidence fail with RoundClosed (272-275) until the sweep (359-362, ROUND_RETENTION_BLOCKS 10_000 at 222). state_digest serializes the whole intake (intake.rs:595-597).
- P07-F08 PARTLY, Medium. policy.now is set only at registration (intake.rs:348-355). ethereum.rs:417-424: slots after now saturate age to 0, so Stale never fires for them. Very old updates are still refused.

New sub-findings: N1 ethereum.rs:233 finalized_root never compared (feeds F02). N2 registry.rs:67 vs 355-356: AdmissionDigestMismatch fires when digests are equal, text says the opposite (Low). N3 registry.rs:437, 546: a successful submit on a Faulted domain records the attestation but stays Faulted (Low). N4 only the RPC validates quorum numbers (server.rs:1954); IntakeState::set_quorum_policy accepts anything (Low).

## Verification V11 (P60 F01 and F02, architect agent, 2026-10-11)

Written by the main session from the agent reply. Static reading of BFV in full.

- P60-F01 CONFIRMED in code. Severity Critical inside the contract, but LATENT: nothing compiles, deploys or calls it (no solc, forge, hardhat config; no Rust code builds attestationDigest or signingRoot or calls isAccepted; searched src, budzero, crates, ops, .github, config). Path: verify (BFV:302) gate is only chainId == budlumChainId (303-305); signingRoot (321) hashes caller fields; _verifyBls checks lengths only (444-455) and never uses root (456-462); pairing input (g1HashedMessage, g2Pubkey, g1Generator, g2NegSignature) (459-461). With g1HashedMessage = g1Generator = X, g2Pubkey = P, g2NegSignature = -P the product is 1, so blsOk is true. mlDsaPubkey is caller supplied (111), so a self made key signs root. Line 339 holds and accepted[digest] = true (340) for any stateRoot. Needs both precompiles present at construction (326, 335); EIP-8051 is a proposal (BFV:18) and P60-F03 says the BLS probe fails on real chains. Without the precompiles the same forgery takes the F02 path.
- P60-F02 CONFIRMED, High, latent. If blsOk && pqOk is false (339) verify does not revert; it opens a window (345-357), even when both precompiles are present. settle (397-411) accepts the digest after the window with no access control. Contradicts docs at 293 and 299-301. Only defence: fraudProofVerifier, an arbitrary contract checked only for non-zero (225).
- Effective impact today: zero. Must fix before any deployment. Fix direction: F01 fix the committee G2 key, G1 generator and ML-DSA key in immutables and derive the G1 point from root (or bind through the circuit). F02 revert when a precompile is present and verification fails; open a window only when a precompile is missing. Complexity medium, coder-deep, Opus check on the diff.

## Part 11. Raw findings P08 (chain A: blockchain.rs 1-3439, fee_market, genesis)

Scan by finder agent (Opus, medium), 2026-10-11, condensed by the main session. Not verified. Read in full: blockchain.rs 1-3439, fee_market.rs, mod.rs, genesis.rs. Not read: blockchain.rs 3440 on (P09).

- P08-F01 High (Certain, invariant). genesis.rs:569-634, 376-469. build_state mints the full 100M tokenomics distribution (623-626), adds min_stake per genesis validator via add_validator, which debits no balance (account.rs:658-660), and adds plain allocations (576-578). Mainnet ceremony needs at least 4 validators (455) and never checks allocations or stake against the cap. A valid ceremony commits more than BUD_TOTAL_SUPPLY at block 0; supply_capacity_remaining() is 0 and every capped mint (blockchain.rs:2138, 3310) refuses. Test genesis.rs:873-891 has no validators, so it misses this.
- P08-F02 High (Certain, replay). blockchain.rs:507-531, 587-597, 659-683. When snapshot height is at or above the stored chain length, replay starts at index 0 but state is still the restored snapshot, never reset to genesis; every block is applied twice. restored_finalized_height stays above the chain length. A node with a block DB shorter than its snapshot restarts with doubled effects and a wrong root. No test for this branch.
- P08-F03 High (Likely; missing step: is submit_verified_domain_commitment, server.rs:2616, on the public listener; P06 V09 says it is public with no operator gate). blockchain.rs:1338-1345, 1378-1382, 1595-1610. The idempotent Ok branches compare only the block hash, not state_root, then submit_verified_domain_commitment overwrites external_roots[domain] with the resubmitted commitment's state_root (account.rs:1894). Replaying an old finalized commitment with its original proof rewinds the anchor. Trigger: accepted heights h1 < h2, resubmit (commitment@h1, proof@h1); expected rejected as stale, actual Ok and anchor = root@h1. Separate from P06-F01a.
- P08-F04 Medium (Certain). blockchain.rs:1436-1445, 1542-1553. On parent hash mismatch the freeze is set in memory only; no save_consensus_domain, no quarantine event; the early return skips save_domain_commitment_batch (1439). After restart the domain is active again and memory and disk differ.
- P08-F05 Medium (Likely; missing step: block or gossip size cap). blockchain.rs:1150-1164, 1181-1196. verify_slashing_evidence never checks that the producer is a validator, nor chain id or height. Pool has a linear dedup scan (1154) and no cap (1162). Gossip feeds it (node.rs:2282) and re-publishes; the whole pool is copied into the next block (4626-4627). Any peer can mint unlimited evidence with throwaway keys: memory growth, quadratic CPU, oversized blocks, no slashing effect (account.rs:1430).
- P08-F06 Medium (Certain). blockchain.rs:2736-2744. submit_zk_proof moves domain last_committed_height and hash in memory but persists only the proof claim registry. After restart the progress reverts while claims remain; the stale claim gate (2632) and bridge checks (2064) see a different tip.
- P08-F07 Low (Certain). blockchain.rs:805-807, 821-823, 830-838, 844-846. A decode error in the universal relayer, proof claims, message registry or external intake silently falls back to empty, unlike bridge state and storage registry (794-803), which stop the node. Wrong message_root after restart.
- P08-F08 Low (Certain). blockchain.rs:409-423. A known network id (mainnet included) with no ceremony tokenomics addresses skips the whole ceremony check with only a warning.
- P08-F09 Low (Certain). blockchain.rs:550-576. The V1 snapshot fallback restores only balances and nonces; validators, registry, epoch and roots stay at genesis. Testnet and devnet only.
- P08-F10 Low test (Certain). blockchain.rs:1306-1320. submit_domain_commitment has opposite behavior under cfg(test) and production; tests skip finality verification and the anchor step, so they cannot catch F03.
- P08-F11 Low dead code (Certain). blockchain.rs:946-984 two private functions under allow(dead_code), 703 and 705 unused locals; freeze_poa_account (1110-1137) takes admin_authorized as a caller bool and has no production caller. Other items are in the idle code baseline.
- P08-F12 Info. fee_market.rs: whole module off the live path (header lines 3-5); arithmetic sound, no floats. Known: idle-code-baseline.txt:1221-1228, 1326.
- P08-F13 Info (Z13). genesis.rs:90, 168 non-English comments.

## Part 12. Raw findings P09 (chain B: blockchain.rs 3440-6836, storage economics tests)

Scan by finder agent (Opus, medium), 2026-10-11, condensed by the main session. Not verified. Read in full: blockchain.rs 3440-6836 (non-test). Skimmed: tests from 6837 and storage_economics_tests.rs. Not repeated: P01-F05b, P01-F22, P04-F01, F24, P05-F05, F06a.

- P09-F01 High (Likely; to verify: deal open allows total_fee(1) about 1e7 to 1e13 since deal_open.rs:270-297 has no upper bound on fee_per_byte_epoch; mempool TTL 3600 s). blockchain.rs:4571-4573, 6216-6266. The boost share is multiplied by each active deal's per epoch fee weight with checked_mul (6241-6243); overflow returns Err, which fails apply_block_effects for the whole block (4645-4649), though collect_block_transactions (4274) still includes the boost tx. produce_block returns None on every node while the tx sits in the mempool: chain halt. Trigger: open a deal with weight W, payer and operator the attacker (escrow returns), then one NftBoost with 0.04*A*W > 1.8e19 (for example W = 1e13, A = 4.6e7).
- P09-F02 Medium (Likely; to verify: a reorg crossing a checkpoint with a QC blob; chain_actor.rs:3587 path open). blockchain.rs:5751-5767, 3685-3691, 5237-5239. handle_finality_cert never compares blob.checkpoint_hash with cert.checkpoint_hash or the local hash; qc.rs:197-237 does not either. try_reorg drops blobs from memory only, so get_qc_blob returns the old fork blob from disk. A new fork checkpoint can be finalized with a PQ blob that signs another block.
- P09-F03 Medium (Certain in code; needs a disk error or the PoA stamp mismatch of STATUS 4.2). blockchain.rs:5253-5254, 5361-5396. try_reorg swaps memory, deletes old fork blocks from disk (5363), then re-commits new blocks with `?` (5395). A failure midway leaves memory on the new chain and disk with neither fork from the fork point on; last_hash not updated.
- P09-F04 Medium (Certain). blockchain.rs:5321-5329. After a reorg the replayed message_registry is overwritten from disk; message_root is in the state root (P05-F06a), so the next block fails with State root mismatch (4960). Same class as STATUS 4.2 bridge_state item, not listed there.
- P09-F05 Medium (Certain). blockchain.rs:5332-5333, 3664-3669, 622-630. After a reorg memory keeps only the current epoch snapshot, so past epoch certs, QC blobs and fault proofs are refused until restart; disk is not cleared, so a restart reloads old fork validator sets.
- P09-F06 Low (Certain). blockchain.rs:5349-5359. A reorg rebuilds the mempool with MempoolConfig::default() (not the per network config, 685-688), keeps the top 1000 txs, and does not re-add txs of orphaned blocks.
- P09-F07 Low (Likely; both steps unreachable today). blockchain.rs:4497-4500, 3822-3834, 5678-5680. commit_block_durable stores a pending, possibly unverified cert as a finality cert; invalidate_finality_from_height adopts any stored cert without verification or hash comparison; try_reorg keeps FINALITY_CERT and QC_BLOB at reorged heights. Known partly: P01-F22, dead-pub-api-baseline.txt:71.
- P09-F08 Low (Certain in code; reach limited by P01-F22). blockchain.rs:3847-3877, 3963-3970. QC fault verdict slashes stake directly in self.state outside block execution and drops the result (3869). Entry points: node.rs:2797 gossip and server.rs:3179 public RPC. Stake is in the state root: consensus split if a proof can pass.
- P09-F09 Low (Certain; not wired). blockchain.rs:5564-5585, 5641-5650. Snapshot apply requires only that the block at snapshot.height exists, not that it is the tip; state can be replaced by an older state.
- P09-F10 Low (Likely). blockchain.rs:3949-3960. import_qc_blob stores the blob, then returns Err when a pending cert at that height fails, so node.rs:2768-2772 reports an honest peer.
- P09-F11 Low test (Certain). storage_economics_tests.rs:150-197 scans source text (include_str) instead of running the persist failure path.

## Verification V12 (P08 F01 to F06, architect agent, 2026-10-11)

Written by the main session from the agent reply. Static reading. All six CONFIRMED.

- P08-F01 CONFIRMED, High (mainnet only, not reached today). The tokenomics split has no validator share (tokenomics/mod.rs:366-372) and sums to exactly BUD_TOTAL_SUPPLY = 1e14 base units (mod.rs:33, 211-212; genesis.rs:344). build_state mints it all (genesis.rs:623-626), then add_validator adds Mainnet min_stake 1_000_000 per validator (genesis.rs:580-582, 651-653; chain_config.rs:129) with no debit (account.rs:658-665); total_bud_committed counts staked supply (account.rs:2036-2040, 2063-2068). With the required 4 validators committed = 1e14 + 4_000_000 > cap, and nothing in validate_consensus_ceremony (376-469) checks it. supply_capacity_remaining() = 0, so every ensure_mint_headroom fails: bridge mints (blockchain.rs:2139/2154, 3311/3323; executor.rs:1171/1191). Block reward mints nothing (executor.rs:2541-2544). Testnet and devnet have bud_tokenomics None (genesis.rs:759, 776). Hits the first real mainnet genesis.
- P08-F02 CONFIRMED, High, production startup path. blockchain.rs:587-597 with 523-526, 659-683. apply_transaction_checked has no nonce check, so the outcome is doubled effects with a wrong root, or exit(1) at 676 when a re-applied debit fails.
- P08-F03 CONFIRMED, High. blockchain.rs:1343-1344, 1378-1381, 1595, 1603; account.rs:1894. Public listener (server.rs:2616-2633, no require_operator; guard at 443) and P2P gossip (node.rs:2832-2848, rebroadcasts on Ok). Caveat: the adapter must still accept the old proof (blockchain.rs:1660 on), for example when the validator set has not rotated.
- P08-F04 CONFIRMED, Medium. blockchain.rs:1542-1550 vs 1350-1371; the commitment inserted at 1436 stays in memory unpersisted. Reached via public RPC and gossip.
- P08-F05 CONFIRMED core, Medium, possibly High. Gossip feeds the pool and rebroadcasts (node.rs:2281-2293); pool copied into the next block (4626-4627), cleared only after commit (4709-4711). Unproven: a 1 MB block limit (consensus/mod.rs:44, 188) may reject the oversized block before commit, so the pool never clears and block production stalls. To confirm.
- P08-F06 CONFIRMED, Medium. Public RPC (server.rs:3050-3054, no operator guard). Side note: the proof fee is debited from state outside a block (blockchain.rs:2697).

## Verification V13 (P09 F01 to F05, architect agent, 2026-10-11)

Written by the main session from the agent reply. Static reading.

- P09-F01 CONFIRMED, High, reachable today (chain halt). blockchain.rs:6216-6266: weight is deal.total_fee(1) (6231); checked_mul overflow returns Err (6241-6243); apply_block_effects passes it up (4572-4573); produce_block maps any Err to None (4645-4649) before the state root and before any mempool removal. collect_block_transactions (4255-4285) never runs the boost distribution, so the NftBoost tx is included every slot and every producer fails while it sits in the mempool. No cap on fee_per_byte_epoch exists (storage_deal.rs:221, 251-256; deal_open.rs); the only cap is the payer balance (deal_open.rs:270-297), so W <= escrowed fee <= balance <= 1e14 base units (tokenomics/mod.rs:33). NftBoost amount A is bounded by spendable balance (executor.rs:895-906); bud_share = 4*A/100 (868), accumulated with checked_add (928). Halt needs 0.04*A*W > 1.845e19, so A*W > 4.61e20 with A + W <= balance: A = W = 2.15e10 needs about 4.3e10 base units, about 43k BUD (0.043 percent of supply); two accounts also work. Attack cost near zero: the boost block never commits, so A is never spent, and the deal fee returns to the attacker as operator. The original example (W = 1e13, A = 4.6e7) is valid but needs a 10M BUD escrow. Fix shape: u128 math or bound the weight. Top priority fix, forgery test first.
- P09-F02 PARTLY, lowered to Medium or Low. The claim that handle_finality_cert never compares the cert hash with the local hash is wrong (blockchain.rs:5732-5744; import_qc_blob also compares, 3906-3911). Real gap: after a reorg get_qc_blob (5751) falls back to the old fork blob on disk (3685-3691, try_reorg drops memory only, 5237-5239) and neither handle_finality_cert nor verify_against_snapshot (qc.rs:197-237) compares blob.checkpoint_hash with cert.checkpoint_hash. A BLS valid cert for the new fork can be finalized with an old fork PQ blob; needs a quorum signed blob on the old fork at the same height, so a PQ binding gap, not a free forgery.
- P09-F03 CONFIRMED, Medium, only on a disk error or the PoA stamp root mismatch (STATUS 4.2). Memory swap 5253-5254, best effort deletes 5361-5366, early exit 5379-5396, save_last_hash only after (5398-5401).
- P09-F04 PARTLY, lowered to Low, merged into P05-F06a. The only writer of message_registry is the off block submit_cross_domain_message (2304, persisted 2307); replay never adds messages, so the disk registry equals the live registry before the reorg. The reorg adds no new mismatch.
- P09-F05 PARTLY, lowered to Low. Memory part confirmed (5332-5333, 3664-3680). The restart part is mostly refuted: startup loads disk (622-630), replay overwrites every covered epoch (659-682) and starts at or below the fork point (5240-5251). Residual: old fork epochs above the new tip epoch stay on disk.

## Part 13. Raw findings P10 (chain actor, finality)

Scan by finder agent (Opus, medium), 2026-10-11, condensed by the main session. Not verified. Read in full: finality.rs 1-1556, chain_actor.rs 1-5040. Not repeated: P01-F22, P01-F05b, P05-F05, F06a, P09 items, STATUS 4.4 and 4.5.

- P10-F01 High (Certain in code; no end to end test of the node vote loop). finality.rs:540-542; node.rs:1568-1574; blockchain.rs:5974-5983. The node signs prevotes with epoch = H / checkpoint_interval (node.rs:1568); the aggregator expects H / epoch_len (blockchain.rs:5974-5975, 5983); add_prevote rejects every honest prevote as epoch mismatch (finality.rs:540-541). Mainnet: interval 10, epoch_len 100 (chain_config.rs:127/130; testnet 50/5, devnet 10/2). The values differ for every H > 0. sign_prevote has one production caller (node.rs:1569). Prevote quorum is never reached, no precommit (node.rs:1531-1535), no FinalityCert: native BLS finality never advances on any network.
- P10-F02 High (Likely; to check: AI requests can be submitted by transaction on mainnet). chain_actor.rs:3336-3354, 3398-3402, 3415-3420. After each committed block the actor prunes AI requests, results and equivocation events (prune_expired, expire_dispute_window); that data is in the state root (account.rs:2452-2454; ai/registry.rs:1921-1923, 1952-1953). Startup replay (blockchain.rs:659-668), try_reorg (5379-5389) and rebuild_state (5432-5435) never prune. After one AI request passes deadline + 10_080 blocks, a restarted node replays to a different root and rejects the next network block (State root mismatch, 4962); a coordinated restart stalls every node. No mainnet gate on the AI half. The storage half (run_storage_maintenance 3008-3326, account.rs:2460-2462) is the same mechanism on non mainnet chains. STATUS 4.5 lists only storage maintenance.
- P10-F03 High (Likely; to check: no other startup path restores a state snapshot before load_chain). chain_actor.rs:4018-4038. The only caller of the retention guard passes the tip height as the latest snapshot height (4019, 4024-4029), so the snapshot bound (snapshot.rs:180, 200-206) does nothing. Blocks 1..min(len-keep, finalized) are deleted with their finality certs and QC blobs (db.rs:664-669). load_chain stops at the first deleted parent (db.rs:1293-1301), chain_vec[0] is not genesis and startup exits (blockchain.rs:410-430). Trigger: operator calls bud_requestPrune (server.rs:6603) on a non archive node past retention. Lasting data loss; the node cannot restart.
- P10-F04 Medium (Certain). chain_actor.rs:2922-2936, 2968-2980. The account proof trie is cached by chain length, but balances change without a block (4424-4437, 4549-4557, 3638-3644, 3714-3839) and an equal length reorg replaces state at the same length (poa.rs:438-451, pow.rs:474-483). bud_getAccountProof returns a proof of old balances that passes the self consistency check.
- P10-F05 Low (Likely; matters once F01 is fixed). chain_actor.rs:3403-3408, 3421-3432; blockchain.rs:5984. Each checkpoint block overwrites the single aggregator with its votes; an older checkpoint without precommit quorum within one interval is never finalized; TryReorg (3586-3588) starts no prevote task.
- P10-F06 Low (Certain). finality.rs:678-698. The equivocation report omits epoch and vote type (183-187, 259-263), so a third party cannot rebuild the signed bytes; it stands on the local ConsensusVerified label.
- P10-F07 Low (Certain). finality.rs:12-14, 119-134. Quorum is hard coded to 2/3; Network::finality_quorum() and the per network numerator are never read (devnet states 1/2).
- P10-F08 Low (Certain). chain_actor.rs:3874-3879, 3892-3898. The AI reclaim query estimates height as epoch * 100 (wrong on testnet and devnet) and clones the whole AI registry inside the actor on every public RPC call (server.rs:6042-6061 has no require_operator).
- P10-F09 Low dead code (Certain). chain_actor.rs:133-136, 1642-1653, 3694-3696 keep an unverified raw commitment path alive; RPC refuses it (server.rs:2608-2613). Not in the baselines.
- P10-F10 Low test (Certain). finality.rs:1469-1511: the subgroup attack test never builds a subgroup breaking key; no test drives the node prevote epoch through FinalityAggregator, which is why F01 was missed.
- P10-F11 Info (Z8). chain_actor.rs:331, 2364: an f64 crosses the actor on a read path (GetAiAgentRanking); no state is written with it.
- Pointer, no id, to check in the next verify batch: handle_prevote and handle_precommit (blockchain.rs:5796-5812, 5841-5857) record an invalid vote against vote.voter_id before checking that the sender controls that key; a forged vote naming a victim may raise the victim's invalid_votes counter (relation to P04-F01).

## Suggested next steps

1. Verify P04-F01 first (possible Critical). Then P04-F02 to F04 (stake and slash arithmetic), P01-F09, P03-F20, P02-F03.
2. Decide the owner questions K1 and K3 (STATUS.md:102) before any handoff.
3. Fix K4 (the try_reorg suffix bug) first: it is plain wiring and it blocks every reorg test.
4. Continue the audit from P05 on the wave order in docs/AUDIT_PLAN.md.
