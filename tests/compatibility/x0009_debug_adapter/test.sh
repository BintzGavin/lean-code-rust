#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Skipping tests."
    exit 0
fi

cd "$DIR"

echo "Validating package.json..."
node -e "const pkg = require('./package.json'); if (!pkg.contributes || !pkg.contributes.debuggers) throw new Error('Invalid package.json');"

echo "Validating extension.js and DAP scenarios..."
node -e "
const Module = require('module');
const originalRequire = Module.prototype.require;
Module.prototype.require = function(path) {
    if (path === 'vscode') {
        return {
            debug: {
                registerDebugAdapterDescriptorFactory: () => ({ dispose: () => {} })
            }
        };
    }
    return originalRequire.apply(this, arguments);
};

const ext = require('./extension.js');
const context = { subscriptions: [] };
const api = ext.activate(context);

async function runTests() {
    console.log('Testing S-0057: Breakpoint/step/variable/output sequence...');
    let client = api.startSession();

    let outputReceived = false;
    client.onEvent('output', (body) => {
        if (body.output.includes('Starting mock debug session')) {
            outputReceived = true;
        }
    });

    let stoppedReason = null;
    client.onEvent('stopped', (body) => {
        stoppedReason = body.reason;
    });

    await client.sendRequest('initialize', { adapterID: 'mock-debug' });
    await client.sendRequest('launch', { program: 'test.js' });

    const bpResult = await client.sendRequest('setBreakpoints', {
        source: { name: 'test.js' },
        breakpoints: [{ line: 10 }]
    });
    if (!bpResult.breakpoints[0].verified) throw new Error('Breakpoint not verified');

    await client.sendRequest('configurationDone', {});

    // Wait for stop event
    await new Promise(r => setTimeout(r, 100));
    if (stoppedReason !== 'breakpoint') throw new Error('Did not stop on breakpoint');
    if (!outputReceived) throw new Error('Did not receive expected output');

    const varResult = await client.sendRequest('variables', { variablesReference: 1 });
    if (varResult.variables[0].name !== 'x' || varResult.variables[0].value !== '42') {
        throw new Error('Variables request failed');
    }

    await client.sendRequest('next', { threadId: 1 });
    await new Promise(r => setTimeout(r, 100));
    if (stoppedReason !== 'step') throw new Error('Did not stop after step');

    console.log('S-0057 verified.');

    console.log('Testing S-0058: Disconnect while paused and start a fresh session...');
    await client.sendRequest('disconnect', { restart: false });
    client.stop();

    client = api.startSession();
    await client.sendRequest('initialize', { adapterID: 'mock-debug' });
    await client.sendRequest('launch', { program: 'test.js' });
    await client.sendRequest('disconnect', {});
    client.stop();

    console.log('S-0058 verified.');
}

runTests().catch(err => {
    console.error(err);
    process.exit(1);
});
"
