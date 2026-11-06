# Findings ledger

Section 14.1 item 9: every new problem found while working is written here, with
an identifier of BLG-268 or later. The 267 records of the directive's own ledger
are not repeated. Their live state is `docs/audit/LIVE_STATUS.md`, which is
generated from `config/audit/ledger.json`, and their subject text stays in the
directive, which is not in the tree (section 0.1 item 5).

A record stays `open` until the commit that closes it is on the branch and CI is
green (section 14.1 item 5). A self-report is not evidence (section 0.3 item 5),
so the closing commit and the CI run are columns, and a row without them is
open. Nothing in this file is closed.

## Recorded

| Id | Severity | Area | Subject | Evidence | Closes with | State | Commit | CI run |
|---|---|---|---|---|---|---|---|---|
| BLG-268 | Medium | repository layout | The binding directive is committed in the tree. `THE_BUDLUM_DIREKTIF.md` is 556 KB of Turkish prose, and section 0.1 item 5 says the document does not enter the repository, which is English because of the `tree_is_english` gate. Two of the repository's own rules therefore point at the same file in opposite directions: the file must be readable by every agent (it is the only authoritative directive) and it must not be in the tree. Until the decision is taken, every reader of the tree sees a 556 KB exception and every translation pass has to step around it. | `THE_BUDLUM_DIREKTIF.md` present at the root; `git ls-files` lists it; the same file is removed by open pull requests #42 and #63 | WP-42 | open | none | none |
| BLG-269 | Medium | mobile lane | The mobile lane has no tree. Three packages lock paths under `mobile/` (WP-36 Android validator, WP-37 device matrix, WP-38 iOS application) and the directory does not exist, so the lock-path coverage of those three cards is 0/2, 0/3 and 0/2. The concept-name gate lists `mobile` in its scan roots, so the gate walks a root that is not there. That is quiet rather than wrong today, and it becomes a silent hole the day the tree appears in a pull request that also carries a legacy name. | `mobile/` absent; `concept_names::SCAN_ROOTS` in `xtask/gates/src/gates/concept_names.rs` lists it; `docs/work/PACKAGES.md` rows WP-36, WP-37 and WP-38 | WP-36, WP-37 | open | none | none |
| BLG-270 | Medium | delivery process | The three ledgers the directive requires were absent from `main`, and the queue had no canonical copy of any of them. Milestone M0 asks for `docs/audit/LIVE_STATUS.md`, section 15.5 item 7 asks for `docs/work/PACKAGES.md` at the close of every package, and section 18 asks for `docs/audit/TLG_REPORT.md`; none existed. Twenty-one open pull requests add a `docs/work/PACKAGES.md`, each with different content, so the answer to "what is the state of WP-14" depended on which pull request a reader opened. This delivery adds the three ledgers and the script that generates them; the finding closes when they are on `main` and a CI run shows the gate green. | `docs/audit/LIVE_STATUS.md`, `docs/work/PACKAGES.md` and `docs/audit/TLG_REPORT.md` absent on `main`; the queue measurement in `docs/audit/QUEUE_COLLISION.md` | this delivery (no package owns a ledger path) | open | none | none |
| BLG-271 | Low | RPC documentation | The `METHOD_CLASSES` table in `src/rpc/method_class.rs` says of itself that it lists "Every method `src/rpc/api.rs` declares, in declaration order", and it lists 183 rows against 181 `#[method(name = ...)]` declarations. The two extra rows are `bud_subscribe` and `bud_unsubscribe`, which `api.rs` declares with a `#[subscription(...)]` attribute rather than `#[method]`. The table is right and the sentence is wrong: a subscription is served by the same table and is not a method, so a reader who counts the two sides finds a defect that is not there, and a tool that compares the counts reports one. | `src/rpc/method_class.rs` row count 183; `src/rpc/api.rs` `#[method]` count 181 plus one `#[subscription]` | this delivery (the comment is corrected) | open | none | none |
| BLG-272 | Medium | documentation gate | The three README checks the directive names do not exist. TLG-23 rests on `readme_definition_block_is_verbatim`, `readme_has_no_placeholders` and `readme_claims_are_wired`, and the tree carries none of the three: the only README gate is `readme_no_deny`, which checks that the README does not advertise `cargo deny`. The README therefore states an unmeasured test count (a badge of 4365) and a launch paragraph that calls the bootstrap peers, the DNS seeds and the allocations placeholders, with nothing that fails when either drifts. | `xtask/gates/src/main.rs` registers `readme_no_deny` and no other README gate; `README.md` badge line and the paragraph at line 294 | WP-43 | open | none | none |

## How these were found

Every record above was read out of the working tree by
`ops/scripts/live_audit.py` or by a check a reader can repeat in one command:

* BLG-268: `git ls-files THE_BUDLUM_DIREKTIF.md`.
* BLG-269: `test -d mobile`, and the scan roots at the top of
  `xtask/gates/src/gates/concept_names.rs`.
* BLG-270: `ls docs/audit docs/work` on `main`, and the queue measurement in
  `docs/audit/QUEUE_COLLISION.md` for the count of pull requests that add the
  same file.
* BLG-271: count `row("` in `src/rpc/method_class.rs` (183) and `#[method(name`
  in `src/rpc/api.rs` (181).
* BLG-272: `grep -n readme_ xtask/gates/src/main.rs`.

Three records carry `this delivery` in the closing package column because no
package card owns the path: the directive's section 15.2 path table has no entry
for `ops/scripts/` or for the audit ledgers, which is itself the reason the
ledgers were missing. That gap belongs in the next revision of section 15.2, and
it is recorded here rather than papered over.
