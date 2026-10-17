#!/bin/bash
set -e
cd "$(dirname "$0")"

echo "Downloading VS Code ESLint extension..."
URL="https://marketplace.visualstudio.com/_apis/public/gallery/publishers/dbaeumer/vsextensions/vscode-eslint/2.4.4/vspackage"
curl -s --compressed -L -o eslint.vsix "$URL"

echo "Extracting VSIX..."
rm -rf extracted_vsix
unzip -q eslint.vsix -d extracted_vsix

echo "Setting up offline synthetic project..."
mkdir -p offline_project

cat << 'PROJ_PKG' > offline_project/package.json
{
  "name": "offline-eslint-test",
  "version": "1.0.0",
  "dependencies": {
    "eslint": "^8.57.0"
  }
}
PROJ_PKG

cat << 'ESLINTRC' > offline_project/.eslintrc.json
{
  "env": {
    "browser": true,
    "es2021": true
  },
  "extends": "eslint:recommended",
  "parserOptions": {
    "ecmaVersion": 12,
    "sourceType": "module"
  },
  "rules": {
    "semi": ["error", "always"],
    "quotes": ["error", "double"]
  }
}
ESLINTRC

cat << 'INDEX' > offline_project/index.js
const greeting = 'Hello World'
console.log(greeting)
INDEX

echo "Qualification fixture setup complete."
