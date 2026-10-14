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

### Attempt 1 — completed

- Claim `editor-core-001-independent-rust-core-20261014-attempt-1` was persisted in commit `202857d` before any product file was created. The backlog now records completion and Claim `none`.
- Independent inputs: this repository's Vision, work order and read-only spec only. No original editor source, tests/assets, source-aware discovery notes or other workspace repositories were inspected. This is procedural source isolation, not legal certification or OS isolation.
- Resource preflight: `memory_pressure` reported 37%, then 36% at command-observed `2026-10-14T14:04:15Z`; compilation paused. Parent setup correction `cda6144` clarified the 40% restriction applies to heavy builds and permits a tiny std-only single-job check below it when pressure is not critical. A subsequent `memory_pressure -Q` reported 33%; read-only `sysctl kern.memorystatus_vm_pressure_level` reported `2` (warning, not critical). The sysctl read initially encountered the sandbox's Operation not permitted and succeeded after read-only escalation. No processes were terminated.
- Tests first: created Cargo manifest/lock and `tests/document.rs` before the library. `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml --offline --locked` exited **101**, with `error[E0432]: unresolved import lean_editor_core` at `tests/document.rs:1:5`. No library implementation existed at that red check.
- Implemented exactly six named product files: `crates/editor-core/Cargo.toml`, `crates/editor-core/Cargo.lock`, `crates/editor-core/src/lib.rs`, `crates/editor-core/tests/document.rs`, `crates/editor-core/examples/edit_roundtrip.rs`, `crates/editor-core/README.md`. No dependencies added. Role state changes are only this Result, own backlog and own status; no critical-memory addition was warranted.
- Green: the same locked/offline single-job Cargo test command exited **0**, with **8 integration tests passed, 0 failed** (0 unit and 0 doc tests). Cases cover clean open/path/text, unsaved edit and cancellation without disk mutation, saved bytes/reopen, dirty comparison against the latest successful save, Unicode/mixed newlines, empty content, missing/invalid UTF-8 opens, and failed save retaining buffer/baseline followed by retry. Replacing the synthetic file with a directory gives a deterministic write failure independent of permissions/root.
- `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check` exited **0**.
- `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip --offline --locked` exited **0**, printing `PASS: open → edit → cancel → save → clean close → reopen; synthetic file removed`. Its synthetic bytes and cleanup are checked inside the example; only its own exact file/directory are removed.
- `python3 tooling/check_scaffold.py --role editor-core` and `python3 tooling/check_scaffold.py` passed. `python3 -m unittest discover -s tooling -p 'test_*.py'` passed **21 tests**. `python3 spec/checks/check_package.py` passed: 20 behaviors, 64 blocked/unknown scenarios, 12 proposed extension fixture families, 4 fixture hashes; imported product scenarios executed remain **0**.
- Non-fatal toolchain warnings: xcrun/cc could not obtain DARWIN_USER_TEMP_DIR via confstr in the sandbox and used `/tmp`. Build, tests and example still exited successfully; no resource errors occurred.
- Limits: UTF-8 only; whole-document storage; close is an advisory decision API; no external-change detection, atomic save, crash durability or metadata-preservation guarantee. Failed save preserves memory but may partially change disk. No desktop launch, extension compatibility, reference parity or performance/network measurement is claimed. No remaining dependency for this bounded work order.
