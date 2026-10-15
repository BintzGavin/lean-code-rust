# EDITOR-CORE-006 — Add undo/redo history to Document

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-006`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Add basic linear undo and redo history to the in-memory document buffer.
- **Vision gap**: Vision priority "Editing, dirty state, save and cancellation preserve user data" implies the ability to recover from mistakes during editing. Scenario `S-0002` explicitly requests an "Edit alpha.txt, undo, redo, then save" workflow, but `crates/editor-core/README.md` explicitly limits current capabilities: "There is no undo".
- **Evidence**: `crates/editor-core/README.md` states "There is no undo, incremental edit, autosave, recovery journal or large-file optimization." `spec/catalog.json` Scenario `S-0002` lists actions "Edit alpha.txt, undo, redo, then save."
- **Impact**: Unlocks the ability for the UI to satisfy the undo/redo workflow requested in `S-0002` and protects user text from accidental replacement.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-006",
  "role": "editor-core",
  "create": [],
  "modify": [
    "crates/editor-core/src/lib.rs",
    "crates/editor-core/tests/document.rs",
    "crates/editor-core/README.md"
  ],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: `Document` must record the previous text state before a change via `replace_text` is applied. `undo()` should revert the document's text to the state before the most recent `replace_text`, and `redo()` should restore the text reverted by the most recent `undo()`. A new `replace_text` operation after an `undo` should clear the redo history.
- **Compatibility constraints**: The existing `is_dirty()` behavior comparing the current text against the `persisted_text` snapshot must remain accurate and unchanged.
- **Architectural constraints**: Maintain the standard-library-only restriction. No external dependencies. Keep it a simple linear history for the whole text; do not implement incremental edits or operational transform yet.
- **Public contract changes**: Add `undo(&mut self) -> bool` and `redo(&mut self) -> bool` to `Document` which return true if an action was taken.

## 4. Test Plan

- **Failing evidence**: Currently, there are no tests for undo/redo, and `Document` lacks these methods.
- **Verification commands**: `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
- **Success criteria**: Tests demonstrate that calling `replace_text`, then `undo` restores the original text. Calling `redo` restores the replaced text. `is_dirty` correctly reflects the relationship to the saved snapshot at any point in the history.
- **Edge cases**: Calling `undo` or `redo` when history is empty should be a safe no-op. Calling `replace_text` clears forward redo history.

## 5. Constraints

- **Non-goals**: No incremental edits (diffs), no multi-cursor support, no autosave, no recovery journal to disk.
- **Forbidden changes**: Do not add dependencies to `Cargo.toml`. Do not change how saving and temp files work.
- **Scope ceiling**: Whole-document snapshot history fits easily in one run without complex data structures.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
