#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([480.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "NexTech - POS, Inventory & Financial Control",
        options,
        Box::new(|cc| Ok(Box::new(nextech::App::new(cc)))),
    )
}
