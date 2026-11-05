//! Tells the app when the OS goes to sleep or the session is locked, so the
//! vault can lock too. Only Windows reports these for now; elsewhere the
//! inactivity timer covers it.

use std::sync::atomic::{AtomicBool, Ordering};

use eframe::egui;

static LOCK_REQUESTED: AtomicBool = AtomicBool::new(false);

/// True once after the OS went to sleep or locked the session.
pub fn take_lock_request() -> bool {
  LOCK_REQUESTED.swap(false, Ordering::Relaxed)
}

/// Starts listening in a background thread. Safe to call more than once.
pub fn start(ctx: egui::Context) {
  #[cfg(windows)]
  windows::start(ctx);
  #[cfg(not(windows))]
  let _ = ctx;
}

#[cfg(windows)]
mod windows {
  use std::{ptr, sync::OnceLock, thread};

  use eframe::egui;
  use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    System::{
      LibraryLoader::GetModuleHandleW,
      RemoteDesktop::{NOTIFY_FOR_THIS_SESSION, WTSRegisterSessionNotification},
    },
    UI::WindowsAndMessaging::{
      CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, MSG, PBT_APMSUSPEND, RegisterClassW, TranslateMessage, WM_POWERBROADCAST,
      WM_WTSSESSION_CHANGE, WNDCLASSW, WTS_SESSION_LOCK,
    },
  };

  use super::{LOCK_REQUESTED, Ordering};

  static CTX: OnceLock<egui::Context> = OnceLock::new();

  pub fn start(ctx: egui::Context) {
    if CTX.set(ctx).is_err() {
      return;
    }
    // If the thread can't start, the inactivity timer still applies.
    let _ = thread::Builder::new().name("keepsave-session".into()).spawn(run);
  }

  fn run() {
    let class_name = wide("KeepSaveSessionWatcher");
    // SAFETY: plain Win32 calls with valid, NUL-terminated strings that
    // outlive the window; the message loop runs on this thread.
    unsafe {
      let instance = GetModuleHandleW(ptr::null());
      let mut class: WNDCLASSW = std::mem::zeroed();
      class.lpfnWndProc = Some(window_proc);
      class.hInstance = instance;
      class.lpszClassName = class_name.as_ptr();
      if RegisterClassW(&class) == 0 {
        return;
      }
      // A hidden top-level window: message-only windows don't receive
      // the power broadcast.
      let hwnd = CreateWindowExW(0, class_name.as_ptr(), class_name.as_ptr(), 0, 0, 0, 0, 0, ptr::null_mut(), ptr::null_mut(), instance, ptr::null());
      if hwnd.is_null() {
        return;
      }
      WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_THIS_SESSION);

      let mut msg: MSG = std::mem::zeroed();
      while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
      }
    }
  }

  unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let event = wparam as u32;
    let lock = (msg == WM_POWERBROADCAST && event == PBT_APMSUSPEND) || (msg == WM_WTSSESSION_CHANGE && event == WTS_SESSION_LOCK);
    if lock {
      LOCK_REQUESTED.store(true, Ordering::Relaxed);
      if let Some(ctx) = CTX.get() {
        ctx.request_repaint();
      }
    }
    // SAFETY: forwards the arguments Windows gave us unchanged.
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
  }

  fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
  }
}
