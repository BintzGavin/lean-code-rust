#!/usr/bin/env bash
set -e
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Skipping syntax validation."
    exit 0
fi
cd "$DIR"
echo "Validating package.json..."
node -e "const pkg = require('./package.json'); if (!pkg.name || !pkg.main) throw new Error('Invalid package.json');"
echo "Validating extension.js syntax and exports..."
node -e "
const Module = require('module');
const originalRequire = Module.prototype.require;
Module.prototype.require = function(path) {
    if (path === 'vscode') {
        return {
            commands: { registerCommand: () => ({ dispose: () => {} }), executeCommand: async () => {} },
            workspace: { getConfiguration: () => ({ update: async () => {}, get: () => 'updated_value' }), onDidChangeConfiguration: () => ({ dispose: () => {} }) },
            window: { showInformationMessage: () => {}, showErrorMessage: () => {} },
            ConfigurationTarget: { Global: 1 }
        };
    }
    return originalRequire.apply(this, arguments);
};
const ext = require('./extension.js');
if (typeof ext.activate !== 'function') throw new Error('Missing activate function');
if (typeof ext.deactivate !== 'function') throw new Error('Missing deactivate function');
ext.activate({ subscriptions: [] });
console.log('Activation syntax and structure validated.');
"
echo "Validation successful!"
