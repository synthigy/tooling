use crate::bridge::Bridge;
use serde_json::{json, Value};
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

const CALL_TIMEOUT: Duration = Duration::from_secs(30);
const RUN_TIMEOUT: Duration = Duration::from_secs(300);
const NOT_RUNNING: &str = "Synthigy isn't running: open the Synthigy app and pick an engine";

#[cfg(unix)]
pub fn socket_path() -> std::path::PathBuf {
    dirs::runtime_dir()
        .or_else(dirs::config_dir)
        .unwrap_or_else(std::env::temp_dir)
        .join("synthigy")
        .join("desktop.sock")
}

#[cfg(windows)]
pub fn pipe_name() -> String {
    format!(r"\\.\pipe\synthigy-desktop-{}", std::env::var("USERNAME").unwrap_or_default())
}

async fn handle<S: AsyncRead + AsyncWrite>(stream: S, bridge: Arc<Bridge>) -> io::Result<()> {
    let (reader, mut writer) = tokio::io::split(stream);
    let mut line = String::new();
    BufReader::new(reader).read_line(&mut line).await?;
    let reply = match serde_json::from_str::<Value>(&line) {
        Ok(request) => {
            let name = request["name"].as_str().unwrap_or_default().to_string();
            let mut args = request["args"].as_object().cloned().unwrap_or_default();
            let window = args.remove("window").and_then(|w| w.as_str().map(String::from));
            let timeout = if name == "xsql_run" { RUN_TIMEOUT } else { CALL_TIMEOUT };
            match bridge.call(&name, Value::Object(args), window.as_deref(), timeout).await {
                Ok(value) => json!({"ok": true, "value": value}),
                Err(error) => json!({"ok": false, "error": error}),
            }
        }
        Err(e) => json!({"ok": false, "error": format!("bad request: {e}")}),
    };
    writer.write_all(format!("{reply}\n").as_bytes()).await?;
    writer.shutdown().await
}

fn spawn_handler<S: AsyncRead + AsyncWrite + Send + 'static>(stream: S, bridge: &Arc<Bridge>) {
    let bridge = bridge.clone();
    tokio::spawn(async move {
        if let Err(e) = handle(stream, bridge).await {
            eprintln!("synthigy desktop: tool connection failed: {e}");
        }
    });
}

/// Serve tool calls from `synthigy-desktop --mcp` over a user-only local socket.
#[cfg(unix)]
pub async fn serve(bridge: Arc<Bridge>) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    use tokio::net::{UnixListener, UnixStream};
    let path = socket_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    if path.exists() {
        if UnixStream::connect(&path).await.is_ok() {
            return Err(io::Error::new(
                io::ErrorKind::AddrInUse,
                format!("another Synthigy already serves {}", path.display()),
            ));
        }
        std::fs::remove_file(&path)?;
    }
    let listener = UnixListener::bind(&path)?;
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    eprintln!("synthigy desktop: tools on {}", path.display());
    loop {
        let (stream, _) = listener.accept().await?;
        spawn_handler(stream, &bridge);
    }
}

/// Serve tool calls from `synthigy-desktop --mcp` over a local named pipe.
#[cfg(windows)]
pub async fn serve(bridge: Arc<Bridge>) -> io::Result<()> {
    use tokio::net::windows::named_pipe::ServerOptions;
    let name = pipe_name();
    let mut server = ServerOptions::new().first_pipe_instance(true).create(&name)?;
    eprintln!("synthigy desktop: tools on {name}");
    loop {
        server.connect().await?;
        let connected = std::mem::replace(&mut server, ServerOptions::new().create(&name)?);
        spawn_handler(connected, &bridge);
    }
}

#[cfg(unix)]
async fn connect() -> io::Result<tokio::net::UnixStream> {
    tokio::net::UnixStream::connect(socket_path()).await
}

#[cfg(windows)]
async fn connect() -> io::Result<tokio::net::windows::named_pipe::NamedPipeClient> {
    use tokio::net::windows::named_pipe::ClientOptions;
    const ERROR_PIPE_BUSY: i32 = 231;
    for _ in 0..20 {
        match ClientOptions::new().open(pipe_name()) {
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => {
                tokio::time::sleep(Duration::from_millis(50)).await
            }
            other => return other,
        }
    }
    ClientOptions::new().open(pipe_name())
}

/// One tool call against the running app.
pub async fn request(name: &str, args: Value) -> Result<Value, String> {
    let stream = connect().await.map_err(|_| NOT_RUNNING.to_string())?;
    let (reader, mut writer) = tokio::io::split(stream);
    let body = json!({"name": name, "args": args});
    writer
        .write_all(format!("{body}\n").as_bytes())
        .await
        .map_err(|e| format!("sending to Synthigy failed: {e}"))?;
    let mut line = String::new();
    BufReader::new(reader)
        .read_line(&mut line)
        .await
        .map_err(|e| format!("reading from Synthigy failed: {e}"))?;
    let reply: Value = serde_json::from_str(&line).map_err(|_| NOT_RUNNING.to_string())?;
    if reply["ok"].as_bool() == Some(true) {
        Ok(reply["value"].clone())
    } else {
        Err(reply["error"].as_str().unwrap_or("tool failed").to_string())
    }
}
