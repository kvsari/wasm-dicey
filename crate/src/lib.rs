
use wasm_bindgen::prelude::*;

use dicey_dice::{session, game, hexagon};

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

    /// There will always be a current turn.
    turn: Option<session::State>,

    /// Index of selected hex if any.
    selected: Option<(usize, hexagon::Cube)>,
}

impl Game {
    fn new(session: session::Session, template: grid::Template) -> Self {
        let tessellation = Some(grid::generate_tessellation(
            &template, session.current_turn().board(),
        ));
        let selected = None;
        let turn = Some(session.current_turn().to_owned());
        Game { session, template, tessellation, turn, selected }
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

        /*
        // 2. Get the hexagon index.
        let hex_index = match self.turn
            .as_ref()
            .unwrap()
            .board()
            .grid()
            .fetch_index(coordinate) {
                Ok(index) => index,
                Err(e) => {
                    jslog!("Invalid hexagon coordinate: {}", &e);
                    return false;
                },
            };
        */

        /*
        // Get the current hexagon
        let detail = self.tessellation
            .as_mut()
            .unwrap()
            .hex_mut(hex_index)
            .unwrap();
        */
        
        // We also want to see and update the colour of any threatened hexes.
        let t_coords: Vec<hexagon::Cube> = self.turn
            .as_ref()
            .unwrap()
            .choices()
            .iter()
            .filter_map(|choice| match choice.action() {
                game::Action::Attack(from_hex, to_hex) => {
                    if *from_hex == coordinate.into() {
                        Some(*to_hex)
                    } else {
                        None
                    }
                },
                _ => None,
            })
            .collect();
        
        // 3. Check if there isn't already a selected hexagon.
        if let Some((selected_index, selected_cube)) = self.selected {
            if selected_cube == coordinate {
                // If it's the selected hexagon. Deselect it.
                let detail = self.tessellation
                    .as_mut()
                    .unwrap()
                    .hex_mut(hex_index)
                    .unwrap();
                detail.set_safe();
                self.selected = None;

                // Reset the threatened hexagons (if any) to safe.
                t_coords
                    .iter()
                    .for_each(|t_coord| {
                        let threatened_index = self.turn
                            .as_ref()
                            .unwrap()
                            .board()
                            .grid()
                            .fetch_index(t_coord)
                            .unwrap();
                        let detail = self.tessellation
                            .as_mut()
                            .unwrap()
                            .hex_mut(threatened_index)
                            .unwrap();
                        
                        detail.set_safe();
                    });

                false
            } else {
                // Otherwise, check if the new hexagon is a valid attack.
                // If not, Return false.
                let mut valid: Option<usize> = None;
                for t_coord in t_coords {
                    //if t_coord == coordinate.into()
                }

                // Carry out attack. Advance game state. Generate new Tesselation.
                // Return true.

                false
            }
        } else {
            // Its a fresh attacking position
            detail.set_attacking();
            self.selected = Some((hex_index, coordinate));
            drop(detail);

            t_coords
                .iter()
                .for_each(|t_coord| {
                    let threatened_index = self.turn
                        .as_ref()
                        .unwrap()
                        .board()
                        .grid()
                        .fetch_index(t_coord)
                        .unwrap();
                    let detail = self.tessellation
                        .as_mut()
                        .unwrap()
                        .hex_mut(threatened_index)
                        .unwrap();
                    
                    detail.set_threatened();                
                });
            
            false
        }
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
