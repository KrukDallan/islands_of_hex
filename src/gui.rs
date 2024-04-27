use eframe::egui;
use egui::{pos2, Pos2};

pub fn hexagon_vertices(center: egui::Pos2, distance: f32) -> Vec<Pos2> {
    let mut vertices: Vec<Pos2> = Vec::new();
    //vertices.push(start);
    let angle_deg: f64 = 60.0; // Angle between two consecutive vertices in degrees

    for i in 0..6 {
        let angle_rad = angle_deg.to_radians() * i as f64;
        let x = center[0] + distance * angle_rad.cos() as f32;
        let y = center[1] + distance * angle_rad.sin() as f32;
        vertices.push(pos2(x, y));
    }

    vertices
}

/* fn thrash_bin() {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 2.0);
    let (mut rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    rect = rect.translate(vec2(100.0, 100.0));
    let square = egui::Shape::rect_stroke(
        rect,
        egui::Rounding::default(),
        (10.0, egui::Color32::WHITE),
    );
    let mut point = egui::pos2(10.0, 10.0);
    let line =
        egui::Shape::line_segment([1.0 * point, 100.0 * point], (10.0, egui::Color32::WHITE));
} */
