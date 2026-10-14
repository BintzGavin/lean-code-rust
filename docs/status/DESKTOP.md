# DESKTOP Status

**Responsibility**: Native window, rendering, input, navigation and accessibility.
**Owned paths**:
- `crates/desktop/`
- `.sys/plans/desktop/`
- `.sys/backlogs/desktop.md`
- `.sys/memory/desktop.md`
- `docs/status/DESKTOP.md`

## Current state

No work order has been completed yet.

## Evidence entry format

```markdown
### [WORK-ORDER-ID] — [completed | blocked | needs_input | cancelled]
- **Result**: [Concise outcome]
- **Changed files**: [Exact paths]
- **Checks**: [Commands and results]
- **Dependency**: [Durable blocked condition, or “None”]
- **Question**: [Exact human question for `needs_input`, or “None”]
- **Resolution evidence**: [Recorded answer or dependency evidence, or “None”]
```

### desktop-001 — blocked
- **Result**: Planner blocked.
- **Changed files**: None
- **Checks**: None
- **Dependency**: Existing `ready` work order `desktop-001` must be completed by the executor before the planner can select new work.
- **Question**: None
- **Resolution evidence**: None
