# Stop report: duplicate WP-04 ownership

## Decision

Stop work on WP-04 in this session. Release its path locks by closing PR #21.
Do not merge either duplicate automatically. The earlier public claim, PR #18,
remains the owner record for the other agent to continue.

This is the concrete scope-conflict stop condition in directive sections 0.3
and 15.1. It is not a claim that the repository has no remaining work.

## Conflicting requirements

1. Section 15.1 items 2-3: one agent executes a package, and an agent may not
   change a path locked by another open package.
2. The user asks this session to open a PR and implement work not already in
   recent commits or live open PRs.
3. The initial live read at 2026-11-06T05:13Z showed PRs #8, #9, #11, #12 and
   #13, with no WP-04 claim. This session selected WP-04's independent first
   delivery and began work from the same main SHA.
4. The required live recheck after opening PR #21 showed PR #18, created at
   2026-11-06T05:19:58Z, already claiming WP-04. PR #21 was created at
   2026-11-06T05:24:02Z. Both PRs change the same package paths.

## Evidence

| Item | Live evidence |
|---|---|
| Main | `cccf1da1f4bcfe97503a3a00cfba24b172b49b31`, rechecked with `git ls-remote origin refs/heads/main` at 2026-11-06T05:24:59Z |
| Earlier claim | PR #18, `WP-04: start lane D live verification and behavioral regression tests`, head `3e37bc137db9384d9f26e7b9f98454feb43a1baf` |
| Earlier claim text | `CLAIM: Arena coding agent, lane D. Start commit: cccf1da1... Locks: docs/audit/live/lane-d.md and src/tests/audit_lane_d.rs.` |
| This session | PR #21, head `24b55263a6939e31896e3c60c1162f600f4d3aa6`, branch `arena/bfdb31fd-budlum` |
| Exact overlapping paths | `docs/audit/live/lane-d.md`, `src/tests/audit_lane_d.rs`; `src/tests/mod.rs` is shared append-only |
| This session's non-overlapping addition | `docs/audit/live/ci-baseline.md`, 30 workflow records and the current-main failed-job inventory |
| GitHub API file comparison | PR #18: lane ledger 85 added lines, test module 117; PR #21: lane ledger 161, CI baseline 110, test module 223 |
| Issues | `hasIssuesEnabled=false`; enabling returned HTTP 403, `Resource not accessible by integration` |
| Base CI | Main run 42050659038 is red; failed jobs/steps and retrieved annotations are preserved in PR #21's `ci-baseline.md` |
| This PR's CI | Checks were queued when the conflict was found; no green or red test result is claimed |

The initial selection was based on a real live read, but the queue changed
while the audit was being written. GitHub Issues could not provide an atomic
claim before coding. A PR body is visible only after publication, so two
agents can select the same free package concurrently. That coordination gap
must be fixed before more parallel implementation claims are accepted.

## Preserved work

Closing PR #21 does not delete its commits or branch. The following remain
available for the WP-04 owner to review or cherry-pick with attribution:

- `d3c5009`: the 70-record lane D ledger, CI baseline and initial regression
  locks.
- `24b5526`: bind the identity count to the findings table and correct live
  anchors.

No production behavior was changed. No finding is closed by this report.
The nine pre-fixed observations in PR #21 still require executed regression
tests, a merge SHA and a green CI run before closure.

## Recommended resolution

1. Keep PR #18 as the single active WP-04 claim. Close PR #21 as a duplicate
   without merging it.
2. Let the WP-04 owner review the CI baseline and the additional ledger/test
   evidence from PR #21. Incorporate useful work explicitly; do not overwrite
   the owner's file wholesale.
3. The repository owner should enable GitHub Issues and permit the integration
   to create issues, comments and labels. Then create the 64 WP records and
   make CLAIM on the issue before opening implementation PRs.
4. Reconcile the other duplicate lane-C claims (#12, #15 and #16) under the
   same one-owner rule. This report does not choose a lane-C owner.
5. Re-evaluate readiness only after dependencies are actually merged on main.
   An open PR, an imported snapshot or a partially present module is not a
   merged dependency. Do not repeat the invalid claim in PR #20 that WP-05 is
   ready while WP-04 is merely in progress.
6. After WP-04 merges and its locks release, WP-42 is the next lane-D candidate
   only if its lock paths are free. Its CI/gate work includes the red base
   recorded here. Until then, do not edit its locked workflows or gates from
   this duplicate package.

## Session constraints

Arena fixes this session to `arena/bfdb31fd-budlum`. The directive's
`wp/04-*` and `audit/repro-d` branch names cannot be created here. This report
records the deviation rather than switching branches. The report path is
session-specific and does not overlap PR #17 or #19.

## Status

- WP-04 in this session: **blocked by duplicate ownership; locks released**.
- PR #21: close without merge.
- Main: unchanged by this report.
- Package completion, finding closure, CI success and testnet readiness:
  **not claimed**.
