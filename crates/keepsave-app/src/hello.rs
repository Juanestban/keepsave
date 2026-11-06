//! Windows Hello quick unlock (DESIGN Q25). The master password is sealed in
//! RAM with a key derived from a Windows Hello signature, so only the user's
//! face, fingerprint or PIN can open it again during this session.
//!
//! The calls block until the user answers the prompt: run them on the worker.

use std::path::{Path, PathBuf};

use aes_gcm::{
  Aes256Gcm, Key, KeyInit, Nonce,
  aead::{Aead, AeadCore, OsRng, Payload, rand_core::RngCore},
};
use keepsave_core::SecretString;
use secrecy::ExposeSecret;
use sha2::{Digest, Sha256, digest::generic_array::GenericArray};
use zeroize::Zeroizing;

/// Mixed into the key so a signature made for anything else never fits.
const KEY_LABEL: &[u8] = b"KeepSave quick unlock v1";

/// A vault's master password, encrypted with a key only Windows Hello can
/// rebuild. Holds no secret in the clear, but is still never logged.
#[derive(Clone)]
pub struct Sealed {
  pub path: PathBuf,
  challenge: [u8; 32],
  nonce: [u8; 12],
  ciphertext: Vec<u8>,
}

pub enum HelloError {
  /// This isn't Windows.
  #[cfg_attr(windows, expect(dead_code, reason = "only returned on other platforms"))]
  Unsupported,
  /// The user dismissed the prompt or chose the password.
  Cancelled,
  /// The signature no longer opens the sealed password, e.g. Hello was
  /// reset. The master password is needed again.
  Stale,
  Failed(String),
}

/// Whether Windows Hello can be used on this device.
pub fn available() -> bool {
  platform::available()
}

/// Asks for Windows Hello and seals `password` for the vault at `path`.
pub fn seal(path: PathBuf, password: &SecretString) -> Result<Sealed, HelloError> {
  let mut challenge = [0; 32];
  OsRng.fill_bytes(&mut challenge);
  let signature = platform::sign(&challenge)?;
  Ok(seal_with(path, challenge, &signature, password))
}

/// Asks for Windows Hello and opens the sealed password.
pub fn unseal(sealed: &Sealed) -> Result<SecretString, HelloError> {
  let signature = platform::sign(&sealed.challenge)?;
  open_with(sealed, &signature)
}

fn seal_with(path: PathBuf, challenge: [u8; 32], signature: &[u8], password: &SecretString) -> Sealed {
  let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
  let payload = Payload { msg: password.expose_secret().as_bytes(), aad: path_bytes(&path) };
  // Encrypting into a Vec only fails on absurd lengths.
  let ciphertext = cipher(signature).encrypt(&nonce, payload).expect("encrypt the master password");
  Sealed { path, challenge, nonce: nonce.into(), ciphertext }
}

fn open_with(sealed: &Sealed, signature: &[u8]) -> Result<SecretString, HelloError> {
  let payload = Payload { msg: &sealed.ciphertext, aad: path_bytes(&sealed.path) };
  let plain = Zeroizing::new(cipher(signature).decrypt(Nonce::from_slice(&sealed.nonce), payload).map_err(|_| HelloError::Stale)?);
  let password = std::str::from_utf8(&plain).map_err(|_| HelloError::Stale)?;
  Ok(SecretString::from(password))
}

fn cipher(signature: &[u8]) -> Aes256Gcm {
  let mut key = Zeroizing::new([0; 32]);
  Sha256::new().chain_update(KEY_LABEL).chain_update(signature).finalize_into(GenericArray::from_mut_slice(&mut key[..]));
  Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key[..]))
}

/// Binds the ciphertext to its vault.
fn path_bytes(path: &Path) -> &[u8] {
  path.as_os_str().as_encoded_bytes()
}

#[cfg(windows)]
mod platform {
  use std::{
    sync::{
      Arc,
      atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
  };

  use windows::{
    Security::{
      Credentials::{KeyCredentialCreationOption, KeyCredentialManager, KeyCredentialStatus},
      Cryptography::CryptographicBuffer,
    },
    Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize},
    core::{Array, HSTRING},
  };
  use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, SetForegroundWindow};
  use zeroize::Zeroizing;

  use super::HelloError;

  /// The name of KeepSave's key credential in Windows Hello.
  const CREDENTIAL: &str = "KeepSave";
  /// The window class of the Windows Hello prompt.
  const PROMPT_CLASS: &str = "Credential Dialog Xaml Host";

  impl From<windows::core::Error> for HelloError {
    fn from(error: windows::core::Error) -> Self {
      HelloError::Failed(error.message())
    }
  }

  pub fn available() -> bool {
    init();
    KeyCredentialManager::IsSupportedAsync().and_then(|operation| operation.join()).unwrap_or(false)
  }

  /// Signs `challenge` with KeepSave's key credential, creating it on first
  /// use. Both ask the user to verify.
  pub fn sign(challenge: &[u8]) -> Result<Zeroizing<Vec<u8>>, HelloError> {
    init();
    let _prompt = PromptToFront::start();
    let name = HSTRING::from(CREDENTIAL);
    let opened = KeyCredentialManager::OpenAsync(&name)?.join()?;
    let credential = match opened.Status()? {
      KeyCredentialStatus::NotFound => {
        let created = KeyCredentialManager::RequestCreateAsync(&name, KeyCredentialCreationOption::FailIfExists)?.join()?;
        check(created.Status()?)?;
        created.Credential()?
      }
      status => {
        check(status)?;
        opened.Credential()?
      }
    };
    let signed = credential.RequestSignAsync(&CryptographicBuffer::CreateFromByteArray(challenge)?)?.join()?;
    check(signed.Status()?)?;
    let mut bytes = Array::<u8>::new();
    CryptographicBuffer::CopyToByteArray(&signed.Result()?, &mut bytes)?;
    Ok(Zeroizing::new(bytes.to_vec()))
  }

  fn check(status: KeyCredentialStatus) -> Result<(), HelloError> {
    match status {
      KeyCredentialStatus::Success => Ok(()),
      KeyCredentialStatus::UserCanceled | KeyCredentialStatus::UserPrefersPassword => Err(HelloError::Cancelled),
      KeyCredentialStatus::NotFound => Err(HelloError::Stale),
      other => Err(HelloError::Failed(format!("KeyCredentialStatus {}", other.0))),
    }
  }

  /// WinRT needs the apartment set up on this thread. Already being set up
  /// is fine, so the result is ignored.
  fn init() {
    // SAFETY: plain FFI call with a valid flag.
    let _ = unsafe { RoInitialize(RO_INIT_MULTITHREADED) };
  }

  /// The prompt can open behind our window. While it is pending, bring it
  /// to the front once it appears.
  struct PromptToFront(Arc<AtomicBool>);

  impl PromptToFront {
    fn start() -> Self {
      let done = Arc::new(AtomicBool::new(false));
      let watcher = done.clone();
      thread::spawn(move || {
        let class: Vec<u16> = PROMPT_CLASS.encode_utf16().chain([0]).collect();
        for _ in 0..100 {
          if watcher.load(Ordering::Relaxed) {
            return;
          }
          // SAFETY: `class` is a NUL-terminated UTF-16 string.
          let window = unsafe { FindWindowW(class.as_ptr(), std::ptr::null()) };
          if !window.is_null() {
            // SAFETY: `window` was just returned by FindWindowW.
            unsafe { SetForegroundWindow(window) };
            return;
          }
          thread::sleep(Duration::from_millis(100));
        }
      });
      Self(done)
    }
  }

  impl Drop for PromptToFront {
    fn drop(&mut self) {
      self.0.store(true, Ordering::Relaxed);
    }
  }
}

#[cfg(not(windows))]
mod platform {
  use zeroize::Zeroizing;

  use super::HelloError;

  pub fn available() -> bool {
    false
  }

  pub fn sign(_challenge: &[u8]) -> Result<Zeroizing<Vec<u8>>, HelloError> {
    Err(HelloError::Unsupported)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const SIGNATURE: &[u8] = &[7; 256];

  fn sealed() -> Sealed {
    seal_with("C:/v/Personal.kdbx".into(), [1; 32], SIGNATURE, &SecretString::from("correct horse"))
  }

  #[test]
  fn opens_with_the_same_signature() {
    let password = open_with(&sealed(), SIGNATURE).ok().unwrap();
    assert_eq!(password.expose_secret(), "correct horse");
  }

  #[test]
  fn another_signature_is_stale() {
    assert!(matches!(open_with(&sealed(), &[8; 256]), Err(HelloError::Stale)));
  }

  #[test]
  fn bound_to_its_vault() {
    let moved = Sealed { path: "C:/v/Other.kdbx".into(), ..sealed() };
    assert!(matches!(open_with(&moved, SIGNATURE), Err(HelloError::Stale)));
  }

  #[test]
  fn ciphertext_hides_the_password() {
    let sealed = sealed();
    assert!(!sealed.ciphertext.windows(5).any(|window| window == b"horse"));
  }
}
