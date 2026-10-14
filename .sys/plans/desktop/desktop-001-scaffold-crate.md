# desktop-001 — Scaffold desktop crate structure

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-001`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Scaffold the initial `desktop` crate structure.
- **Vision gap**: A usable local desktop editor is required, but `crates/desktop/` does not yet exist.
- **Evidence**: `ls crates/desktop/` fails.
- **Impact**: Unlocks future desktop, window, and UI feature implementations.

## 2. File Inventory

```json
{
  "id": "desktop-001",
  "role": "desktop",
  "create": [
    "crates/desktop/Cargo.toml",
    "crates/desktop/src/lib.rs"
  ],
  "modify": [],
  "read_only": [
    "VISION.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: The `desktop` crate is initialized with a basic `Cargo.toml` and an empty `src/lib.rs`.
- **Compatibility constraints**: Follow Rust standard practices for a new crate.
- **Architectural constraints**: Only standard library usage.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: `ls crates/desktop/` currently fails.
- **Verification commands**: `ls -l crates/desktop/Cargo.toml` and `ls -l crates/desktop/src/lib.rs`.
- **Success criteria**: Files are present and represent a valid Rust crate.
- **Edge cases**: None.

## 5. Constraints

- **Non-goals**: No actual window, rendering, or UI logic yet.
- **Forbidden changes**: Do not edit `Cargo.toml` in the repository root (owned by `integration`).
- **Scope ceiling**: Just files and folders creation.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
