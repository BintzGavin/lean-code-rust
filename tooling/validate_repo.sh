#!/bin/bash
set -e

python3 tooling/check_scaffold.py
python3 -m unittest discover -s tooling -p 'test_*.py'
python3 spec/checks/check_package.py
CARGO_BUILD_JOBS=1 cargo test --workspace
cargo fmt --all --check
CARGO_BUILD_JOBS=1 cargo run --manifest-path crates/editor-core/Cargo.toml --example edit_roundtrip --offline --locked
