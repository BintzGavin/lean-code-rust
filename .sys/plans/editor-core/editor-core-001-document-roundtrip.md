# EDITOR-CORE-001 — Independent document roundtrip

**Role**: `editor-core`
**Backlog entry**: `.sys/backlogs/editor-core.md#EDITOR-CORE-001`

The backlog owns lifecycle; Sections 1–6 are frozen when ready. Append only Result after claim.

## 1. Context & Goal

Build a dependency-free Rust library and runnable CLI example for opening one UTF-8 file, replacing buffer text, dirty-state observation, save, close cancellation and reopening. No implementation exists at bootstrap. This is the requested initial core contract, not observed reference parity or a desktop UI.

## 2. File Inventory

```json
{
  "id": "EDITOR-CORE-001",
  "role": "editor-core",
  "create": [
    "crates/editor-core/Cargo.toml",
    "crates/editor-core/Cargo.lock",
    "crates/editor-core/src/lib.rs",
    "crates/editor-core/tests/document.rs",
    "crates/editor-core/examples/edit_roundtrip.rs",
    "crates/editor-core/README.md"
  ],
  "modify": [],
  "read_only": [
    "VISION.md",
    "spec/requirements.md",
    "spec/interaction-cases.md"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

Expose a small public document API chosen independently by the implementer. Open preserves UTF-8 content and path; replacement affects the buffer, not disk; dirty state compares with the last successful persisted content. Saving writes current text and clears dirty state only on success. A close request for dirty content offers a cancel outcome that leaves the document and disk untouched; successful clean close can be represented explicitly. Reopening sees saved bytes. Invalid UTF-8/open errors return errors; failed save preserves buffer/dirty state. Do not claim atomic save, permissions preservation or full encoding support unless implemented and tested. The example uses its own temporary synthetic file and cleans only that file/directory.

Use Rust std only, one standalone crate and no source from any existing editor. Public APIs are new and need no source translation. Document all limitations. Tests and example must avoid personal files and network.

## 4. Test Plan

- Write integration tests first and record their compile/test failure before library implementation.
- `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml --offline --locked`
- `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
- `cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip --offline --locked`
- `python3 tooling/check_scaffold.py --role editor-core`
- Assert initial clean state, unsaved edit leaves disk unchanged, save and reopen bytes, cancel preserves buffer/disk, restoring original text clears dirty, Unicode/newlines, missing/invalid UTF-8 open and failed save with buffer retained. At least one deterministic failed-save case must avoid relying on root/permission behavior.
- No launch/memory/network/extension parity measurement claims.

## 5. Constraints

No GUI, host bridge, third-party dependency, root workspace manifest, extension fixture implementation, copied source, network, schedule, remote or publication. At most the six named product files plus own backlog, Result, status and critical memory. Resource-check first; single Cargo job with no downloads; stop if insufficient resources.

## 6. Dependencies

None for this isolated core. Future desktop integration and extension behavior remain separate owner-scoped work. Block if the contract needs cross-role edits or resources cannot support the small compile. Do not broaden scope to satisfy full product promises.

## 7. Result

No execution evidence yet.
