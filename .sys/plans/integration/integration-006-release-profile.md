# integration-006 — Release Profile Optimization

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-006`

## 1. Context & Goal

- **Objective**: Optimize the Cargo release profile to reduce the artifact size of the desktop binary.
- **Vision gap**: VISION.md requires "Signed bundle reduction | At least 30% smaller" and measuring artifact size. The current artifact is built with default release settings.
- **Evidence**: `packaging/build_release.sh` reports a `dist/desktop` size of around 4.2M. `Cargo.toml` lacks a `[profile.release]` section for optimizations like `strip`, `lto`, and `opt-level = "z"`.
- **Impact**: Reduces the distribution artifact size, bringing the product closer to the performance targets in `spec/requirements.md`.

## 2. File Inventory

```json
{
  "id": "integration-006",
  "role": "integration",
  "create": [],
  "modify": [
    "Cargo.toml"
  ],
  "read_only": [
    "packaging/build_release.sh"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The `Cargo.toml` file at the repository root must contain a `[profile.release]` section configured for minimal artifact size (e.g., `strip = true`, `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`).
- **Compatibility constraints**: The desktop binary must still compile successfully and run.
- **Architectural constraints**: Only modify the workspace `Cargo.toml`.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `grep profile.release Cargo.toml` returns no results.
- **Verification commands**:
  - `bash packaging/build_release.sh`
  - `cargo tree`
- **Success criteria**: The artifact size reported by `packaging/build_release.sh` is noticeably reduced. The build succeeds.
- **Edge cases**: N/A.

## 5. Constraints

- **Non-goals**: Modifying source code or adding new dependencies like `upx`.
- **Forbidden changes**: Do not change the `[workspace]` definition in `Cargo.toml`.
- **Scope ceiling**: Can be completed in one run by adding the configuration block.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A

## 7. Result
