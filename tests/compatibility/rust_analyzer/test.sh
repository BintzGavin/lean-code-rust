#!/bin/bash
set -e
cd "$(dirname "$0")"

# Determine target platform based on OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

TARGET_PLATFORM=""
if [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        TARGET_PLATFORM="darwin-arm64"
    elif [ "$ARCH" = "x86_64" ]; then
        TARGET_PLATFORM="darwin-x64"
    fi
elif [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET_PLATFORM="linux-x64"
    elif [ "$ARCH" = "aarch64" ]; then
        TARGET_PLATFORM="linux-arm64"
    elif [ "$ARCH" = "armv7l" ]; then
        TARGET_PLATFORM="linux-armhf"
    fi
elif [[ "$OS" == MINGW* ]] || [[ "$OS" == CYGWIN* ]] || [[ "$OS" == MSYS* ]]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET_PLATFORM="win32-x64"
    elif [ "$ARCH" = "aarch64" ]; then
        TARGET_PLATFORM="win32-arm64"
    elif [ "$ARCH" = "i686" ]; then
        TARGET_PLATFORM="win32-ia32"
    fi
fi

if [ -z "$TARGET_PLATFORM" ]; then
    echo "Unsupported OS/ARCH combination: $OS $ARCH"
    # Fallback to an un-platform-specific download (might not exist, but let's try)
    URL="https://marketplace.visualstudio.com/_apis/public/gallery/publishers/rust-lang/vsextensions/rust-analyzer/0.3.2121/vspackage"
else
    URL="https://marketplace.visualstudio.com/_apis/public/gallery/publishers/rust-lang/vsextensions/rust-analyzer/0.3.2121/vspackage?targetPlatform=$TARGET_PLATFORM"
fi

echo "Downloading rust-analyzer extension for $TARGET_PLATFORM..."
curl -s --compressed -L -o rust-analyzer.vsix "$URL"

echo "Extracting VSIX..."
rm -rf extracted_vsix
unzip -q rust-analyzer.vsix -d extracted_vsix

echo "Setting up offline synthetic project..."
mkdir -p offline_project/src
cat << 'RUST' > offline_project/src/main.rs
fn main() {
    println!("Hello, world!");
}
RUST

cat << 'TOML' > offline_project/Cargo.toml
[package]
name = "offline_project"
version = "0.1.0"
edition = "2021"

[dependencies]
TOML

echo "Qualification fixture setup complete."
