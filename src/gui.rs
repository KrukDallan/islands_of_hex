use std::vec;

use eframe::egui;
use egui::{ahash::HashMap, pos2, Color32, Pos2};

pub struct GameMap {
    hexagons: Vec<HexagonTile>,
    turn: bool,
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            hexagons: vec![],
            turn: false,
        }
    }

    pub fn build(&mut self, centers: &Vec<Pos2>) {
        let mut hexagons: Vec<HexagonTile> = vec![];

        for c in centers.as_slice() {
            hexagons.push(HexagonTile {
                center: *c,
                color: Color32::TRANSPARENT,
            });
        }

        self.hexagons = hexagons;
    }

    pub fn get_tile_color(&mut self, center: &Pos2) -> Color32 {
        for h in self.hexagons.as_slice() {
            if h.center == *center {
                return h.color;
            }
        }
        return Color32::TRANSPARENT;
    }

    pub fn change_tile_color(&mut self, center: &Pos2, color: &Color32) {
        for h in self.hexagons.as_mut_slice() {
            if h.center == *center {
                h.change_color(color);
                self.turn = !self.turn;
            }
        }
    }

    pub fn get_turn(&self) -> bool {
        self.turn
    }
}

pub struct HexagonTile {
    center: Pos2,
    color: Color32,
}

impl HexagonTile {
    pub fn build(center: Pos2, color: Color32) -> HexagonTile {
        HexagonTile {
            center: center,
            color: color,
        }
    }

    pub fn change_color(&mut self, new_color: &Color32) {
        self.color = *new_color;
    }
}

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

pub fn gen_map(side: f32) -> Vec<Pos2> {
    let mut hex_centers: Vec<Pos2> = vec![];

    let mut points: Vec<Pos2> = vec![];

    let margin: f32 = side / 4.0;

    let dist: f32 = side * 2 as f32;

    for i in -2..=2 {
        let mut offset: f32 = side;
        let y = (320.0 - 2.0 * dist) + (dist) * i as f32;
        let mut x: f32 = 0.0;
        for j in -2..=2 {
            x = (240.0 - 2.0 * dist) + (dist + margin) * (j as f32);

            hex_centers.push(pos2(x, y - offset));
            offset = offset + side;
        }
    }

    hex_centers
}

pub fn gen_points(side: f32) -> Vec<Pos2> {
    let mut points: Vec<Pos2> = vec![];

    let dist: f32 = side * 2 as f32;

    for i in 1..=7 {
        let y = 320.0 + (dist) * i as f32;
        let mut x: f32 = 0.0;
        for j in 1..=7 {
            x = 240.0 + (dist) * (j as f32);
            points.push(pos2(x, y));
        }
    }

    points
}

/// Returns the index of the nearest point to 'target'
pub fn nearest(target: Pos2, group: &Vec<Pos2>) -> Option<usize> {
    let mut index: Option<usize> = None;

    for p in group.as_slice() {
        if target.distance(*p) < target.distance(group[index.unwrap_or(0)]) {
            index = group.iter().position(|x| *x == *p);
        }
    }

    index
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

// TODO:
// per modalità griglia 5x5
// genera griglia 7x7 di punti equidistanti fra loro ad una distanza di (lato_esagono + offset) (o altra dimensione, sicuramente maggiore di 5x5)
// prendi la griglia 3x3 centrale, e salva la pos di quei punti
// partendo da quei 9, trova i restanti 16 usando greedy+random approach (ogni punto "esterno" già scelto avrà uno o due punti non scelti più vicini a lui)
// genera esagoni partendo dai punti
