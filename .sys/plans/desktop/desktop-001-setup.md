# desktop-001 — Initial desktop crate scaffolding

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-001`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Scaffold the `crates/desktop` directory with an initial empty `Cargo.toml` and basic main application entry point to prepare for native window development.
- **Vision gap**: A usable local desktop editor is required by the Vision, but no desktop application crate currently exists.
- **Evidence**: The `crates/desktop` directory does not exist.
- **Impact**: Establishes the foundational crate where window, rendering, and input handling will be implemented, allowing subsequent execution of native GUI development.

## 2. File Inventory

```json
{
  "id": "desktop-001",
  "role": "desktop",
  "create": [
    "crates/desktop/Cargo.toml",
    "crates/desktop/src/main.rs"
  ],
  "modify": [],
  "read_only": [
    "VISION.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: A standard `main.rs` binary entry point that simply prints a placeholder message (e.g., "Hello from desktop") and exits successfully. The `Cargo.toml` must define the package name as `desktop` and version `0.1.0`.
- **Compatibility constraints**: Must integrate with standard Cargo workflows.
- **Architectural constraints**: Rust standard library only for this initial slice. No GUI toolkit dependencies should be added yet.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `cargo run --manifest-path crates/desktop/Cargo.toml` fails because the manifest does not exist.
- **Verification commands**:
  ```sh
  cargo check --manifest-path crates/desktop/Cargo.toml
  cargo run --manifest-path crates/desktop/Cargo.toml
  ```
- **Success criteria**: The binary compiles and successfully prints the placeholder message to stdout.
- **Edge cases**: None.

## 5. Constraints

- **Non-goals**: Initializing actual windowing libraries (like Winit), graphics rendering, or integrating with `editor-core`.
- **Forbidden changes**: Editing `Cargo.toml` in the repository root or modifying `crates/editor-core`.
- **Scope ceiling**: Scaffolding an empty binary crate fits easily into a single execution run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: A `integration` work order may later be needed to add `crates/desktop` to the root workspace `Cargo.toml`.
- **Blocked when**: Cargo is unable to initialize a package in the `crates/desktop` directory.

## 7. Result


- **Outcome**: Completed successfully. The `desktop` crate was scaffolded.
- **Changed files**: `crates/desktop/Cargo.toml`, `crates/desktop/src/main.rs`.
- **Checks**: Ran `cargo check`, `cargo run`, and `python3 tooling/check_scaffold.py --role desktop`. All passed.
