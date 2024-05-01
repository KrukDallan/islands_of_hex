use std::{
    cell::RefCell,
    rc::Weak,
};

use egui::{Color32, Pos2};

#[derive(Clone, Debug)]
pub struct HexagonTile {
    pub id: usize,
    pub center: Pos2,
    pub color: RefCell<Color32>,
    pub neighbors: RefCell<Vec<Weak<HexagonTile>>>,
    pub checked: bool,
}

impl PartialEq for HexagonTile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for HexagonTile {}

impl HexagonTile {
    pub fn build(id: usize, center: Pos2, color: Color32) -> HexagonTile {
        HexagonTile {
            id,
            center,
            color: RefCell::new(color),
            neighbors: RefCell::new(vec![]),
            checked: false,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_color(&self) -> Color32 {
        let color = self.color.clone();
        return color.take();
    }
}
