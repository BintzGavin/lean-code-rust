#!/bin/bash
set -e

echo "Running X-0012 Compatibility boundary tests..."

if ! command -v node &> /dev/null
then
    echo "node could not be found"
    false
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cat << 'MOCK_EOF' > /tmp/mock_vscode_x0012.js
module.exports = {
    commands: {
        registerCommand: (name, callback) => {
            return {
                dispose: () => {},
                execute: callback
            };
        }
    },
    window: {
        showInformationMessage: (msg) => {
            console.log("MOCK SHOW_INFO:", msg);
        }
    },
    proposed: {
        get excludedMethod() {
            throw new Error("Excluded API 'vscode.proposed.excludedMethod' not available or denied.");
        }
    }
};
MOCK_EOF

cat << RUN_EOF > /tmp/run_ext_x0012.js
const Module = require('module');
const originalRequire = Module.prototype.require;
Module.prototype.require = function() {
    if (arguments[0] === 'vscode') {
        return require('/tmp/mock_vscode_x0012.js');
    }
    return originalRequire.apply(this, arguments);
};

const ext = require('${DIR}/extension.js');
const context = { subscriptions: [] };
ext.activate(context);

if (context.subscriptions.length !== 2) {
    console.error("Expected 2 subscriptions, got " + context.subscriptions.length);
    process.exit(1);
}

console.log("Activate complete. Triggering commands...");

// Trigger the excluded API path
try {
    context.subscriptions[0].execute();
} catch (e) {
    console.error("Test failed: Uncaught error during excluded API execution: " + e.message);
    process.exit(1);
}

// Trigger the stable control path
try {
    context.subscriptions[1].execute();
} catch (e) {
    console.error("Test failed: Error during stable API execution: " + e.message);
    process.exit(1);
}

RUN_EOF

node /tmp/run_ext_x0012.js
echo "X-0012 extension logic successfully executed in mock environment."

rm /tmp/mock_vscode_x0012.js /tmp/run_ext_x0012.js

echo "X-0012 test script completed successfully."
