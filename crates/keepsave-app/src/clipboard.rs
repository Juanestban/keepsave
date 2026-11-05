//! Copies to the system clipboard and clears the value again after a
//! timeout, but only if the clipboard still holds what we put there.

use std::time::{Duration, Instant};

use zeroize::Zeroizing;

pub struct SecureClipboard {
    clipboard: Option<arboard::Clipboard>,
    pending: Option<Pending>,
}

struct Pending {
    value: Zeroizing<String>,
    clear_at: Instant,
}

impl SecureClipboard {
    pub fn new() -> Self {
        Self {
            clipboard: arboard::Clipboard::new().ok(),
            pending: None,
        }
    }

    /// Copies `text`, to be cleared after `clear_after` (`None` keeps it).
    /// On Windows the value stays out of the clipboard history and cloud
    /// clipboard. Returns false if the clipboard is unavailable.
    pub fn copy(&mut self, text: &str, clear_after: Option<Duration>) -> bool {
        // A new copy replaces whatever we were waiting to clear.
        self.pending = None;
        let Some(clipboard) = &mut self.clipboard else {
            return false;
        };
        let set = clipboard.set();
        #[cfg(windows)]
        let set = {
            use arboard::SetExtWindows;
            set.exclude_from_history().exclude_from_cloud()
        };
        if set.text(text).is_err() {
            return false;
        }
        self.pending = clear_after.map(|after| Pending {
            value: Zeroizing::new(text.to_owned()),
            clear_at: Instant::now() + after,
        });
        true
    }

    /// Clears an expired value. Returns how long until the next one expires.
    pub fn tick(&mut self) -> Option<Duration> {
        let clear_at = self.pending.as_ref()?.clear_at;
        let now = Instant::now();
        if now >= clear_at {
            self.clear();
            None
        } else {
            Some(clear_at - now)
        }
    }

    /// Clears the clipboard now if it still holds the last copied value.
    pub fn clear(&mut self) {
        let (Some(pending), Some(clipboard)) = (self.pending.take(), &mut self.clipboard) else {
            return;
        };
        let current = clipboard.get_text().map(Zeroizing::new);
        if current.is_ok_and(|current| *current == *pending.value) {
            let _ = clipboard.clear();
        }
    }
}

impl Drop for SecureClipboard {
    fn drop(&mut self) {
        self.clear();
    }
}
