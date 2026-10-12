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
| P09 | NOT COVERED | 0 |
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
