//! The optional Windows 11 Mica backdrop behind the window (Q8). The window
//! must be created transparent, with wgpu presenting through DirectComposition,
//! for the backdrop to show through.

use eframe::{egui_wgpu, wgpu};

/// Whether this OS can show the backdrop: Windows 11 22H2 or later.
pub fn supported() -> bool {
  #[cfg(windows)]
  {
    windows::build_number() >= 22621
  }
  #[cfg(not(windows))]
  {
    false
  }
}

/// Renderer settings for a transparent window: DX12 with a swapchain made
/// from a composition visual, the only kind that composes with alpha.
pub fn wgpu_options() -> egui_wgpu::WgpuConfiguration {
  let mut options = egui_wgpu::WgpuConfiguration::default();
  if let egui_wgpu::WgpuSetup::CreateNew(setup) = &mut options.wgpu_setup {
    setup.instance_descriptor.backends = wgpu::Backends::DX12;
    setup.instance_descriptor.backend_options.dx12.presentation_system = wgpu::Dx12SwapchainKind::DxgiFromVisual;
  }
  options
}

/// Turns on Mica and dark window chrome. Returns false if it couldn't.
pub fn apply(cc: &eframe::CreationContext<'_>) -> bool {
  #[cfg(windows)]
  {
    use eframe::wgpu::rwh::{HasWindowHandle, RawWindowHandle};
    let Ok(handle) = cc.window_handle() else {
      return false;
    };
    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
      return false;
    };
    windows::apply_mica(handle.hwnd.get() as _)
  }
  #[cfg(not(windows))]
  {
    let _ = cc;
    false
  }
}

#[cfg(windows)]
mod windows {
  use std::{ffi::c_void, mem};

  use windows_sys::{
    Wdk::System::SystemServices::RtlGetVersion,
    Win32::{
      Foundation::HWND,
      Graphics::Dwm::{
        DWMSBT_MAINWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DWMWA_USE_IMMERSIVE_DARK_MODE, DwmExtendFrameIntoClientArea, DwmSetWindowAttribute,
      },
      System::SystemInformation::OSVERSIONINFOW,
      UI::Controls::MARGINS,
    },
  };

  /// The real build number; `GetVersionEx` lies to unmanifested apps.
  pub fn build_number() -> u32 {
    // SAFETY: a zeroed OSVERSIONINFOW with its size set is what
    // RtlGetVersion expects, and it only writes into it.
    unsafe {
      let mut info: OSVERSIONINFOW = mem::zeroed();
      info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOW>() as u32;
      if RtlGetVersion(&mut info) == 0 { info.dwBuildNumber } else { 0 }
    }
  }

  pub fn apply_mica(hwnd: HWND) -> bool {
    let dark: i32 = 1;
    let backdrop = DWMSBT_MAINWINDOW;
    // The whole client area is "frame", so the backdrop shows wherever
    // we draw transparent pixels.
    let margins = MARGINS { cxLeftWidth: -1, cxRightWidth: -1, cyTopHeight: -1, cyBottomHeight: -1 };
    // SAFETY: `hwnd` is the live window from eframe, and each attribute
    // gets a pointer to a value of the size it expects.
    unsafe {
      DwmSetWindowAttribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE as u32, (&raw const dark).cast::<c_void>(), mem::size_of::<i32>() as u32);
      DwmExtendFrameIntoClientArea(hwnd, &margins);
      DwmSetWindowAttribute(hwnd, DWMWA_SYSTEMBACKDROP_TYPE as u32, (&raw const backdrop).cast::<c_void>(), mem::size_of::<i32>() as u32) == 0
    }
  }
}
