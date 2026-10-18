const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

rl.on('line', (line) => {
    try {
        const msg = JSON.parse(line);
        if (msg.type === 'activate') {
            try {
                const ext = require(msg.path);
                if (typeof ext.activate === 'function') {
                    Promise.resolve(ext.activate()).then(() => {
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
        } else {
            console.log(JSON.stringify({ type: 'error', error: 'Unknown command type: ' + msg.type }));
        }
    } catch (e) {
        console.log(JSON.stringify({ type: 'error', error: e.toString() }));
    }
});
