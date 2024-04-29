use std::{
    ops::{Deref, DerefMut},
    vec,
};

use eframe::egui;
use egui::{ahash::HashMap, emath::Numeric, pos2, Color32, Pos2};

#[derive(Default)]
pub struct GameMap<'a> {
    hexagons: Vec<HexagonTile<'a>>,
    turn: bool,
    player1_score: u8,
    player2_score: u8,
}

impl<'a> GameMap<'a> {
    pub fn new() -> GameMap<'a> {
        GameMap {
            hexagons: vec![],
            turn: false,
            player1_score: 0,
            player2_score: 0,
        }
    }

    pub fn build(&mut self, side: f32) {
        let mut hex_centers: Vec<Pos2> = vec![];

        let distance: f32 = side * 2.0;

        for i in -2..=2 {
            let mut offset: f32 = side;
            let y = 320.0 + distance * i as f32;
            let mut x: f32 = 0.0;
            for j in -2..=2 {
                x = 240.0 + distance * j as f32;

                let tile = HexagonTile::build(pos2(x, y - offset), Color32::TRANSPARENT);
                hex_centers.push(pos2(x, y - offset));
                self.hexagons.push(tile);
                offset += side;
            }
        }
    }

    pub fn get_centers(&self) -> Vec<Pos2> {
        let mut centers: Vec<Pos2> = vec![];

        for i in self.hexagons.as_slice() {
            centers.push(i.center);
        }

        centers
    }

    pub fn get_tile_color(&mut self, center: &Pos2) -> Color32 {
        for h in self.hexagons.as_slice() {
            if h.center == *center {
                return h.color;
            }
        }
        return Color32::TRANSPARENT;
    }

    pub fn change_tile_color(&mut self, center: &Pos2, color: Color32) {
        for h in self.hexagons.as_mut_slice() {
            if h.center == *center {
                h.change_color(color);
                self.turn = !self.turn;
                break;
            }
        }
    }

    pub fn get_turn(&self) -> bool {
        self.turn
    }

    pub fn get_player1_score(&self) -> u8 {
        self.player1_score
    }

    pub fn get_player2_score(&self) -> u8 {
        self.player2_score
    }

    pub fn update_scores(&mut self) {
        let mut islands: Vec<Island> = vec![];
        let mut checked: Vec<Pos2> = vec![];

        for h in self.hexagons.as_slice() {
            let index = self
                .hexagons
                .iter()
                .position(|e| e.center == h.center)
                .unwrap();

            let mut island: Island = Island {
                centers: vec![],
                color: Color32::TRANSPARENT,
            };

            if !checked.contains(&h.center) {
                checked.push(h.center);
                if h.color == Color32::GREEN {
                    island.color = Color32::GREEN;
                } else if h.color == Color32::LIGHT_RED {
                    island.color = Color32::LIGHT_RED;
                } else {
                    continue;
                }
                island.centers.push(h.center);

                // check neighbors
                if index < self.hexagons.as_slice().len() - 1
                    && (index + 1) % 5 != 0
                    && self.hexagons.as_slice().get(index + 1).unwrap().color == h.color
                {
                    
                    island
                        .centers
                        .push(self.hexagons.as_slice().get(index + 1).unwrap().center);
                    //checked.push(self.hexagons.as_slice().get(index + 1).unwrap().center);
                }

                if index < 20 && self.hexagons.as_slice().get(index + 5).unwrap().color == h.color {
                    
                    island
                        .centers
                        .push(self.hexagons.as_slice().get(index + 5).unwrap().center);
                    //checked.push(self.hexagons.as_slice().get(index + 5).unwrap().center);
                }

                if index < 19
                    && (index + 1) % 5 != 0
                    && self.hexagons.as_slice().get(index + 6).unwrap().color == h.color
                {
                  
                    island
                        .centers
                        .push(self.hexagons.as_slice().get(index + 6).unwrap().center);
                    //checked.push(self.hexagons.as_slice().get(index + 6).unwrap().center);
                }

                if index > 4 && self.hexagons.as_slice().get(index - 5).unwrap().color == h.color {
                    
                    island
                        .centers
                        .push(self.hexagons.as_slice().get(index - 5).unwrap().center);
                    //checked.push(self.hexagons.as_slice().get(index - 5).unwrap().center);
                }

                if index > 5
                    && index % 5 != 0
                    && self.hexagons.as_slice().get(index - 6).unwrap().color == h.color
                {
                    
                    island
                        .centers
                        .push(self.hexagons.as_slice().get(index - 6).unwrap().center);
                    //checked.push(self.hexagons.as_slice().get(index - 6).unwrap().center);
                }

                // check if the current tile or any of its neighbor is already present in one of the isle in isles
                let mut to_be_added: Vec<Pos2> = vec![];

                let mut idx: Vec<usize> = vec![];
                let mut should_change: bool = false;

                'outer: for i in islands.as_slice() {
                    for h in island.centers.as_slice() {
                        if i.centers.contains(&h) {
                            if !idx.contains(
                                &islands.iter().position(|e| e.centers == i.centers).unwrap(),
                            ) {
                                if should_change == true {
                                    idx.push(
                                        islands
                                            .iter()
                                            .position(|e| e.centers == i.centers)
                                            .unwrap()
                                            - 1,
                                    );
                                } else {
                                    idx.push(
                                        islands
                                            .iter()
                                            .position(|e| e.centers == i.centers)
                                            .unwrap(),
                                    );
                                }
                            }

                            for element in i.centers.as_slice() {
                                to_be_added.push(*element);
                            }
                            should_change = true;
                            continue 'outer;
                        }
                    }
                }

                for e in to_be_added.as_slice() {
                    if !island.centers.contains(e) {
                        island.centers.push(*e);
                    }
                }

                if !islands.is_empty() && should_change {
                    for i in idx.as_slice() {
                        islands.remove(*i);
                    }
                }

                islands.push(island);
            }
        }
        self.player1_score = 0;
        self.player2_score = 0;
        for i in islands.as_slice() {
            if i.color == Color32::GREEN {
                self.player1_score += 1;
            } else if i.color == Color32::LIGHT_RED {
                self.player2_score += 1;
            }
        }
        //println!("Islands: {:?}", islands);
    }
}

pub struct HexagonTile<'a> {
    center: Pos2,
    color: Color32,
    neighbors: Vec<&'a mut HexagonTile<'a>>,
    checked: bool,
}

impl<'a> HexagonTile<'a> {
    pub fn build(center: Pos2, color: Color32) -> HexagonTile<'a> {
        HexagonTile {
            center,
            color,
            neighbors: vec![],
            checked: false,
        }
    }

    pub fn change_color(&mut self, new_color: Color32) {
        self.color = new_color;
    }
}

#[derive(Debug)]
pub struct Island {
    centers: Vec<Pos2>,
    color: Color32,
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
