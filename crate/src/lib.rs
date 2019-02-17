use std::iter::Iterator;

use wasm_bindgen::prelude::*;

use dicey_dice::{session, game};

mod utils;
pub mod hex;
pub mod grid;
pub mod primitive;

pub use self::primitive::Point;
pub use self::grid::Tessellation;

/// Re-export for debug purposes.
pub use self::hex::pointy_hex_corner;

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

/// Run a game handling the web input/output with the `Session`.
#[wasm_bindgen]
pub struct Game {
    session: session::Session,
    template: grid::Template,

    /// There will always be a tessellation. It is a bug if this field is left `None`.
    tessellation: Option<Tessellation>,
}

impl Game {
    fn new(session: session::Session, template: grid::Template) -> Self {
        let tessellation = Some(grid::generate_tessellation(
            &template, session.current_turn().board(),
        ));
        Game { session, template, tessellation }
    }    
}

#[wasm_bindgen]
impl Game {
    pub fn tessellation(&self) -> Tessellation {
        self.tessellation.clone().unwrap()
    }
}

/// Test start a `Game` using one of the pre-baked `Boards`.
#[wasm_bindgen]
pub fn game_3x1_init(board_top_left: Point, hex_radius: u32) -> Game {
    let template = grid::generate_template(3, 1, board_top_left, hex_radius);
    let board = game::canned_3x1_start01();
    let session = session::Setup::new()
        .set_board(board)
        .session()
        .expect("Invalid session initialization");

    Game::new(session, template)
}

/*
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
*/
