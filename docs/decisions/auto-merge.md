# Jules auto-merge

The maintainer-installed workflow is `.github/workflows/auto-merge.yml`. It follows
the Helios/Apastra workflow policy and applies to PR authors `google-labs-jules`,
`google-labs-jules[bot]`, and `BintzGavin`.

On PR opening, reopening, or a new head commit, it merges the base branch into
the PR branch, resolves conflicts only in `docs/PROGRESS.md` by retaining both
versions, and pushes the result. Other unresolved conflicts stop the workflow.
It then marks a draft ready if necessary and requests squash auto-merge, with
up to five attempts. Merge, push, readiness, and exhausted retry failures are
reported as failures rather than success.

The repository must allow auto-merge and squash merging, and GitHub Actions
must be enabled. The workflow requests contents and pull-request write access;
it does not require permission to approve reviews or a separate stored token.

This workflow does not run product tests or add a required-check gate. GitHub's
branch rules govern when a requested merge can complete. Without such rules,
an eligible PR can merge immediately. A successful workflow proves automation
execution, not product correctness, parity, or release readiness.

Existing PRs need one of the supported events after installation to run this
workflow. Empty results remain diagnostic evidence; this workflow adds no
empty-PR suppression. Recurring roles must not edit the workflow.
