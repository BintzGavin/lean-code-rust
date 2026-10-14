# [extensions-001] — Bootstrap extensions crate

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-001`

## 1. Context & Goal

- **Objective**: Initialize the `extensions` Rust crate to establish the foundational structure for VSIX lifecycle and VS Code extension compatibility.
- **Vision gap**: The `VISION.md` requires existing ordinary local VS Code extensions as a hard compatibility requirement, but the `crates/extensions/` directory and crate do not exist in reality.
- **Evidence**: `crates/extensions/` does not exist in the repository.
- **Impact**: Unlocks further work on the extensions domain, such as VSIX parsing, Node extension host, and extension compatibility features.

## 2. File Inventory

```json
{
  "id": "extensions-001",
  "role": "extensions",
  "create": [
    "crates/extensions/Cargo.toml",
    "crates/extensions/src/lib.rs"
  ],
  "modify": [],
  "read_only": [
    "VISION.md",
    ".sys/black-hole/role-map.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The `crates/extensions` directory contains a minimal, valid Rust library crate named `lean-extensions`.
- **Compatibility constraints**: Follow the established repository patterns (e.g., using Rust edition 2021, publish = false).
- **Architectural constraints**: Remain within the `crates/extensions/` boundary. No external dependencies are required for the empty crate.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `cargo check --manifest-path crates/extensions/Cargo.toml` fails because the file does not exist.
- **Verification commands**: `cargo check --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: The command completes successfully, confirming the crate is valid.
- **Edge cases**: None for an empty crate.

## 5. Constraints

- **Non-goals**: Implementing VSIX parsing, Node extension host, or LSP/DAP logic. Only the crate scaffolding is required.
- **Forbidden changes**: Modifying files outside `crates/extensions/`.
- **Scope ceiling**: This is a simple crate initialization bounded to creating two files.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
