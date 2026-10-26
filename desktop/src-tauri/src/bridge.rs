use dashmap::DashMap;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use tauri::ipc::Channel;
use tokio::sync::oneshot;

pub type Outcome = Result<Value, String>;

#[derive(Default)]
pub struct Bridge {
    channels: Mutex<HashMap<String, Channel<Value>>>,
    focused: Mutex<Option<String>>,
    pending: DashMap<u64, (String, oneshot::Sender<Outcome>)>,
    next_id: AtomicU64,
}

impl Bridge {
    pub fn attach(&self, window: &str, channel: Channel<Value>) {
        self.channels.lock().unwrap().insert(window.to_string(), channel);
        self.focused.lock().unwrap().get_or_insert_with(|| window.to_string());
    }

    pub fn focus(&self, window: &str) {
        *self.focused.lock().unwrap() = Some(window.to_string());
    }

    pub fn detach(&self, window: &str) {
        self.channels.lock().unwrap().remove(window);
        let mut focused = self.focused.lock().unwrap();
        if focused.as_deref() == Some(window) {
            *focused = None;
        }
        let ids: Vec<u64> = self
            .pending
            .iter()
            .filter(|e| e.value().0 == window)
            .map(|e| *e.key())
            .collect();
        for id in ids {
            if let Some((_, (_, tx))) = self.pending.remove(&id) {
                let _ = tx.send(Err(format!("window {window} closed during the call")));
            }
        }
    }

    pub fn resolve(&self, id: u64, outcome: Outcome) {
        if let Some((_, (_, tx))) = self.pending.remove(&id) {
            let _ = tx.send(outcome);
        }
    }

    fn target(&self, requested: Option<&str>) -> Result<(String, Channel<Value>), String> {
        let channels = self.channels.lock().unwrap();
        let label = match requested {
            Some(w) => w.to_string(),
            None => self
                .focused
                .lock()
                .unwrap()
                .clone()
                .or_else(|| channels.keys().next().cloned())
                .ok_or("no open canvas: open a Synthigy window first")?,
        };
        let ch = channels
            .get(&label)
            .cloned()
            .ok_or_else(|| format!("no window named {label}"))?;
        Ok((label, ch))
    }

    pub async fn call(&self, name: &str, args: Value, window: Option<&str>, timeout: Duration) -> Outcome {
        let (label, ch) = self.target(window)?;
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.insert(id, (label, tx));
        if let Err(e) = ch.send(json!({"id": id, "name": name, "args": args})) {
            self.pending.remove(&id);
            return Err(format!("IPC send failed: {e}"));
        }
        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(outcome)) => outcome,
            Ok(Err(_)) => Err("call dropped".into()),
            Err(_) => {
                self.pending.remove(&id);
                Err(format!("{name} timed out after {}s", timeout.as_secs()))
            }
        }
    }
}
