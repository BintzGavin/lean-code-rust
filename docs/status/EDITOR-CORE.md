# EDITOR-CORE Status

**Current Work Order ID:** EDITOR-CORE-005
**Outcome:** Completed
**Changed Files:**
- `crates/editor-core/src/lib.rs`
- `crates/editor-core/tests/document.rs`
- `crates/editor-core/README.md`
- `.sys/backlogs/editor-core.md`
- `.sys/plans/editor-core/editor-core-005-document-reload.md`
**Checks Run:**
- `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml`
- `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`
- `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip`
- `python3 tooling/check_scaffold.py --role editor-core`