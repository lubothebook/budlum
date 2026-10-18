# P01 Consensus
Base 7c496cd. Files read: src/consensus/mod.rs 1-346, split_resolver.rs 1-168, merkle_tree.rs 1-461, pos.rs 1-1368, pow.rs 1-793, poa.rs 1-833, qc.rs 1-1374 (all tracked .rs files of the part, 5343 lines). Not read: none. Outside the part, rg first and short ranges only: blockchain.rs (1150-1200, 4640-4700, 4738-4765, 4890-4960, 5100-5262, 5740-5800), node.rs (755-765, 2380-2392, 3040-3100), main.rs (548-560, 725-760, 1026-1036, 1588-1650), account.rs (650-675, 985-1050, 1410-1475, 2235-2265), block.rs (60-200, 335-535), db.rs (1287-1303, 1420-1500).
History: `git log --oneline 25ede0b..HEAD -- src/consensus` shows one commit (f1b8a4a, audit notes only). No consensus code changed since the first pass, so line numbers below are HEAD lines of the same code.

## First-pass verdicts
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

## New findings
### SPA-P01-F01
File: src/consensus/qc.rs:73-88, 655-672; src/chain/blockchain.rs:3885, 5779-5800
Severity: Medium
Axis: integration
Confidence: Likely (missing check: a producer of QC blobs outside src/ is not excluded)
Evidence: `rg -n 'QcBlob::new|pq_signing_message|sign_attestation' src --type rust -g '!src/tests/*' -g '!src/consensus/qc.rs'` returns only light_client/mod.rs:492 and 910-924 (inside `mod tests`, mod tests starts at light_client/mod.rs:485). The only production entry for a blob is the network import (node.rs:2761 to blockchain.rs:3885). `rg -n 'finalized_height = ' src/chain/blockchain.rs` shows only 3837 (invalidate), 5661 and 5703 (snapshot restore) and 5800 (handle_finality_cert).
What must be refused and what happens today: A finality cert must be refused without a verified QC blob (blockchain.rs:5779-5784 does refuse and queues it). Nothing in production signs a PQ attestation or builds a blob, so on a network of production nodes that requirement cannot be met.
Impact: finalized_height can stay at 0, so the chain-layer finality guards (blockchain.rs:4893, 5247) never bind and reorg safety rests on MAX_REORG_DEPTH and the PoS node-local checkpoint. Mainnet blocker for finality.
Known: dead-pub-api-baseline.txt:92 (sign_attestation) and :91 (new_zk_invalid_attestation) cover the names only.

### SPA-P01-F02
File: src/main.rs:739; src/core/chain_config.rs:240-244; src/chain/blockchain.rs:4325-4328; src/consensus/pos.rs:721, 983
Severity: Low
Axis: invariant (epoch transition)
Confidence: Likely (missing check: a run with `--chain-id` set to a value that is not a known network id)
Evidence: engine epoch length comes from `network_params.epoch_len` (main.rs:739). The chain layer uses `epoch_len_for_chain_id(block.chain_id)` (blockchain.rs:4325), which falls back to `EPOCH_LEN = 100` for an unknown id (chain_config.rs:243). Chain id can be overridden at main.rs:548.
What must be refused and what happens today: The engine and the state machine must use one epoch length. With a custom chain id on devnet (10) or testnet (50), engine epoch, checkpoint cadence and randomness use one value while state epoch advance uses 100. Nothing refuses the mix.
Impact: operator running a non-standard chain id. Epoch bound state (rewards, validator snapshots, QC epoch) and engine epoch drift apart on that network.
Known: none (pos.rs:23-37 documents the default only).

### SPA-P01-F03
File: src/consensus/pow.rs:208-229, 309-318; src/chain/blockchain.rs:5229-5260
Severity: Low
Axis: security
Confidence: Likely (missing check: reach depends on the F05b fix, which feeds try_reorg a real chain)
Evidence: try_reorg calls `is_better_chain` first (blockchain.rs:5230) and `validate_candidate_chain` later (5260). PoW is_better_chain derives the claimed difficulty path from the candidate own timestamps (pow.rs:310, 245) before any block of the candidate is validated.
What must be refused and what happens today: A candidate whose claimed work comes only from unvalidated timestamps should not buy a full replay from genesis. Today the order is cheap score first, then full replay. Today the exposure is closed by F05b.
Impact: after F05b is fixed, a peer can force one replay from genesis per crafted candidate (depth cap at blockchain.rs:5239-5243 does not bound the replay of the common prefix).
Known: none.

### SPA-P01-F04
File: src/consensus/pos.rs:147-170; src/core/block.rs:59-80, 151-161, 484-510
Severity: Low
Axis: security
Confidence: Certain for the code; dormant because no writer exists
Evidence: BlockHeader has no producer_public_key field (block.rs:59-80) and BlockHeader::verify_signature checks Ed25519 against the producer bytes only (block.rs:158-160). Block::verify_signature has a second branch for an ML-DSA signed block (block.rs:468-510). `rg -n 'producer_public_key' src -g '!src/tests/*'` finds no assignment of a non-empty value.
What must be refused and what happens today: Double-sign evidence for a block signed with the V5 (ML-DSA) producer path cannot verify, so it would be refused and the producer cannot be slashed. Today no production code sets the V5 field.
Impact: when V5 producers are switched on, equivocation by them is not provable.
Known: none.

### SPA-P01-F05
File: src/consensus/poa.rs:228-246
Severity: Low
Axis: security
Confidence: Certain (code), Likely for reach (stake tx path read only from the first-pass note)
Evidence: the leader hash input includes each active validator stake: `hasher.update(v.stake.to_le_bytes())` (poa.rs:239).
What must be refused and what happens today: Leader choice for a height should not change because an unrelated account changed stake. Today any change to the stake of an active validator changes the leader for all later heights, and a Stake transaction is open to any funded account (P01-F07).
Impact: an authority (or a staker in an open PoA domain) can steer the next leader with a stake change, in addition to the nonce lever in P01-F10.
Known: none.

### SPA-P01-F06
File: src/consensus/qc.rs:450-492, 494-539
Severity: Low
Axis: security
Confidence: Likely (missing check: the RPC body limit for bud_submitQcFaultProof was not read)
Evidence: verify_inclusion loops over every element of `merkle_proof` and hashes each one with no length or count cap (qc.rs:462-483). validate() checks emptiness only (qc.rs:513-519). The proof reaches this code from the public RPC named at qc.rs:1062-1063.
What must be refused and what happens today: A proof with more elements than the tree depth (at most 32 for u32 leaf index) or with an element that is not 32 bytes should be refused before hashing. Today it is hashed in full. verify_against_blob later compares the leaf with the stored blob.
Impact: CPU cost proportional to the payload size, bounded only by the transport limit.
Known: none.

### SPA-P01-F07
File: src/consensus/poa.rs:118, 126; src/consensus/mod.rs:24, 88
Severity: Low
Axis: invariant (Z12, Z13)
Confidence: Certain
Evidence: mod.rs:88 reads "mainnet L1's fact" in a rustdoc comment (Z12). Bold markers are in poa.rs:118, poa.rs:126 and mod.rs:24 (Z13), besides the merkle_tree.rs:8-9 hits already in P01-F25.
What must be refused and what happens today: Public text must use the approved name from rule Z12 and plain text. The comments carry the banned term and bold markers.
Impact: style rule only.
Known: partly P01-F25 (mod.rs:88, merkle_tree.rs:8-9).

### SPA-P01-F08
File: src/consensus/pos.rs:373-377
Severity: Info
Axis: security
Confidence: Certain
Evidence: for target epochs 0 and 1 the randomness is the genesis bootstrap hash with no VRF term (`hasher.update(b"genesis_bootstrap")`).
What must be refused and what happens today: Nothing is refused. The leader schedule of the first two epochs (20 blocks devnet, 100 testnet, 200 mainnet by the pos.rs:31 comment) is known from genesis alone.
Impact: leader prediction in the bootstrap epochs.
Known: none.

### SPA-P01-F09
File: src/consensus/poa.rs:353-417; src/consensus/pow.rs:376-415
Severity: Info
Axis: correctness
Confidence: Certain
Evidence: PoS requires `block.epoch == index / epoch_length` and `block.slot == index` (pos.rs:721-733). PoA and PoW validate neither field. `rg -n '(block|header)\.(epoch|slot)\b' src -g '!src/tests/*' -g '!src/consensus/*'` finds only copy sites (block.rs:70-71, proto_conversions.rs).
What must be refused and what happens today: Nothing reads the fields for PoA and PoW, so a free value does no harm today. A later reader would inherit unchecked data.
Impact: none today.
Known: none.

### SPA-P01-F10
File: src/consensus/pow.rs:244-245, 309-318
Severity: Info
Axis: correctness
Confidence: Certain
Evidence: the window runs from block `boundary + 1 - interval` to `boundary`, so it spans `interval - 1` block gaps, while `expected_time = target_block_time * interval`.
What must be refused and what happens today: The measured span and the expected span should cover the same number of gaps. Today the ratio is biased by about 1 percent at interval 100, toward a higher difficulty.
Impact: small drift, no halt by itself.
Known: none.

## Cross-part notes
- Leader eligibility reads live state, not an epoch snapshot: stake and total stake at pos.rs:797 and validator set hash at pos.rs:737. total stake counts validators below min_stake (account.rs:992-997).
- PoS block validation depends on node-local mutable data (checkpoints, pos.rs:715) in addition to state. Two nodes with equal state can disagree, and a node can refuse its own history (blockchain.rs:5130 is_valid).
- PoS randomness indexes the chain slice by height (pos.rs:343-353) and requires the genesis block at slice position 0 (pos.rs:308-315). A pruned or suffix chain cannot validate PoS blocks. Pruning deletes old block records on disk (blockchain.rs:5112-5116); the restart path was not traced.
- Finality irreversibility is enforced at the chain layer, not in the engines: live path blockchain.rs:4893-4906, reorg path blockchain.rs:5247-5258. Engine fork choice (mod.rs:245-251, pos.rs:886-933, pow.rs:471-490, poa.rs:435-452) ignores finality. See SPA-P01-F01 for how finalized_height advances.
- Fork choice order is deterministic and arrival independent: split_resolver.rs:74-105 orders by weight, height, lower hash, lower proposer. PoW work is exact in 256 bits (pow.rs:23-77); the accumulated work walk and the per-prefix walk fold the same boundaries (pow.rs:213-219, 320-332).
- Block hash covers epoch, slot, VRF fields, validator set hash and the evidence bytes (block.rs:99-147, 339-360). Evidence apply is idempotent per validator through the slashed flag (account.rs:1425-1436).
- QC threshold math is not in P01. qc.rs enforces entry count at most validator count (qc.rs:255-262), unique signers (qc.rs:292-297) and optional required signers (qc.rs:318-326). The quorum comparison sits in chain/finality.rs (quorum_stake at finality.rs:119, quorum_count at 129), which belongs to another part.
- Slashing evidence reaches state only through block.slashing_evidence (blockchain.rs:4310-4321). Evidence has no height or age bound at apply time. The engine queue is in memory only (pos.rs:102) and is not persisted by the live path.
- Serialization determinism: consensus size check uses serde_json::to_vec and returns an error on failure (mod.rs:185-187). Validator set is a BTreeMap and get_active_validators sorts by address (account.rs:259, 999-1007), so leader selection does not depend on HashMap order. seen_blocks is a HashMap but is never iterated for output (pos.rs:1017 retain only).

## Checks done
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
