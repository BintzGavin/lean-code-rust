# extensions-022 — List installed VSIX extensions

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-022`

## 1. Context & Goal

- **Objective**: Introduce a `list_installed_extensions` function that discovers and parses the manifests of all local extensions installed in a given directory.
- **Vision gap**: The X-0010 VSIX lifecycle specifies support for managing installed extensions. While we can install and uninstall extensions to standard paths, we lack the capability to discover and load the list of currently installed extensions from disk.
- **Evidence**: `crates/extensions/src/vsix.rs` contains `install_vsix` and `uninstall_vsix` but lacks a discovery mechanism to list them.
- **Impact**: Enables the editor to find, validate, and load installed extensions during startup.

## 2. File Inventory

```json
{
  "id": "extensions-022",
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

- **Required behavior**: Add a public `list_installed_extensions<D: AsRef<std::path::Path>>(extensions_dir: D) -> Result<Vec<ExtensionManifest>, VsixError>` function in `vsix.rs`. It must read the `extensions_dir` directory using `std::fs::read_dir`. If the directory does not exist, return `Ok(Vec::new())`. For each entry that is a directory, it attempts to read the `package.json` file inside it. If the file exists, it reads its contents and parses it via `ExtensionManifest::parse`. Valid manifests are collected into a `Vec<ExtensionManifest>` and returned. Directories lacking a valid `package.json` should be gracefully skipped.
- **Compatibility constraints**: Must parse the same manifest schema as the archive-based `read_manifest`.
- **Architectural constraints**: Keep logic within `crates/extensions/src/vsix.rs`.
- **Public contract changes**: Expose `list_installed_extensions`.

## 4. Test Plan

- **Failing evidence**: No tests exist for `list_installed_extensions`.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: A test sets up a temporary directory with a valid installed extension (by creating a directory and writing a `package.json`) and an invalid directory. `list_installed_extensions` should return exactly the valid `ExtensionManifest`.
- **Edge cases**: `extensions_dir` does not exist (should return an empty vector).

## 5. Constraints

- **Non-goals**: Enabling or disabling extensions, dependency resolution, or loading extension code into memory.
- **Forbidden changes**: Do not modify `crates/extensions/src/manifest.rs`.
- **Scope ceiling**: Reading files from a directory and parsing JSON is a bounded, self-contained operation.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
