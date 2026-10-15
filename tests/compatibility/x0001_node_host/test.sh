#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Check if node is available; we mock the `vscode` import to test syntax
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Skipping syntax validation."
    exit 0
fi

cd "$DIR"

echo "Validating package.json..."
node -e "const pkg = require('./package.json'); if (!pkg.name || !pkg.main) throw new Error('Invalid package.json');"

echo "Validating extension.js syntax and exports..."
# Use a simple mock for vscode to allow requiring the extension
node -e "
const Module = require('module');
const originalRequire = Module.prototype.require;
Module.prototype.require = function(path) {
    if (path === 'vscode') {
        return {
            commands: { registerCommand: () => ({ dispose: () => {} }) },
            window: { showInformationMessage: () => {}, showErrorMessage: () => {} }
        };
    }
    return originalRequire.apply(this, arguments);
};

const ext = require('./extension.js');
if (typeof ext.activate !== 'function') throw new Error('Missing activate function');
if (typeof ext.deactivate !== 'function') throw new Error('Missing deactivate function');

// Test successful activation
ext.activate({ subscriptions: [] });
console.log('Activation syntax and structure validated (success flow).');

// Test failure activation
process.env.X0001_FAIL_ACTIVATION = '1';
try {
    ext.activate({ subscriptions: [] });
    throw new Error('Expected activation to fail');
} catch (e) {
    if (e.message !== 'Intentional activation failure for S-0042') {
        throw e;
    }
}
console.log('Activation syntax and structure validated (exception flow).');
"

echo "Validation successful!"
