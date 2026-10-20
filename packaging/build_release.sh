#!/bin/bash
set -e
echo "Building release..."
cargo build --release --workspace
echo "Assembling artifacts..."
mkdir -p dist
cp target/release/desktop dist/
cd dist
tar -czf desktop.tar.gz desktop
cd ..
echo "Done."
du -h dist/desktop.tar.gz
