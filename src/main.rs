#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod app_data;
mod ui;

use app::SudoSeeApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    // Embed the icon into the executable to load at runtime for the taskbar
    let icon = include_bytes!("../icon.ico");
    let image = image::load_from_memory(icon)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let icon_data = eframe::egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "SudoSee",
        options,
        Box::new(|cc| {
            // Force dark theme as user likes visually pleasing apps
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(SudoSeeApp::default()))
        }),
    )
}
