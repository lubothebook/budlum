# WP-04 blocker handoff: duplicate ownership and unmerged dependencies

Session: `arena/23b67c2a-budlum` (lane D)
Recorded: 2026-11-06, live queue check at 05:53:31 UTC
Main: `cccf1da1f4bcfe97503a3a00cfba24b172b49b31`
Status: this session's duplicate WP-04 claim is withdrawn; WP-05 is not ready.
This is a blocker report, not a new work package and not a completion claim.

## Conflicting requirements

1. The request asks this session to implement packages absent from recent
   commits and live open PRs, and to follow the binding directive strictly.
2. `THE_BUDLUM_DIREKTIF.md:2939` assigns one agent to each package;
   `:2942` requires every dependency to be merged into `main` and every lock
   path to be free; `:2991` forbids changing another open package's lock path.
3. The only dependency-free packages are WP-01, WP-02, WP-03 and WP-04.
   All four already have earlier open claims. Every other dependency row
   requires a package or milestone that is not merged on the live main SHA.
4. This session's earlier WP-04 delivery duplicates the earlier claim in
   PR #18. Its WP-05 delivery also treated WP-04 being present on the same
   branch as a merged dependency. That interpretation contradicts `:2942`,
   `:3058`, `:3066` and the WP-05 card at `:3220`.

This is the concrete scope-conflict stop condition in directive `:70`.
The required report fields are supplied here under directive `:72`.

## Live evidence

The dependency table was parsed from section 15.7: 64 rows, exactly four
with no prerequisite (WP-01..WP-04), and 60 requiring merged packages or
milestones. The following earlier claims were still open at the final check:

| Package | Earlier open claim | Created (UTC) | Head at check |
|---|---|---|---|
| WP-01, WP-02, WP-03 | [#12](https://github.com/lubothebook/budlum/pull/12) | 05:07:33 | `ab89e67f9b0c3b388dfbe6b32ee4406a219b8172` |
| WP-03 | [#15](https://github.com/lubothebook/budlum/pull/15) | 05:17:04 | `581e381a6881f005a3af4ab40912daae3c808891` |
| WP-03 | [#16](https://github.com/lubothebook/budlum/pull/16) | 05:18:57 | `0757719e4dcf1f4386956629f4c81f79b6eeceef` |
| WP-04 | [#18](https://github.com/lubothebook/budlum/pull/18) | 05:19:58 | `3e37bc137db9384d9f26e7b9f98454feb43a1baf` |

PR #18 explicitly claims `docs/audit/live/lane-d.md` and
`src/tests/audit_lane_d.rs`. This session's PR #32 was created at 05:41:34
and changed those same paths. PR #35 later proposed the same lane ledger and
`docs/audit/LIVE_STATUS.md`. An open PR is not a merged dependency.

The live main tree contains none of these required audit outputs:

- `docs/audit/live/lane-a.md`
- `docs/audit/live/lane-b.md`
- `docs/audit/live/lane-c.md`
- `docs/audit/live/lane-d.md`
- `docs/audit/LIVE_STATUS.md`
- `docs/audit/FINDINGS.md`
- `docs/work/PACKAGES.md`

Their absence was checked with `git cat-file -e origin/main:<path>` after
fetching main. The recent main history was also read through the GitHub API;
the latest codec and dedup commits do not merge the four live-audit packages.

GitHub Issues are disabled: the repository reports `has_issues: false`, and
the prescribed package-record creation returned HTTP 410, "Issues has been
disabled in this repository." The issue CLAIM/in-progress protocol at
directive `:2985-2990` therefore cannot be completed. A PR-body substitute is
not an atomic claim and did not prevent the duplicate selections above.

## Correction already pushed

The implementation changes were reverted rather than left as competing locks:

| Commit | Effect |
|---|---|
| `6f71f3cf9344c9e71d27351f195d4519f6ee4df1` | Preserved original WP-04 audit draft and locks for handoff only. |
| `6f241c55580d22fd33fa4bd68642cba0f3a1a05e` | Preserved original WP-05 draft for handoff only; its readiness claim was invalid. |
| `d389085e0174385b85d40da707396722dd534048` | Reverted WP-05; pushed immediately. |
| `d5c17325d58acd3da521f38bc08d6959df8da61f` | Reverted duplicate WP-04; pushed immediately. |

After these reverts the branch has no implementation diff against main.
This report is its only proposed addition. The original commits remain
available for the rightful package owner to review or cherry-pick with
attribution; they are not evidence that a test ran or a finding closed.

The earlier audit's "Closed (pre-fixed)" labels are withdrawn as closure
claims. Source inspection and a parser check are not executed green regression
evidence, a merge SHA and a CI run number. No such result is claimed here.

## Proposed resolution

1. Keep one active owner per audit package. PR #18 is the earlier public
   WP-04 claim; this session does not take it over. Reconcile the duplicate
   lane-C claims without overwriting another owner's work.
2. The repository owner should enable Issues and permit the integration to
   create records, comment and label. Then create the 64 WP records and make
   CLAIM before implementation, as the directive requires.
3. Complete the four audits with their required execution evidence and merge
   them only through the directive's green-CI squash rule. Do not merge an
   audit merely to unblock coding while its evidence is missing.
4. Re-read main and every open lock after those merges. WP-05 becomes a
   candidate only after WP-04 is actually merged and its gate paths are free.
   Do not stack it on an unmerged audit and call the dependency satisfied.
5. If this fixed Arena session must continue implementation before the queue
   is reconciled, the user/repository owner must explicitly resolve the
   ownership conflict. This report does not silently weaken the directive.

## PR evidence fields

- Package and order: WP-04 blocker handoff, 1/1; no implementation claim.
- Findings closed: none.
- Lock paths: this report only; the duplicate audit and WP-05 paths are released.
- Measurements: none. Queue counts are repository observations, not hardware
  measurements.
- Wire formats, field tags and RPC names: unchanged.
- Concept-name counter: unchanged from main; no baseline is edited.
- Validation: dependency-table parse and live path/PR checks above; ASCII and
  whitespace checks on this report. No Rust test or CI success is claimed.
- CI: not polled or rerun for this correction. No running red job was observed
  in this pass, so no unrelated CI repair was attempted.
