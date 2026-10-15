# integration-004 — Workspace Validation

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-004`

## 1. Context & Goal

- **Objective**: Update the repository validation script to run Cargo commands on the entire workspace rather than just `editor-core`.
- **Vision gap**: A reproducible green build and launch precede performance claims and release. The current validation script only checks `crates/editor-core`, ignoring other crates in the workspace.
- **Evidence**: `tooling/validate_repo.sh` explicitly passes `--manifest-path crates/editor-core/Cargo.toml` instead of using `--workspace`.
- **Impact**: Ensures that all workspace crates (`editor-core`, `desktop`, `extensions`) are tested and formatted before any release or performance claims.

## 2. File Inventory

```json
{
  "id": "integration-004",
  "role": "integration",
  "create": [],
  "modify": [
    "tooling/validate_repo.sh"
  ],
  "read_only": [
    "Cargo.toml"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**:
  - `tooling/validate_repo.sh` must use `cargo test --workspace` and `cargo fmt --all --check` to check the entire repository.
  - Keep running the specific example `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip --offline --locked`.
- **Compatibility constraints**: Keep `--offline --locked` for tests where appropriate to ensure reproducible builds if it was there before.
- **Architectural constraints**: None.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `tooling/validate_repo.sh` doesn't test the workspace.
- **Verification commands**:
  ```sh
  bash tooling/validate_repo.sh
  ```
- **Success criteria**: The script successfully tests and checks formatting for all crates in the workspace.
- **Edge cases**: None.

## 5. Constraints

- **Non-goals**: Fixing format or clippy issues in other crates.
- **Forbidden changes**: Do not modify crate source code or Cargo.toml workspace definitions.
- **Scope ceiling**: This is a simple bash script change.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: Any crate fails `cargo test --workspace` or `cargo fmt --all --check`.

## 7. Result
