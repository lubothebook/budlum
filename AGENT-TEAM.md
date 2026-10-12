# Budlum Agent Team

This file defines the agent team for Claude Code on the Budlum repos.
Goal: mainnet preparation. Language: ASD-STE100.

## Install

1. Put the four agent files in `.claude/agents/` in the repo root.
2. Put this file next to `CLAUDE.md`.
3. Add this line to `CLAUDE.md`: `Read AGENT-TEAM.md before each task.`

## Team

| Agent | Model | Job | Writes code |
|---|---|---|---|
| `scout` | Haiku | Find files, symbols, call sites, tests. | No |
| `planner` | Opus | Audit the task. Make the plan. Split into PR-sized steps. | No |
| `coder` | Sonnet | Do one step. Open one PR. | Yes |
| `verifier` | Opus | Check the PR against the plan and CI. Give a verdict. | No |

The main session is the lead. The lead calls the agents. Agents do not call other agents.

## Flow

```
Ayaz gives task
  -> planner (uses scout for search)
  -> plan with steps
  -> for each step: coder -> PR -> verifier
       verifier PASS -> next step
       verifier FAIL -> back to coder (max 3 tries)
       3 fails -> STOP, report to Ayaz
```

## Handoff format

Each agent returns one block:

```
STATUS: DONE | BLOCKED | FAIL
OUTPUT: <files, PR number, commit SHA, CI run number>
EVIDENCE: <file:line refs, test names, command output>
NEXT: <one line>
```

No other text goes back to the lead.

## Stop conditions

Agents stop and ask Ayaz only for these four:

1. Architecture decision.
2. Economic decision (fees, tokenomics, revenue share).
3. Conflicting requirements.
4. Scope conflict.

Context size or turn count is not a stop condition.

## Hard rules (all agents)

- Never push to main in `budlum-xyz/budlum`. Use a branch and a PR.
- CI is the only judge: `cargo fmt`, `cargo clippy -D warnings`, `cargo test`.
- Do not weaken CI. A new `#[allow]`, `#[ignore]`, pinned old commit, or weakened logic is a violation.
- "Done" needs a commit SHA and a CI run number. A self-report is not proof.
- Code languages: Rust and BudL only. No shell scripts in the repo.
- Never write a token or PAT in a commit, PR, or log.
- Verify repo state live on GitHub. Do not trust old notes.

## Cost control

- Use `scout` for all broad search. Do not search with Opus.
- `planner` and `verifier` read only the files the scout names.
- One PR per step. Keep diffs small.
- Log each step result in the PR description, not in chat.
