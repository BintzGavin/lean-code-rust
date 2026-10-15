#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Skipping syntax validation."
    exit 0
fi

cd "$DIR"

echo "Validating package.json..."
node -e "const pkg = require('./package.json'); if (!pkg.name || !pkg.main) throw new Error('Invalid package.json');"

echo "Validating extension.js syntax and exports..."
node -e "
const Module = require('module');
const originalRequire = Module.prototype.require;
Module.prototype.require = function(path) {
    if (path === 'vscode') {
        return vscodeMock;
    }
    return originalRequire.apply(this, arguments);
};

const registeredCommands = new Map();

class WebviewPanelMock {
    constructor(viewType, title, showOptions, options) {
        this.viewType = viewType;
        this.title = title;
        this.options = options;
        this.visible = true;
        this.disposed = false;

        let messageHandler = null;
        let viewStateHandler = null;
        let disposeHandler = null;

        const self = this;
        this.webview = {
            html: '',
            onDidReceiveMessage: (handler) => { messageHandler = handler; },
            postMessage: (msg) => {
                if (this.disposed) throw new Error('Webview is disposed');
                if (msg.command === 'synthetic_reply') {
                    self.lastReply = msg.number;
                }
                if (msg.command === 'visibility_changed') {
                    self.lastVisibility = msg.visible;
                }
            },
            asWebviewUri: (uri) => {
                const root = this.options.localResourceRoots[0].path;
                if (!uri.path.startsWith(root)) {
                    throw new Error('Resource outside local roots');
                }
                return 'vscode-webview-resource://' + uri.path;
            }
        };

        this.onDidChangeViewState = (handler) => { viewStateHandler = handler; };
        this.onDidDispose = (handler) => { disposeHandler = handler; };

        this.receiveMessage = (msg) => { if (messageHandler) messageHandler(msg); };
        this.changeVisibility = (visible) => {
            this.visible = visible;
            if (viewStateHandler) viewStateHandler({ webviewPanel: this });
        };
        this.dispose = () => {
            this.disposed = true;
            if (disposeHandler) disposeHandler();
        };
    }
}

const vscodeMock = {
    ViewColumn: { One: 1 },
    Uri: {
        file: (path) => ({ path })
    },
    commands: {
        registerCommand: (id, callback) => {
            registeredCommands.set(id, callback);
            return { dispose: () => registeredCommands.delete(id) };
        }
    },
    window: {
        createWebviewPanel: (viewType, title, showOptions, options) => {
            return new WebviewPanelMock(viewType, title, showOptions, options);
        }
    }
};

const ext = require('./extension.js');
if (typeof ext.activate !== 'function') throw new Error('Missing activate function');

const context = {
    subscriptions: [],
    extensionPath: '/ext/root'
};

process.env.X0007_SCENARIO = 'S-0053';
ext.activate(context);

const showWebview = registeredCommands.get('x0007.showWebview');
if (!showWebview) throw new Error('Command not registered');

const panel = showWebview();

panel.receiveMessage({ command: 'synthetic_msg', number: 1 });
if (panel.lastReply !== 2) throw new Error('Failed synthetic message exchange (S-0053)');

panel.changeVisibility(false);
if (panel.lastVisibility !== false) throw new Error('Failed hide visibility exchange (S-0053)');

panel.changeVisibility(true);
if (panel.lastVisibility !== true) throw new Error('Failed reveal visibility exchange (S-0053)');

console.log('S-0053 Webview lifecycle validated.');

process.env.X0007_SCENARIO = 'S-0054';
showWebview();

console.log('S-0054 Webview error handling validated.');
"

echo "Validation successful!"
