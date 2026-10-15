const vscode = require('vscode');

function activate(context) {
    const disposable1 = vscode.commands.registerCommand('x0002.editSave', async () => {
        try {
            const doc = await vscode.workspace.openTextDocument({ content: 'Initial content', language: 'plaintext' });
            const editor = await vscode.window.showTextDocument(doc);
            const edit = new vscode.WorkspaceEdit();
            edit.insert(doc.uri, new vscode.Position(0, 0), 'Prepended ');
            const success = await vscode.workspace.applyEdit(edit);
            if (success) {
                await doc.save();
                vscode.window.showInformationMessage('Edit and save successful');
            }
        } catch (e) {
            vscode.window.showErrorMessage(`Edit failed: ${e.message}`);
        }
    });

    const disposable2 = vscode.commands.registerCommand('x0002.cancelEdit', async () => {
        try {
            await vscode.workspace.openTextDocument('/does/not/exist.txt');
        } catch (e) {
            vscode.window.showInformationMessage('Caught expected failure opening missing file');
        }
    });

    context.subscriptions.push(disposable1, disposable2);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
