const cp = require('child_process');
const path = require('path');

function activate(context) {
    const adapterPath = path.join(__dirname, 'adapter.js');

    function startSession() {
        const adapterProcess = cp.fork(adapterPath, [], {
            execArgv: []
        });

        let seq = 1;
        const client = {
            process: adapterProcess,
            sendRequest: function(command, args) {
                return new Promise((resolve, reject) => {
                    const currentSeq = seq++;
                    const listener = (msg) => {
                        if (msg.type === 'response' && msg.request_seq === currentSeq) {
                            adapterProcess.removeListener('message', listener);
                            if (!msg.success) reject(msg.message);
                            else resolve(msg.body);
                        }
                    };
                    adapterProcess.on('message', listener);
                    adapterProcess.send({ type: 'request', seq: currentSeq, command, arguments: args });
                });
            },
            onEvent: function(event, callback) {
                adapterProcess.on('message', (msg) => {
                    if (msg.type === 'event' && msg.event === event) {
                        callback(msg.body);
                    }
                });
            },
            stop: function() {
                adapterProcess.kill();
            }
        };
        return client;
    }

    const factory = {
        createDebugAdapterDescriptor: () => {
            return {
                command: process.execPath,
                args: [adapterPath]
            };
        }
    };

    context.subscriptions.push({ dispose: () => {} });

    return {
        startSession,
        factory
    };
}

module.exports = { activate };
