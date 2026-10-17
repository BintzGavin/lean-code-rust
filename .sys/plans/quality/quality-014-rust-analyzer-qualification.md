# quality-014 — Rust analyzer qualification

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-014`

## 1. Context & Goal

- **Objective**: Select, pin and qualify the rust-analyzer existing extension candidate for compatibility.
- **Vision gap**: Existing VS Code extensions remain a hard requirement. The qualification candidate "rust-analyzer" is identified in `spec/extension-compatibility.md` but has not been downloaded, pinned, and set up as a fixture.
- **Evidence**: `tests/compatibility/` currently contains X-0001 through X-0012, but lacks any real qualification candidates.
- **Impact**: Provides a real existing language extension and server bundle to observe existing Rust language behavior without relying exclusively on synthetic fixtures.

## 2. File Inventory

```json
{
  "id": "quality-014",
  "role": "quality",
  "create": [
    "tests/compatibility/rust_analyzer/package.json",
    "tests/compatibility/rust_analyzer/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Identify the exact local VSIX version of rust-analyzer compatible with the baseline (VS Code 1.133.0 public contract). Record its redistribution terms, dependency artifacts and hashes. Define a tiny offline synthetic project to verify. Provide a script (`test.sh`) to download (or simulate the download for qualification) the VSIX, extract it if needed, or otherwise set up the fixture.
- **Compatibility constraints**: Do not run arbitrary untrusted network execution. The setup should be deterministic.
- **Architectural constraints**: Maintain the isolation of fixtures in `tests/compatibility/`. The extension should not be globally installed, but localized to the fixture directory.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/rust_analyzer` fails (directory does not exist).
- **Verification commands**: `bash tests/compatibility/rust_analyzer/test.sh`
- **Success criteria**: The script successfully pins the extension version, prepares the local test project, and exits successfully without unintended external side-effects.
- **Edge cases**: Network unavailability during download; handle gracefully or require a pre-downloaded artifact checksum.

## 5. Constraints

- **Non-goals**: Implementing full LSP server execution inside the editor process. We are only setting up the qualification candidate for independent inspection.
- **Forbidden changes**: No changes outside `tests/compatibility/rust_analyzer/`.
- **Scope ceiling**: Setting up one existing-extension candidate fits one execution run.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: Cannot find a suitable compatible release.

## 7. Result
