#!/bin/bash
set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$DIR"

echo "Setup GitHub Theme qualification fixture..."
VSIX_URL="https://marketplace.visualstudio.com/_apis/public/gallery/publishers/GitHub/vsextensions/github-vscode-theme/6.3.3/vspackage"
VSIX_FILE="github_theme.vsix"

if [ ! -f "$VSIX_FILE" ]; then
    echo "Downloading GitHub Theme VSIX..."
    curl -s -L "$VSIX_URL" -o "$VSIX_FILE"
fi

# Decompress if needed
if gzip -t "$VSIX_FILE" 2>/dev/null; then
    mv "$VSIX_FILE" "$VSIX_FILE.gz"
    gunzip "$VSIX_FILE.gz"
fi

rm -rf extension
unzip -q "$VSIX_FILE" "extension/*"

# Check if package.json exists in extracted folder
if [ ! -f "extension/package.json" ]; then
    echo "Failed to extract package.json"
    exit 1
fi

echo "Fixture setup successful."
