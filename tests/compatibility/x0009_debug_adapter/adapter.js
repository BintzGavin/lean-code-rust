process.on('message', (msg) => {
    if (msg.type === 'request') {
        const response = {
            type: 'response',
            seq: msg.seq,
            request_seq: msg.seq,
            command: msg.command,
            success: true
        };

        switch (msg.command) {
            case 'initialize':
                response.body = {
                    supportsConfigurationDoneRequest: true
                };
                process.send(response);
                process.send({ type: 'event', event: 'initialized' });
                break;
            case 'launch':
                process.send(response);
                process.send({
                    type: 'event',
                    event: 'output',
                    body: { category: 'stdout', output: 'Starting mock debug session...\n' }
                });
                break;
            case 'setBreakpoints':
                response.body = {
                    breakpoints: msg.arguments.breakpoints.map(bp => ({ verified: true, line: bp.line }))
                };
                process.send(response);
                break;
            case 'configurationDone':
                process.send(response);
                process.send({
                    type: 'event',
                    event: 'stopped',
                    body: { reason: 'breakpoint', threadId: 1 }
                });
                break;
            case 'next':
                process.send(response);
                process.send({
                    type: 'event',
                    event: 'stopped',
                    body: { reason: 'step', threadId: 1 }
                });
                break;
            case 'variables':
                response.body = {
                    variables: [
                        { name: "x", value: "42", type: "number", variablesReference: 0 }
                    ]
                };
                process.send(response);
                break;
            case 'disconnect':
                process.send(response);
                // Allow IPC to flush before exiting
                setTimeout(() => process.exit(0), 10);
                break;
            default:
                response.success = false;
                response.message = `Unknown command: ${msg.command}`;
                process.send(response);
                break;
        }
    }
});
