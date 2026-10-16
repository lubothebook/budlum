---
name: steward
description: Guide for taking Budlum pull requests to green. Read on a PR event (CI failure, review, conflict). Applies together with CLAUDE.md and MODEL_ROUTING.md.
---

# Steward: Budlum PR guide

This file is short. Where it conflicts, CLAUDE.md and MODEL_ROUTING.md win.

## Stance

- Do not wait for an order. Red CI, a conflict and an open review comment are work.
- Merge and approval belong to Ayaz. Do not merge a PR, do not approve one.
- No empty commit, no closing and reopening a PR, no skipped test, no new `#[ignore]` (Z3).

## Order

1. Merge conflict: merge the base branch into the PR branch. No rebase, no force push.
2. Red CI: read the cause first, then fix it.
3. Review comment: apply small local requests; ask Ayaz about large ones.

## How to read red CI

- Take at least 150 lines with `get_job_logs`. The last 30 lines are usually cleanup output, not the error.
- The Gates job names one gate (`FAIL [name]`). Run the gate locally:
  `cargo run --release --manifest-path xtask/gates/Cargo.toml -- <gate-name>`
- Format: `cargo fmt --all -- --check`. `budzero/` is a separate workspace; run it there with `cd budzero`.
- Without `protoc`, `cargo test` and clippy do not run locally. Say so in the PR description and leave it to CI. Do not install packages; ask.
- If Dependency Review reports a "Dependency graph" error, that is a repository setting, not code. Tell Ayaz.
- Docker Trivy finding: name the package explicitly in `ops/Dockerfile` (libpcre2-8-0 and perl-base are the examples). No blanket `apt-get upgrade` (hadolint DL3005).

## Baseline files

- Never loosen a `.github/*-baseline.txt` file. Loosening needs approval under section 8.
- If a gate says a wired or deleted line is still listed, remove only that line (tightening). If a tool permission blocks the edit, do not work around it; tell Ayaz.

## R3 change

- For an R3 area (consensus, crypto, privacy, tokenomics, settlement, cross_domain, registry, chain, network/node.rs, budzero/bud-proof) a fresh `architect` call verifies the diff before push.
- Tag bytes, signature inputs and root computations do not change. If they do, stop and ask under section 8.

## Writing and secrecy

- Commit and PR text use short plain sentences. No long dash, no bold marker (Z13).
- An unfixed security finding is never written to a PR, comment, commit or file. Use the private channel in `docs/SECURITY.md`.
- The PR description follows the headings in `.github/PULL_REQUEST_TEMPLATE.md`. Test counts are written per file.
