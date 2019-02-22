
use wasm_bindgen::prelude::*;

use dicey_dice::{session, game};

mod utils;
pub mod hex;
pub mod dice;
pub mod grid;
pub mod primitive;

pub use self::primitive::Point;
pub use self::grid::Tessellation;
pub use self::dice::{DiceTemplate, Position};

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

    /// Index of selected hex if any.
    selected: Option<usize>,
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

    /// This method will take a pixel coordinate with (0, 0) being the center point of the
    /// first hexagon Cube(0, 0, 0). If a valid (for the current player) hex is selected,
    /// this state will be stored in the `Tessellation`. Then if the player enters a second
    /// follow up valid click then the game state will advance. True is returned in this
    /// case. False if the game state wasn't advanced.
    pub fn select_hex_by_pixel(&mut self, pixel: Point) -> bool {
        // 1. Get the coordinate from the pixel point.
        let coordinate = pixel.hexagon_axial(self.template.radius());

        // 2. Get the hexagon index.
        let board = self.session.current_turn().board();
        let hex_index = match board.grid().fetch_index(coordinate) {
            Ok(index) => index,
            Err(e) => {
                jslog!("Invalid hexagon coordinate: {}", &e);
                return false;
            },
        };

        // 3. Check if there isn't already a selected hexagon.

        // 4. If it's the selected hexagon. Deselect it.
        // 4a. Go through the entire grid and reset all colours to standard. Return false.

        // 5. Otherwise, check if the new hexagon is a valid attack.
        // 5b. If not, Return false.

        // 6. Carry out attack. Advance game state. Generate new Tesselation.
        //    Return true.

        false
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

#[wasm_bindgen]
pub fn game_2x2_init(board_top_left: Point, hex_radius: u32) -> Game {
    let template = grid::generate_template(2, 2, board_top_left, hex_radius);
    let board = game::canned_2x2_start01();
    let session = session::Setup::new()
        .set_board(board)
        .session()
        .expect("Invalid session initialization");

    Game::new(session, template)
}

#[wasm_bindgen]
pub fn game_3x3_init(board_top_left: Point, hex_radius: u32) -> Game {
    let template = grid::generate_template(3, 3, board_top_left, hex_radius);
    let board = game::canned_3x3_start01();
    let session = session::Setup::new()
        .set_board(board)
        .session()
        .expect("Invalid session initialization");

    Game::new(session, template)
}
