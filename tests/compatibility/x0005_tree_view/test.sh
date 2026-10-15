#!/usr/bin/env bash
set -e
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Skipping syntax validation."
    exit 0
fi

cd "$DIR"

echo "Validating package.json..."
node -e "
const pkg = require('./package.json');
if (!pkg.name || !pkg.main) throw new Error('Invalid package.json');
if (!pkg.contributes) throw new Error('Missing contributes section');
if (!pkg.contributes.views) throw new Error('Missing views contribution');
if (!pkg.contributes.views.explorer) throw new Error('Missing views.explorer contribution');

const view = pkg.contributes.views.explorer.find(v => v.id === 'syntheticTreeView');
if (!view) throw new Error('Missing syntheticTreeView contribution');
"

echo "Validating extension.js syntax and behavior with mock vscode API..."
node -e "
const assert = require('assert');

// Mock vscode API
const vscode = {
    TreeItem: class {
        constructor(label, collapsibleState) {
            this.label = label;
            this.collapsibleState = collapsibleState;
        }
    },
    TreeItemCollapsibleState: {
        None: 0,
        Collapsed: 1,
        Expanded: 2
    },
    EventEmitter: class {
        constructor() {
            this.event = () => {};
            this.fired = false;
        }
        fire() {
            this.fired = true;
        }
    },
    window: {
        createTreeView: (id, options) => {
            assert.strictEqual(id, 'syntheticTreeView');
            assert.ok(options.treeDataProvider);
            return {
                onDidChangeSelection: (cb) => {},
                reveal: (element, options) => {
                    assert.ok(element);
                    assert.ok(options);
                },
                dispose: () => {}
            };
        },
        registerTreeDataProvider: (id, provider) => {
            assert.strictEqual(id, 'syntheticTreeView');
            assert.ok(provider);
            return { dispose: () => {} };
        }
    },
    commands: {
        registerCommand: (id, cb) => {
            return { dispose: () => {} };
        }
    }
};

// Override require to return our mock for 'vscode'
const Module = require('module');
const originalRequire = Module.prototype.require;
Module.prototype.require = function(id) {
    if (id === 'vscode') return vscode;
    return originalRequire.apply(this, arguments);
};

// Load the extension
const ext = require('./extension.js');
if (typeof ext.activate !== 'function') throw new Error('Missing activate function');
if (typeof ext.deactivate !== 'function') throw new Error('Missing deactivate function');
if (typeof ext.SyntheticTreeProvider !== 'function') throw new Error('Missing SyntheticTreeProvider export');

// Test Provider Behavior
async function testProvider() {
    const provider = new ext.SyntheticTreeProvider();

    // Test getChildren (root)
    const children = await provider.getChildren();
    assert.strictEqual(children.length, 3);
    assert.strictEqual(children[0].id, '1');

    // Test getChildren (element)
    const noChildren = await provider.getChildren(children[0]);
    assert.strictEqual(noChildren.length, 0);

    // Test getTreeItem
    const item = provider.getTreeItem(children[0]);
    assert.strictEqual(item.label, 'Item 1');
    assert.strictEqual(item.id, '1');
    assert.strictEqual(item.collapsibleState, vscode.TreeItemCollapsibleState.None);

    // Test refresh and event firing
    assert.strictEqual(provider._onDidChangeTreeData.fired, false);
    provider.refresh();
    assert.strictEqual(provider._onDidChangeTreeData.fired, true);

    // Test remove item
    provider._onDidChangeTreeData.fired = false;
    provider.removeItem('2');
    const newChildren = await provider.getChildren();
    assert.strictEqual(newChildren.length, 2);
    assert.ok(!newChildren.find(c => c.id === '2'));
    assert.strictEqual(provider._onDidChangeTreeData.fired, true);

    // Test getParent
    assert.strictEqual(provider.getParent(newChildren[0]), null);
}

// Test Activation
function testActivation() {
    const context = { subscriptions: [] };
    ext.activate(context);
    assert.ok(context.subscriptions.length > 0);
}

// Run tests
Promise.resolve()
    .then(testProvider)
    .then(testActivation)
    .then(() => console.log('Validation successful!'))
    .catch(err => {
        console.error(err);
        process.exit(1);
    });
"
