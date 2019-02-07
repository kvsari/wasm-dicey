use std::iter::Iterator;

use wasm_bindgen::prelude::*;

use dicey_dice::hexagon::Grid;
//use dicey_dice::game::Hold;

mod utils;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Yo!");
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! jslog {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HexagonTile {
    pub center_x: u32,
    pub center_y: u32,
    pub radius: u32,
    // Add pointy or flat.
    // Add dice count.
}

impl HexagonTile {
    pub fn new(center_x: u32, center_y: u32, radius: u32) -> Self {
        HexagonTile { center_x, center_y, radius }
    }
}

/// A rectangular grid of `HexagonTile`s.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct HexagonGrid {
    rows: u32,
    columns: u32,    
    tiles: Vec<HexagonTile>,
}

#[wasm_bindgen]
impl HexagonGrid {
    pub fn new(rows: u32, columns: u32, h_radius: u32) -> Self {
        let total = rows * columns;
        let mut tiles: Vec<HexagonTile> = Vec::with_capacity(total as usize);
        for c in 0..columns {
            let y = c * 10;
            for r in 0..rows {
                let x = r * 10;
                let ht = HexagonTile::new(x, y, 5);
                tiles.push(ht);
            }
        }

        HexagonGrid { rows, columns, tiles }        
    }

    pub fn tiles(&self) -> Vec<HexagonTile> {
        self.tiles.clone()
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    Black = 0,
    White = 1,
    LightBlue = 2,
    Blue = 3,
    LightGreen = 4,
    Gree = 5,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Extra {
    colour: Colour,
    selected: bool,
}

impl Extra {
    fn new(colour: Colour) -> Self {
        Extra {
            colour,
            selected: false,
        }
    }    

    fn toggle(&mut self) {
        if self.selected {
            self.selected = false;
        } else {
            self.selected = true;
        }
    }

    fn colour(&self) -> Colour {
        self.colour
    }

    fn selected(&self) -> bool {
        self.selected
    }
}

/*
/// View wrapping the `Grid<Hold>` data structure and providing a view into it.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ViewGrid {
    rows: u32,
    columns: u32,
    grid: Grid<Hold>,
    extra: Vec<Extra>,
}

impl ViewGrid {
    pub fn new(grid: Grid<Hold>) -> Self {
        let shape
    }
}
*/
