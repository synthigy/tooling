use serde_json::{json, Value};
use std::path::PathBuf;

fn path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("synthigy")
        .join("desktop-engines.json")
}

#[tauri::command]
pub fn engines_load() -> Result<Value, String> {
    match std::fs::read(path()) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| e.to_string()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(json!([])),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn engines_save(engines: Value) -> Result<(), String> {
    let path = path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let body = serde_json::to_vec_pretty(&engines).map_err(|e| e.to_string())?;
    std::fs::write(&path, body).map_err(|e| e.to_string())
}
