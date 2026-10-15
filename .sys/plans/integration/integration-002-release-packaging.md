# integration-002 — Release Packaging Framework

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-002`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Establish the initial packaging framework by creating a release build script and release documentation, and update `.gitignore` for build artifacts.
- **Vision gap**: A reproducible green build and launch precede performance claims and release. We currently lack a defined release packaging path (`packaging/`) and release documentation (`docs/release/`).
- **Evidence**: `packaging/` and `docs/release/` directories are absent. There is no script to assemble a release build into a distribution folder.
- **Impact**: Unlocks the ability to package the compiled Rust editor core (and future binaries) into a distributable artifact directory, which is a prerequisite for measuring release artifact size and conducting user tests.

## 2. File Inventory

```json
{
  "id": "integration-002",
  "role": "integration",
  "create": [
    "packaging/build_release.sh",
    "docs/release/README.md"
  ],
  "modify": [
    ".gitignore"
  ],
  "read_only": [
    "VISION.md",
    "Cargo.toml"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**:
  - `packaging/build_release.sh` must be a bash script that builds the workspace in release mode (`cargo build --release --workspace`) and copies the resulting binary (`lean-editor-core` or other expected artifacts, though currently we only have `editor-core`, `desktop`, `extensions` crates) into a new `dist/` directory at the repository root.
  - `docs/release/README.md` should document how to use the release build script and where the artifacts are placed.
  - `.gitignore` must be updated to ignore the `dist/` directory.
- **Compatibility constraints**: The build script should fail fast if the cargo build fails (`set -e`).
- **Architectural constraints**: All release artifacts must be placed in a single `dist/` directory to keep the repository root clean.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls packaging/build_release.sh` fails. `dist/` is not in `.gitignore`.
- **Verification commands**:
  ```sh
  bash packaging/build_release.sh
  ls -l dist/
  grep dist/ .gitignore
  ```
- **Success criteria**: `build_release.sh` executes successfully, creates a `dist/` directory containing the release binaries, and `.gitignore` prevents `dist/` from being tracked.
- **Edge cases**: The `dist/` directory might already exist from a previous run; the script should handle this gracefully (e.g., by recreating or updating it).

## 5. Constraints

- **Non-goals**: Signing the binaries, building installers (e.g., `.dmg`, `.msi`), or setting up CI pipelines.
- **Forbidden changes**: Modifying Rust source code or existing workspace configurations (`Cargo.toml`).
- **Scope ceiling**: Setting up a basic shell script and documentation fits easily within one execution run.

## 6. Dependencies

- **Required first**: None (integration-001 is already completed).
- **Cross-role needs**: None.
- **Blocked when**: Release builds fail due to unforeseen compiler errors in the crates that only manifest in release mode.

## 7. Result
