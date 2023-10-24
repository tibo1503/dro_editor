#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, Theme};

mod app;
use app::DROEditor;

mod worker;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        default_theme: Theme::Dark,
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        "DRO Dofe's Editor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<DROEditor>::default()
        }),
    )
}