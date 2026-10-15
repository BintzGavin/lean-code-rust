# integration-003 — Repository Validation Script

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-003`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Consolidate repository validation commands into a single executable script and update the README to reference it.
- **Vision gap**: A reproducible green build and launch precede performance claims and release. The project currently has scattered verification commands listed in the README, making it harder to ensure all necessary checks are run consistently.
- **Evidence**: `README.md` lists multiple disjointed commands for scaffolding checks, core tests, and examples.
- **Impact**: Provides a unified, single-command entry point to validate the repository state, ensuring consistency and ease of use.

## 2. File Inventory

```json
{
  "id": "integration-003",
  "role": "integration",
  "create": [
    "tooling/validate_repo.sh"
  ],
  "modify": [
    "README.md"
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
  - `tooling/validate_repo.sh` must be a bash script that exits on error (`set -e`) and runs the following:
    - `python3 tooling/check_scaffold.py`
    - `python3 -m unittest discover -s tooling -p 'test_*.py'`
    - `python3 spec/checks/check_package.py`
    - `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
    - `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
    - `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip`
  - `README.md` must be updated to replace the disjointed lists of commands with an instruction to run `bash tooling/validate_repo.sh`.
- **Compatibility constraints**: None.
- **Architectural constraints**: The validation script should reside in `tooling/`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls tooling/validate_repo.sh` fails.
- **Verification commands**:
  ```sh
  bash tooling/validate_repo.sh
  grep validate_repo.sh README.md
  ```
- **Success criteria**: `validate_repo.sh` executes successfully, running all necessary checks. The `README.md` explicitly instructs the user to run the script.
- **Edge cases**: None.

## 5. Constraints

- **Non-goals**: Modifying the existing tests or creating new ones.
- **Forbidden changes**: Modifying Cargo.toml or crates code.
- **Scope ceiling**: This fits well within one execution run as it is merely consolidating existing commands.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Any of the underlying test or validation commands fail on `main`.

## 7. Result
Completed: Consolidated validation commands into `tooling/validate_repo.sh` and updated README.
