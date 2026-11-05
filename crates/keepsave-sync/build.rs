//! Embeds the Google OAuth client, read from the environment or from `.env`
//! at the repo root. Without it the crate builds with Drive turned off.

use std::{env, fs, path::Path};

const KEYS: [&str; 2] = ["KEEPSAVE_GOOGLE_CLIENT_ID", "KEEPSAVE_GOOGLE_CLIENT_SECRET"];

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
  let dotenv = Path::new(&manifest_dir).join("../../.env");
  println!("cargo:rerun-if-changed={}", dotenv.display());
  let file = fs::read_to_string(&dotenv).unwrap_or_default();

  for key in KEYS {
    println!("cargo:rerun-if-env-changed={key}");
    let value = env::var(key).ok().filter(|value| !value.trim().is_empty()).or_else(|| dotenv_value(&file, key));
    if let Some(value) = value {
      println!("cargo:rustc-env={key}={value}");
    }
  }
}

/// The value of `key` in a `KEY=value` file, without surrounding quotes.
fn dotenv_value(file: &str, key: &str) -> Option<String> {
  file
    .lines()
    .map(str::trim)
    .filter(|line| !line.starts_with('#'))
    .filter_map(|line| line.split_once('='))
    .find(|(name, _)| name.trim() == key)
    .map(|(_, value)| value.trim().trim_matches(['"', '\'']).to_owned())
    .filter(|value| !value.is_empty())
}
