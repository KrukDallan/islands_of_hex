#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
                                                                   //#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{pos2, vec2, Color32, Pos2};
use gui::GameMap;

mod gui;
fn main() -> Result<(), eframe::Error> {
    let mut game_map = GameMap::new();

    let side: f32 = 15.0;
    let mut hex_centers = gui::gen_map(side);

    game_map.build(&hex_centers);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 640.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Islands of hex", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            for h in hex_centers.as_mut_slice() {
                let mut color = game_map.get_tile_color(h);
                hexagon_ui(ui, &mut game_map, h, &mut color, &side);
            }
            //painter.extend(shapes);
        });
    })
}

pub fn hexagon_ui(
    ui: &mut egui::Ui,
    game_map: &mut GameMap,
    center: &mut Pos2,
    color: &mut Color32,
    side: &f32,
) -> egui::Response {
    let desired_size = vec2(*side, *side);
    let mut response = ui.allocate_rect(
        egui::Rect::from_center_size(*center, desired_size),
        egui::Sense::click(),
    );
    //let (mut rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        response.mark_changed();
        if *color == Color32::TRANSPARENT {
            *color = Color32::GOLD;
        } else if *color == Color32::GOLD {
            *color = Color32::GREEN;
        } else {
            *color = Color32::GOLD;
        }
        game_map.change_tile_color(center, color);
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, ""));

    let hexagon = egui::Shape::convex_polygon(
        gui::hexagon_vertices(*center, *side),
        *color,
        (1.0, egui::Color32::WHITE),
    );
    ui.painter().add(hexagon);
    ui.end_row();

    return response;
}
