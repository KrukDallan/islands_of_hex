use std::vec;

use eframe::egui;
use egui::{pos2, Pos2};

pub fn hexagon_vertices(center: egui::Pos2, distance: f32) -> Vec<Pos2> {
    let mut vertices: Vec<Pos2> = Vec::new();
    //vertices.push(start);
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

    let offset: f32 = 10.0;

    let dist: f32 = side*2 as f32;

    for i in 1..=7 {
        let y = (dist)*i as f32;
        let mut x: f32 = 0.0;
        for j in 1..=7 {
            x = (dist) * (j as f32);
            if i >= 3 && i <= 4 {
                if j >= 3 && j <= 4 {
                    hex_centers.push(pos2(x, y));
                }
            } else {
                points.push(pos2(x, y));
            }
        }
    }
    // we will now find the remaining centers
    let mut counter = 25 - 9;
    let mut new_centers: Vec<Pos2> = vec![];
    while counter > 0 {
        for p in hex_centers.as_slice(){
            let index = nearest(*p, &points);
            match index{
                Some(x) =>{
                    if !hex_centers.contains(&points[x]){
                        counter -= 1;
                        new_centers.push(points[x]);
                        points.remove(x);
                    }
                }
                None => {
                    continue
                }
            }
        }
    }

/*     for i in new_centers.as_slice(){
        hex_centers.push(*i);
    } */

    hex_centers 
}

pub fn gen_points(side: f32) -> Vec<Pos2>{

    let mut points: Vec<Pos2> = vec![];

    let dist: f32 = side*2 as f32;

    for i in 1..=7 {
        let y = (dist)*i as f32;
        let mut x: f32 = 0.0;
        for j in 1..=7 {
            x = (dist) * (j as f32);
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
