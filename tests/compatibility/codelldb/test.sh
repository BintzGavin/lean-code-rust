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
    URL="https://marketplace.visualstudio.com/_apis/public/gallery/publishers/vadimcn/vsextensions/vscode-lldb/1.11.0/vspackage"
else
    URL="https://marketplace.visualstudio.com/_apis/public/gallery/publishers/vadimcn/vsextensions/vscode-lldb/1.11.0/vspackage?targetPlatform=$TARGET_PLATFORM"
fi

echo "Downloading CodeLLDB extension for $TARGET_PLATFORM..."
curl -s --compressed -L -o codelldb_download "$URL"
if gzip -t codelldb_download 2>/dev/null; then
    mv codelldb_download codelldb.vsix.gz
    gzip -d codelldb.vsix.gz
else
    mv codelldb_download codelldb.vsix
fi

echo "Extracting VSIX..."
rm -rf extracted_vsix
unzip -q codelldb.vsix -d extracted_vsix

echo "Setting up offline synthetic debug target..."
mkdir -p offline_project
cat << 'TARGET' > offline_project/main.c
#include <stdio.h>
int main() {
    printf("Debug target\n");
    return 0;
}
TARGET

cat << 'LAUNCH' > offline_project/launch.json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "program": "${workspaceFolder}/a.out",
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
LAUNCH

cat << 'GITIGNORE' > .gitignore
codelldb.vsix
extracted_vsix/
offline_project/
GITIGNORE

echo "Qualification fixture setup complete."
