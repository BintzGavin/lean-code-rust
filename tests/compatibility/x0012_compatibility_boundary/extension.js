const vscode = require('vscode');

function activate(context) {
    let dispExcluded = vscode.commands.registerCommand('x0012.triggerExcluded', () => {
        try {
            // S-0063: Access an excluded proposed API
            if (vscode.proposed && vscode.proposed.excludedMethod) {
                 vscode.proposed.excludedMethod();
            } else {
                 throw new Error("Excluded API 'vscode.proposed.excludedMethod' not available or denied.");
            }
        } catch (err) {
            console.log("Expected rejection of excluded API: " + err.message);
        }
    });

    let dispStable = vscode.commands.registerCommand('x0012.triggerStable', () => {
        // S-0064: Exercise non-excluded stable control path
        console.log("Stable control path 'vscode.window.showInformationMessage' executed without excluded services");
        vscode.window.showInformationMessage('X-0012 Stable Action');
    });

    context.subscriptions.push(dispExcluded);
    context.subscriptions.push(dispStable);
}

function deactivate() {}

module.exports = { activate, deactivate };
