#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
//#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::pos2;

mod gui;
fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 640.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Islands of hex", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );

            let mut shapes: Vec<egui::Shape> = vec![];
            let hex_centers = gui::gen_map(2.0);
            for h in hex_centers.as_slice(){
                let hexagon = egui::Shape::convex_polygon(gui::hexagon_vertices(*h, 50.0), egui::Color32::TRANSPARENT, (1.0, egui::Color32::WHITE));
            shapes.push(hexagon);
            }
            painter.extend(shapes);
        });
    })
}



