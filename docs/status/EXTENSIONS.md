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

### EXTENSIONS-PLANNER — blocked
- **Result**: Blocked from planning due to existing ready work order.
- **Changed files**: crates/extensions/src/manifest.rs
- **Checks**: None
- **Dependency**: Existing ready entry extensions-007 in .sys/backlogs/extensions.md
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
