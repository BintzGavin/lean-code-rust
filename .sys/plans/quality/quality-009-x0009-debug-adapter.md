# quality-009 — X-0009 Debug adapter compatibility fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-009`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement the X-0009 Debug adapter compatibility fixture to test extension debug contributions, configuration, and DAP sessions.
- **Vision gap**: Product promise 2 requires compatibility with existing VS Code extensions, including DAP. The X-0009 fixture and scenarios S-0057 and S-0058 are currently proposed but not authored or installed, leaving DAP integration unverified.
- **Evidence**: `spec/extension-compatibility.md` lists X-0009 as an unexecuted boundary fixture. `spec/catalog.json` maps it to S-0057 and S-0058 with status `proposed_not_authored_or_installed`.
- **Impact**: Unlocks the ability to observe and measure DAP session behaviors, breakpoint/step sequences, and session restarts, fulfilling the debugging compatibility promise.

## 2. File Inventory

```json
{
  "id": "quality-009",
  "role": "quality",
  "create": [
    "tests/compatibility/x0009_debug_adapter/package.json",
    "tests/compatibility/x0009_debug_adapter/extension.js",
    "tests/compatibility/x0009_debug_adapter/adapter.js",
    "tests/compatibility/x0009_debug_adapter/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The fixture must demonstrate extension debug contributions and DAP session handling. It must synthesize a debug target and an adapter. It should cover the "Breakpoint/step/variable/output sequence" (S-0057) and "disconnect while paused and start a fresh session" (S-0058) flows.
- **Compatibility constraints**: Must be a valid local VS Code extension structure (VSIX-compatible) running under a Node host. Use the stable `vscode` API where applicable.
- **Architectural constraints**: Must operate independently of external real debug targets. Use a synthetic target or mock DAP messages directly in Node.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Running `bash tests/compatibility/x0009_debug_adapter/test.sh` currently fails (file does not exist).
- **Verification commands**:
  - `bash tests/compatibility/x0009_debug_adapter/test.sh`
  - `python3 tooling/check_scaffold.py --role quality`
- **Success criteria**: `test.sh` correctly executes the synthetic DAP sequence and succeeds. The scaffold check passes.
- **Edge cases**: Verify proper cleanup and disconnection if the session restarts while paused.

## 5. Constraints

- **Non-goals**: Not testing C++ or Rust compilation, real debuggers like LLDB or GDB, or UI rendering of debugger views.
- **Forbidden changes**: Do not modify product core Rust code or other fixtures.
- **Scope ceiling**: Creating one synthetic extension with an associated `test.sh` fits well within a single execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Node or required basic API definitions for debugging are not resolvable synthetically.

## 7. Result
