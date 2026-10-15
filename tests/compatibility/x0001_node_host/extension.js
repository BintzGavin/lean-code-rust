const fs = require('fs');
const path = require('path');
const vscode = require('vscode');

function activate(context) {
    if (process.env.X0001_FAIL_ACTIVATION === '1') {
        throw new Error('Intentional activation failure for S-0042');
    }

    const disposable1 = vscode.commands.registerCommand('x0001.activate', () => {
        try {
            const syntheticFilePath = path.join(__dirname, 'synthetic.txt');
            const data = fs.readFileSync(syntheticFilePath, 'utf8');
            vscode.window.showInformationMessage(`Read synthetic file: ${data}`);
        } catch (e) {
            vscode.window.showErrorMessage(`Failed to read synthetic file: ${e.message}`);
        }
    });

    const disposable2 = vscode.commands.registerCommand('x0001.subsequentCommand', () => {
        vscode.window.showInformationMessage('Executed subsequent command successfully');
    });

    context.subscriptions.push(disposable1, disposable2);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
