#!/bin/bash
set -e
echo "Building release..."
cargo build --release --workspace
echo "Assembling artifacts..."
mkdir -p dist
cp target/release/desktop dist/
echo "Done."
du -h dist/desktop
