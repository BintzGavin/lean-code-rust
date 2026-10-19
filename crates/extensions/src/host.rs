use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdout, Command, ExitStatus, Stdio};

pub struct ExtensionHost {
    child: Child,
    stdout_reader: BufReader<ChildStdout>,
}

impl ExtensionHost {
    pub fn spawn(entry_point_path: &Path) -> io::Result<Self> {
        let mut child = Command::new("node")
            .arg(entry_point_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not capture stdout"))?;
        let stdout_reader = BufReader::new(stdout);

        Ok(Self {
            child,
            stdout_reader,
        })
    }

    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        if let Some(stdin) = self.child.stdin.as_mut() {
            writeln!(stdin, "{}", message)?;
            stdin.flush()?;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not access stdin",
            ))
        }
    }

    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        let bytes_read = self.stdout_reader.read_line(&mut line)?;
        if bytes_read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Stream closed",
            ));
        }
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        Ok(line)
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.child.wait()
    }

    pub fn invoke_command(
        &mut self,
        command: &str,
        args: Option<serde_json::Value>,
    ) -> io::Result<serde_json::Value> {
        let msg = serde_json::json!({
            "type": "invoke_command",
            "command": command,
            "args": args.unwrap_or(serde_json::Value::Array(vec![]))
        });
        self.send_message(&msg.to_string())?;

        let reply = self.read_message()?;
        if reply.starts_with('{') {
            let json: serde_json::Value = serde_json::from_str(&reply)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            if json["type"] == "command_success" {
                Ok(json["result"].clone())
            } else if json["type"] == "command_error" {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    json["error"].as_str().unwrap_or("Unknown error"),
                ))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unexpected response type",
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid JSON response",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_spawn_and_kill_extension_host() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "setTimeout(() => console.log('running'), 10000);").unwrap();

        let mut host = ExtensionHost::spawn(file.path()).expect("Failed to spawn node");
        host.kill().expect("Failed to kill node");
        let status = host.wait().expect("Failed to wait for node");
        assert!(!status.success()); // Should be killed
    }

    #[test]
    fn test_spawn_missing_script() {
        let result = ExtensionHost::spawn(Path::new("/does/not/exist.js"));
        if let Ok(mut host) = result {
            let status = host.wait().expect("wait failed");
            assert!(!status.success());
        }
    }

    #[test]
    fn test_ipc_communication() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
            const readline = require('readline');
            const rl = readline.createInterface({{
                input: process.stdin,
                output: process.stdout,
                terminal: false
            }});
            rl.on('line', (line) => {{
                console.log("echo: " + line);
            }});
        "#
        )
        .unwrap();

        let mut host = ExtensionHost::spawn(file.path()).expect("Failed to spawn node");

        host.send_message("hello ipc")
            .expect("Failed to send message");

        let reply = host.read_message().expect("Failed to read message");
        assert_eq!(reply, "echo: hello ipc");

        host.kill().expect("Failed to kill node");
    }

    #[test]
    fn test_bootstrap_activation() {
        let bootstrap_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("bootstrap.js");

        let mut ext_file = NamedTempFile::new().unwrap();
        writeln!(
            ext_file,
            r#"
            exports.activate = function() {{
                console.log("activate was called");
            }};
        "#
        )
        .unwrap();

        let mut host = ExtensionHost::spawn(&bootstrap_path).expect("Failed to spawn node");

        let msg = serde_json::json!({
            "type": "activate",
            "path": ext_file.path()
        });

        host.send_message(&msg.to_string())
            .expect("Failed to send message");

        let mut success = false;
        for _ in 0..10 {
            if let Ok(reply) = host.read_message() {
                if reply.starts_with('{') {
                    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
                    if json["type"] == "activate_success" {
                        success = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }

        assert!(success);

        host.kill().expect("Failed to kill node");
    }

    #[test]
    fn test_bootstrap_activation_and_deactivation() {
        let bootstrap_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("bootstrap.js");

        let mut ext_file = NamedTempFile::new().unwrap();
        writeln!(
            ext_file,
            r#"
            exports.activate = function(context) {{
                console.log("activate was called");
                context.subscriptions.push({{
                    dispose: function() {{
                        console.log("dispose was called");
                    }}
                }});
            }};

            exports.deactivate = function() {{
                console.log("deactivate was called");
            }};
        "#
        )
        .unwrap();

        let mut host = ExtensionHost::spawn(&bootstrap_path).expect("Failed to spawn node");

        let msg_activate = serde_json::json!({
            "type": "activate",
            "path": ext_file.path()
        });

        host.send_message(&msg_activate.to_string())
            .expect("Failed to send message");

        let mut activate_success = false;
        for _ in 0..10 {
            if let Ok(reply) = host.read_message() {
                if reply.starts_with('{') {
                    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
                    if json["type"] == "activate_success" {
                        activate_success = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }
        assert!(activate_success);

        let msg_deactivate = serde_json::json!({
            "type": "deactivate",
            "path": ext_file.path()
        });

        host.send_message(&msg_deactivate.to_string())
            .expect("Failed to send message");

        let mut deactivate_success = false;
        for _ in 0..10 {
            if let Ok(reply) = host.read_message() {
                if reply.starts_with('{') {
                    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
                    if json["type"] == "deactivate_success" {
                        deactivate_success = true;
                        break;
                    }
                } else if reply == "dispose was called" {
                    // Check if dispose was logged, although test is mainly based on deactivate_success
                }
            } else {
                break;
            }
        }

        assert!(deactivate_success);

        host.kill().expect("Failed to kill node");
    }

    #[test]
    fn test_bootstrap_vscode_module() {
        let bootstrap_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("bootstrap.js");

        let mut ext_file = NamedTempFile::new().unwrap();
        writeln!(
            ext_file,
            r#"
            const vscode = require('vscode');
            vscode.window;
            exports.activate = function(context) {{
                console.log("activate was called with synthetic vscode");
            }};
        "#
        )
        .unwrap();

        let mut host = ExtensionHost::spawn(&bootstrap_path).expect("Failed to spawn node");

        let msg_activate = serde_json::json!({
            "type": "activate",
            "path": ext_file.path()
        });

        host.send_message(&msg_activate.to_string())
            .expect("Failed to send message");

        let mut activate_success = false;
        for _ in 0..10 {
            if let Ok(reply) = host.read_message() {
                if reply.starts_with('{') {
                    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
                    if json["type"] == "activate_success" {
                        activate_success = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }
        assert!(activate_success);

        host.kill().expect("Failed to kill node");
    }

    #[test]
    fn test_register_and_invoke_command() {
        let bootstrap_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("bootstrap.js");

        let mut ext_file = NamedTempFile::new().unwrap();
        writeln!(
            ext_file,
            r#"
            const vscode = require('vscode');
            exports.activate = function(context) {{
                let disposable = vscode.commands.registerCommand('myExtension.sayHello', function(arg1, arg2) {{
                    return "Hello " + arg1 + " and " + arg2;
                }});
                context.subscriptions.push(disposable);
            }};
        "#
        )
        .unwrap();

        let mut host = ExtensionHost::spawn(&bootstrap_path).expect("Failed to spawn node");

        let msg_activate = serde_json::json!({
            "type": "activate",
            "path": ext_file.path()
        });

        host.send_message(&msg_activate.to_string())
            .expect("Failed to send message");

        let mut activate_success = false;
        for _ in 0..10 {
            if let Ok(reply) = host.read_message() {
                if reply.starts_with('{') {
                    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
                    if json["type"] == "activate_success" {
                        activate_success = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }
        assert!(activate_success);

        let args = serde_json::json!(["Alice", "Bob"]);
        let result = host
            .invoke_command("myExtension.sayHello", Some(args))
            .expect("Failed to invoke command");

        assert_eq!(result.as_str().unwrap(), "Hello Alice and Bob");

        host.kill().expect("Failed to kill node");
    }

    #[test]
    fn test_synthetic_vscode_types() {
        let bootstrap_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("bootstrap.js");

        let mut ext_file = NamedTempFile::new().unwrap();
        writeln!(
            ext_file,
            r#"
            const vscode = require('vscode');
            exports.activate = function(context) {{
                let disposable = vscode.commands.registerCommand('myExtension.testTypes', function() {{
                    const uri = vscode.Uri.file('/test/path');
                    const emitter = new vscode.EventEmitter();
                    let receivedUri = null;
                    const listenerDisposable = emitter.event(e => {{
                        receivedUri = e;
                    }});
                    emitter.fire(uri);
                    listenerDisposable.dispose();
                    emitter.dispose();

                    if (receivedUri && receivedUri.fsPath === '/test/path') {{
                        return "types_ok";
                    }}
                    return "types_fail";
                }});
                context.subscriptions.push(disposable);
            }};
        "#
        )
        .unwrap();

        let mut host = ExtensionHost::spawn(&bootstrap_path).expect("Failed to spawn node");

        let msg_activate = serde_json::json!({
            "type": "activate",
            "path": ext_file.path()
        });

        host.send_message(&msg_activate.to_string())
            .expect("Failed to send message");

        let mut activate_success = false;
        for _ in 0..10 {
            if let Ok(reply) = host.read_message() {
                if reply.starts_with('{') {
                    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
                    if json["type"] == "activate_success" {
                        activate_success = true;
                        break;
                    }
                }
            } else {
                break;
            }
        }
        assert!(activate_success);

        let result = host
            .invoke_command("myExtension.testTypes", None)
            .expect("Failed to invoke command");

        assert_eq!(result.as_str().unwrap(), "types_ok");

        host.kill().expect("Failed to kill node");
    }
}
