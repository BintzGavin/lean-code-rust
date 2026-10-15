# QUALITY-006 — X-0006 State/processes compatibility fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-006`

## 1. Context & Goal

- **Objective**: Implement the X-0006 State/processes compatibility fixture to observe extension storage interfaces, secret-storage behavior, Node child processes, and terminal/task boundaries.
- **Vision gap**: Existing ordinary local VS Code extensions are a hard compatibility requirement (Vision E-0001, D-0003), but exact state and process boundaries (workspace/global state, synthetic secrets, explicitly invoked local child processes) are unobserved in the current implementation.
- **Evidence**: `spec/extension-compatibility.md` lists X-0006 State/processes with canonical scenarios S-0051 and S-0052. The `tests/compatibility/` directory lacks this fixture.
- **Impact**: Provides measured evidence of how state persistence, secrets, and bounded child processes operate under the current architecture, preventing regressions and establishing a baseline for extension parity.

## 2. File Inventory

```json
{
  "id": "quality-006",
  "role": "quality",
  "create": [
    "tests/compatibility/x0006_state_processes/package.json",
    "tests/compatibility/x0006_state_processes/extension.js",
    "tests/compatibility/x0006_state_processes/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: A Node extension that uses `ExtensionContext.workspaceState`, `ExtensionContext.globalState`, `ExtensionContext.secrets`, and creates a Node child process (e.g., using `child_process.exec` or `child_process.spawn`). The `test.sh` script must validate the `package.json` structure, execute the extension synthetically (mocking the `vscode` API where necessary to verify syntax and exports), and verify that the extension attempts to access these state interfaces and create a child process.
- **Compatibility constraints**: Must conform to the VS Code extension structure and Node module resolution used in other compatibility tests.
- **Architectural constraints**: Must remain self-contained within `tests/compatibility/x0006_state_processes/` and use only standard built-in node tools for the mock `vscode` API.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/x0006_state_processes` (No such file or directory)
- **Verification commands**:
  `bash tests/compatibility/x0006_state_processes/test.sh`
  `python3 tooling/check_scaffold.py --role quality`
- **Success criteria**: The `test.sh` script exits with code 0, successfully parsing `package.json` and validating the syntax and required exports of `extension.js`.
- **Edge cases**: Missing fields in `package.json`, missing `activate`/`deactivate` exports in `extension.js`, syntax errors in standard VS Code API usage for state and process creation.

## 5. Constraints

- **Non-goals**: Do not implement a real `vscode` API backend for state or processes; only mock enough to verify extension syntax and logic. Do not test terminal UI or real secret storage.
- **Forbidden changes**: Do not modify files outside of `tests/compatibility/x0006_state_processes/`. Do not modify existing fixtures.
- **Scope ceiling**: Can be completed in one run by defining the package, extension script, and test harness.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Node environment is unavailable to execute `test.sh` synthetically.

## 7. Result
