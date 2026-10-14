# integration-001 — Workspace Setup

**Role**: `integration`
**Backlog entry**: `.sys/backlogs/integration.md#integration-001`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Establish the root `Cargo.toml` and `rust-toolchain.toml` to define the Cargo workspace.
- **Vision gap**: A reproducible green build and launch precede performance claims and release.
- **Evidence**: The root directory lacks a `Cargo.toml`, causing commands like `cargo build` at the root to fail.
- **Impact**: Unlocks repository-wide builds, tests, and formatting across all nested crates.

## 2. File Inventory

```json
{
  "id": "integration-001",
  "role": "integration",
  "create": [
    "Cargo.toml",
    "rust-toolchain.toml"
  ],
  "modify": [],
  "read_only": [
    "VISION.md",
    "README.md",
    "crates/editor-core/Cargo.toml"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The root `Cargo.toml` must define a workspace that includes the existing `crates/*` directories. `rust-toolchain.toml` should specify the stable Rust toolchain. A valid `Cargo.lock` will be generated implicitly by cargo.
- **Compatibility constraints**: Must preserve the build process of existing crates.
- **Architectural constraints**: Must not add dependencies directly to the workspace root unless using workspace dependencies.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `cargo build` in the repository root fails with "could not find `Cargo.toml`".
- **Verification commands**:
  ```sh
  cargo tree
  cargo build --workspace
  ```
- **Success criteria**: The verification commands succeed and output dependency information for the workspace crates.
- **Edge cases**: Existing crates might have incompatible editions or toolchains, requiring workspace alignment.

## 5. Constraints

- **Non-goals**: Modifying individual crate `Cargo.toml` files, configuring CI/CD pipelines, or publishing packages.
- **Forbidden changes**: Modifying product code in `crates/` or testing directories.
- **Scope ceiling**: The creation of a few standard configuration files is straightforward and fits within a single execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: The crates fail to build together in a workspace due to unforeseen conflicting requirements.


## 7. Result

- **State**: completed
- **Changed files**: `Cargo.toml`, `rust-toolchain.toml`
- **Checks**: `cargo tree`, `cargo build --workspace`, `python3 tooling/check_scaffold.py --role integration`
- **Evidence**: Workspace successfully established.
