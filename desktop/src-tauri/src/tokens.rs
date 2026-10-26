use keyring::{Entry, Error};
use serde_json::{Map, Value};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

const SERVICE: &str = "com.synthigy.desktop";

static WARNED: AtomicBool = AtomicBool::new(false);

fn file_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("synthigy")
        .join("desktop-tokens.json")
}

fn read_file() -> Map<String, Value> {
    std::fs::read(file_path())
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default()
}

fn write_file(tokens: &Map<String, Value>) -> Result<(), String> {
    let path = file_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let body = serde_json::to_vec_pretty(tokens).map_err(|e| e.to_string())?;
    std::fs::write(&path, body).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn fall_back(e: &Error) {
    if !WARNED.swap(true, Ordering::Relaxed) {
        eprintln!(
            "synthigy desktop: no OS secret store ({e}); refresh tokens go to {} (0600)",
            file_path().display()
        );
    }
}

pub fn load(engine: &str) -> Result<Option<String>, String> {
    let entry = Entry::new(SERVICE, engine).map_err(|e| e.to_string())?;
    match entry.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(Error::NoEntry) => Ok(read_file().get(engine).and_then(Value::as_str).map(String::from)),
        Err(e) => {
            fall_back(&e);
            Ok(read_file().get(engine).and_then(Value::as_str).map(String::from))
        }
    }
}

pub fn save(engine: &str, token: Option<&str>) -> Result<(), String> {
    let entry = Entry::new(SERVICE, engine).map_err(|e| e.to_string())?;
    let stored = match token {
        Some(t) => entry.set_password(t),
        None => match entry.delete_credential() {
            Err(Error::NoEntry) => Ok(()),
            other => other,
        },
    };
    let mut file = read_file();
    match stored {
        Ok(()) => {
            if file.remove(engine).is_some() {
                write_file(&file)?;
            }
            Ok(())
        }
        Err(e) => {
            fall_back(&e);
            match token {
                Some(t) => file.insert(engine.to_string(), Value::String(t.to_string())),
                None => file.remove(engine),
            };
            write_file(&file)
        }
    }
}
