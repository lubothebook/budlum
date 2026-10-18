# Second pass audit, beta

Base commit: 7c496cd (consolidate/audit-62-mainnet). Date: 2026-10-12.
Method: docs/AUDIT_PLAN.md, parts P01 to P62. Static reading only: rg and git, no builds, no test runs.
Scope of this file: confirm or challenge the first pass (docs/audit/FINDINGS.md, notes/Pxx-scan.md) and record new findings.
New finding ids: SPB-Pxx-Fnn. Severity scale: docs/AUDIT_PLAN.md section 6.
Wording is defensive: what must be refused, and what happens today.

## Progress

| Part | Status | New findings |
|---|---|---|
| P01 | DONE | 4 (1 High, 2 Low, 1 Info) |
| P02 | NOT COVERED | 0 |
| P03 | NOT COVERED | 0 |
| P04 | NOT COVERED | 0 |
| P05 | NOT COVERED | 0 |
| P06 | NOT COVERED | 0 |
| P07 | NOT COVERED | 0 |
| P08 | NOT COVERED | 0 |
| P09 | DONE | 5 (1 High, 3 Medium, 1 Low) |
| P10 | NOT COVERED | 0 |
| P11 | NOT COVERED | 0 |
| P12 | NOT COVERED | 0 |
| P13 | NOT COVERED | 0 |
| P14 | NOT COVERED | 0 |
| P15 | NOT COVERED | 0 |
| P16 | NOT COVERED | 0 |
| P17 | NOT COVERED | 0 |
| P18 | NOT COVERED | 0 |
| P19 | NOT COVERED | 0 |
| P20 | NOT COVERED | 0 |
| P21 | NOT COVERED | 0 |
| P22 | NOT COVERED | 0 |
| P23 | NOT COVERED | 0 |
| P24 | NOT COVERED | 0 |
| P25 | NOT COVERED | 0 |
| P26 | NOT COVERED | 0 |
| P27 | NOT COVERED | 0 |
| P28 | NOT COVERED | 0 |
| P29 | NOT COVERED | 0 |
| P30 | NOT COVERED | 0 |
| P31 | NOT COVERED | 0 |
| P32 | NOT COVERED | 0 |
| P33 | NOT COVERED | 0 |
| P34 | NOT COVERED | 0 |
| P35 | NOT COVERED | 0 |
| P36 | NOT COVERED | 0 |
| P37 | NOT COVERED | 0 |
| P38 | NOT COVERED | 0 |
| P39 | NOT COVERED | 0 |
| P40 | NOT COVERED | 0 |
| P41 | NOT COVERED | 0 |
| P42 | NOT COVERED | 0 |
| P43 | NOT COVERED | 0 |
| P44 | NOT COVERED | 0 |
| P45 | NOT COVERED | 0 |
| P46 | NOT COVERED | 0 |
| P47 | NOT COVERED | 0 |
| P48 | NOT COVERED | 0 |
| P49 | NOT COVERED | 0 |
| P50 | NOT COVERED | 0 |
| P51 | NOT COVERED | 0 |
| P52 | NOT COVERED | 0 |
| P53 | NOT COVERED | 0 |
| P54 | NOT COVERED | 0 |
| P55 | NOT COVERED | 0 |
| P56 | NOT COVERED | 0 |
| P57 | NOT COVERED | 0 |
| P58 | NOT COVERED | 0 |
| P59 | NOT COVERED | 0 |
| P60 | NOT COVERED | 0 |
| P61 | NOT COVERED | 0 |
| P62 | NOT COVERED | 0 |

PROGRESS_END

## P01 Consensus (src/consensus/*, 5343 lines)

Coverage: all seven files read in full at 7c496cd (mod.rs, poa.rs, pos.rs, pow.rs, qc.rs, merkle_tree.rs, split_resolver.rs). Callers outside the part were read only where named below. No consensus file changed between the first-pass base (25ede0b) and 7c496cd, so first-pass line numbers hold.

Confirmed at 7c496cd (re-read, same root cause):
- P01-F01 confirmed. Leader test is per validator and independent: pos.rs:434-452 (threshold), pos.rs:607-621 (producer), pos.rs:797-802 (verifier). Seed has no round term: pos.rs:383-403. Total stake counts every active unslashed validator, also those below min_stake: account.rs:992-998 against pos.rs:754.
- P01-F02, F03 confirmed. poa.rs:383-405 accepts one signer per height. main.rs:1595 starts the producer loop for PoS and PoW only.
- P01-F04 confirmed. The only add_checkpoint caller is pos.rs:993, on local commit at an epoch boundary. pos.rs:715 then refuses any block below it.
- P01-F05, F06 confirmed on the consensus side: pos.rs:715-719 and pos.rs:935-981 (evidence only inside record_block).
- P01-F07 confirmed. add_admin has only test callers (poa.rs:626, 659, 716, 768; poa_onboarding.rs:470). No production code builds PoAEngine with authorities (rg with_authorities, with_config).
- P01-F08 confirmed. pow.rs:309-318 scales the nibble count linearly while work is 2^(4d) (pow.rs:222-226).
- P01-F10 confirmed. Block hash includes nonce and timestamp (block.rs:339-380). PoA validation does not constrain either (poa.rs:353-417).
- P01-F12 confirmed. pos.rs:579-586 moves queued evidence into the block before the leader check at 599-636.
- P01-F13 confirmed. consensus_validator_set_hash (account.rs:1009-1042) runs a PoP check per active validator at pos.rs:737, before the cheap signature check at pos.rs:804. The checks before it (previous hash, epoch, slot) use public values only.
- P01-F16, F20, F21, F23, F24 confirmed (pow.rs:285-307, mod.rs:22 and 139-174, pos.rs:754, poa.rs:252-351).
- P01-F25 partly confirmed: mod.rs:88 still says "mainnet L1's fact" (Z12).
- P01-F14, F15 not re-read: the code is in storage/db.rs, outside this part. Left to P26.

Challenges:
- P01-F27 is partly stale. QcBlob::leaf_hash is now length prefixed (qc.rs:106-118) and the odd tail has its own tag (merkle_tree.rs:46-54). Only combine_sha3 (merkle_tree.rs:32-40) has no tag. A leaf preimage is at least 4+8+8 bytes plus a 2.4 KB signature for any entry that passes verify_against_snapshot (qc.rs:299-315), so it cannot equal a 64 byte inner preimage. Keep as Info.
- P01-F22 holds, with one note: detect_fault_proofs (qc.rs:340-383) only emits proofs for signatures that fail, and verify_against_blob (qc.rs:594-598) refuses valid ones. Because verify_against_snapshot rejects a blob with any bad signature (qc.rs:305-315), no stored blob can contain a faulty entry. The path decides nothing.

New findings:

### SPB-P01-F01
File: src/chain/blockchain.rs:4310-4323; src/consensus/poa.rs:353-417; src/consensus/pow.rs:376-415; src/consensus/pos.rs:808-824
Severity: High
Axis: security
Confidence: Certain (all callers read; rg for slashing_evidence in src outside tests shows no other check)
Evidence: apply_system_effects calls state.apply_slashing(evidences, ratio) for every block that carries slashing_evidence. The only code that verifies block evidence is the PoS branch pos.rs:808-824 (verify_evidence). PoA validate_block and PoW validate_block never read block.slashing_evidence. rg -n 'slashing_evidence' src (no tests) lists no check in blockchain.rs, executor.rs or the proto path.
Description: a block from a PoA authority or a PoW miner can carry evidence entries that no engine checks. The state machine applies them: slash_validator (account.rs:1422-1474) cuts stake, marks the validator slashed, inactive and jailed, and mirrors the slash into the registry.
Impact: on a PoA or PoW chain the block producer can remove any validator in state without proof. In a PoA network an authority can remove the other authorities from the active set (get_active_validators drops slashed ones, account.rs:999-1007), and the first-pass PoA leader checks then accept only the remaining authority. Precondition: be the leader or win one block on a non-PoS engine. The slash does not enter the state root (account.rs:2266-2276 hashes balances and nonces only), so no root mismatch flags it.
Trigger: entry is validate_and_add_block (blockchain.rs:4883) through full_validate then apply_block_effects (4546-4563). Input shape: Block.slashing_evidence with entries whose header signatures do not verify, or whose two headers are equal. Expected correct result: the block is refused. Today: the block is accepted and the named producer is slashed.
Known: none. Related to P01-F11 (chain id binding) but different: that one needs real signatures.
Fix direction for owners: verify block evidence in one chain level function for every engine, not in PoS only.

### SPB-P01-F02
File: src/core/block.rs:151-162 against block.rs:443-510
Severity: Low
Axis: correctness (latent)
Confidence: Certain
Evidence: BlockHeader::verify_signature checks Ed25519 only (block.rs:158-161) and BlockHeader has no producer_public_key field. Block::verify_signature also accepts version 0x05 ML-DSA-87 producers (block.rs:487-509).
Description: if a producer ever signs with ML-DSA-87, its equivocation cannot be proven, because verify_evidence (pos.rs:158-163) uses the header check.
Impact: no effect today: nothing sets producer_public_key (rg shows only genesis and proto conversion).
Known: none.

### SPB-P01-F03
File: src/consensus/split_resolver.rs:101-111; pow.rs:471-490; poa.rs:435-452; pos.rs:910-933
Severity: Low
Axis: correctness (design)
Confidence: Certain
Evidence: for equal weight and height the lower tip hash wins (split_resolver.rs:101-107). The incumbent has no preference.
Description: any rival tip of equal weight replaces the current tip with probability one half. In PoW a miner who finds a competing block at the same height and difficulty displaces the earlier block half of the time. In PoS and PoA a producer that signs two blocks at its height can pick the lower hash after the fact (the PoA double sign is not slashed, see P01-F06).
Impact: depth one reorgs are cheaper than with a first seen rule. Bounded by MAX_REORG_DEPTH (mod.rs:46).
Known: none.

### SPB-P01-F04
File: src/consensus/qc.rs:640-653
Severity: Info
Axis: invariant (domain separation)
Confidence: Certain
Evidence: pq_signing_message hashes tag, epoch, height, checkpoint hash string and validator index. It has no chain id and no validator address.
Description: the checkpoint hash is a block hash and block hashes include the chain id (block.rs:352-360), so the message is chain bound only through that field.
Impact: none today. Stays safe only while checkpoint_hash is always a block hash.
Known: none.

## P09 Chain B (blockchain.rs 3440-8393, storage_economics_tests.rs)

Coverage: blockchain.rs 3440-6837 read in full (production code). Tests 6837-8393 read by name list only (about 75 tests). storage_economics_tests.rs read in part (14-30, 150-400); the persist-failure source scan test (150-197) and the bond tests were read, the rest skimmed by name. Reads outside the part: block.rs 55-230 and 280-520, account.rs 992-1042 and 1410-1475 and 2010-2105, db.rs 514-660, executor.rs 2531-2560, storage_deal.rs 3180-3260 and 3716-3775, deal_open.rs 255-345.

Confirmed at 7c496cd:
- P09-F01 confirmed. blockchain.rs:6247-6283 multiplies the boost share by each deal weight with checked_mul and returns Err on overflow; the call at 4575 uses `?`; produce_block returns None on any Err (4645-4649 region, unchanged). collect_block_transactions (4238-4292) never runs the distribution, so the same transaction is retried each slot. Highest priority of this part.
- P09-F02 partly confirmed (as V13): handle_finality_cert compares the cert hash with the local block (5764-5770) but never compares blob.checkpoint_hash with cert.checkpoint_hash (5782-5797). get_qc_blob falls back to disk (3685-3691), and try_reorg removes blobs from memory only (5268-5270).
- P09-F03 confirmed: 5284-5285 swap memory first, 5393-5427 delete and re-commit afterwards with `?`.
- P09-F05, F06 confirmed as Low (5363-5364 clears snapshots; 5386-5390 rebuilds the mempool with MempoolConfig::default and the top 1000).
- P09-F08 confirmed: apply_qc_fault_verdict (3847-3883) slashes in self.state outside any block and drops the result (3869).
- P09-F10 confirmed: import_qc_blob stores the blob (3949-3957) then returns the error of process_pending_finality_certs (3959).
- Quorum check against the first pass notes: the PQ blob quorum is by count (3936-3947) and the BLS certificate needs both stake and count (finality.rs:119-133, 869-883). The blob must also hold every BLS signer (5792-5797). No mismatch.

Challenges:
- P09-F09 holds but is weaker than written: apply_state_snapshot checks block hash, finalized hash and state root equality with the block at snapshot.height (5611-5651, 5679-5694). A state at an older height is accepted only if its root matches that block, so the state is the true state of that height. The fault is that the node then runs at an old state while its chain is longer. Not wired today (as the first pass says).

New findings:

### SPB-P09-F01
File: src/chain/blockchain.rs:5146, 5399-5408, 5460, 5490-5495; src/chain/blockchain.rs:425, 656-683, 873 (P08 range); src/registry/identity.rs:536-546; src/execution/executor.rs:2425-2446
Severity: High
Axis: correctness (replay path against live path)
Confidence: Certain for the code path. Missing step: a run on a PoA chain that holds an Identity write (the tests in src/tests/identity_tx_door.rs set the domain by hand, never through restart or reorg).
Evidence: IdentityRegistry::apply refuses every write unless domain is PoA (identity.rs:542-546). The executor reads the domain from state.execution_domain (executor.rs:2446). AccountState starts with PoS (account.rs:414, 471, 536, 619). Only the live constructor path stamps it, at blockchain.rs:873, after the startup replay loop (656-683). validate_candidate_chain builds its state at 5146 and rebuild_state at 5460, and neither stamps the domain. try_reorg stamps only the state for the suffix replay (5408), after rebuild_state of the prefix (5400) already ran.
Description: on a PoA node every replay of a block that contains an Identity write gets NotPoaDomain. Startup replay then exits the process with the "Corrupted database" message (blockchain.rs:672-679). A reorg candidate with such a block fails validation (5260). get_state_snapshot returns None (5491 `.ok()?`).
Impact: after the first block with an Identity write, a PoA node cannot restart, cannot reorg and cannot serve a snapshot. All nodes of the network fail the same way on restart. Precondition: PoA engine and one accepted Identity transaction.
Trigger: entry is Blockchain::new_with_genesis with a stored chain (restart). Input shape: stored block with TransactionType::Identity. Expected: replay equals the live result. Today: the replay refuses the write.
Known: none. Related note in the code at blockchain.rs:5404-5407 shows the same bug was fixed for one site only.

### SPB-P09-F02
File: src/core/account.rs:2036-2068; src/domain/deal_open.rs:311-333; src/chain/blockchain.rs:6217-6221, 6589; src/domain/storage_deal.rs:3243-3256, 3716-3758
Severity: Medium
Axis: invariant (supply ceiling)
Confidence: Likely for the double penalty (missing step: intended model; the test at storage_economics_tests.rs:353-381 only asserts that a slashed deal is not repaid). Certain for the supply counting.
Evidence: debit_escrow removes the deal fee from the payer and the bond from the operator and writes them nowhere (deal_open.rs:311-333). total_bud_committed adds balances, validator stake, unbonding and role bonds only (account.rs:2063-2068). Headroom for bridge mints is cap minus that number (account.rs:2071-2075, 1966). Rewards are credited back from "the virtual locked escrow" with try_add_balance (blockchain.rs:6217). A missed challenge sets the deal to Slashed and records the full operator_bond as slash amount (storage_deal.rs:3243-3256); the sweep then burns that amount from the operator liquid balance (blockchain.rs:6589), while the escrowed bond is never returned because expire_deal pays only Active deals (storage_deal.rs:3727-3729, 3755).
Description: BUD held in storage escrow is not counted in committed supply, so while a deal is open the cap check sees extra headroom equal to the escrow. The slash path takes the bond amount twice: once by never returning the escrow, once by burning liquid funds, and the burn is clipped to the liquid balance (account.rs:2082-2102).
Impact: (a) a bridge mint can go above the 100M ceiling by the open escrow total; the excess shows when escrow is released. Precondition: bridge mint path enabled and open deals. (b) a slashed operator loses up to twice its bond, or less than the bond if it keeps liquid funds low, so the penalty does not match the rule in the code comments. Storage economics is off on mainnet today (known gate), so this is a mainnet blocker for the storage economics switch.
Known: P23-F01 (escrow with no tracked lock) is the same root for the missing lock; the supply and double penalty effects are new.

### SPB-P09-F03
File: src/chain/blockchain.rs:4482-4516; src/storage/db.rs:514-600
Severity: Medium
Axis: correctness (scalability, availability)
Confidence: Certain
Evidence: commit_block_durable collects every account in the state (4492-4495) and commit_durable_batch writes all of them (db.rs:588-593), plus a full encoded BridgeState under BRIDGE_STATE_AT:{height} on every block (db.rs:583-587), then flushes. rg BRIDGE_STATE_AT shows deletes only for an interrupted commit (db.rs:484-489); no pruning path removes them.
Description: per block disk work grows with the number of accounts, and the BridgeState copies grow with height times bridge state size. The in memory state is cloned at least three times per accepted block (validate_and_add_block 4957, apply_block_effects 4552, and the producer path 4247 and 4552).
Impact: block time and disk use degrade with chain age and account count; a node that prunes blocks (5107-5118) still keeps every bridge snapshot. No consensus break.
Known: none.

### SPB-P09-F04
File: src/chain/blockchain.rs:5229-5260, 5399-5400
Severity: Medium (latent)
Axis: security (DoS ordering)
Confidence: Likely (missing step: the sync path hands over a suffix today, so the check at 5233-5238 stops it first; see P01-F05b)
Evidence: try_reorg runs consensus.is_better_chain, find_fork_point and a depth test, then validate_candidate_chain (a full replay from genesis with signature, tx and state root checks per block, 5137-5219), and later rebuild_state (a second full replay, 5400). The only gate before the replay is the tip tie break, where a candidate of equal length wins with probability one half by having the lower tip hash (split_resolver.rs:101-107), and the tip hash is recomputed from content, so a crafted tip can be ground cheaply.
Description: a candidate that fails only at its last block costs two full replays.
Impact: once sync passes full candidates, any peer can make the chain actor spend time proportional to chain length per message while the actor serves no other command.
Known: none.

### SPB-P09-F05
File: src/chain/blockchain.rs:3464-3555
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: submit_registry_slashing_report debits the reporter fee from self.state (3501-3513), runs the registry slash (3519) and refunds on success (3521-3528). On Err the debit stays (3549-3553). All of it runs outside block execution.
Description: this is a state change on one node only. Balances are in the state root, so a node that burned a fee differs from its peers. Only reports with provenance other than Unverified reach it (3490-3497).
Impact: low today because no caller supplies a verified provenance except the consensus aggregator path; the vote handlers call it (5842, 5866, 5889, 5912) and ignore the result, so a vote based report can change the balance of one node.
Known: related to the off-block mutation class (P24-F05, P21-F04).
