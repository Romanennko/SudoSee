#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod app_data;
mod ui;

use app::SudoSeeApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
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
