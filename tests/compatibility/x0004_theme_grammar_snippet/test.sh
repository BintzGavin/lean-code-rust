#!/usr/bin/env bash
set -e
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Skipping syntax validation."
    exit 0
fi
cd "$DIR"
echo "Validating package.json..."
node -e "
const pkg = require('./package.json');
if (!pkg.name || !pkg.main) throw new Error('Invalid package.json');
if (!pkg.contributes) throw new Error('Missing contributes section');
const fs = require('fs');

if (pkg.contributes.themes) {
  for (const t of pkg.contributes.themes) {
    if (!fs.existsSync(t.path)) throw new Error('Missing theme file: ' + t.path);
    JSON.parse(fs.readFileSync(t.path));
  }
} else throw new Error('Missing themes contribution');

if (pkg.contributes.languages) {
  for (const l of pkg.contributes.languages) {
    if (!fs.existsSync(l.configuration)) throw new Error('Missing language config: ' + l.configuration);
    JSON.parse(fs.readFileSync(l.configuration));
  }
} else throw new Error('Missing languages contribution');

if (pkg.contributes.grammars) {
  for (const g of pkg.contributes.grammars) {
    if (!fs.existsSync(g.path)) throw new Error('Missing grammar file: ' + g.path);
    JSON.parse(fs.readFileSync(g.path));
  }
} else throw new Error('Missing grammars contribution');

if (pkg.contributes.snippets) {
  for (const s of pkg.contributes.snippets) {
    if (!fs.existsSync(s.path)) throw new Error('Missing snippets file: ' + s.path);
    JSON.parse(fs.readFileSync(s.path));
  }
} else throw new Error('Missing snippets contribution');
"
echo "Validating extension.js syntax and exports..."
node -e "
const Module = require('module');
const ext = require('./extension.js');
if (typeof ext.activate !== 'function') throw new Error('Missing activate function');
if (typeof ext.deactivate !== 'function') throw new Error('Missing deactivate function');
"
echo "Validation successful!"
