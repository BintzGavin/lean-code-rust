# Black Hole Architecture Role Map

## Gravity

**Authoritative Vision**: `VISION.md`

This is the sole definition of the desired state. Each role's planner and executor may read it but never edit it.

## Cadence

- **Operating window**: continuous, 24/7
- **System tick**: one hour
- **Phase order**: planning → execution → repeat
- **Planning wave**: run one eligible planner per role in parallel
- **Execution wave**: run one eligible executor per role in parallel one hour later
- **Per-role recurrence**: two-hour planner and executor streams offset by one hour
- **Overlap policy**: same-role overlap is prohibited; unrelated roles continue independently
- **No-work policy**: a role with no eligible planning or execution work completes as a successful no-op
- **Missed-run policy**: preserve the phase order for that role; never stack a new plan over a nonterminal backlog entry
- **Recovery policy**: preserve `blocked` and `needs_input` work for dependency resolution or manual intervention; only `completed` and explicit `cancelled` are terminal

## Global invariants

- Planning and execution remain separate responsibilities.
- Every role has one plan namespace, one authoritative execution backlog, one memory file, one status file, and one planner/executor prompt pair.
- Each backlog is role-local, shared only by that role's planner and executor, and owns lifecycle state; other roles must not write it.
- Each memory file is a role-local memory ledger shared only by that role's planner and executor across runs; other roles must not write it.
- Product write ownership is disjoint across roles.
- Shared writable files have exactly one owner.
- Cross-role needs are recorded as dependencies.
- Permission-sensitive work remains recoverable through a durable `needs_input` state.
- Missing eligible work produces no repository changes.

## Roles

### EDITOR-CORE (`editor-core`)

- **Responsibility**: Document buffers, local edits and safe persistence.
- **Domain / Reality paths**: `crates/editor-core/`. Absent paths are expected future surfaces.
- **Owned paths**: see the ownership JSON below, including this role's state.
- **Read-only paths**: `VISION.md`, `spec/`, peer interfaces and evidence, `.sys/black-hole/role-map.md`.
- **Plan namespace**: `.sys/plans/editor-core/`
- **Execution backlog**: `.sys/backlogs/editor-core.md`; this role's planner/executor only.
- **Memory**: `.sys/memory/editor-core.md`; planner reads, executor writes critical learnings only.
- **Status**: `docs/status/EDITOR-CORE.md`
- **Planner prompt**: `docs/prompts/planning-editor-core.md`
- **Executor prompt**: `docs/prompts/execution-editor-core.md`
- **Vision priorities**: Editing, dirty state, save and cancellation preserve user data.
- **Verification commands**: `python3 tooling/check_scaffold.py --role editor-core`. Behavioral commands belong in bounded plans.
- **Implementation patterns**: independent code; no product implementation exists at bootstrap.
- **Documentation duties**: truthful role-local outcomes and critical lessons.
- **Dependencies**: future peer interfaces require explicit owning-role evidence; none declared for the initial core.

### DESKTOP (`desktop`)

- **Responsibility**: Native window, rendering, input, navigation and accessibility.
- **Domain / Reality paths**: `crates/desktop/`. Absent paths are expected future surfaces.
- **Owned paths**: see the ownership JSON below, including this role's state.
- **Read-only paths**: `VISION.md`, `spec/`, peer interfaces and evidence, `.sys/black-hole/role-map.md`.
- **Plan namespace**: `.sys/plans/desktop/`
- **Execution backlog**: `.sys/backlogs/desktop.md`; this role's planner/executor only.
- **Memory**: `.sys/memory/desktop.md`; planner reads, executor writes critical learnings only.
- **Status**: `docs/status/DESKTOP.md`
- **Planner prompt**: `docs/prompts/planning-desktop.md`
- **Executor prompt**: `docs/prompts/execution-desktop.md`
- **Vision priorities**: A usable local desktop editor, including keyboard and accessibility paths.
- **Verification commands**: `python3 tooling/check_scaffold.py --role desktop`. Behavioral commands belong in bounded plans.
- **Implementation patterns**: independent code; no product implementation exists at bootstrap.
- **Documentation duties**: truthful role-local outcomes and critical lessons.
- **Dependencies**: future peer interfaces require explicit owning-role evidence; none declared for the initial core.

### EXTENSIONS (`extensions`)

- **Responsibility**: VSIX lifecycle and the pinned stable VS Code public extension contract.
- **Domain / Reality paths**: `crates/extensions/`. Absent paths are expected future surfaces.
- **Owned paths**: see the ownership JSON below, including this role's state.
- **Read-only paths**: `VISION.md`, `spec/`, peer interfaces and evidence, `.sys/black-hole/role-map.md`.
- **Plan namespace**: `.sys/plans/extensions/`
- **Execution backlog**: `.sys/backlogs/extensions.md`; this role's planner/executor only.
- **Memory**: `.sys/memory/extensions.md`; planner reads, executor writes critical learnings only.
- **Status**: `docs/status/EXTENSIONS.md`
- **Planner prompt**: `docs/prompts/planning-extensions.md`
- **Executor prompt**: `docs/prompts/execution-extensions.md`
- **Vision priorities**: Existing VS Code extensions remain a hard requirement; LSP/DAP alone is insufficient.
- **Verification commands**: `python3 tooling/check_scaffold.py --role extensions`. Behavioral commands belong in bounded plans.
- **Implementation patterns**: independent code; no product implementation exists at bootstrap.
- **Documentation duties**: truthful role-local outcomes and critical lessons.
- **Dependencies**: future peer interfaces require explicit owning-role evidence; none declared for the initial core.

### QUALITY (`quality`)

- **Responsibility**: Independent reference observations, compatibility fixtures and measured evidence.
- **Domain / Reality paths**: `tests/compatibility/`, `tests/reference/`, `evidence/`. Absent paths are expected future surfaces.
- **Owned paths**: see the ownership JSON below, including this role's state.
- **Read-only paths**: `VISION.md`, `spec/`, peer interfaces and evidence, `.sys/black-hole/role-map.md`.
- **Plan namespace**: `.sys/plans/quality/`
- **Execution backlog**: `.sys/backlogs/quality.md`; this role's planner/executor only.
- **Memory**: `.sys/memory/quality.md`; planner reads, executor writes critical learnings only.
- **Status**: `docs/status/QUALITY.md`
- **Planner prompt**: `docs/prompts/planning-quality.md`
- **Executor prompt**: `docs/prompts/execution-quality.md`
- **Vision priorities**: Observe parity and startup, process-tree memory and network before making claims.
- **Verification commands**: `python3 tooling/check_scaffold.py --role quality`. Behavioral commands belong in bounded plans.
- **Implementation patterns**: independent code; no product implementation exists at bootstrap.
- **Documentation duties**: truthful role-local outcomes and critical lessons.
- **Dependencies**: future peer interfaces require explicit owning-role evidence; none declared for the initial core.

### INTEGRATION (`integration`)

- **Responsibility**: Build, packaging, release assembly and repository validation.
- **Domain / Reality paths**: `tooling/`, `packaging/`, `docs/release/`, `README.md`, `.gitignore`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`. Absent paths are expected future surfaces.
- **Owned paths**: see the ownership JSON below, including this role's state.
- **Read-only paths**: `VISION.md`, `spec/`, peer interfaces and evidence, `.sys/black-hole/role-map.md`.
- **Plan namespace**: `.sys/plans/integration/`
- **Execution backlog**: `.sys/backlogs/integration.md`; this role's planner/executor only.
- **Memory**: `.sys/memory/integration.md`; planner reads, executor writes critical learnings only.
- **Status**: `docs/status/INTEGRATION.md`
- **Planner prompt**: `docs/prompts/planning-integration.md`
- **Executor prompt**: `docs/prompts/execution-integration.md`
- **Vision priorities**: A reproducible green build and launch precede performance claims and release.
- **Verification commands**: `python3 tooling/check_scaffold.py --role integration`. Behavioral commands belong in bounded plans.
- **Implementation patterns**: independent code; no product implementation exists at bootstrap.
- **Documentation duties**: truthful role-local outcomes and critical lessons.
- **Dependencies**: future peer interfaces require explicit owning-role evidence; none declared for the initial core.



## Ownership data

This block is the authoritative static path map in this document. Trailing slash means directory; other entries are exact files. Governance and imported spec paths are read-only to every recurring role.

```json
{
  "editor-core": {
    "responsibility": "Document buffers, local edits and safe persistence.",
    "product": [
      "crates/editor-core/"
    ],
    "owned": [
      "crates/editor-core/",
      ".sys/plans/editor-core/",
      ".sys/backlogs/editor-core.md",
      ".sys/memory/editor-core.md",
      "docs/status/EDITOR-CORE.md"
    ]
  },
  "desktop": {
    "responsibility": "Native window, rendering, input, navigation and accessibility.",
    "product": [
      "crates/desktop/"
    ],
    "owned": [
      "crates/desktop/",
      ".sys/plans/desktop/",
      ".sys/backlogs/desktop.md",
      ".sys/memory/desktop.md",
      "docs/status/DESKTOP.md"
    ]
  },
  "extensions": {
    "responsibility": "VSIX lifecycle and the pinned stable VS Code public extension contract.",
    "product": [
      "crates/extensions/"
    ],
    "owned": [
      "crates/extensions/",
      ".sys/plans/extensions/",
      ".sys/backlogs/extensions.md",
      ".sys/memory/extensions.md",
      "docs/status/EXTENSIONS.md"
    ]
  },
  "quality": {
    "responsibility": "Independent reference observations, compatibility fixtures and measured evidence.",
    "product": [
      "tests/compatibility/",
      "tests/reference/",
      "evidence/"
    ],
    "owned": [
      "tests/compatibility/",
      "tests/reference/",
      "evidence/",
      ".sys/plans/quality/",
      ".sys/backlogs/quality.md",
      ".sys/memory/quality.md",
      "docs/status/QUALITY.md"
    ]
  },
  "integration": {
    "responsibility": "Build, packaging, release assembly and repository validation.",
    "product": [
      "tooling/",
      "packaging/",
      "docs/release/",
      "README.md",
      ".gitignore",
      "Cargo.toml",
      "Cargo.lock",
      "rust-toolchain.toml"
    ],
    "owned": [
      "tooling/",
      "packaging/",
      "docs/release/",
      "README.md",
      ".gitignore",
      "Cargo.toml",
      "Cargo.lock",
      "rust-toolchain.toml",
      ".sys/plans/integration/",
      ".sys/backlogs/integration.md",
      ".sys/memory/integration.md",
      "docs/status/INTEGRATION.md"
    ]
  }
}
```

## Overlap review

- **Conflicts found**: none; equality and directory-prefix containment checked in both directions before file creation.
- **Approved by**: parent setup agent under Gavin's new-repository instruction.
- **Cadence anchor**: UTC; planning at even hours, execution at the following odd hour. Continuous 24/7. Declarative only; no scheduler was installed or modified. Same-role missed runs preserve phase order and cannot overlap an unresolved claim. No elapsed-time claim stealing.
- **Idle/publication**: no eligible work means successful unchanged output. Empty PRs remain diagnostic evidence; this repository adds no suppression check and does not authorize publishing PRs.
