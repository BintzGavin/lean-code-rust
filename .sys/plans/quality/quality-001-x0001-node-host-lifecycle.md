# QUALITY-001 — X-0001 Node host lifecycle fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-001`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Create the independent X-0001 Node host lifecycle compatibility fixture and baseline behavior oracle.
- **Vision gap**: Existing VS Code extensions are a hard compatibility requirement (Decision D-0003), but no observable verification exists for manifest activation, context subscriptions, and Node `fs`/`path` behavior. The `tests/compatibility/` directory does not yet exist.
- **Evidence**: `tests/compatibility/` is absent. `spec/extension-compatibility.md` lists X-0001 (S-0041, S-0042) as unexecuted with unknown original oracles.
- **Impact**: This unlocks the ability to observe and measure the baseline extension host lifecycle behavior, which is the foundational prerequisite for testing VSIX compatibility.

## 2. File Inventory

```json
{
  "id": "quality-001",
  "role": "quality",
  "create": [
    "tests/compatibility/x0001_node_host/package.json",
    "tests/compatibility/x0001_node_host/extension.js",
    "tests/compatibility/x0001_node_host/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must implement S-0041 ("Activate on a named command, read synthetic file") and S-0042 ("activation exception then independent subsequent command"). The extension should register activation events and export an activation function conforming to the standard public API (`vscode.ExtensionContext`). It should demonstrate a synthetic `fs` file read.
- **Compatibility constraints**: Use only ordinary stable public extension APIs. Do not use proposed or excluded APIs.
- **Architectural constraints**: The fixture must be completely independent of the product runtime and testable on its own.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/x0001_node_host` will fail because it does not exist.
- **Verification commands**: `bash tests/compatibility/x0001_node_host/test.sh` (which should run a self-contained validation or linting on the created fixture, verifying the manifest and syntax).
- **Success criteria**: The directory and fixture files exist, and the manifest is valid.
- **Edge cases**: The test must cover both the successful activation flow and the exception flow.

## 5. Constraints

- **Non-goals**: Do not install a runtime or test runner. Do not implement the `vscode` API or a mock extension host; this is purely creating the extension fixture.
- **Forbidden changes**: Do not modify product code, existing specs, or paths outside `tests/compatibility/` and the `quality` role boundaries.
- **Scope ceiling**: Creating a synthetic extension fixture and a basic validation script easily fits in one run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result
