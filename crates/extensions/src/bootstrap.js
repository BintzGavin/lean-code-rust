const Module = require('module');

class Disposable {
    constructor(callOnDispose) {
        this._callOnDispose = callOnDispose;
    }
    dispose() {
        if (this._callOnDispose) {
            this._callOnDispose();
            this._callOnDispose = undefined;
        }
    }
    static from(...disposables) {
        return new Disposable(() => {
            for (const disposable of disposables) {
                if (disposable && typeof disposable.dispose === 'function') {
                    disposable.dispose();
                }
            }
        });
    }
}

class EventEmitter {
    constructor() {
        this._listeners = [];
    }
    get event() {
        if (!this._event) {
            this._event = (listener, thisArgs, disposables) => {
                const record = { listener, thisArgs };
                this._listeners.push(record);
                const disposable = new Disposable(() => {
                    const index = this._listeners.indexOf(record);
                    if (index > -1) {
                        this._listeners.splice(index, 1);
                    }
                });
                if (Array.isArray(disposables)) {
                    disposables.push(disposable);
                }
                return disposable;
            };
        }
        return this._event;
    }
    fire(data) {
        const listeners = this._listeners.slice();
        for (const record of listeners) {
            record.listener.call(record.thisArgs, data);
        }
    }
    dispose() {
        this._listeners = [];
    }
}

class Uri {
    constructor(scheme, authority, path, query, fragment) {
        this.scheme = scheme || '';
        this.authority = authority || '';
        this.path = path || '';
        this.query = query || '';
        this.fragment = fragment || '';
    }
    get fsPath() {
        return this.path;
    }
    static file(path) {
        return new Uri('file', '', path, '', '');
    }
    static parse(value) {
        let scheme = '';
        let path = value || '';
        const schemeMatch = /^([a-zA-Z][a-zA-Z0-9+.-]*):/.exec(value);
        if (schemeMatch) {
            scheme = schemeMatch[1];
            path = value.substring(schemeMatch[0].length);
        }
        if (path.startsWith('//')) {
            path = path.substring(2);
        }
        return new Uri(scheme, '', path, '', '');
    }
}

const syntheticVscode = {
    window: {
        activeTextEditor: undefined,
        visibleTextEditors: [],
    },
    workspace: {
        workspaceFolders: undefined,
        name: undefined,
        textDocuments: [],
        workspaceFile: undefined,
        getConfiguration: () => ({ get: () => undefined }),
    },
    Disposable,
    EventEmitter,
    Uri,
    commands: {
        registerCommand(command, callback, thisArg) {
            commandRegistry.set(command, { callback, thisArg });
            return {
                dispose() {
                    commandRegistry.delete(command);
                }
            };
        },
        executeCommand(command, ...rest) {
            const record = commandRegistry.get(command);
            if (record) {
                return Promise.resolve(record.callback.apply(record.thisArg, rest));
            }
            return Promise.reject(new Error("Command not found: " + command));
        }
    },
    languages: {},
    env: {}
};

const originalLoad = Module._load;
Module._load = function(request, parent, isMain) {
    if (request === 'vscode') {
        return syntheticVscode;
    }
    return originalLoad.apply(this, arguments);
};

const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

const activatedExtensions = new Map();
const commandRegistry = new Map();

rl.on('line', (line) => {
    try {
        const msg = JSON.parse(line);
        if (msg.type === 'activate') {
            try {
                const ext = require(msg.path);
                const context = { subscriptions: [] };
                activatedExtensions.set(msg.path, { context, ext });
                if (typeof ext.activate === 'function') {
                    Promise.resolve(ext.activate(context)).then(() => {
                        console.log(JSON.stringify({ type: 'activate_success', path: msg.path }));
                    }).catch(err => {
                        console.log(JSON.stringify({ type: 'activate_error', path: msg.path, error: err.toString() }));
                    });
                } else {
                    console.log(JSON.stringify({ type: 'activate_success', path: msg.path }));
                }
            } catch (err) {
                console.log(JSON.stringify({ type: 'activate_error', path: msg.path, error: err.toString() }));
            }
        } else if (msg.type === 'deactivate') {
            try {
                const record = activatedExtensions.get(msg.path);
                if (record) {
                    const { context, ext } = record;
                    const doDeactivate = () => {
                        try {
                            if (typeof ext.deactivate === 'function') {
                                return Promise.resolve(ext.deactivate());
                            }
                            return Promise.resolve();
                        } catch (e) {
                            return Promise.reject(e);
                        }
                    };

                    doDeactivate().then(() => {
                        // Success path
                        for (const sub of context.subscriptions) {
                            if (sub && typeof sub.dispose === 'function') {
                                try {
                                    sub.dispose();
                                } catch (e) {
                                    console.error(e);
                                }
                            }
                        }
                        activatedExtensions.delete(msg.path);
                        console.log(JSON.stringify({ type: 'deactivate_success', path: msg.path }));
                    }).catch(err => {
                        // Error path, still clean up but report error
                        for (const sub of context.subscriptions) {
                            if (sub && typeof sub.dispose === 'function') {
                                try {
                                    sub.dispose();
                                } catch (e) {
                                    console.error(e);
                                }
                            }
                        }
                        activatedExtensions.delete(msg.path);
                        console.log(JSON.stringify({ type: 'deactivate_error', path: msg.path, error: err.toString() }));
                    });
                } else {
                    console.log(JSON.stringify({ type: 'deactivate_success', path: msg.path }));
                }
            } catch (err) {
                console.log(JSON.stringify({ type: 'deactivate_error', path: msg.path, error: err.toString() }));
            }
        } else if (msg.type === 'invoke_command') {
            const record = commandRegistry.get(msg.command);
            if (record) {
                const args = msg.args || [];
                Promise.resolve(record.callback.apply(record.thisArg, args)).then(result => {
                    console.log(JSON.stringify({ type: 'command_success', result }));
                }).catch(err => {
                    console.log(JSON.stringify({ type: 'command_error', error: err.toString() }));
                });
            } else {
                console.log(JSON.stringify({ type: 'command_error', error: 'Command not found' }));
            }
        } else {
            console.log(JSON.stringify({ type: 'error', error: 'Unknown command type: ' + msg.type }));
        }
    } catch (e) {
        console.log(JSON.stringify({ type: 'error', error: e.toString() }));
    }
});
