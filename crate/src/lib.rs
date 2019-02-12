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
pub struct HexagonTile {
    player_id: u8,
    player_colour: Colour,
    dice: u8,
}

impl HexagonTile {
    pub fn new(player_id: u8, player_colour: Colour, dice: u8) -> Self {
        HexagonTile { player_id, player_colour, dice }
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
    /*
    pub fn new(rows: u32, columns: u32, h_radius: u32) -> Self {
        let total = rows * columns;
        let mut tiles: Vec<HexagonTile> = Vec::with_capacity(total as usize);
        for c in 0..columns {
            for r in 0..rows {
                let ht = HexagonTile::new(
                tiles.push(ht);
            }
        }

        HexagonGrid { rows, columns, tiles }        
    }
     */

    pub fn blank(rows: u32, columns: u32) -> Self {
        let mut tiles: Vec<HexagonTile> = Vec::new();
        for _ in 0..(rows * columns) {
            tiles.push(HexagonTile::new(1, Colour::Blue, 1));
        }

        HexagonGrid { rows, columns, tiles }
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    pub fn tiles(&self) -> *const HexagonTile {
        self.tiles.as_ptr()
    }
}

impl Default for HexagonGrid {
    fn default() -> Self {
        HexagonGrid {
            rows: 1,
            columns: 1,
            tiles: vec![HexagonTile::new(1, Colour::Blue, 1)],
        }
    }
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

