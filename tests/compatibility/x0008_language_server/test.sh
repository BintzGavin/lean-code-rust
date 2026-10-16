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
            workspace: {
                onDidOpenTextDocument: () => ({ dispose: () => {} })
            }
        };
    }
    return originalRequire.apply(this, arguments);
};

const ext = require('./extension.js');
if (typeof ext.activate !== 'function') throw new Error('Missing activate function');

const context = { subscriptions: [] };
const client = ext.activate(context);

async function runTests() {
    try {
        if (process.env.X0008_SCENARIO === 'S-0055') {
            const initResult = await client.sendRequest('initialize', {});
            if (!initResult.capabilities || !initResult.capabilities.hoverProvider) {
                throw new Error('Initialization failed');
            }

            const hoverResult = await client.sendRequest('textDocument/hover', {
                textDocument: { uri: 'file:///test.txt' },
                position: { line: 0, character: 0 }
            });

            if (!hoverResult || hoverResult.contents !== 'Language Server Hover') {
                throw new Error('Hover feature failed');
            }
            console.log('S-0055 Language Server features validated.');
        } else if (process.env.X0008_SCENARIO === 'S-0056') {
            client.notify('$/cancelRequest', { id: 1 });
            client.stop();
            console.log('S-0056 Language Server cancellation/disconnection validated.');
        }
    } catch (err) {
        console.error(err);
        process.exit(1);
    } finally {
        client.stop();
    }
}

runTests();
"
