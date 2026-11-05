//! KeepSave desktop app.

// No console window behind the GUI in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod backdrop;
mod background;
mod clipboard;
mod config;
mod drive;
mod fonts;
mod i18n;
mod screens;
mod session;
mod theme;
mod widgets;
mod worker;

use eframe::egui;

use config::Config;

fn main() -> eframe::Result {
  let config_path = Config::default_path();
  let config = config_path.as_deref().map(Config::load_from).unwrap_or_default();
  // Decided once: the window and renderer are created for it.
  let transparent = config.system_transparency && backdrop::supported();

  let viewport = egui::ViewportBuilder::default()
    .with_title("KeepSave")
    .with_inner_size([1100.0, 720.0])
    .with_min_inner_size([720.0, 560.0])
    .with_transparent(transparent);
  let options = eframe::NativeOptions {
    renderer: eframe::Renderer::Wgpu,
    viewport,
    wgpu_options: if transparent { backdrop::wgpu_options() } else { Default::default() },
    ..Default::default()
  };
  eframe::run_native("KeepSave", options, Box::new(move |cc| Ok(Box::new(app::KeepSaveApp::new(cc, config, config_path, transparent)))))
}
