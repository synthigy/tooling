mod auth;
mod bridge;
mod engines;
mod ipc;
mod mcp;
mod tokens;

use bridge::Bridge;
use serde_json::Value;
use std::sync::Arc;
use tauri::ipc::Channel;
use tauri::{Manager, WindowEvent};

#[tauri::command]
fn mcp_attach(window: tauri::Window, bridge: tauri::State<Arc<Bridge>>, channel: Channel<Value>) {
    bridge.attach(window.label(), channel);
}

#[tauri::command]
fn mcp_result(
    bridge: tauri::State<Arc<Bridge>>,
    id: u64,
    ok: bool,
    value: Option<Value>,
    error: Option<String>,
) {
    let outcome = if ok {
        Ok(value.unwrap_or(Value::Null))
    } else {
        Err(error.unwrap_or_else(|| "tool failed".into()))
    };
    bridge.resolve(id, outcome);
}

/// `synthigy-desktop --mcp`: no window, MCP over stdio.
pub fn mcp_stdio() {
    let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
    if let Err(e) = runtime.block_on(mcp::stdio()) {
        eprintln!("synthigy desktop --mcp: {e}");
        std::process::exit(1);
    }
}

pub fn run() {
    let bridge = Arc::new(Bridge::default());
    tauri::Builder::default()
        .manage(bridge.clone())
        .manage(auth::Auth::default())
        .setup(move |_app| {
            tauri::async_runtime::spawn(async move {
                if let Err(e) = ipc::serve(bridge).await {
                    eprintln!("synthigy desktop: tool socket failed: {e}");
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            let bridge = window.state::<Arc<Bridge>>();
            match event {
                WindowEvent::Focused(true) => bridge.focus(window.label()),
                WindowEvent::Destroyed => bridge.detach(window.label()),
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            mcp_attach,
            mcp_result,
            engines::engines_load,
            engines::engines_save,
            auth::auth_token,
            auth::auth_invalidate,
            auth::auth_login,
            auth::auth_cancel,
            auth::auth_logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running synthigy desktop");
}
