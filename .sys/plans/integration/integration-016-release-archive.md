# integration-016 — Create compressed release archive

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-016`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Create a compressed archive of the release artifact and report its final size.
- **Vision gap**: `spec/requirements.md` defines a "Signed bundle reduction" metric tracking the final bundle size. `packaging/build_release.sh` currently only copies the uncompressed executable without creating an actual release bundle.
- **Evidence**: `packaging/build_release.sh` executes `cp target/release/desktop dist/` and reports the size of the uncompressed binary via `du -h`.
- **Impact**: Establishes the correct measurement for bundle reduction claims and produces a deployable artifact.

## 2. File Inventory

```json
{
  "id": "integration-016",
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

- **Required behavior**: `packaging/build_release.sh` must compress the compiled desktop binary into a tarball (e.g., `dist/desktop.tar.gz`) after moving it to the `dist` directory. The script must report the file size of the compressed archive to properly reflect the release bundle size.
- **Compatibility constraints**: Output format should remain standard (e.g., tar.gz or zip).
- **Architectural constraints**: Keep logic contained in `packaging/build_release.sh`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Running `bash packaging/build_release.sh` does not produce a `.tar.gz` archive in `dist/`.
- **Verification commands**:
  - `bash packaging/build_release.sh`
  - `ls -l dist/desktop.tar.gz`
- **Success criteria**: The script successfully creates a compressed archive in `dist/` and outputs its size.
- **Edge cases**: Missing `tar` or `gzip` utilities in the environment.

## 5. Constraints

- **Non-goals**: Actual code signing of the binary (requires certificates not currently available).
- **Forbidden changes**: Modifying `Cargo.toml` build profiles.
- **Scope ceiling**: Fits in one run; modifying a single packaging script.

## 6. Dependencies

- **Required first**: None
- **Cross-role needs**: None
- **Blocked when**: The release build fails.

## 7. Result

Completed: Created a compressed release archive desktop.tar.gz and reported its size.
