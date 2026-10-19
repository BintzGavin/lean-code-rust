# EXTENSIONS Status

**Responsibility**: VSIX lifecycle and the pinned stable VS Code public extension contract.
**Owned paths**:
- `crates/extensions/`
- `.sys/plans/extensions/`
- `.sys/backlogs/extensions.md`
- `.sys/memory/extensions.md`
- `docs/status/EXTENSIONS.md`

## Current state



## Evidence entry format

```markdown
### [WORK-ORDER-ID] — [completed | blocked | needs_input | cancelled]
- **Result**: [Concise outcome]
- **Changed files**: [Exact paths]
- **Checks**: [Commands and results]
- **Dependency**: [Durable blocked condition, or “None”]
- **Question**: [Exact human question for `needs_input`, or “None”]
- **Resolution evidence**: [Recorded answer or dependency evidence, or “None”]
```

### extensions-001 — completed
- **Result**: Successfully bootstrapped `extensions` crate.
- **Changed files**: `crates/extensions/Cargo.toml`, `crates/extensions/src/lib.rs`
- **Checks**: `cargo check --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-003 — completed
- **Result**: Extended `ExtensionManifest` to parse `activationEvents` and basic `contributes.commands`.
- **Changed files**: `crates/extensions/src/manifest.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-002 — completed
- **Result**: Parsed VS Code extension manifest package.json.
- **Changed files**: crates/extensions/Cargo.toml, crates/extensions/src/lib.rs, crates/extensions/src/manifest.rs
- **Checks**: cargo test, check_scaffold.py
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-004 — completed
- **Result**: Extended ExtensionManifest to parse extensionDependencies and extensionPack.
- **Changed files**: crates/extensions/src/manifest.rs
- **Checks**: cargo test --manifest-path crates/extensions/Cargo.toml --locked, python3 tooling/check_scaffold.py --role extensions
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-005 — completed
- **Result**: Extended ExtensionContributes to parse configuration and keybindings.
- **Changed files**: crates/extensions/src/manifest.rs
- **Checks**: cargo test --manifest-path crates/extensions/Cargo.toml, python3 tooling/check_scaffold.py --role extensions
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-006 — completed
**Result:** Added parsing for contributes.menus
**Changed files:** crates/extensions/src/manifest.rs
**Checks:** `cargo test --manifest-path crates/extensions/Cargo.toml`
**Dependencies:** none

### EXTENSIONS-PLANNER — completed
- **Result**: Created work order `extensions-029-synthetic-vscode-types.md` and appended to backlog.
- **Changed files**: `.sys/backlogs/extensions.md`, `.sys/plans/extensions/extensions-029-synthetic-vscode-types.md`
- **Checks**: `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`, `cargo fmt --all --check`, `CARGO_BUILD_JOBS=1 cargo test --workspace`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None


### extensions-007 — Parse contributes.themes, languages, grammars, snippets
**State**: completed
**Result**: Parsed themes, languages, grammars, and snippets from VS Code extension manifest package.json.
**Changed files**:
- `crates/extensions/src/manifest.rs`
**Checks**:
- `cargo test --manifest-path crates/extensions/Cargo.toml`
- `python3 tooling/check_scaffold.py --role extensions`
- `bash tooling/validate_repo.sh`

### extensions-008 — completed
- **Result**: Parsed contributes.views and contributes.viewsContainers
- **Changed files**: crates/extensions/src/manifest.rs
- **Checks**: None
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-009 — completed
- **Result**: Parsed contributes.debuggers and contributes.breakpoints from package.json.
- **Changed files**: crates/extensions/src/manifest.rs
- **Checks**: cargo test --manifest-path crates/extensions/Cargo.toml, python3 tooling/check_scaffold.py --role extensions, bash tooling/validate_repo.sh
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-010 — completed
- **Result**: Parsed contributes.taskDefinitions from package.json.
- **Changed files**: crates/extensions/src/manifest.rs
- **Checks**: CARGO_BUILD_JOBS=1 cargo test --workspace, python3 tooling/check_scaffold.py --role extensions, bash tooling/validate_repo.sh
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-011 — completed
- **Result**: Parsed `capabilities` and `extensionKind` from VS Code extension manifest `package.json`.
- **Changed files**: `crates/extensions/src/manifest.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-012 — completed
- **Result**: Parsed enabledApiProposals from package.json manifest.
- **Changed files**: crates/extensions/src/manifest.rs
- **Checks**: cargo test --manifest-path crates/extensions/Cargo.toml, python3 tooling/check_scaffold.py --role extensions, bash tooling/validate_repo.sh
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-013 — completed
- **Result**: Parsed contributes.problemMatchers and contributes.problemPatterns.
- **Changed files**: crates/extensions/src/manifest.rs
- **Checks**: cargo test --manifest-path crates/extensions/Cargo.toml, python3 tooling/check_scaffold.py --role extensions, bash tooling/validate_repo.sh
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-014 — completed
- **Result**: Parsed metadata and browser entry point from package.json manifest.
- **Changed files**: `crates/extensions/src/manifest.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-015
**Status:** Completed
**Files Changed:** `crates/extensions/src/manifest.rs`
**Checks Run:** `cargo test --manifest-path crates/extensions/Cargo.toml`, `cargo fmt --all`, `python3 tooling/check_scaffold.py --role extensions`
**Result:** Parsed contributes.colors and contributes.configurationDefaults from package.json.

## extensions-016
**Status:** completed
**Result:** Parsed contributes.submenus from package.json manifest.
**Files:**
- crates/extensions/src/manifest.rs
**Checks:**
- cargo test --manifest-path crates/extensions/Cargo.toml
- python3 tooling/check_scaffold.py --role extensions

### extensions-017 — completed
- **Result**: Successfully parsed the missing manifest metadata fields into ExtensionManifest.
- **Changed files**: `crates/extensions/src/manifest.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-019 — completed
- **Result**: Implemented extract_vsix function to extract extension contents from vsix archives.
- **Changed files**: `crates/extensions/src/vsix.rs`
- **Checks**: `cargo test --workspace`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-020 — completed
- **Result**: Implemented `install_vsix` to extract extensions into standard canonical paths.
- **Changed files**: `crates/extensions/src/vsix.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-021 — completed
- **Result**: Implemented `uninstall_vsix` to remove canonical extension directories.
- **Changed files**: `crates/extensions/src/vsix.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-022 — completed
- **Result**: Implemented list_installed_extensions to discover extensions from disk.
- **Changed files**: `crates/extensions/src/vsix.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-023 — completed
- **Result**: Implemented the ExtensionHost process manager for isolated Node hosts.
- **Changed files**: `crates/extensions/src/host.rs`, `crates/extensions/src/lib.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-024 — completed
- **Result**: Successfully implemented basic IPC for ExtensionHost.
- **Changed files**: `crates/extensions/src/host.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-025 — completed
- **Result**: Implemented a Node bootstrap shim to allow ExtensionHost to dynamically load and activate extensions via IPC.
- **Changed files**: `crates/extensions/src/host.rs`, `crates/extensions/src/bootstrap.js`
- **Checks**: `cargo test --workspace`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-026 — completed
- **Result**: Successfully implemented ExtensionContext subscriptions and disposal via deactivate IPC command.
- **Changed files**: `crates/extensions/src/bootstrap.js`, `crates/extensions/src/host.rs`
- **Checks**: `cargo test --manifest-path crates/extensions/Cargo.toml`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None

### extensions-027 — completed
- **Result**: Successfully injected synthetic vscode module into Node host.
- **Changed files**: `crates/extensions/src/bootstrap.js`, `crates/extensions/src/host.rs`
- **Checks**: `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/extensions/Cargo.toml`, `python3 tooling/check_scaffold.py --role extensions`, `bash tooling/validate_repo.sh`
- **Dependency**: None
- **Question**: None
- **Resolution evidence**: None
