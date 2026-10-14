# EXTENSIONS-002 — Extension Manifest Parser

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-002`

The role-local backlog owns lifecycle state. The planner freezes Sections 1–6 when the entry becomes `ready`. While an entry is `needs_input`, the planner may revise those sections only to apply its recorded resolution. The executor may append only the Result section.

## 1. Context & Goal

- **Objective**: Implement data structures and parser for a minimal VS Code extension manifest (`package.json`).
- **Vision gap**: VS Code extensions are a hard compatibility requirement (VISION.md). To enable their lifecycle (install, enable, update) and execution, the system must first parse their metadata.
- **Evidence**: `crates/extensions/src/lib.rs` currently only has minimal initialization. The compatibility matrix in `spec/extension-compatibility.md` relies on manifest parsing (e.g., X-0010 VSIX lifecycle).
- **Impact**: Unlocks the ability to read package metadata, determine compatibility and dependencies, and understand contribution points, which is the foundational step for all extension lifecycle operations.

## 2. File Inventory

```json
{
  "id": "extensions-002",
  "role": "extensions",
  "create": [
    "crates/extensions/src/manifest.rs"
  ],
  "modify": [
    "crates/extensions/src/lib.rs",
    "crates/extensions/Cargo.toml"
  ],
  "read_only": [
    "spec/extension-compatibility.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Introduce a Rust struct representing an extension manifest (e.g., `ExtensionManifest`). It should parse standard fields like `name`, `publisher`, `version`, `engines`, and `main` using `serde_json`. Provide a function to parse the manifest from a JSON string.
- **Compatibility constraints**: The parser should align with the standard VS Code `package.json` format.
- **Architectural constraints**: Keep the crate minimal and independent. Add `serde` and `serde_json` to `Cargo.toml`.
- **Public contract changes**: Expose `ExtensionManifest` and its parsing functionality from `crates/extensions/src/lib.rs`.

## 4. Test Plan

- **Failing evidence**: `cargo test --manifest-path crates/extensions/Cargo.toml` currently passes but does not test manifest parsing.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml --locked`
- **Success criteria**: Unit tests successfully parse valid manifest JSON strings and return expected data structures. Parsing invalid JSON should return appropriate errors.
- **Edge cases**: Missing optional fields (e.g., `main`), unexpected extra fields (which should be ignored or captured based on strictness).

## 5. Constraints

- **Non-goals**: Not implementing VSIX extraction or filesystem traversal. This is strictly parsing a string.
- **Forbidden changes**: Do not modify files outside `crates/extensions/`.
- **Scope ceiling**: Parsing JSON into a Rust struct is a single, well-defined problem achievable in one run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: N/A.

## 7. Result
