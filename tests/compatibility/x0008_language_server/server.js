process.on('message', (msg) => {
    if (msg.method === 'initialize') {
        process.send({
            jsonrpc: '2.0',
            id: msg.id,
            result: { capabilities: { hoverProvider: true } }
        });
    } else if (msg.method === 'textDocument/hover') {
        process.send({
            jsonrpc: '2.0',
            id: msg.id,
            result: { contents: 'Language Server Hover' }
        });
    } else if (msg.method === '$/cancelRequest') {
        // Ignore for mock
    } else if (msg.method === 'exit') {
        process.exit(0);
    }
});
