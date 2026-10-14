# Bootstrap validation

This is a setup checkpoint, not a release or convergence certification. The repository is a new independent Rust exploration; the existing Lean checkout and rejected repair branch were preserved. Its porcelain status matches the pre-reference snapshot.

## Delivered contract

- Sole Vision: `VISION.md`.
- Five disjoint roles: editor-core, desktop, extensions, quality, integration. Each has its own planner/executor prompts, plan namespace, authoritative backlog, critical memory and status.
- Cadence is declarative: hourly UTC waves, even-hour planning then odd-hour execution, no same-role overlap, no stacked nonterminal work. No external schedule was changed or installed.
- Empty ledgers and unchanged runs are valid. No empty-PR suppression or publication guard was added.
- Product implementation used a fresh agent with no inherited source-aware transcript, restricted by instruction to this repo and its independent `spec/`. This is procedural isolation, not an OS-enforced sandbox or legal certification.

## Commit evidence

| Commit | Evidence |
| --- | --- |
| `b089e8b` | Initial scaffold and independent specification |
| `202857d` | Durable EDITOR-CORE-001 claim before product edits |
| `cda6144` | Resource policy corrected to apply the 40% threshold to heavy jobs; tiny offline single-job checks allowed without critical pressure |
| `c7fa75e` | Review-found ownership, symlink, lifecycle and exact-inventory gaps corrected with regressions |
| `93cc473` | Interrupted-claim evidence and prior Result history explicitly preserved |
| `b04a3e9` | Six-file std-only Rust core plus own completion evidence |

## Executed checks

The setup agent reran these successfully after the implementation commit:

| Command | Observed result |
| --- | --- |
| `python3 tooling/check_scaffold.py` | PASS; structural contracts only |
| `python3 -m unittest discover -s tooling -p 'test_*.py'` | 21 tests passed |
| `python3 spec/checks/check_package.py` | 20 behavior areas, 64 blocked/unknown scenario cells, 12 proposed extension fixture families, 4 fixture hashes; 0 reference product scenarios executed |
| `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml --offline --locked` | 8 integration tests passed; no dependency downloads |
| `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check` | Passed |
| `CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip --offline --locked` | Synthetic open/edit/cancel/save/close/reopen passed; owned temporary file removed |

The implementer recorded the pre-implementation E0432 unresolved-import failure, then green tests, in the work-order Result. Non-fatal xcrun/linker warnings reported that the macOS user temp path was unavailable and `/tmp` was used. There was no resource error. No heavy editor build or browser job was started.

The 21 structural regressions exercise ownership overlap and mismatch, stale/incorrect dependency owners, blocked dependency recovery, cancelled prerequisites, dependency cycles, missing claims, false completion, exact file inventory, symlink/path escapes, orphan plans and unchanged idle validation. They validate structure and evidence presence, not the truth of arbitrary Result prose or executability of arbitrary dependency command strings. The checker never executes dependency commands or publishes anything.

## Independent review

An independent read-only BHA/scaffold reviewer demonstrated four bypasses in disposable copies and identified lifecycle contradictions. Fixes and regression tests landed in `c7fa75e`; its recheck passed all 21 tests. The final `93cc473` review confirmed preservation of prior claim/recovery history and governance approval for ownership changes, with no remaining material findings. No standing peer-review service or coordinating agent was installed.

A separate fresh read-only Rust reviewer found no blocking findings in `b04a3e9`. It checked the API, eight tests, deterministic save failure/retry, exact temporary-file cleanup and disclosed limitations. The reviewer inspected recorded execution evidence without rerunning tests; the setup agent independently reran the final checks listed above.

## Unverified product work and decisions

- No desktop GUI exists or was launched. No VS Code interaction parity, ordinary VSIX behavior, extension host, webview security, startup/memory/network baseline or release artifact is established.
- Core saves are explicitly non-atomic and not crash-safe. Failed saves preserve memory but may partially change disk. See the crate README before using it with real data.
- The reference GUI launch was denied through the supported Terminal app route; no launch occurred. See `reference-checkpoint.md` for the exact denial and recovery requirement.
- No remote exists. Proposed owner/name is `BintzGavin/lean-code-rust`; visibility remains Gavin's decision before remote creation/publication. No PR, merge, deployment, credential or external task change occurred.
