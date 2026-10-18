const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

const activatedExtensions = new Map();

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
        } else {
            console.log(JSON.stringify({ type: 'error', error: 'Unknown command type: ' + msg.type }));
        }
    } catch (e) {
        console.log(JSON.stringify({ type: 'error', error: e.toString() }));
    }
});
