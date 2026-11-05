//! KeepSave desktop app.

// No console window behind the GUI in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod background;
mod clipboard;
mod config;
mod drive;
mod fonts;
mod i18n;
mod screens;
mod session;
mod theme;
mod update;
mod widgets;
mod worker;

use eframe::egui;

use config::Config;

fn main() -> eframe::Result {
  let config_path = Config::default_path();
  let config = config_path.as_deref().map(Config::load_from).unwrap_or_default();

  let viewport =
    egui::ViewportBuilder::default().with_title("KeepSave").with_inner_size([1100.0, 720.0]).with_min_inner_size([720.0, 560.0]).with_icon(icon());
  let options = eframe::NativeOptions { renderer: eframe::Renderer::Wgpu, viewport, ..Default::default() };
  eframe::run_native("KeepSave", options, Box::new(move |cc| Ok(Box::new(app::KeepSaveApp::new(cc, config, config_path)))))
}

/// The window and taskbar icon, drawn by `scripts/make_icon.py`.
fn icon() -> egui::IconData {
  const SIZE: u32 = 128;
  egui::IconData { rgba: include_bytes!("../assets/icon/keepsave-128.rgba").to_vec(), width: SIZE, height: SIZE }
}
