//! The few Drive v3 endpoints KeepSave needs, written by hand.
//! With the `drive.file` scope, Drive only shows files this app created.

use std::time::Duration;

use rand::{RngExt, rand_core::UnwrapErr, rngs::SysRng};
use reqwest::{RequestBuilder, Response, StatusCode};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{
  BrowserPage, Credentials, RemoteFile, RemoteStore, Result, SyncError,
  oauth::{self, AccessToken},
  token_store,
};

/// The visible folder in the user's Drive that holds the vaults.
pub const FOLDER_NAME: &str = "KeepSave";
const API: &str = "https://www.googleapis.com/drive/v3";
const UPLOAD_API: &str = "https://www.googleapis.com/upload/drive/v3";
const FOLDER_MIME: &str = "application/vnd.google-apps.folder";
const FILE_FIELDS: &str = "id,name,size,modifiedTime,version,trashed";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);

/// A signed-in Drive connection. Share it behind an `Arc`.
pub struct DriveClient {
  http: reqwest::Client,
  credentials: Credentials,
  refresh_token: SecretString,
  access: Mutex<Option<AccessToken>>,
  folder_id: Mutex<Option<String>>,
}

#[derive(Deserialize)]
struct FileList {
  #[serde(default)]
  files: Vec<RemoteFile>,
}

#[derive(Deserialize)]
struct ApiError {
  error: ApiErrorBody,
}

#[derive(Deserialize)]
struct ApiErrorBody {
  message: String,
}

impl DriveClient {
  /// A client for a refresh token stored earlier. Makes no request.
  pub fn new(credentials: Credentials, refresh_token: SecretString) -> Result<Self> {
    Ok(Self { http: http_client()?, credentials, refresh_token, access: Mutex::new(None), folder_id: Mutex::new(None) })
  }

  /// Signs in through the browser and stores the refresh token in the OS
  /// credential store. `pages` are shown in the tab on success and denial.
  pub async fn connect(
    credentials: Credentials,
    pages: (BrowserPage, BrowserPage),
    open_browser: impl FnOnce(&str) -> std::io::Result<()>,
  ) -> Result<Self> {
    let http = http_client()?;
    let tokens = oauth::authorize(&http, &credentials, pages, open_browser).await?;
    token_store::save_refresh_token(&tokens.refresh)?;
    Ok(Self { http, credentials, refresh_token: tokens.refresh, access: Mutex::new(Some(tokens.access)), folder_id: Mutex::new(None) })
  }

  /// Revokes the grant and forgets the stored token. The token is removed
  /// locally even when Google can't be reached.
  pub async fn disconnect(&self) -> Result<()> {
    let revoked = oauth::revoke(&self.http, &self.refresh_token).await;
    token_store::delete_refresh_token()?;
    revoked
  }

  async fn access_token(&self, force_refresh: bool) -> Result<SecretString> {
    let mut access = self.access.lock().await;
    if let Some(token) = access.as_ref().filter(|token| !force_refresh && token.is_fresh()) {
      return Ok(token.token.clone());
    }
    let token = oauth::refresh(&self.http, &self.credentials, &self.refresh_token).await?;
    let value = token.token.clone();
    *access = Some(token);
    Ok(value)
  }

  /// Sends an authorized request, refreshing the access token once if
  /// Google rejects it. `build` is called again for the retry.
  async fn send(&self, build: impl Fn(&reqwest::Client) -> RequestBuilder) -> Result<Response> {
    let token = self.access_token(false).await?;
    let response = build(&self.http).bearer_auth(token.expose_secret()).send().await?;
    let response = if response.status() == StatusCode::UNAUTHORIZED {
      let token = self.access_token(true).await?;
      build(&self.http).bearer_auth(token.expose_secret()).send().await?
    } else {
      response
    };
    check(response).await
  }

  async fn json<T: for<'de> Deserialize<'de>>(&self, build: impl Fn(&reqwest::Client) -> RequestBuilder) -> Result<T> {
    let bytes = self.send(build).await?.bytes().await?;
    serde_json::from_slice(&bytes).map_err(|error| SyncError::Protocol(error.to_string()))
  }

  /// The KeepSave folder in the Drive root, created on first use.
  async fn folder_id(&self) -> Result<String> {
    let mut cached = self.folder_id.lock().await;
    if let Some(id) = cached.as_ref() {
      return Ok(id.clone());
    }
    let query = format!("name = '{}' and mimeType = '{FOLDER_MIME}' and 'root' in parents and trashed = false", escape_query(FOLDER_NAME));
    let list: FileList =
      self.json(|http| http.get(format!("{API}/files")).query(&[("q", query.as_str()), ("fields", "files(id,name)"), ("spaces", "drive")])).await?;
    let id = match list.files.into_iter().next() {
      Some(folder) => folder.id,
      None => {
        let body = json!({ "name": FOLDER_NAME, "mimeType": FOLDER_MIME, "parents": ["root"] });
        let folder: RemoteFile = self.json(|http| http.post(format!("{API}/files")).query(&[("fields", "id,name")]).json(&body)).await?;
        folder.id
      }
    };
    *cached = Some(id.clone());
    Ok(id)
  }
}

impl RemoteStore for DriveClient {
  async fn list_vaults(&self) -> Result<Vec<RemoteFile>> {
    let folder = self.folder_id().await?;
    let query = format!("'{}' in parents and trashed = false and mimeType != '{FOLDER_MIME}'", escape_query(&folder));
    let fields = format!("files({FILE_FIELDS})");
    let list: FileList = self
      .json(|http| {
        http.get(format!("{API}/files")).query(&[
          ("q", query.as_str()),
          ("fields", fields.as_str()),
          ("orderBy", "name"),
          ("pageSize", "200"),
          ("spaces", "drive"),
        ])
      })
      .await?;
    Ok(list.files.into_iter().filter(|file| file.name.to_ascii_lowercase().ends_with(".kdbx")).collect())
  }

  async fn download(&self, id: &str) -> Result<Vec<u8>> {
    let response = self.send(|http| http.get(format!("{API}/files/{id}")).query(&[("alt", "media")])).await?;
    Ok(response.bytes().await?.to_vec())
  }

  async fn create(&self, name: &str, bytes: Vec<u8>) -> Result<RemoteFile> {
    let folder = self.folder_id().await?;
    let metadata = json!({ "name": name, "parents": [folder] });
    let (boundary, body) = multipart_related(&metadata, &bytes);
    let content_type = format!("multipart/related; boundary={boundary}");
    self
      .json(|http| {
        http
          .post(format!("{UPLOAD_API}/files"))
          .query(&[("uploadType", "multipart"), ("fields", FILE_FIELDS)])
          .header(reqwest::header::CONTENT_TYPE, content_type.as_str())
          .body(body.clone())
      })
      .await
  }

  async fn update(&self, id: &str, bytes: Vec<u8>) -> Result<RemoteFile> {
    self
      .json(|http| {
        http
          .patch(format!("{UPLOAD_API}/files/{id}"))
          .query(&[("uploadType", "media"), ("fields", FILE_FIELDS)])
          .header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
          .body(bytes.clone())
      })
      .await
  }

  async fn metadata(&self, id: &str) -> Result<RemoteFile> {
    self.json(|http| http.get(format!("{API}/files/{id}")).query(&[("fields", FILE_FIELDS)])).await
  }
}

pub(crate) fn http_client() -> Result<reqwest::Client> {
  // rustls with `ring`: the default aws-lc backend needs a C toolchain that
  // doesn't build reliably on Windows. Fails only if a provider is already
  // installed, which is fine.
  let _ = rustls::crypto::ring::default_provider().install_default();
  Ok(reqwest::Client::builder().timeout(REQUEST_TIMEOUT).user_agent(concat!("KeepSave/", env!("CARGO_PKG_VERSION"))).build()?)
}

/// Turns an error status into [`SyncError::Http`] with Google's message.
async fn check(response: Response) -> Result<Response> {
  let status = response.status();
  if status.is_success() {
    return Ok(response);
  }
  let body = response.bytes().await.unwrap_or_default();
  let message = serde_json::from_slice::<ApiError>(&body).map(|error| error.error.message).unwrap_or_else(|_| status.to_string());
  Err(SyncError::Http { status: status.as_u16(), message })
}

/// Escapes a value for a single-quoted string in a Drive `q` query.
fn escape_query(value: &str) -> String {
  value.replace('\\', "\\\\").replace('\'', "\\'")
}

/// A `multipart/related` upload body: JSON metadata, then the file.
fn multipart_related(metadata: &serde_json::Value, bytes: &[u8]) -> (String, Vec<u8>) {
  let random: u128 = UnwrapErr(SysRng).random();
  let boundary = format!("keepsave-{random:032x}");
  let mut body = Vec::with_capacity(bytes.len() + 512);
  body.extend_from_slice(format!("--{boundary}\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{metadata}\r\n").as_bytes());
  body.extend_from_slice(format!("--{boundary}\r\nContent-Type: application/octet-stream\r\n\r\n").as_bytes());
  body.extend_from_slice(bytes);
  body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
  (boundary, body)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn query_values_are_escaped() {
    assert_eq!(escape_query("it's"), "it\\'s");
    assert_eq!(escape_query("a\\b"), "a\\\\b");
  }

  #[test]
  fn multipart_body_has_metadata_then_file() {
    let (boundary, body) = multipart_related(&json!({ "name": "v.kdbx" }), b"\x03\xd9\xa2\x9a");
    let text = String::from_utf8_lossy(&body);
    assert!(text.starts_with(&format!("--{boundary}\r\nContent-Type: application/json")));
    assert!(text.contains("{\"name\":\"v.kdbx\"}"));
    assert!(body.windows(4).any(|window| window == b"\x03\xd9\xa2\x9a"));
    assert!(text.ends_with(&format!("\r\n--{boundary}--\r\n")));
  }
}
