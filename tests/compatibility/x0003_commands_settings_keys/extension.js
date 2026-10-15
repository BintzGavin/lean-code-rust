const vscode = require('vscode');

function activate(context) {
    const disposable1 = vscode.commands.registerCommand('x0003.invokeCommand', (args) => {
        vscode.window.showInformationMessage(`Invoked with: ${JSON.stringify(args)}`);
    });

    const disposable2 = vscode.commands.registerCommand('x0003.testConfiguration', async () => {
        try {
            vscode.commands.executeCommand('setContext', 'x0003Context', true);
            const config = vscode.workspace.getConfiguration('x0003');
            await config.update('testSetting', 'updated_value', vscode.ConfigurationTarget.Global);
            vscode.window.showInformationMessage('Configuration updated and context set');
        } catch (e) {
            vscode.window.showErrorMessage(`Configuration test failed: ${e.message}`);
        }
    });

    const disposable3 = vscode.workspace.onDidChangeConfiguration((e) => {
        if (e.affectsConfiguration('x0003.testSetting')) {
            const config = vscode.workspace.getConfiguration('x0003');
            const newValue = config.get('testSetting');
            vscode.window.showInformationMessage(`Configuration changed: ${newValue}`);
        }
    });

    context.subscriptions.push(disposable1, disposable2, disposable3);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
