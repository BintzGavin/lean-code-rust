# quality-019 — B-0019 Interruption and recovery

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-019`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Author the test fixture and implement scenario S-0037 (B-0019) to verify interruption and recovery behavior when closing a dirty document.
- **Vision gap**: `VISION.md` requires recovery to preserve user choices and document destinations. B-0019 lacks a test fixture and remains unexecuted in `spec/catalog.json`.
- **Evidence**: `spec/catalog.json` lists S-0037 as blocked and unexecuted. `spec/interaction-cases.md` details the S-0037 scenario.
- **Impact**: Provides measured evidence of interruption handling and dirty state preservation for editor core.

## 2. File Inventory

```json
{
  "id": "quality-019",
  "role": "quality",
  "create": [
    "tests/reference/b0019_interruption_recovery/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/catalog.json",
    "spec/interaction-cases.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Implement the reference test script `tests/reference/b0019_interruption_recovery/test.sh` to test scenario S-0037. The script should verify that a file can be opened, modified (without saving), and when closing is requested, the dirty state is appropriately handled.
- **Compatibility constraints**: Must be deterministic and isolated, using a synthetic workspace.
- **Architectural constraints**: Test script must reside in `tests/reference/b0019_interruption_recovery/`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash tests/reference/b0019_interruption_recovery/test.sh` should fail or be absent before implementation.
- **Verification commands**: `bash tests/reference/b0019_interruption_recovery/test.sh`
- **Success criteria**: Script executes successfully, demonstrating the interruption recovery behavior on the lean-editor-core.
- **Edge cases**: Ensure the file on disk remains unmodified when a dirty document is closed with a cancel outcome.

## 5. Constraints

- **Non-goals**: Do not test OS crash recovery or other interruption cases like external file modification.
- **Forbidden changes**: Do not modify `crates/editor-core/` source code.
- **Scope ceiling**: One bash script and an inline rust test wrapper is sufficient for this bounded scenario.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: `editor-core` is missing required cancellation or request-close outcomes.

## 7. Result
