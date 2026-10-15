# desktop-005 — Save Document Shortcut

**Role**: `desktop`
**Backlog entry**: `.sys/backlogs/desktop.md#desktop-005`

## 1. Context & Goal

- **Objective**: Implement a keyboard shortcut to save the current document.
- **Vision gap**: A local-first desktop code editor needs to edit/save documents. Currently, documents can be loaded but not saved via the UI.
- **Evidence**: `crates/desktop/src/main.rs` does not route save shortcuts to the `Document::save` method.
- **Impact**: Enables users to save documents.

## 2. File Inventory

```json
{
  "id": "desktop-005",
  "role": "desktop",
  "create": [],
  "modify": ["crates/desktop/src/main.rs"],
  "read_only": ["crates/editor-core/src/lib.rs"],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Intercept keyboard shortcuts (Ctrl+S / Cmd+S) in `main.rs` and invoke `save()` on the loaded `Document` if present.
- **Compatibility constraints**: Standard OS shortcuts.
- **Architectural constraints**: Must happen within the existing `winit` event loop.
- **Public contract changes**: None.

## 4. Test Plan

- **Failing evidence**: Pressing Ctrl+S does not trigger a save.
- **Verification commands**: `cargo test --manifest-path crates/desktop/Cargo.toml` and `cargo check`.
- **Success criteria**: File is successfully saved without panicking.
- **Edge cases**: Pressing save with no document loaded.

## 5. Constraints

- **Non-goals**: Adding document editing logic (this is strictly wiring up the save action).
- **Forbidden changes**: Modifying `crates/editor-core/`.
- **Scope ceiling**: Fits in one run (adding one keyboard event arm).

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
