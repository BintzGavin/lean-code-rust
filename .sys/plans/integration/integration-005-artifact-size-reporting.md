# integration-005 — Artifact Size Reporting

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-005`

## 1. Context & Goal

- **Objective**: Update `packaging/build_release.sh` to measure and report the size of the produced `desktop` release artifact.
- **Vision gap**: E-0001 requests a signed bundle reduction of at least 30% smaller, but currently the packaging script does not measure or report any artifact sizes.
- **Evidence**: `packaging/build_release.sh` copies `target/release/desktop` to `dist/` but outputs no size information.
- **Impact**: Enables observability of the binary size baseline, a necessary step before evaluating performance/size claims against the reference product.

## 2. File Inventory

```json
{
  "id": "integration-005",
  "role": "integration",
  "create": [],
  "modify": [
    "packaging/build_release.sh"
  ],
  "read_only": [
    "spec/requirements.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `packaging/build_release.sh` must print the file size (in bytes or human-readable format) of the `dist/desktop` artifact at the end of the script.
- **Compatibility constraints**: None.
- **Architectural constraints**: None.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `bash packaging/build_release.sh` finishes with "Done." without size metrics.
- **Verification commands**:
  ```sh
  bash packaging/build_release.sh
  ```
- **Success criteria**: The script successfully builds the release artifact and prints its file size to standard output.
- **Edge cases**: None.

## 5. Constraints

- **Non-goals**: Optimizing or stripping the binary to reduce size; this order only adds measurement.
- **Forbidden changes**: Do not modify Cargo profiles or source code.
- **Scope ceiling**: This is a simple bash script addition (`ls -lh`, `du -h`, or `stat`).

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Release build fails for unrelated reasons.

## 7. Result
