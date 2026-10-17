# extensions-018 — Parse VSIX archive

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-018`

## 1. Context & Goal

- **Objective**: Extract and parse `extension/package.json` from a VS Code `.vsix` (ZIP) archive.
- **Vision gap**: X-0010 VSIX lifecycle specifies support for VSIX package installation and updates. We need to read VSIX archives to extract manifests before processing installations.
- **Evidence**: `crates/extensions/src/manifest.rs` parses string representations of `package.json`. There is no current logic handling `.vsix` ZIP binaries in `crates/extensions/src/`. Downloading a real vsix package (like rust-analyzer) and running `unzip -l` shows that the manifest is indeed found at `extension/package.json`.
- **Impact**: Unlocks the ability to inspect packaged extensions, satisfying the first step of VSIX lifecycle management.

## 2. File Inventory

```json
{
  "id": "extensions-018",
  "role": "extensions",
  "create": [
    "crates/extensions/src/vsix.rs"
  ],
  "modify": [
    "crates/extensions/Cargo.toml",
    "crates/extensions/src/lib.rs"
  ],
  "read_only": [
    "crates/extensions/src/manifest.rs"
  ],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Introduce a `vsix` module providing `read_manifest(archive_path)` that opens a `.vsix` file as a ZIP archive, reads the bytes of `extension/package.json`, and parses it into an `ExtensionManifest`. Add the `zip` crate to `Cargo.toml`.
- **Compatibility constraints**: VSIX files are ZIP archives. As verified from existing real VSIX packages (like rust-analyzer), the extension manifest is located at `extension/package.json`.
- **Architectural constraints**: Keep logic isolated within the `extensions` crate.
- **Public contract changes**: Expose VSIX parsing via `crates/extensions/src/lib.rs`.

## 4. Test Plan

- **Failing evidence**: No tests exist for VSIX extraction.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: A test writes a mock VSIX file containing a valid `extension/package.json` and successfully parses it back into an `ExtensionManifest`. The test should clean up the created file or run in memory.
- **Edge cases**: Handle missing `extension/package.json` and malformed ZIP files gracefully.

## 5. Constraints

- **Non-goals**: Do not implement full VSIX extraction (unpacking all files). Just read the manifest.
- **Forbidden changes**: Do not modify `manifest.rs`.
- **Scope ceiling**: Reading one file from a zip is a bounded task for one run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
