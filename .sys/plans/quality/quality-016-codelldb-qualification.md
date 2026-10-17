# quality-016 — CodeLLDB qualification

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-016`

## 1. Context & Goal

- **Objective**: Select, pin and qualify the CodeLLDB existing native debugger extension candidate for compatibility.
- **Vision gap**: Existing VS Code extensions remain a hard requirement. The qualification candidate "CodeLLDB" is identified in `spec/extension-compatibility.md` but has not been downloaded, pinned, and set up as a fixture.
- **Evidence**: `tests/compatibility/` lacks the `codelldb` real qualification candidate fixture.
- **Impact**: Provides a real existing native debugger extension and adapter to observe existing behavior without relying exclusively on synthetic fixtures.

## 2. File Inventory

```json
{
  "id": "quality-016",
  "role": "quality",
  "create": [
    "tests/compatibility/codelldb/package.json",
    "tests/compatibility/codelldb/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Identify the exact local VSIX version of CodeLLDB compatible with the macOS arm64 baseline. Define a small synthetic prebuilt debug target. Provide a script (`test.sh`) to download the VSIX and set up the fixture without compilation in this pass.
- **Compatibility constraints**: Do not run arbitrary untrusted network execution. The setup should be deterministic.
- **Architectural constraints**: Maintain the isolation of fixtures in `tests/compatibility/`. The extension and dependencies should be localized to the fixture directory.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/codelldb` fails (directory does not exist).
- **Verification commands**: `bash tests/compatibility/codelldb/test.sh`
- **Success criteria**: The script successfully pins the extension version, prepares the prebuilt debug target, and exits successfully without unintended external side-effects.
- **Edge cases**: Network unavailability during download; handle gracefully.

## 5. Constraints

- **Non-goals**: Implementing full DAP/debugger execution inside the editor process. We are only setting up the qualification candidate for independent inspection.
- **Forbidden changes**: No changes outside `tests/compatibility/codelldb/`.
- **Scope ceiling**: Setting up one existing-extension candidate fits one execution run.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: Cannot find a suitable compatible release.

## 7. Result
