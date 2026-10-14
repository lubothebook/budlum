# PR 11 continuation record

## Scope and authority

- Repository: `lubothebook/budlum`.
- Working branch: `arena/6bd1c076-budlum`.
- Open work: [PR 11](https://github.com/lubothebook/budlum/pull/11).
- Source: [PR 9](https://github.com/lubothebook/budlum/pull/9), imported through
  `4a2132fa60681c8a38f1d6b44a1db227994e779c`. PR 9 was not changed or closed.
- The owner uploaded the two Arena directives in `ba81620`. Merge `30e0331`
  retains them without rewriting either document or the owner's commit.
- The explicit session instruction is to do the work without sub-agents.
  No sub-agent has been opened. No independent approval is claimed.
- The finding ledger remains `docs/bulgular/BULGU_DEFTERI.md`. This document
  records execution and evidence; it does not replace or discard that ledger.
- GitHub Issues are disabled, verified through the repository API on 2026-10-14.
- All validation is on GitHub CI. No local cargo, formatter, linter or test run.

## Current phase

Cycle 1, implementation and CI evidence collection. The new directives have
been read in full. Work started before their upload remains identifiable by
commit. Existing test-first evidence is not invented after the fact.

The inherited ledger contains 1,099 findings: 787 open, 218 awaiting CI,
87 awaiting a decision, and 7 labelled false positive. These are inherited
labels, not new review decisions. No finding is declared merged or complete
by this continuation. A passing job on a failed run is not a green whole PR.

## Packages and ownership

All packages below are implemented by the current session on the one working
branch. They are class C. Each owns only the files listed. No package opens
a production opcode, changes consensus bytes, changes an economic parameter,
or introduces a dependency.

| Package | Finding | Owned files | State and evidence |
|---|---|---|---|
| GP-11-01 | BLG-0004; P57-F02 / BLG-1029 | `.github/workflows/ci.yml`, `xtask/gates/src/main.rs`, `src/crypto/domain_tags.rs` | `2c5d73d`: gate failure annotations, annotation escaping test, relayer tag inventory, tooling tests. Follow-up moves tooling tests, clippy and fmt into an independent matrix job. |
| GP-11-02 | P54-F01..F03 / BLG-0966..0968 | `xtask/gates/src/gates/semver.rs`, `.github/semver-exceptions.txt`, `.github/workflows/semver.yml` | `dc80dd6`: an old exception cannot approve unrelated breakage; baseline absence fails. Test and canary changes await CI. No version bump or API compatibility waiver. |
| GP-11-03 | P57-F03..F04 / BLG-1030..1031 | `xtask/tools/src/smoke_rpc.rs` | `4f92dc4`: never delete an existing database path; validate the RPC response and chain id; reject occupied ports; check child exit before accepting a reply. Tests cover malformed/error replies, wrong network, occupied ports, existing paths and symlinks. |
| GP-11-04 | P57-F06 / BLG-1033 | `xtask/tools/src/fsf_fit.rs` | `4f92dc4`: self-test no longer bypasses the document check; check plus write is refused without writing. Two regression tests. |
| GP-11-05 | P51-F12 / BLG-0905 | `xtask/gates/src/gates/coverage.rs` | `5064f1c`: reject non-finite and out-of-range percentages. Partial: enforcement against a lower but valid historical floor still needs work. No baseline value changed. |
| GP-11-06 | P56-F01 / BLG-1012; related P51-F05 and P52-F07 | `xtask/gates/src/gates/named_tests.rs` | `5064f1c`: explicitly test the ignored-with-reason case. The exact-match fix already exists in inherited `4cc9b67`; it is not counted as a new fix. |
| GP-11-07 | Owner directive text calibration | `.quality/typos.toml`, `xtask/gates/src/gates/tree_is_english.rs` | Register only the two exact owner-uploaded directive names under the existing Turkish directive policy. The files themselves stay unchanged. No source directory or filename prefix exemption. |

Independent packages can progress while CI is queued. No package is approved
or merged by its author. Regression fixes and verification remain separate
claims. These class C changes include tests in the fix commits; they do not
have a test-only red run and are not represented as red-green proof.

## Live evidence

- Starting CI: PR 9 run `38428202021`, head `4a2132f`.
- Secret Scan job `115405636367` succeeded, including the gitleaks canary and
  full-history scan. This supplies scoped evidence for inherited `b35dd86`
  (BLG-0001), but does not make the entire PR green or merged.
- Core quality job `115405635789`: normal clippy and fmt succeeded; the
  pedantic/nursery ratchet failed. Do not diagnose it as a format failure.
- Core library and coverage jobs exceeded 90 and 45 minutes respectively.
  Several duplicate full-library gate runs exceeded six hours. Their
  underlying test failure or stall is not yet established from available logs.
- PR 11 run `38559171428` at `2c5d73d` was cancelled after the owner's upload
  triggered a newer run. It is not test evidence.
- PR 11 CI run `38560075471` at `ba81620` was queued at the last read.
- Typos run `38560075430`, job `115838674284`, failed after the upload.
  Checks annotations report the Turkish word `Analist` as an English typo.
  The owner directive policy already preserves such documents in Turkish.
- `gh run view --log-failed` could not reach the Actions log download host.
  The GitHub API for job steps and annotations works. Failure annotations
  and the independent tooling job retain readable evidence without masking
  a command's exit status.

## Question queue and constraints

Do not use `ask_user` while independent, unblocked work remains.

1. The uploaded generic orchestration rules request sub-agents and independent
   reviewers; the direct session request forbids sub-agents. Continue solo
   implementation. Independent approval and merge remain unfulfilled, not
   simulated. No sub-agent management API is present among the session tools.
2. The uploaded reference names `budlum-xyz/budlum`, PR 3, PR 50, `deneme`
   and `workspace`; this session is explicitly assigned to `lubothebook/budlum`
   and PR 9. PR 3 in this repository is closed and is a different CI task.
   Work on the supplied PR 9 ledger remains in scope. Do not invent other
   repository locations, issues or agent IDs.
3. The generic branch convention conflicts with the fixed session branch.
   Retain `arena/6bd1c076-budlum`; no branch switch or force push.
4. The ledger's 87 decision-waiting records remain pending. In particular,
   proposer liveness, consensus-state format, proof constraints, economic
   changes, the code-owner team and deployment trust roots are not decisions
   that can be silently filled in by a CI cleanup.

## Next actions

1. Push the retained owner documents, implemented packages and this record to
   PR 11. Read the independent tooling matrix results before claiming success.
2. Fix any compile, format, lint or regression failure without changing the
   required verdicts or loosening a baseline.
3. Read gate annotations to identify the original Repo Lint and clippy-ratchet
   failures. Keep the original full-library timeout investigation open.
4. Continue the next unblocked ledger item while CI is queued. Use a separate
   test-only commit before a new non-class-C defect fix. Never replace missing
   red-green evidence with a statement that tests should pass.
5. Update this record and PR 11 when evidence changes. The main commit whose
   entire required CI is green has not yet been established from live data.
