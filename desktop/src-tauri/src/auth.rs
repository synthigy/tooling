use crate::tokens;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tauri::Manager;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{oneshot, Mutex};

const SCOPE: &str = "openid profile offline_access";
const LOGIN_TIMEOUT: Duration = Duration::from_secs(300);
const EXPIRY_MARGIN: Duration = Duration::from_secs(60);

struct Session {
    access: String,
    expires_at: SystemTime,
}

#[derive(Default)]
pub struct Auth {
    sessions: Mutex<HashMap<String, Session>>,
    cancel: std::sync::Mutex<Option<oneshot::Sender<()>>>,
}

struct Oidc {
    client_id: String,
    audience: Option<String>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
}

fn random_token(n: usize) -> String {
    URL_SAFE_NO_PAD.encode(rand::random_iter::<u8>().take(n).collect::<Vec<_>>())
}

async fn discover(endpoint: &str) -> Result<Oidc, String> {
    let body: Value = reqwest::get(format!("{endpoint}/.well-known/synthigy"))
        .await
        .map_err(|e| format!("can't reach {endpoint}: {e}"))?
        .json()
        .await
        .map_err(|e| format!("bad discovery document: {e}"))?;
    let oidc = &body["auth"]["oidc"];
    let client_id = oidc["desktop_client_id"]
        .as_str()
        .ok_or("this engine has no Synthigy Desktop client: upgrade the engine")?;
    Ok(Oidc {
        client_id: client_id.to_string(),
        audience: oidc["audience"].as_str().map(String::from),
    })
}

async fn token_request(endpoint: &str, form: &[(&str, &str)]) -> Result<TokenResponse, (u16, String)> {
    let resp = reqwest::Client::new()
        .post(format!("{endpoint}/oauth/token"))
        .header("Accept", "application/json")
        .form(form)
        .send()
        .await
        .map_err(|e| (0, e.to_string()))?;
    let status = resp.status().as_u16();
    let text = resp.text().await.map_err(|e| (status, e.to_string()))?;
    if status == 200 {
        serde_json::from_str(&text).map_err(|e| (status, e.to_string()))
    } else {
        Err((status, text))
    }
}

async fn store_refresh(endpoint: String, token: Option<String>) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || tokens::save(&endpoint, token.as_deref()))
        .await
        .map_err(|e| e.to_string())?
}

async fn load_refresh(endpoint: String) -> Result<Option<String>, String> {
    tauri::async_runtime::spawn_blocking(move || tokens::load(&endpoint))
        .await
        .map_err(|e| e.to_string())?
}

async fn accept(
    sessions: &mut HashMap<String, Session>,
    endpoint: &str,
    tokens: TokenResponse,
) -> Result<String, String> {
    if let Some(refresh) = tokens.refresh_token {
        store_refresh(endpoint.to_string(), Some(refresh)).await?;
    }
    let expires_at = SystemTime::now() + Duration::from_secs(tokens.expires_in.unwrap_or(300));
    sessions.insert(
        endpoint.to_string(),
        Session { access: tokens.access_token.clone(), expires_at },
    );
    Ok(tokens.access_token)
}

/// A live access token for `endpoint`, refreshed from the keychain when needed; None means sign in.
#[tauri::command]
pub async fn auth_token(auth: tauri::State<'_, Auth>, endpoint: String) -> Result<Option<String>, String> {
    let mut sessions = auth.sessions.lock().await;
    if let Some(s) = sessions.get(&endpoint) {
        if s.expires_at > SystemTime::now() + EXPIRY_MARGIN {
            return Ok(Some(s.access.clone()));
        }
    }
    let Some(refresh) = load_refresh(endpoint.clone()).await? else {
        return Ok(None);
    };
    let oidc = discover(&endpoint).await?;
    let mut form = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh.as_str()),
        ("client_id", oidc.client_id.as_str()),
    ];
    if let Some(a) = &oidc.audience {
        form.push(("audience", a));
    }
    match token_request(&endpoint, &form).await {
        Ok(tokens) => accept(&mut sessions, &endpoint, tokens).await.map(Some),
        Err((400 | 401, _)) => {
            sessions.remove(&endpoint);
            store_refresh(endpoint, None).await?;
            Ok(None)
        }
        Err((status, body)) => Err(format!("token refresh failed ({status}): {body}")),
    }
}

/// Forget the cached access token for `endpoint` (the engine refused it); the next auth_token refreshes.
#[tauri::command]
pub async fn auth_invalidate(auth: tauri::State<'_, Auth>, endpoint: String) -> Result<(), String> {
    auth.sessions.lock().await.remove(&endpoint);
    Ok(())
}

/// Sign in through the system browser with a loopback redirect (RFC 8252) and PKCE.
#[tauri::command]
pub async fn auth_login(
    app: tauri::AppHandle,
    auth: tauri::State<'_, Auth>,
    endpoint: String,
) -> Result<String, String> {
    let oidc = discover(&endpoint).await?;
    let listener = TcpListener::bind("127.0.0.1:0").await.map_err(|e| e.to_string())?;
    let redirect_uri = format!("http://127.0.0.1:{}/callback", listener.local_addr().unwrap().port());
    let verifier = random_token(32);
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let state = random_token(24);

    let mut authorize = url::Url::parse(&format!("{endpoint}/oauth/authorize")).map_err(|e| e.to_string())?;
    {
        let mut q = authorize.query_pairs_mut();
        q.append_pair("response_type", "code")
            .append_pair("client_id", &oidc.client_id)
            .append_pair("redirect_uri", &redirect_uri)
            .append_pair("scope", SCOPE)
            .append_pair("state", &state)
            .append_pair("code_challenge", &challenge)
            .append_pair("code_challenge_method", "S256");
        if let Some(a) = &oidc.audience {
            q.append_pair("audience", a);
        }
    }

    let (cancel_tx, cancel_rx) = oneshot::channel();
    if let Some(previous) = auth.cancel.lock().unwrap().replace(cancel_tx) {
        let _ = previous.send(());
    }
    open::that(authorize.as_str()).map_err(|e| format!("can't open the browser: {e}"))?;

    let code = tokio::select! {
        r = wait_for_code(&listener, &state) => r?,
        _ = tokio::time::sleep(LOGIN_TIMEOUT) => return Err("sign-in timed out".into()),
        _ = cancel_rx => return Err("sign-in cancelled".into()),
    };
    drop(listener);
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.set_focus();
    }

    let tokens = token_request(
        &endpoint,
        &[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", &redirect_uri),
            ("client_id", &oidc.client_id),
            ("code_verifier", &verifier),
        ],
    )
    .await
    .map_err(|(status, body)| format!("code exchange failed ({status}): {body}"))?;
    let mut sessions = auth.sessions.lock().await;
    accept(&mut sessions, &endpoint, tokens).await
}

async fn wait_for_code(listener: &TcpListener, state: &str) -> Result<String, String> {
    loop {
        let (mut stream, _) = listener.accept().await.map_err(|e| e.to_string())?;
        let mut buf = vec![0u8; 8192];
        let n = stream.read(&mut buf).await.map_err(|e| e.to_string())?;
        let request = String::from_utf8_lossy(&buf[..n]);
        let target = request.split_whitespace().nth(1).unwrap_or("/");
        let url = url::Url::parse(&format!("http://127.0.0.1{target}")).map_err(|e| e.to_string())?;
        if url.path() != "/callback" {
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").await;
            continue;
        }
        let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
        let outcome = if params.get("state").map(String::as_str) != Some(state) {
            Err("sign-in answered with a foreign state".to_string())
        } else if let Some(code) = params.get("code") {
            Ok(code.clone())
        } else {
            Err(params
                .get("error_description")
                .or(params.get("error"))
                .cloned()
                .unwrap_or_else(|| "sign-in failed".into()))
        };
        let message = match &outcome {
            Ok(_) => "Signed in. You can close this tab and return to Synthigy.".to_string(),
            Err(e) => format!("Sign-in failed: {}", html_escape(e)),
        };
        let page = format!(
            "<!doctype html><meta charset=utf-8><title>Synthigy</title>\
             <body style=\"font:15px system-ui;display:grid;place-items:center;height:100vh;margin:0\">\
             <p>{message}</p></body>"
        );
        let _ = stream
            .write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{page}",
                    page.len()
                )
                .as_bytes(),
            )
            .await;
        return outcome;
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[tauri::command]
pub fn auth_cancel(auth: tauri::State<'_, Auth>) {
    if let Some(tx) = auth.cancel.lock().unwrap().take() {
        let _ = tx.send(());
    }
}

#[tauri::command]
pub async fn auth_logout(auth: tauri::State<'_, Auth>, endpoint: String) -> Result<(), String> {
    auth.sessions.lock().await.remove(&endpoint);
    store_refresh(endpoint, None).await
}
