const cp = require('child_process');
const path = require('path');

function activate(context) {
    const serverPath = path.join(__dirname, 'server.js');
    const serverProcess = cp.fork(serverPath, [], {
        execArgv: []
    });

    const client = {
        process: serverProcess,
        sendRequest: function(method, params) {
            return new Promise((resolve, reject) => {
                const id = Math.random().toString(36).substr(2, 9);
                const listener = (msg) => {
                    if (msg.id === id) {
                        serverProcess.removeListener('message', listener);
                        if (msg.error) reject(msg.error);
                        else resolve(msg.result);
                    }
                };
                serverProcess.on('message', listener);
                serverProcess.send({ jsonrpc: '2.0', id, method, params });
            });
        },
        notify: function(method, params) {
            serverProcess.send({ jsonrpc: '2.0', method, params });
        },
        stop: function() {
            serverProcess.kill();
        }
    };

    context.subscriptions.push({ dispose: () => client.stop() });
    return client;
}

module.exports = { activate };
