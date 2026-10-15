const child_process = require('child_process');

function activate(context) {
    if (process.env.X0006_FAIL_ACTIVATION === '1') {
        throw new Error('Intentional activation failure for S-0052');
    }

    // S-0051: Store distinct workspace/global values and invoke bounded child
    context.workspaceState.update('x0006WorkspaceKey', 'x0006WorkspaceValue');
    context.globalState.update('x0006GlobalKey', 'x0006GlobalValue');
    context.secrets.store('x0006SecretKey', 'x0006SecretValue');

    const workspaceValue = context.workspaceState.get('x0006WorkspaceKey');
    const globalValue = context.globalState.get('x0006GlobalKey');

    if (workspaceValue !== 'x0006WorkspaceValue' || globalValue !== 'x0006GlobalValue') {
       throw new Error('State storage failed');
    }

    const child = child_process.spawn('echo', ['"bounded child process"']);
    child.on('error', (err) => {
        console.error('Failed to start child process.', err);
    });
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
