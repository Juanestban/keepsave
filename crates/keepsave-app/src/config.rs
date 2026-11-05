//! User settings in `config.toml` under the platform config dir, e.g.
//! `%APPDATA%\KeepSave\config.toml`. Never holds secrets.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

const MAX_RECENT_VAULTS: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Language code (`en`, `es`). `None` follows the OS.
    pub language: Option<String>,
    /// Most recently used first.
    pub recent_vaults: Vec<PathBuf>,
    /// Lock after this long without input; 0 turns it off.
    pub lock_after_minutes: u32,
    pub lock_on_minimize: bool,
    pub lock_on_os_sleep: bool,
    /// Clear copied secrets after this long; 0 keeps them.
    pub clipboard_clear_seconds: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: None,
            recent_vaults: Vec::new(),
            lock_after_minutes: 5,
            lock_on_minimize: true,
            lock_on_os_sleep: true,
            clipboard_clear_seconds: 15,
        }
    }
}

impl Config {
    pub fn default_path() -> Option<PathBuf> {
        BaseDirs::new().map(|dirs| dirs.config_dir().join("KeepSave").join("config.toml"))
    }

    /// Loads the config, falling back to defaults when it is missing or
    /// unreadable, so a broken file never keeps the app from starting.
    pub fn load_from(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save_to(&self, path: &Path) -> io::Result<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let text = toml::to_string_pretty(self).map_err(io::Error::other)?;
        let mut tmp_name = path.as_os_str().to_owned();
        tmp_name.push(".tmp");
        fs::write(&tmp_name, text)?;
        fs::rename(&tmp_name, path)
    }

    /// Moves `path` to the front of the recent list.
    pub fn remember_vault(&mut self, path: &Path) {
        self.recent_vaults.retain(|known| known != path);
        self.recent_vaults.insert(0, path.to_owned());
        self.recent_vaults.truncate(MAX_RECENT_VAULTS);
    }

    pub fn forget_vault(&mut self, path: &Path) {
        self.recent_vaults.retain(|known| known != path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("config.toml");
        assert_eq!(Config::load_from(&path), Config::default());

        let mut config = Config {
            language: Some("es".into()),
            ..Config::default()
        };
        config.remember_vault(Path::new("C:/vaults/personal.kdbx"));
        config.save_to(&path).unwrap();
        assert_eq!(Config::load_from(&path), config);
    }

    #[test]
    fn missing_keys_take_their_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(
            &path,
            "language = \"es\"
lock_after_minutes = 2
",
        )
        .unwrap();
        let config = Config::load_from(&path);
        assert_eq!(config.language.as_deref(), Some("es"));
        assert_eq!(config.lock_after_minutes, 2);
        assert!(config.lock_on_minimize);
        assert_eq!(config.clipboard_clear_seconds, 15);
    }

    #[test]
    fn corrupt_file_falls_back_to_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, "recent_vaults = 42").unwrap();
        assert_eq!(Config::load_from(&path), Config::default());
    }

    #[test]
    fn recent_vaults_are_unique_ordered_and_capped() {
        let mut config = Config::default();
        for i in 0..10 {
            config.remember_vault(Path::new(&format!("v{i}.kdbx")));
        }
        config.remember_vault(Path::new("v5.kdbx"));
        assert_eq!(config.recent_vaults.len(), MAX_RECENT_VAULTS);
        assert_eq!(config.recent_vaults[0], Path::new("v5.kdbx"));
        assert_eq!(
            config
                .recent_vaults
                .iter()
                .filter(|p| *p == Path::new("v5.kdbx"))
                .count(),
            1
        );

        config.forget_vault(Path::new("v5.kdbx"));
        assert!(
            !config
                .recent_vaults
                .iter()
                .any(|p| p == Path::new("v5.kdbx"))
        );
    }
}
