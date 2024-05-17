use std::{cell::RefCell, rc::Rc, vec};

use eframe::egui;
use egui::{pos2, Color32, Pos2};

use crate::hexagon_tile::HexagonTile;

pub struct GameMap {
    hexagons: RefCell<Vec<Rc<HexagonTile>>>,
    turn: bool,
    player1_score: u8,
    player2_score: u8,
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            hexagons: RefCell::new(vec![]),
            turn: false,
            player1_score: 0,
            player2_score: 0,
        }
    }

    pub fn build(&mut self, side: f32) {
        let mut hex_centers: Vec<Pos2> = vec![];
        let rc_tiles: RefCell<Vec<Rc<HexagonTile>>> = RefCell::new(vec![]);

        let distance: f32 = side * 2.0;
        let mut id: usize = 0;

        for i in -2..=2 {
            let mut offset: f32 = side;
            let y = 320.0 + distance * i as f32;
            let mut x: f32;
            for j in -2..=2 {
                x = 240.0 + distance * j as f32;

                let tile = HexagonTile::build(id, pos2(x, y - offset), Color32::TRANSPARENT);
                id += 1;
                hex_centers.push(pos2(x, y - offset));
                self.hexagons.borrow_mut().push(Rc::new(tile.clone()));
                rc_tiles.borrow_mut().push(Rc::new(tile.clone()));
                //self.hexagons.insert(id, tile);
                offset += side;
            }
        }

        // neighbors
        let binding = self.hexagons.borrow_mut();
        let slice = binding.as_slice();
        for tile in slice {
            let index = tile.id as usize;

            if index > 0 && index % 5 != 0 {
                tile.neighbors
                    .borrow_mut()
                    .push(Rc::downgrade(&slice[index - 1]));
            }

            if index < 25 - 1 && (index + 1) % 5 != 0 {
                tile.neighbors
                    .borrow_mut()
                    .push(Rc::downgrade(&slice[index + 1]));
            }

            if index < 20 {
                tile.neighbors
                    .borrow_mut()
                    .push(Rc::downgrade(&slice[index + 5]));
            }

            if index < 19 && (index + 1) % 5 != 0 {
                tile.neighbors
                    .borrow_mut()
                    .push(Rc::downgrade(&slice[index + 6]));
            }

            if index > 4 {
                tile.neighbors
                    .borrow_mut()
                    .push(Rc::downgrade(&slice[index - 5]));
            }

            if index > 5 && index % 5 != 0 {
                tile.neighbors
                    .borrow_mut()
                    .push(Rc::downgrade(&slice[index - 6]));
            }
        }
    }

    pub fn get_centers(&self) -> Vec<Pos2> {
        let mut centers: Vec<Pos2> = vec![];

        for tile in self.hexagons.borrow().as_slice() {
            centers.push(tile.center);
        }

        centers
    }

    pub fn get_tile_color(&mut self, center: &Pos2) -> Color32 {
        for tile in self.hexagons.borrow_mut().as_mut_slice() {
            if tile.center == *center {
                let color = tile.color.take();
                *tile.color.borrow_mut() = color;
                return color;
            }
        }
        Color32::TRANSPARENT
    }

    pub fn change_tile_color(&mut self, center: &Pos2, color: Color32) {
        for tile in self.hexagons.borrow_mut().as_mut_slice() {
            if tile.center == *center {
                *tile.color.borrow_mut() = color;
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

        let mut checked: Vec<usize> = vec![];

        for tile in self.hexagons.borrow().as_slice() {
            let color = tile.color.clone().take();

            let mut island: Island = Island {
                ids: vec![],
                color: Color32::TRANSPARENT,
            };

            if !checked.contains(&tile.id) {
                checked.push(tile.id);
                if color == Color32::GREEN {
                    island.color = Color32::GREEN;
                } else if color == Color32::LIGHT_RED {
                    island.color = Color32::LIGHT_RED;
                } else {
                    continue;
                }
                island.ids.push(tile.get_id());

                for n in tile.neighbors.borrow().as_slice() {
                    println!("n: {:?}", n.upgrade());
                    let n_color = n.upgrade().unwrap().get_color();
                    if n_color == color {
                        checked.push(n.upgrade().unwrap().get_id());
                        island.ids.push(n.upgrade().unwrap().get_id());
                    }
                }

                // Check if other islands contain any of the id of the current island.
                // If so, we will store the ids that are not in common between the islands that share at least an id.
                let iterator = islands.iter();
                let mut indexes: Vec<usize> = vec![];
                let mut index: usize = 0;
                // Ids stored here
                let mut ids_to_add: Vec<usize> = vec![];
                for i in iterator {
                    for id in island.ids.as_slice() {
                        if i.ids.contains(id) {
                            for i_id in i.ids.as_slice() {
                                if !ids_to_add.contains(i_id) {
                                    ids_to_add.push(*i_id);
                                }
                            }
                            indexes.push(index - indexes.len());
                        }
                    }
                    index += 1;
                }
                for index in indexes.as_slice() {
                    islands.remove(*index);
                }

                for new_id in ids_to_add.as_slice() {
                    island.ids.push(*new_id);
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
    }
}

#[derive(Debug)]
pub struct Island {
    ids: Vec<usize>,
    color: Color32,
}
