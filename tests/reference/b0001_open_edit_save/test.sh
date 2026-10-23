#!/bin/bash
set -e

# S-0001 Open, edit, save text document

REPO_ROOT="$(pwd)"

# Create a temporary workspace for the test
WORKSPACE_DIR=$(mktemp -d)
CARGO_DIR=$(mktemp -d)

cleanup() {
    rm -rf "$WORKSPACE_DIR" "$CARGO_DIR"
}
trap cleanup EXIT
cd "$WORKSPACE_DIR"

# 1. Setup workspace
echo -n "cedar" > alpha.txt

# Initial file content and hash
echo "Initial text: $(cat alpha.txt)"
INITIAL_HASH=$(shasum -a 256 alpha.txt | awk '{print $1}')
echo "Initial hash: $INITIAL_HASH"

# Compile and run a Rust script to perform the edit using lean-editor-core
cd "$CARGO_DIR"
cargo init --bin test_runner >/dev/null 2>&1

# Add lean-editor-core as a dependency using its absolute path
cat << TOML_EOF > Cargo.toml
[package]
name = "test_runner"
version = "0.1.0"
edition = "2021"

[dependencies]
lean-editor-core = { path = "$REPO_ROOT/crates/editor-core" }
TOML_EOF

mkdir -p src
cat << 'RUST_EOF' > src/main.rs
use lean_editor_core::{CloseOutcome, Document};
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_path = std::env::var("TEST_FILE").expect("TEST_FILE not set");

    // Open
    let mut document = Document::open(&file_path)?;
    assert!(!document.is_dirty());
    assert_eq!(document.text(), "cedar");

    // Edit
    document.replace_text("maple");
    assert!(document.is_dirty());

    // Pre-save state check (file shouldn't have changed on disk yet)
    let disk_content = fs::read_to_string(&file_path)?;
    assert_eq!(disk_content, "cedar");

    // Save
    document.save()?;
    assert!(!document.is_dirty());

    // Close
    assert_eq!(document.request_close(), CloseOutcome::Closed);

    Ok(())
}
RUST_EOF

export TEST_FILE="$WORKSPACE_DIR/alpha.txt"
cargo run -q

# Re-open and verify
cd "$WORKSPACE_DIR"
REOPENED_TEXT=$(cat alpha.txt)
echo "Reopened text: $REOPENED_TEXT"

if [ "$REOPENED_TEXT" != "maple" ]; then
    echo "Error: Expected 'maple', got '$REOPENED_TEXT'"
    exit 1
fi


echo "PASS: B-0001 Open, edit, save text document"
