# quality-012 — X-0012 Compatibility boundary fixture

**Role**: `quality`
**Backlog entry**: `.sys/backlogs/quality.md#quality-012`

## 1. Context & Goal

- **Objective**: Implement the X-0012 Compatibility boundary fixture to test excluded/proposed APIs and stable control paths.
- **Vision gap**: We need to observe parity and boundaries before making claims. The X-0012 fixture scenarios (S-0063 and S-0064) defined in `spec/extension-compatibility.md` remain blocked/unexecuted.
- **Evidence**: `spec/extension-compatibility.md` details X-0012 for compatibility boundary testing. `tests/compatibility/x0012_compatibility_boundary/` path does not exist.
- **Impact**: Demonstrates the editor safely denies excluded/proposed APIs while permitting stable control without invoking excluded product services.

## 2. File Inventory

```json
{
  "id": "quality-012",
  "role": "quality",
  "create": [
    "tests/compatibility/x0012_compatibility_boundary/package.json",
    "tests/compatibility/x0012_compatibility_boundary/extension.js",
    "tests/compatibility/x0012_compatibility_boundary/test.sh"
  ],
  "modify": [],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The test script must invoke the extension, attempt to access an excluded or proposed API (S-0063), and verify that it fails precisely. It must also exercise a non-excluded stable API control (S-0064) and prove it succeeds without launching an excluded service.
- **Compatibility constraints**: Must conform to VS Code extension manifest and activation patterns.
- **Architectural constraints**: All test outputs must be verifiable via a shell script without modifying core product paths.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tests/compatibility/x0012_compatibility_boundary/` fails because the fixture does not exist.
- **Verification commands**: `bash tests/compatibility/x0012_compatibility_boundary/test.sh`
- **Success criteria**: The script successfully executes, handles S-0063 and S-0064 correctly, exiting with 0 on success.
- **Edge cases**: Failure messages from excluded APIs must not crash the host but be handled gracefully by the test logic.

## 5. Constraints

- **Non-goals**: Not creating an exhaustive list of all excluded APIs, just a representative sample.
- **Forbidden changes**: Do not modify `crates/` paths.
- **Scope ceiling**: A standalone fixture for a single bounded scenario.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Node/VSIX mock environment constraints block test completion.

## 7. Result

Completed implementation of X-0012 Compatibility boundary fixture. Verified S-0063 and S-0064 scenarios successfully.
