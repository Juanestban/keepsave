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
mod widgets;
mod worker;

use eframe::egui;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions {
    renderer: eframe::Renderer::Wgpu,
    viewport: egui::ViewportBuilder::default().with_title("KeepSave").with_inner_size([1100.0, 720.0]).with_min_inner_size([720.0, 560.0]),
    ..Default::default()
  };
  eframe::run_native("KeepSave", options, Box::new(|cc| Ok(Box::new(app::KeepSaveApp::new(cc)))))
}
