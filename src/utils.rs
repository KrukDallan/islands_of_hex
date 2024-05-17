use egui::{pos2, Pos2};

pub fn hexagon_vertices(center: egui::Pos2, distance: f32) -> Vec<Pos2> {
    let mut vertices: Vec<Pos2> = Vec::new();
    let angle_deg: f32 = 60.0; // Angle between two consecutive vertices in degrees

    for i in 0..6 {
        let angle_rad = angle_deg.to_radians() * i as f32;
        let x = center[0] + distance * angle_rad.cos() as f32;
        let y = center[1] + distance * angle_rad.sin() as f32;
        vertices.push(pos2(x, y));
    }

    vertices
}
