# Repository instructions

Read `VISION.md`, `.sys/black-hole/role-map.md`, the matching prompt, backlog, memory and status before work. Only the Vision defines desired product state. The role map defines static write boundaries; repository files are the complete durable state.

Planners write only their plan and backlog. Executors persist a claim before product edits and follow one bounded work order. Never discard unrelated user changes. No role edits `VISION.md`, this file, `.sys/black-hole/`, `docs/prompts/`, `docs/decisions/` or `spec/`; those are human-locked inputs. Setup and explicitly authorized governance repairs are separate from recurring roles.

No role may write a peer's state. Dependencies refer to the actual owner and repository evidence. Preserve recoverable blockers and negative experiments. Stop successfully with no changes when no eligible work exists. Empty PRs are diagnostic evidence, not a trigger to fabricate work or suppress publication. No PR publishing, schedule changes, deployment, credentials or remote creation are authorized by a work order unless explicitly stated.

Implementation must remain independent: use `spec/`, public interface documentation and this repository only. Do not load original editor source, original tests/assets or source-aware discovery notes. The setup analyst must hand product implementation to a fresh agent without inherited source-aware history. This is a procedural information boundary, not a legal clean-room certification or OS access control.

On the shared 18 GB Mac, inspect resources first; no heavy builds or concurrent browser jobs below 40% free memory. A std-only single-job unit build may run above that threshold. Do not terminate unrelated processes. No dependency downloads for the initial slice.

Checks: `python3 tooling/check_scaffold.py`; `python3 -m unittest discover -s tooling -p 'test_*.py'`; `python3 spec/checks/check_package.py`. After the core exists: `CARGO_BUILD_JOBS=1 cargo test --manifest-path crates/editor-core/Cargo.toml --offline --locked` and `cargo fmt --manifest-path crates/editor-core/Cargo.toml --check`.
