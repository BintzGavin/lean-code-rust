# INTEGRATION Critical Memory

This role-local ledger belongs to `integration`. The matching planner reads it and the matching executor may update it across runs. Other roles must not write it.

Store only reusable facts that materially change future planning or execution.

Admit an entry only for a recurring domain trap, hidden architectural constraint, material verification lesson, or dependency pattern. Do not record routine work or narration.

## Entry format

```markdown
## [ROLE-LOCAL VERSION OR STABLE PLAN ID] — [Title]
**Learning:** [Reusable fact]
**Action:** [How a later run should apply it]
```
