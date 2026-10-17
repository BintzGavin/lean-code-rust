# quality-015 — VS Code ESLint qualification

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-015`

## 1. Context & Goal

- **Objective**: Select, pin and qualify the VS Code ESLint existing extension candidate for compatibility.
- **Vision gap**: Existing VS Code extensions remain a hard requirement. The qualification candidate "VS Code ESLint" is identified in `spec/extension-compatibility.md` but has not been downloaded, pinned, and set up as a fixture.
- **Evidence**: `tests/compatibility/` lacks the `eslint` real qualification candidate fixture.
- **Impact**: Provides a real existing language diagnostics and code-action workflow to observe existing behavior without relying exclusively on synthetic fixtures.

## 2. File Inventory

```json
{
  "id": "quality-015",
  "role": "quality",
  "create": [
    "tests/compatibility/eslint/package.json",
    "tests/compatibility/eslint/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Identify the exact local VSIX version of VS Code ESLint compatible with the baseline. Define a small synthetic project containing project-local ESLint and config. Provide a script (`test.sh`) to download (or simulate the download for qualification) the VSIX, extract it if needed, or otherwise set up the fixture.
- **Compatibility constraints**: Do not run arbitrary untrusted network execution. The setup should be deterministic. No global dependency assumption.
- **Architectural constraints**: Maintain the isolation of fixtures in `tests/compatibility/`. The extension and dependencies should be localized to the fixture directory.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/eslint` fails (directory does not exist).
- **Verification commands**: `bash tests/compatibility/eslint/test.sh`
- **Success criteria**: The script successfully pins the extension version, prepares the local test project with local ESLint, and exits successfully without unintended external side-effects.
- **Edge cases**: Network unavailability during download; handle gracefully.

## 5. Constraints

- **Non-goals**: Implementing full ESLint execution inside the editor process. We are only setting up the qualification candidate for independent inspection.
- **Forbidden changes**: No changes outside `tests/compatibility/eslint/`.
- **Scope ceiling**: Setting up one existing-extension candidate fits one execution run.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: Cannot find a suitable compatible release.

## 7. Result
