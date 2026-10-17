# quality-017 — GitHub Theme qualification

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-017`

## 1. Context & Goal

- **Objective**: Select, pin and qualify the GitHub Theme existing declarative theme contribution candidate for compatibility.
- **Vision gap**: Existing VS Code extensions remain a hard requirement. The qualification candidate "GitHub Theme" is identified in `spec/extension-compatibility.md` but has not been downloaded, pinned, and set up as a fixture.
- **Evidence**: `tests/compatibility/` lacks the `github_theme` real qualification candidate fixture.
- **Impact**: Provides a real existing declarative theme contribution and selection to observe existing behavior without relying exclusively on synthetic fixtures.

## 2. File Inventory

```json
{
  "id": "quality-017",
  "role": "quality",
  "create": [
    "tests/compatibility/github_theme/package.json",
    "tests/compatibility/github_theme/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Identify the exact local VSIX version of GitHub Theme compatible with the baseline. Pin the VSIX and a theme variant. Provide a script (`test.sh`) to download the VSIX and set up the fixture. Measure appearance separately from accessibility.
- **Compatibility constraints**: Do not run arbitrary untrusted network execution. The setup should be deterministic.
- **Architectural constraints**: Maintain the isolation of fixtures in `tests/compatibility/`. The extension and dependencies should be localized to the fixture directory.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/github_theme` fails (directory does not exist).
- **Verification commands**: `bash tests/compatibility/github_theme/test.sh`
- **Success criteria**: The script successfully pins the extension version, prepares the fixture, and exits successfully without unintended external side-effects.
- **Edge cases**: Network unavailability during download; handle gracefully.

## 5. Constraints

- **Non-goals**: Rendering the theme in a UI. We are only setting up the qualification candidate for independent inspection.
- **Forbidden changes**: No changes outside `tests/compatibility/github_theme/`.
- **Scope ceiling**: Setting up one existing-extension candidate fits one execution run.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: Cannot find a suitable compatible release.

## 7. Result
Completed setting up the GitHub Theme qualification fixture. Downloaded VSIX and created an offline project for testing.
