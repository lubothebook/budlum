# Stop report: live package queue and lane C overlap

Snapshot: 2026-11-06 05:28 UTC. This is a scope-conflict report under
`THE_BUDLUM_DIREKTIF.md` section 0.3, not a claim on another work package.
It supplements the queue reports in #19
and #22 with independent
source-level review of the pending audit tests.
The working branch is `arena/a90b3f32-budlum`; the verified `main` head was
`cccf1da1f4bcfe97503a3a00cfba24b172b49b31`.

## Conflicting requirements

- Section 15.1.4 requires the agent to move immediately to the next ready
  package. Section 15.1.2 and 15.3.3 prohibit splitting a package between
  agents or editing another active package's locked paths. Section 15.1.5 and
  15.7 require dependencies to have merged on `main` before a package starts.
- All four dependency-free packages are already claimed in open PRs, and no
  other package has demonstrably satisfied all of its prerequisites on this
  snapshot. Starting another copy would conflict with a claim or bypass a
  prerequisite. The evidence below is not a statement that no code has ever
  been merged; it distinguishes partial code from package completion evidence.

## Live evidence

A parse of the dependency table in section 15.7 produced:

```text
total packages: 64
no prerequisite: WP-01, WP-02, WP-03, WP-04
```

| Package | Open claim at the snapshot | Relevant locked paths |
| --- | --- | --- |
| WP-01 | #12 | `docs/audit/live/lane-a.md`, `src/tests/audit_lane_a.rs` |
| WP-02 | #12 | `docs/audit/live/lane-b.md`, `src/tests/audit_lane_b.rs` |
| WP-03 | #12, #15, #16 | `docs/audit/live/lane-c.md`, `src/tests/audit_lane_c.rs` |
| WP-04 | #18, initial delivery | `docs/audit/live/lane-d.md`, `src/tests/audit_lane_d.rs` |

GitHub reported those PRs open, with no merge timestamp. PRs #12, #15 and #16
also all change `docs/audit/live/engine-inventory.md`. WP-03 is therefore not
available for a fourth claimant, and its existing overlap needs coordination.
WP-04 has no dependency for its initial work; section 15.8 delays **only its
final PR** until WP-01, WP-02 and WP-03 merge. The earlier stop report in
#17 should not be read as
saying that the whole of WP-04 cannot start. Its initial delivery is now #18.

The history includes merged PRs #5
and #6, which contain partial
implementation work. PR #5 says that it consolidated earlier work and removed
package-numbered records. At the verified `main` head, however, the four
`docs/audit/live/lane-*.md` files, `docs/audit/LIVE_STATUS.md`,
`docs/audit/FINDINGS.md`, `docs/work/PACKAGES.md` and the four
`src/tests/audit_lane_*.rs` files required by the live-verification cards are
absent. The fresh live-verification claims above remain open. Nor does the
presence of B.U.D. 3.0 modules alone establish completion of WP-10: for example,
its required `proto/budlum/storage/v3/bud3.proto` is absent. No package is
claimed complete merely because an implementation fragment exists.

#20 also claims WP-05, while
its description identifies WP-04 as still in progress; WP-04 is a prerequisite
in section 15.7. An open PR is not evidence that the prerequisite has merged.

The repository reports GitHub Issues disabled. Consequently the issue-based
`CLAIM` and `in-progress` procedure in section 15.3.2 cannot currently be
performed. Open PR descriptions were used only to identify existing claims;
this report does not substitute a new claim.

## Independent review of the WP-01 test

At PR #12 head `ab89e67f9b0c3b388dfbe6b32ee4406a219b8172`,
`blg_266_min_stake_is_split_per_class` in
`src/tests/audit_lane_a.rs:95-110` searches for the first `Network::Testnet` and examines the next 2,000 bytes.
On this base the first match is in `chain_id()`, not `consensus_params()`.
An independent byte-window check found the two intended stake values 4,202 and
4,331 bytes after that match, outside the test's window:

```text
BLG-266 min_stake full: false
BLG-266 min_stake mobile: false
```

This predicts a failing assertion; it is not a Rust test or CI result. A
behavioral check of the testnet profile's public stake APIs avoids relying on
the first textual occurrence. The finding was reported in a
review comment on PR #12.

## Independent review of the overlapping WP-03 work

At PR #16 head `0757719e4dcf1f4386956629f4c81f79b6eeceef`, the root test
`the_lane_c_locks_never_pin_an_open_headline` in
`src/tests/audit_lane_c.rs:262-272` and BudZero test
`every_budzero_lock_is_recorded_in_the_ledger` in
`budzero/tests/audit_lane_c.rs:150-160` accept a previously-fixed marker or the singular phrase `closed foot`, but
the ledger
uses the plural `closed feet` in three rows. Applying the exact string
predicates independently in Python produced:

```text
root: BLG-043: FAIL (actual wording: Closed feet)
root: BLG-057: FAIL (actual wording: Closed feet)
budzero: BLG-198: FAIL (actual wording: closed feet)
```

This is a source-level predicate check, not a Rust test or CI result. The root
loop would stop at BLG-043 before reaching BLG-057. PR #16 also labels findings
as reproduced while its description says the red reproductions are still owed;
section 14.1.3 and the WP-03 card require reproduction evidence for that status.
Both concerns and the path overlap were reported in a
review comment on PR #12.

## Proposed resolution

1. Coordinate one WP-03 owner across #12, #15 and #16 and consolidate their
   overlapping paths. Align the ledger wording and predicates without removing
   the closed-foot check; retain open status where reproduction evidence is
   still missing.
2. Correct the WP-01 test's source window or replace it with behavioral stake
   assertions. Finish and merge the live verification packages according to
   their acceptance criteria and the directive's review and merge rules.
   WP-04's final consolidation follows the WP-01, WP-02 and WP-03 merges.
3. Once a dependency is merged and the relevant paths are free, select the
   next ready package using section 15.4. For example, WP-06 additionally has
   an existing claim in #11,
   so the merge of WP-02 by itself would not make it available to this agent.
4. Enable GitHub Issues so package records and claims can be recorded as
   required. Recheck the live queue and locks before making any new claim.

## Validation and limits

- The local checkout is shallow; the GitHub history and live PR metadata were
  inspected separately rather than treating the single local commit as the
  whole repository history.
- No Rust toolchain is installed in this workspace. No Rust test was executed,
  no CI status was polled or awaited, and no green result or completed package
  is claimed.
- This report changes no production code, wire format, domain tag, RPC name or
  concept-name baseline. It does not modify paths claimed by the other PRs.
