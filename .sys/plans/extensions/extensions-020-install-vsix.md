# extensions-020 — Install VSIX extension locally

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-020`

## 1. Context & Goal

- **Objective**: Introduce an `install_vsix` function that determines the standard extraction path and extracts a VSIX archive into a target extensions directory.
- **Vision gap**: The X-0010 VSIX lifecycle specifies support for installing, enabling, and managing local VSIX extensions. While we can now read manifests and extract archives to arbitrary folders, we lack the logic to compose the standard `<publisher>.<name>-<version>` path and coordinate extraction.
- **Evidence**: `crates/extensions/src/vsix.rs` contains `extract_vsix` and `read_manifest` but no higher-level `install_vsix` composition.
- **Impact**: Enables placing extension contents in the correct conventional directory structure required by the extension host.

## 2. File Inventory

```json
{
  "id": "extensions-020",
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

- **Required behavior**: Add a public `install_vsix<P: AsRef<Path>, D: AsRef<Path>>(archive_path: P, extensions_dir: D) -> Result<PathBuf, VsixError>` function in `vsix.rs`. It must call `read_manifest`, construct the canonical directory name (e.g., `publisher.name-1.0.0`), join it to `extensions_dir`, and pass the joined path to `extract_vsix`. It returns the full extracted path on success.
- **Compatibility constraints**: The directory naming format (`<publisher>.<name>-<version>`) must match VS Code conventions.
- **Architectural constraints**: Keep logic within `crates/extensions`.
- **Public contract changes**: Expose `install_vsix` as a public function in `vsix.rs`.

## 4. Test Plan

- **Failing evidence**: No tests exist for `install_vsix`.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: A test creating a mock VSIX with `extension/package.json` calls `install_vsix` with a temp dir. It asserts that the function returns a path ending with `<publisher>.<name>-<version>` and that the directory exists.
- **Edge cases**: Existing directory at the target path (should either overwrite, or fail gracefully, but not panic).

## 5. Constraints

- **Non-goals**: Modifying extension manifest parsing logic. Managing a global registry/database of installed extensions.
- **Forbidden changes**: Do not modify `manifest.rs`.
- **Scope ceiling**: Composing manifest reading and archive extraction is a bounded, simple orchestration task.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result

Implemented `install_vsix` which composes manifest reading and VSIX extraction to standard publisher.name-version directories.
