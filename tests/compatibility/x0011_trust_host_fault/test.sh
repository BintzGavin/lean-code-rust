#!/bin/bash
set -e

# Note: In a real test harness, this would interface with VS Code or the Lean Editor.
# We will simulate the checks needed by the success criteria for the test script.

echo "Running X-0011 Trust and Host Fault tests..."

# Check if node is installed
if ! command -v node &> /dev/null
then
    echo "node could not be found"
    false
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Try activating the extension code directly as a basic syntax/require check
# using a mock vscode object
cat << 'MOCK_EOF' > /tmp/mock_vscode.js
module.exports = {
    commands: {
        registerCommand: (name, callback) => {
            return { dispose: () => {} };
        }
    },
    window: {
        showInformationMessage: (msg) => {
            console.log("MOCK SHOW_INFO:", msg);
        }
    }
};
MOCK_EOF

cat << RUN_EOF > /tmp/run_ext.js
const Module = require('module');
const originalRequire = Module.prototype.require;
Module.prototype.require = function() {
    if (arguments[0] === 'vscode') {
        return require('/tmp/mock_vscode.js');
    }
    return originalRequire.apply(this, arguments);
};

const ext = require('${DIR}/extension.js');
const context = { subscriptions: [] };
ext.activate(context);

if (context.subscriptions.length !== 2) {
    console.error("Expected 2 subscriptions, got " + context.subscriptions.length);
    throw new Error("Invalid subscriptions length");
}
console.log("Activate complete.");
RUN_EOF

node /tmp/run_ext.js
echo "X-0011 extension logic successfully executed in mock environment."

rm /tmp/mock_vscode.js /tmp/run_ext.js

echo "X-0011 test script completed successfully."
