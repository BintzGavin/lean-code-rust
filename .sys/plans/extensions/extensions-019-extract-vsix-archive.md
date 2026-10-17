# extensions-019 — Extract VSIX archive

**Role**: `extensions`
**Backlog entry**: `.sys/backlogs/extensions.md#extensions-019`

## 1. Context & Goal

- **Objective**: Extract the `extension/` contents of a `.vsix` archive to a target directory.
- **Vision gap**: X-0010 VSIX lifecycle specifies support for VSIX package installation. Currently, we can read the manifest from a `.vsix` archive, but we cannot unpack the package contents to disk for installation.
- **Evidence**: `crates/extensions/src/vsix.rs` only implements `read_manifest` and lacks extraction logic.
- **Impact**: Enables local installation of extensions by placing their files on disk, which is a prerequisite for activating and executing them.

## 2. File Inventory

```json
{
  "id": "extensions-019",
  "role": "extensions",
  "create": [],
  "modify": [
    "crates/extensions/src/vsix.rs"
  ],
  "read_only": [],
  "dependencies": []
}
```

## 3. Implementation Spec

- **Required behavior**: Introduce an `extract_vsix<P: AsRef<Path>, D: AsRef<Path>>(archive_path: P, target_dir: D) -> Result<(), VsixError>` function in `vsix.rs`. The function should iterate over the files in the zip archive. If a file's name starts with `extension/`, it should strip that prefix and extract it into `target_dir`. Parent directories should be created as needed.
- **Compatibility constraints**: Protect against Zip Slip vulnerabilities. The final destination path must be verified to be lexically inside `target_dir`. Entries that attempt to traverse outside `target_dir` (e.g., using `../`) must result in a `VsixError` (e.g., a new `InvalidPath` variant).
- **Architectural constraints**: Keep logic isolated within the `extensions` crate. Use the existing `zip` dependency.
- **Public contract changes**: Expose `extract_vsix` as a public function in `vsix.rs`.

## 4. Test Plan

- **Failing evidence**: No tests exist for extracting VSIX contents.
- **Verification commands**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Success criteria**: A test creates a mock VSIX with `extension/package.json` and `extension/out/main.js`, calls `extract_vsix` to a temporary directory, and verifies both files exist at the correct stripped paths. Another test creates a malicious VSIX containing a Zip Slip path (e.g., `extension/../escaped.txt`) and verifies `extract_vsix` returns an error.
- **Edge cases**: Missing `extension/` prefix for some files in the archive (ignore them or handle gracefully without erroring, as VSIX can contain `[Content_Types].xml` or `extension.vsixmanifest` outside the `extension/` folder).

## 5. Constraints

- **Non-goals**: Do not implement full installation state management (like registering the extension in a local database) in this step. Just the extraction primitive.
- **Forbidden changes**: Do not modify `manifest.rs`.
- **Scope ceiling**: Implementing secure archive extraction is a clear, bounded task for one run.

## 6. Dependencies

- **Required first**: None.
- **Cross-role needs**: None.
- **Blocked when**: None.

## 7. Result
- **Outcome**: Successfully implemented extract_vsix
