use eframe::egui;
use egui::{pos2, Color32, Pos2};
use std::{
    cell::RefCell,
    rc::Rc,
    vec,
};

use crate::hexagon_tile::HexagonTile;

pub struct GameMap {
    hexagons: RefCell<Vec<Rc<HexagonTile>>>,
    islands: Vec<Island>,
    turn: bool,
    player1_score: u8,
    player2_score: u8,
    winner: String,
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            hexagons: RefCell::new(vec![]),
            islands: vec![],
            turn: false,
            player1_score: 0,
            player2_score: 0,
            winner: String::from(""),
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

    pub fn reset(&mut self) {
        self.hexagons.replace(vec![]);
        self.turn = false;
        self.player1_score = 0;
        self.player2_score = 0;
        self.winner = String::from("");
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

    pub fn get_winner(&self) -> String {
        self.winner.clone()
    }

    pub fn is_full(&mut self) -> bool {
        let mut counter = 0;
        for tile in self.hexagons.borrow().as_slice() {
            if tile.get_color() != Color32::TRANSPARENT {
                counter += 1;
            }
        }
        return counter == 25;
    }

    pub fn update_scores(&mut self) {
        if self.islands.len() == 0 {
            for tile in self.hexagons.borrow().as_slice() {
                if tile.get_color() != Color32::TRANSPARENT {
                    self.islands.push(Island {
                        ids: vec![tile.get_id()],
                        color: tile.get_color(),
                    });
                    if tile.get_color() == Color32::GREEN {
                        self.player1_score += 1;
                    } else {
                        self.player2_score += 1;
                    }
                    return;
                }
            }
        } else {
            let mut checked: Vec<usize> = vec![];
          

            for i in self.islands.as_slice() {
                for id in i.ids.as_slice() {
                    checked.push(*id);
                }
            }

            for tile in self.hexagons.borrow().as_slice() {
                if !checked.contains(&tile.get_id()) {
                    checked.push(tile.get_id());

                    let color = tile.get_color();
                    if color != Color32::TRANSPARENT {
                     
                        let mut new_island: Island = Island {
                            ids: vec![],
                            color: color,
                        };
                        let mut added_by_neighbour = false;
                        // Check every neighbor n to see if n is already in an island
                        // If so, add tile to that island.
                        for n in tile.neighbors.borrow().as_slice() {
                            if n.upgrade().unwrap().get_color() == tile.get_color() {
                                for island in self.islands.as_mut_slice() {
                                    if island.ids.contains(&n.upgrade().unwrap().get_id())
                                        && !island.ids.contains(&tile.get_id())
                                    {
                                        island.ids.push(tile.get_id());
                                        added_by_neighbour = true;
                                        continue;
                                    }
                                }
                            }
                        }
                        // Check if two or more islands share the same id(s)
                        // If so, merge them
                        if added_by_neighbour {
                            let mut merged_ids: Vec<usize> = vec![];
                            let mut idxs_of_islands_to_delete: Vec<usize> = vec![];
                            for island in self.islands.as_slice() {
                                if island.ids.contains(&tile.get_id()) {
                                    idxs_of_islands_to_delete.push(
                                        self.islands
                                            .iter()
                                            .position(|r| r.ids == island.ids)
                                            .unwrap(),
                                    );
                                    for id in island.ids.as_slice() {
                                        if !merged_ids.contains(id) {
                                            merged_ids.push(*id);
                                        }
                                    }
                                }
                            }
                            if idxs_of_islands_to_delete.len() > 0 {
                                for i in 0..idxs_of_islands_to_delete.len() {
                                    self.islands
                                        .remove(idxs_of_islands_to_delete.as_slice()[i] - i);
                                }
                                for m in merged_ids.as_slice() {
                                    new_island.ids.push(*m);
                                }
                                self.islands.push(new_island);
                            }
                        } else {
                            new_island.ids.push(tile.get_id());
                            self.islands.push(new_island);
                        }
                    }
                }
            }

            //println!("{:?}", self.islands);
            self.player1_score = 0;
            self.player2_score = 0;
            for i in self.islands.as_slice() {
                if i.color == Color32::GREEN {
                    self.player1_score += 1;
                } else if i.color == Color32::LIGHT_RED {
                    self.player2_score += 1;
                }
            }

            if self.is_full(){
                if self.player1_score > self.player2_score{
                    self.winner = String::from("Player 1");
                }
                else {
                    {
                        self.winner = String::from("Player 2");
                    }
                }
            }
            

            self.check_winning_conditions();
        }
    }

    pub fn check_winning_conditions(&mut self) {
        // check islands for paths that divide the board
        for island in self.islands.as_slice() {
            if island.ids.len() >= 5 {
                if island.color == Color32::GREEN {
                    let mut top_tile_index = false;
                    let mut bottom_tile_index = false;
                    for idx in island.ids.as_slice() {
                        if *idx < 5 {
                            top_tile_index = true;
                            continue;
                        }
                        if *idx > 19 {
                            bottom_tile_index = true;
                            continue;
                        }
                    }
                    if top_tile_index && bottom_tile_index {
                        // On second thought, there is no need to build a graph
                        // If an isle contains both a top tile and a bottom tile, then
                        // (in this case) player 1 has lost.
                        self.winner = String::from("Player 2");
                        return;
                    }
                } else {
                    let mut left_tile_index = false;
                    let mut right_tile_index = false;
                    let left_idxs: Vec<usize> = vec![0, 5, 10, 15, 20];
                    let right_idxs: Vec<usize> = vec![4, 9, 14, 19, 24];
                    for idx in island.ids.as_slice() {
                        if left_idxs.contains(idx) {
                            left_tile_index = true;
                            continue;
                        }
                        if right_idxs.contains(idx) {
                            right_tile_index = true;
                            continue;
                        }
                    }
                    if left_tile_index && right_tile_index {
                        // On second thought, there is no need to build a graph
                        // If an isle contains both a top tile and a bottom tile, then
                        // (in this case) player 2 has lost.
                        self.winner = String::from("Player 1");
                        return;
                    }
                }
            }
        }
    }

   
}

#[derive(Debug)]
pub struct Island {
    ids: Vec<usize>,
    color: Color32,
}
