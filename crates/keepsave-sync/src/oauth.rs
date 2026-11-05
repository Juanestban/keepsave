//! Google OAuth for installed apps: the browser signs in, Google redirects
//! to a one-shot listener on 127.0.0.1, and the code is exchanged with PKCE.
//! See <https://developers.google.com/identity/protocols/oauth2/native-app>.

use std::time::{Duration, Instant};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::{RngExt, rand_core::UnwrapErr, rngs::SysRng};
use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::{TcpListener, TcpStream},
};
use zeroize::Zeroizing;

use crate::{Credentials, Result, SyncError};

pub const SCOPE: &str = "https://www.googleapis.com/auth/drive.file";
const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const REVOKE_URL: &str = "https://oauth2.googleapis.com/revoke";
/// How long to wait for the user in the browser.
const SIGN_IN_TIMEOUT: Duration = Duration::from_secs(5 * 60);
/// Refresh this long before the access token actually expires.
const EXPIRY_MARGIN: Duration = Duration::from_secs(60);

/// What the browser tab shows after the redirect, in the user's language.
#[derive(Clone)]
pub struct BrowserPage {
  pub title: String,
  pub message: String,
}

pub(crate) struct AccessToken {
  pub token: SecretString,
  pub expires_at: Instant,
}

impl AccessToken {
  pub fn is_fresh(&self) -> bool {
    Instant::now() + EXPIRY_MARGIN < self.expires_at
  }
}

pub(crate) struct Tokens {
  pub access: AccessToken,
  pub refresh: SecretString,
}

#[derive(Deserialize)]
struct TokenResponse {
  access_token: SecretString,
  expires_in: u64,
  #[serde(default)]
  refresh_token: Option<SecretString>,
}

#[derive(Deserialize)]
struct TokenError {
  error: String,
  #[serde(default)]
  error_description: Option<String>,
}

/// Runs the browser sign-in. `open_browser` receives the consent URL; the
/// pages are shown in the tab once Google redirects back.
pub(crate) async fn authorize(
  http: &reqwest::Client,
  credentials: &Credentials,
  pages: (BrowserPage, BrowserPage),
  open_browser: impl FnOnce(&str) -> std::io::Result<()>,
) -> Result<Tokens> {
  let listener = TcpListener::bind(("127.0.0.1", 0)).await?;
  let redirect_uri = format!("http://127.0.0.1:{}", listener.local_addr()?.port());
  let verifier = Zeroizing::new(random_token(32));
  let state = random_token(16);

  let url = authorization_url(&credentials.client_id, &redirect_uri, &challenge_for(&verifier), &state);
  open_browser(url.as_str())?;

  let code = tokio::time::timeout(SIGN_IN_TIMEOUT, wait_for_code(&listener, &state, &pages)).await.map_err(|_| SyncError::TimedOut)??;

  let response = token_request(
    http,
    &[
      ("code", code.as_str()),
      ("client_id", &credentials.client_id),
      ("client_secret", credentials.client_secret.expose_secret()),
      ("redirect_uri", &redirect_uri),
      ("grant_type", "authorization_code"),
      ("code_verifier", &verifier),
    ],
  )
  .await?;
  let refresh = response.refresh_token.clone().ok_or_else(|| SyncError::Protocol("no refresh token in the response".into()))?;
  Ok(Tokens { access: access_from(response), refresh })
}

pub(crate) async fn refresh(http: &reqwest::Client, credentials: &Credentials, refresh_token: &SecretString) -> Result<AccessToken> {
  let response = token_request(
    http,
    &[
      ("client_id", &credentials.client_id),
      ("client_secret", credentials.client_secret.expose_secret()),
      ("refresh_token", refresh_token.expose_secret()),
      ("grant_type", "refresh_token"),
    ],
  )
  .await?;
  Ok(access_from(response))
}

/// Revokes the grant at Google. Revoking the refresh token also invalidates
/// its access tokens.
pub(crate) async fn revoke(http: &reqwest::Client, refresh_token: &SecretString) -> Result<()> {
  let response = http.post(REVOKE_URL).form(&[("token", refresh_token.expose_secret())]).send().await?;
  let status = response.status();
  // 400 means the token was already invalid, which is what we want anyway.
  if status.is_success() || status.as_u16() == 400 {
    Ok(())
  } else {
    Err(SyncError::Http { status: status.as_u16(), message: "revoke failed".into() })
  }
}

fn access_from(response: TokenResponse) -> AccessToken {
  AccessToken { token: response.access_token, expires_at: Instant::now() + Duration::from_secs(response.expires_in) }
}

async fn token_request(http: &reqwest::Client, form: &[(&str, &str)]) -> Result<TokenResponse> {
  let response = http.post(TOKEN_URL).form(form).send().await?;
  let status = response.status();
  let body = response.bytes().await?;
  if status.is_success() {
    return serde_json::from_slice(&body).map_err(|error| SyncError::Protocol(error.to_string()));
  }
  match serde_json::from_slice::<TokenError>(&body) {
    Ok(error) if error.error == "invalid_grant" => Err(SyncError::Reauthorize),
    Ok(error) => {
      let message = match error.error_description {
        Some(description) => format!("{}: {description}", error.error),
        None => error.error,
      };
      Err(SyncError::Http { status: status.as_u16(), message })
    }
    Err(_) => Err(SyncError::Http { status: status.as_u16(), message: status.to_string() }),
  }
}

fn authorization_url(client_id: &str, redirect_uri: &str, challenge: &str, state: &str) -> Url {
  Url::parse_with_params(
    AUTH_URL,
    [
      ("client_id", client_id),
      ("redirect_uri", redirect_uri),
      ("response_type", "code"),
      ("scope", SCOPE),
      ("code_challenge", challenge),
      ("code_challenge_method", "S256"),
      ("state", state),
      // Always return a refresh token, even when the user approved before.
      ("access_type", "offline"),
      ("prompt", "consent"),
    ],
  )
  .expect("AUTH_URL is a valid URL")
}

/// A URL-safe random string from `bytes` random bytes.
fn random_token(bytes: usize) -> String {
  let mut rng = UnwrapErr(SysRng);
  let raw: Zeroizing<Vec<u8>> = Zeroizing::new((0..bytes).map(|_| rng.random()).collect());
  URL_SAFE_NO_PAD.encode(&*raw)
}

/// The S256 PKCE challenge for `verifier` (RFC 7636).
fn challenge_for(verifier: &str) -> String {
  URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
}

#[derive(Debug, PartialEq, Eq)]
enum Redirect {
  Code(String),
  Denied,
  /// Not our redirect, e.g. a favicon request or a wrong `state`.
  Ignore,
}

fn parse_redirect(target: &str, expected_state: &str) -> Redirect {
  let Ok(url) = Url::parse(&format!("http://127.0.0.1{target}")) else {
    return Redirect::Ignore;
  };
  let mut code = None;
  let mut state = None;
  let mut error = false;
  for (key, value) in url.query_pairs() {
    match key.as_ref() {
      "code" => code = Some(value.into_owned()),
      "state" => state = Some(value.into_owned()),
      "error" => error = true,
      _ => {}
    }
  }
  if state.as_deref() != Some(expected_state) {
    return Redirect::Ignore;
  }
  match (code, error) {
    (_, true) => Redirect::Denied,
    (Some(code), false) => Redirect::Code(code),
    (None, false) => Redirect::Ignore,
  }
}

/// Accepts connections until Google's redirect arrives.
async fn wait_for_code(listener: &TcpListener, state: &str, pages: &(BrowserPage, BrowserPage)) -> Result<Zeroizing<String>> {
  loop {
    let (mut stream, _) = listener.accept().await?;
    let Some(target) = read_request_target(&mut stream).await else {
      continue;
    };
    match parse_redirect(&target, state) {
      Redirect::Code(code) => {
        respond(&mut stream, "200 OK", &page_html(&pages.0)).await;
        return Ok(Zeroizing::new(code));
      }
      Redirect::Denied => {
        respond(&mut stream, "200 OK", &page_html(&pages.1)).await;
        return Err(SyncError::Denied);
      }
      Redirect::Ignore => respond(&mut stream, "404 Not Found", "").await,
    }
  }
}

/// The path and query of an HTTP request line, e.g. `/?code=…`.
async fn read_request_target(stream: &mut TcpStream) -> Option<String> {
  let mut buffer = Zeroizing::new(vec![0u8; 8192]);
  let mut read = 0;
  while read < buffer.len() {
    let n = stream.read(&mut buffer[read..]).await.ok()?;
    if n == 0 {
      break;
    }
    read += n;
    if buffer[..read].windows(4).any(|window| window == b"\r\n\r\n") {
      break;
    }
  }
  let head = std::str::from_utf8(&buffer[..read]).ok()?;
  let mut parts = head.lines().next()?.split_whitespace();
  (parts.next()? == "GET").then_some(())?;
  parts.next().map(str::to_owned)
}

async fn respond(stream: &mut TcpStream, status: &str, body: &str) {
  let response =
    format!("HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
  // The browser may already be gone; the code is what matters.
  let _ = stream.write_all(response.as_bytes()).await;
  let _ = stream.shutdown().await;
}

fn page_html(page: &BrowserPage) -> String {
  format!(
    "<!doctype html><html><head><meta charset=\"utf-8\"><title>KeepSave</title>\
     <style>body{{margin:0;height:100vh;display:grid;place-items:center;background:#090a10;color:#eef0f7;\
     font-family:system-ui,sans-serif}}main{{text-align:center;padding:32px 40px;border-radius:22px;\
     background:rgba(255,255,255,.06);border:1px solid rgba(255,255,255,.13)}}p{{color:#969caf}}</style>\
     </head><body><main><h1>{}</h1><p>{}</p></main></body></html>",
    escape_html(&page.title),
    escape_html(&page.message)
  )
}

fn escape_html(text: &str) -> String {
  text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
  use super::*;

  fn pages() -> (BrowserPage, BrowserPage) {
    let page = |title: &str| BrowserPage { title: title.into(), message: "m".into() };
    (page("ok"), page("denied"))
  }

  #[test]
  fn pkce_challenge_matches_rfc_7636() {
    assert_eq!(challenge_for("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"), "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
  }

  #[test]
  fn random_tokens_are_url_safe_and_unique() {
    let a = random_token(32);
    assert_eq!(a.len(), 43);
    assert!(a.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
    assert_ne!(a, random_token(32));
  }

  #[test]
  fn authorization_url_asks_for_offline_drive_file_access() {
    let url = authorization_url("id.apps", "http://127.0.0.1:5000", "chal", "st8");
    let pairs: Vec<(String, String)> = url.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();
    let get = |key: &str| pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str());
    assert_eq!(get("scope"), Some(SCOPE));
    assert_eq!(get("redirect_uri"), Some("http://127.0.0.1:5000"));
    assert_eq!(get("code_challenge_method"), Some("S256"));
    assert_eq!(get("access_type"), Some("offline"));
    assert_eq!(get("state"), Some("st8"));
  }

  #[test]
  fn redirects_are_matched_by_state() {
    assert_eq!(parse_redirect("/?state=s&code=4%2Fabc", "s"), Redirect::Code("4/abc".into()));
    assert_eq!(parse_redirect("/?state=s&error=access_denied", "s"), Redirect::Denied);
    assert_eq!(parse_redirect("/?state=other&code=x", "s"), Redirect::Ignore);
    assert_eq!(parse_redirect("/favicon.ico", "s"), Redirect::Ignore);
  }

  #[test]
  fn browser_pages_are_escaped() {
    let html = page_html(&BrowserPage { title: "<b>".into(), message: "a & b".into() });
    assert!(html.contains("&lt;b&gt;") && html.contains("a &amp; b"));
  }

  /// Talks to Google over TLS with a bogus refresh token; only checks that
  /// HTTPS works and the error is parsed. Run with `--ignored`.
  #[tokio::test]
  #[ignore = "needs the network"]
  async fn google_rejects_a_bogus_refresh_token() {
    let http = crate::drive::http_client().unwrap();
    let credentials = Credentials { client_id: "bogus.apps.googleusercontent.com".into(), client_secret: SecretString::from("bogus") };
    let error = refresh(&http, &credentials, &SecretString::from("bogus")).await.err().unwrap();
    assert!(matches!(error, SyncError::Http { .. } | SyncError::Reauthorize), "{error}");
  }

  #[tokio::test]
  async fn the_loopback_listener_returns_the_code() {
    let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let browser = tokio::spawn(async move {
      let mut responses = Vec::new();
      for target in ["/favicon.ico", "/?state=st&code=the-code"] {
        let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        stream.write_all(format!("GET {target} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n").as_bytes()).await.unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).await.unwrap();
        responses.push(response);
      }
      responses
    });

    let code = wait_for_code(&listener, "st", &pages()).await.unwrap();
    assert_eq!(code.as_str(), "the-code");
    let responses = browser.await.unwrap();
    assert!(responses[0].starts_with("HTTP/1.1 404"));
    assert!(responses[1].starts_with("HTTP/1.1 200") && responses[1].contains("<h1>ok</h1>"));
  }
}
