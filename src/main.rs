use eframe::{egui, Theme};
use egui::{vec2, Color32, Pos2, RichText, Visuals};
use game_map::GameMap;
use utils::hexagon_vertices;

mod game_map;
mod hexagon_tile;
mod utils;

fn main() -> Result<(), eframe::Error> {
    let mut game_map = GameMap::new();

    let side: f32 = 14.0;

    game_map.build(side);

    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 640.0]),
        ..Default::default()
    };
    options.default_theme = Theme::Dark;

    eframe::run_simple_native("Islands of hex", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(Visuals::dark());
            let player_label = if game_map.get_turn() {
                "Player 2"
            } else {
                "Player 1"
            };

            ui.vertical_centered(|ui| {
                ui.add(|ui: &mut egui::Ui| {
                    ui.label(RichText::new(player_label).color(Color32::WHITE))
                })
            });

            ui.add_space(50.0);

            if game_map.get_winner() != String::from(""){
                ui.vertical_centered(|ui| {
                    ui.add(|ui: &mut egui::Ui| {
                        let caption = String::from("Winner: ") + &game_map.get_winner();
                        ui.label(RichText::new(caption).color(Color32::WHITE))
                    })
                });
            }

            for h in game_map.get_centers().as_mut_slice() {
                let mut color = game_map.get_tile_color(h);
                hexagon_ui(ui, &mut game_map, h, &mut color, &side);
            }
            ui.add_space(100.0);

            if game_map.get_winner() != String::from("") || game_map.is_full(){
                ui.vertical_centered(|ui|{
                    ui.add(|ui: &mut egui::Ui| {
                        let response = ui.button(RichText::new("Play again").color(Color32::WHITE));
                        if response.clicked() {
                            game_map.reset();
                            game_map.build(side);
                        }
                        
                        return response;
                    })
                });
            } 

            ui.add_space(100.0);

            let score = String::from("Player 1:  ")
                + game_map.get_player1_score().to_string().as_str()
                + "  |  "
                + "Player 2:  "
                + game_map.get_player2_score().to_string().as_str();
            ui.vertical_centered(|ui| {
                ui.add(|ui: &mut egui::Ui| ui.label(RichText::new(score).color(Color32::WHITE)))
            });

            
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

    if response.clicked() && game_map.get_winner() == String::from(""){
        response.mark_changed();
        if *color == Color32::TRANSPARENT {
            if game_map.get_turn() {
                *color = Color32::LIGHT_RED;
            } else {
                *color = Color32::GREEN;
            }

            game_map.change_tile_color(center, *color);
            game_map.update_scores();
        }
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, true, ""));

    let hexagon = egui::Shape::convex_polygon(
        hexagon_vertices(*center, *side),
        *color,
        (1.0, egui::Color32::WHITE),
    );
    ui.painter().add(hexagon);
    ui.end_row();

    response
}
