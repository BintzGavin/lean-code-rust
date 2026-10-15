const vscode = require('vscode');

class SyntheticTreeProvider {
    constructor() {
        this._onDidChangeTreeData = new vscode.EventEmitter();
        this.onDidChangeTreeData = this._onDidChangeTreeData.event;
        this.items = [
            { id: '1', label: 'Item 1' },
            { id: '2', label: 'Item 2' },
            { id: '3', label: 'Item 3' }
        ];
    }

    refresh() {
        this._onDidChangeTreeData.fire();
    }

    removeItem(id) {
        this.items = this.items.filter(item => item.id !== id);
        this.refresh();
    }

    getTreeItem(element) {
        const item = new vscode.TreeItem(element.label, vscode.TreeItemCollapsibleState.None);
        item.id = element.id;
        return item;
    }

    getChildren(element) {
        if (element) {
            return Promise.resolve([]);
        }
        return Promise.resolve(this.items);
    }

    getParent(element) {
        return null;
    }
}

function activate(context) {
    const provider = new SyntheticTreeProvider();

    const view = vscode.window.createTreeView('syntheticTreeView', {
        treeDataProvider: provider,
        showCollapseAll: true,
        canSelectMany: false
    });

    context.subscriptions.push(view);
    context.subscriptions.push(vscode.window.registerTreeDataProvider('syntheticTreeView', provider));

    context.subscriptions.push(vscode.commands.registerCommand('syntheticTreeView.refresh', () => {
        provider.refresh();
    }));

    context.subscriptions.push(vscode.commands.registerCommand('syntheticTreeView.remove', (item) => {
        if (item && item.id) {
            provider.removeItem(item.id);
        }
    }));

    view.onDidChangeSelection(e => {
        if (e.selection && e.selection.length > 0) {
            console.log('Selected:', e.selection[0].id);
        }
    });

    // Simulate reveal
    setTimeout(() => {
        if (provider.items.length > 0) {
            view.reveal(provider.items[1], { select: true, focus: true });
        }
    }, 1000);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate,
    SyntheticTreeProvider
};
