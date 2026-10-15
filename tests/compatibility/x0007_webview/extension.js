const vscode = require('vscode');

function activate(context) {
    let disposable = vscode.commands.registerCommand('x0007.showWebview', () => {
        const panel = vscode.window.createWebviewPanel(
            'x0007Webview',
            'X0007 Webview',
            vscode.ViewColumn.One,
            {
                enableScripts: true,
                localResourceRoots: [vscode.Uri.file(context.extensionPath)]
            }
        );

        panel.webview.html = `<html><body>X0007</body></html>`;

        panel.webview.onDidReceiveMessage(
            message => {
                if (message.command === 'synthetic_msg') {
                    panel.webview.postMessage({ command: 'synthetic_reply', number: message.number + 1 });
                }
            },
            undefined,
            context.subscriptions
        );

        panel.onDidChangeViewState(
            e => {
                panel.webview.postMessage({ command: 'visibility_changed', visible: panel.visible });
            },
            null,
            context.subscriptions
        );

        panel.onDidDispose(() => {
            // cleanup
        }, null, context.subscriptions);

        if (process.env.X0007_SCENARIO === 'S-0054') {
            try {
                const badUri = vscode.Uri.file('/outside/resource.txt');
                panel.webview.asWebviewUri(badUri);
                throw new Error("Allowed outside resource read");
            } catch (e) {
                if (e.message !== 'Resource outside local roots') {
                    throw e;
                }
            }

            panel.dispose();
            try {
                panel.webview.postMessage({ command: 'pending' });
                throw new Error("Allowed postMessage after dispose");
            } catch(e) {
                if (e.message !== 'Webview is disposed') {
                    throw e;
                }
            }
        }

        return panel;
    });

    context.subscriptions.push(disposable);
}

function deactivate() {}

module.exports = { activate, deactivate };
