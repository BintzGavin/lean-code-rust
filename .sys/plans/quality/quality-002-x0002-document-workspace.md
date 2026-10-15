# QUALITY-002 — X-0002 Document/workspace fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-002`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Create the independent X-0002 Document/workspace compatibility fixture and baseline behavior oracle.
- **Vision gap**: Existing VS Code extensions are a hard compatibility requirement (Decision D-0003), but no observable verification exists for public document, edit, URI, file and event interfaces.
- **Evidence**: `tests/compatibility/x0002_document_workspace/` is absent. `spec/extension-compatibility.md` lists X-0002 (S-0043, S-0044) as unexecuted with unknown original oracles.
- **Impact**: This unlocks the ability to observe and measure the baseline document, edit, URI, file and event interfaces behavior.

## 2. File Inventory

```json
{
  "id": "quality-002",
  "role": "quality",
  "create": [
    "tests/compatibility/x0002_document_workspace/package.json",
    "tests/compatibility/x0002_document_workspace/extension.js",
    "tests/compatibility/x0002_document_workspace/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must implement S-0043 ("Edit/save and capture events/bytes") and S-0044 ("denied or missing disposable file and cancellation").
- **Compatibility constraints**: Use only ordinary stable public extension APIs. Do not use proposed or excluded APIs.
- **Architectural constraints**: The fixture must be completely independent of the product runtime and testable on its own.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/x0002_document_workspace` will fail because it does not exist.
- **Verification commands**: `bash tests/compatibility/x0002_document_workspace/test.sh`
- **Success criteria**: The directory and fixture files exist, and the manifest is valid.
- **Edge cases**: The test must cover both the successful edit flow and the denied/cancellation flow.

## 5. Constraints

- **Non-goals**: Do not install a runtime or test runner.
- **Forbidden changes**: Do not modify product code, existing specs, or paths outside `tests/compatibility/` and the `quality` role boundaries.
- **Scope ceiling**: Creating a synthetic extension fixture and a basic validation script easily fits in one run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result
