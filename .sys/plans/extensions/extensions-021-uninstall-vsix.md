# extensions-021 — Uninstall VSIX extension locally

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-021`

## 1. Context & Goal

- **Objective**: Introduce an `uninstall_vsix` function that removes an installed extension directory given the publisher, name, and version.
- **Vision gap**: The X-0010 VSIX lifecycle specifies support for uninstalling local VSIX extensions. While we can install them, we lack the logic to remove the corresponding canonical directory.
- **Evidence**: `crates/extensions/src/vsix.rs` contains `install_vsix` but no corresponding `uninstall_vsix`.
- **Impact**: Enables removing extension directories during uninstall or update flows.

## 2. File Inventory

```json
{
  "id": "extensions-021",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/vsix.rs"
  ],
  "read_only": [
    "crates/extensions/src/manifest.rs"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Add a public `uninstall_vsix<D: AsRef<std::path::Path>>(publisher: &str, name: &str, version: &str, extensions_dir: D) -> Result<(), VsixError>` function in `vsix.rs`. It must construct the canonical directory name (`publisher.name-version`), join it to `extensions_dir`, and recursively remove the directory if it exists using `std::fs::remove_dir_all`. If the directory does not exist, it should return `Ok(())` or ignore the `NotFound` error.
- **Compatibility constraints**: The directory naming format (`<publisher>.<name>-<version>`) must exactly match `install_vsix`.
- **Architectural constraints**: Keep logic within `crates/extensions/src/vsix.rs`.
- **Public contract changes**: Expose `uninstall_vsix` as a public function in `vsix.rs`.

## 4. Test Plan

- **Failing evidence**: No tests exist for `uninstall_vsix`.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: A test uses `install_vsix` to install a mock extension, verifies it exists, then calls `uninstall_vsix` and asserts the directory no longer exists.
- **Edge cases**: Calling `uninstall_vsix` on a non-existent extension directory should succeed (no-op).

## 5. Constraints

- **Non-goals**: Modifying extension manifest parsing logic. Disabling extensions without removing files.
- **Forbidden changes**: Do not modify `crates/extensions/src/manifest.rs`.
- **Scope ceiling**: Removing a specific directory is a bounded, simple operation completing the install/uninstall lifecycle.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
- **Outcome**: Successfully implemented `uninstall_vsix` and corresponding tests.
- **Completed**: true
