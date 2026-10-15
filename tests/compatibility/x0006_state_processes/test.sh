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
        return {};
    }
    return originalRequire.apply(this, arguments);
};

const ext = require('./extension.js');
if (typeof ext.activate !== 'function') throw new Error('Missing activate function');
if (typeof ext.deactivate !== 'function') throw new Error('Missing deactivate function');

const workspaceStore = new Map();
const globalStore = new Map();
const secretStore = new Map();

const context = {
    workspaceState: {
        get: (k) => workspaceStore.get(k),
        update: (k, v) => workspaceStore.set(k, v)
    },
    globalState: {
        get: (k) => globalStore.get(k),
        update: (k, v) => globalStore.set(k, v)
    },
    secrets: {
        store: (k, v) => secretStore.set(k, v),
        get: (k) => secretStore.get(k)
    }
};

// Test successful activation (S-0051)
ext.activate(context);

if (workspaceStore.get('x0006WorkspaceKey') !== 'x0006WorkspaceValue') throw new Error('Workspace state validation failed');
if (globalStore.get('x0006GlobalKey') !== 'x0006GlobalValue') throw new Error('Global state validation failed');
if (secretStore.get('x0006SecretKey') !== 'x0006SecretValue') throw new Error('Secret state validation failed');

console.log('Activation syntax and structure validated (success flow).');

// Test failure activation (S-0052)
process.env.X0006_FAIL_ACTIVATION = '1';
try {
    ext.activate(context);
    throw new Error('Expected activation to fail');
} catch (e) {
    if (e.message !== 'Intentional activation failure for S-0052') {
        throw e;
    }
}
console.log('Activation syntax and structure validated (exception flow).');
"

echo "Validation successful!"
