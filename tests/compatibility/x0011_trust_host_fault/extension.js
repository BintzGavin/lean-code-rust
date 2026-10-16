const vscode = require('vscode');

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    console.log('X-0011 fixture activated');

    let disposableAction = vscode.commands.registerCommand('x0011.controlledAction', () => {
        vscode.window.showInformationMessage('X-0011: Controlled Action executed');
        console.log('x0011.controlledAction executed');
    });

    let disposableFault = vscode.commands.registerCommand('x0011.triggerFault', () => {
        console.log('X-0011: Triggering host fault');
        // A hard crash of the extension host to test fault recovery/isolation
        process.exit(1);
    });

    context.subscriptions.push(disposableAction);
    context.subscriptions.push(disposableFault);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
}
