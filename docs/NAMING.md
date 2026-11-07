# Naming: the concept map and its counter

The directive keeps two concept names, BudZero and B.U.D., and replaces every
other architectural metaphor with the professional term that states the
function. This document is the human-readable half of that rename; the machine
half is `config/naming/concept_map.toml`, which lists, per retired name, the
regular expressions that find it, the paths it may appear in, and the work
package that owns the rename.

The map is written verbatim from the directive's Appendix C, so the retired
spellings themselves live there and are not reproduced here: the tree is
English, and a code block full of Turkish identifiers is not. What this file
records is the part a reviewer needs, which is who owns each rename and what
replaces it.

## How the count works

`xtask/gates` runs `concept-names`. The gate reads the map, scans `src`,
`crates`, `budzero`, `execution`, `mobile`, `xtask`, `docs`, `config`, `proto`
and `contracts`, and counts two numbers per record: the files that still carry
the retired name, and the lines that carry it. A line that carries the name
three times counts once, because the work is to rewrite the line.

The numbers are recorded in `.github/concept-names-baseline.txt`. The count
only falls:

* a record that gains a file or a line fails the gate,
* a record whose count falls must have its numbers lowered in the same change,
  so finished work is recorded instead of quietly banked,
* the map starts in `count` mode. WP-43 sets `absolute` at the end of
  T-Launch; from then on a single hit fails.

To lower the baseline, run the gate, read the numbers it reports for the record
you renamed, and write exactly those numbers into the baseline in the same
change. Nothing in the baseline was guessed: every line was measured on this
tree.

## The table

| Replacing term (code) | Term (English) | Area | Owner | Retired spellings |
|---|---|---|---|---|
| `hash_chain_election` | hash chain leader election (HCLE) | consensus | WP-12 | 1 |
| `faucet` | testnet faucet | operations | WP-41 | 1 |
| `QuantumAlertLevel::{Advisory, Restricted, Isolated}` | quantum alert level 1, 2, 3 | security | WP-12 | 3 |
| `shared_security` | shared security | settlement | WP-44 | 2 |
| `app_binding` | remote app binding | settlement | WP-44 | 2 |
| `data_market` | Data Market | application | WP-23 | 1 |
| `budlum-explorer` | Budlum Explorer | application | WP-40 | 2 |
| `explorer_evidence` | explorer evidence model | application | WP-23 | 1 |
| `identity_profile` | identity profile | application | WP-23 | 1 |
| `nft_folders` | NFT folders | application | WP-23 | 1 |
| `education_domain` | education domain | application | WP-23 | 1 |
| `app_registry` | application registry | application | WP-23 | 1 |
| `project_manifest` | project manifest | application | WP-23 | 1 |
| `content_gateway` | content gateway | budzero storage | WP-23 | 1 |
| `state_recovery` | state recovery | chain | WP-06 | 1 |
| `sigagg` (crate `bud-sigagg`) | signature aggregation | budzero | WP-33 | 1 |
| `bud3::Object`, `ObjectCommitment`, `bof1` | B.U.D. 3.0 object and BOF-1 | budzero storage | WP-19 | 3 |
| `src/storage/bud3/*` | B.U.D. 3.0 storage tree | budzero storage | WP-07 | 1 |
| `wide_stripe` | wide stripe | budzero storage | WP-24 | 2 |
| `access_tiering`, `TierTransition` | access tiering | budzero storage | WP-24 | 2 |
| `(removed)` | removed; replaced by the Mobile validator class | budzero storage | WP-07 | 2 |
| `sealed_object`, `ciphertext_chunks`, `plaintext_root`, `deletion_set` | sealed object and its parts | budzero privacy | WP-56 | 8 |
| `private_transfer_v1` | Private Transfer Protocol 1 (PTP-1) | budzero privacy | WP-56 | 1 |
| `DomainFinalityProof`, `BlockAggregateProof`, `RemoteStateView` | domain finality and block aggregate proofs | budzero | WP-60 | 3 |
| `ai_verification`, `VerificationLevel`, `AnswerProofBundle` | AI Verification Layer, VL-0 to VL-3, Answer Proof Bundle | ai | WP-45 | 5 |
| `budzero core`, `budzero-rv32`, `budzero-guest` | BudZero core, BudZero RV32, BudZero guest SDK | budzero | WP-56 | 4 |
| `object_layer` | Universal Object Layer (UOL) terms | object layer | WP-49 | 16 |

## Adding a record

When a rename finds a retired name that the map does not list, add a record to
the map with `old`, `patterns`, `new_code`, `term_en`, `term_tr`, `area`,
`owner` and `reason`, and add its numbers to the baseline in the same change.
Adding a record is an addition to the map and nothing else: the package that
owns the new name owns the new record.

## What the gate does not do

It does not rename anything. It counts, and it refuses a count that rises. The
rename itself belongs to the work package in the Owner column, and the record
stays until that package reaches zero.

It also cannot read every regular expression. The supported subset is literal
characters, an `(?i)` flag at the start of a pattern, `\b`, `^`, `$`, character
classes with ranges and negation, single-level groups with alternation, and the
`?` quantifier. A pattern outside that subset is a finding: the gate says it
cannot read the pattern rather than reporting the tree clean. If a new record
needs more syntax, extend the matcher and its canaries in the same change.
