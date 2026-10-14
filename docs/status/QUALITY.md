# QUALITY Status

**Responsibility**: Independent reference observations, compatibility fixtures and measured evidence.
**Owned paths**:
- `tests/compatibility/`
- `tests/reference/`
- `evidence/`
- `.sys/plans/quality/`
- `.sys/backlogs/quality.md`
- `.sys/memory/quality.md`
- `docs/status/QUALITY.md`

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
