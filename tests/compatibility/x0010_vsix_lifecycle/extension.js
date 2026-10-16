const vscode = require('vscode');

function activate(context) {
    console.log('X-0010 VSIX lifecycle activated');
    let disposable = vscode.commands.registerCommand('x0010.invoke', function () {
        console.log('X-0010 invoked');
    });
    context.subscriptions.push(disposable);
}

function deactivate() {
    console.log('X-0010 VSIX lifecycle deactivated');
}

module.exports = {
    activate,
    deactivate
};