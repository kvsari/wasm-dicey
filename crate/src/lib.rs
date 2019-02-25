
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Threatened {
    index: usize,
    cube: hexagon::Cube,
}

impl Threatened {
    fn new(index: usize, cube: hexagon::Cube) -> Self {
        Threatened { index, cube }
    }
}

#[derive(Debug, Clone)]
struct Selected {
    index: usize,
    cube: hexagon::Cube,
    threatened: Vec<Threatened>,
}

impl Selected {
    fn new(index: usize, cube: hexagon::Cube, threatened: &[Threatened]) -> Self {
        Selected {
            index,
            cube,
            threatened: threatened.into_iter().map(|t| t.to_owned()).collect(),
        }
    }
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

    /// Index of selected hex if any with threatened.
    selected: Option<Selected>,
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

    fn select_hexagon(&mut self, coordinate: hexagon::Cube) {
        // 1. Determine that the hexagon coordinate is valid.
        let index = match self.turn
            .as_ref()
            .unwrap()
            .board()
            .grid()
            .fetch_index(coordinate) {
                Ok(index) => index,
                Err(e) => {
                    // There is nothing more to do here. Log error then exit.
                    jslog!("Invalid hexagon coordinate: {}", &e);
                    return;
                },
            };

        // 2. Check if a hexagon is already selected or not. That determines how we
        //    treat this hexagon selection.
        if let Some(selection) = self.selected.take() {
            // A hexagon is already clicked.
            self.second_select_hexagon(selection, coordinate, index);
        } else {
            // No hexagon is clicked. This'll be easy.
            jslog!("Hexagon at {} is ready to attack.", &coordinate);
            self.first_select_hexagon(coordinate, index);
        }
    }

    /// Select a hexagon and fill the `selected` slot with `Some(Selection)`. Expects
    /// a valid `coordinate` and that no hexagon is currently selected.
    fn first_select_hexagon(&mut self, coordinate: hexagon::Cube, index: usize) {
        // 1. Get the hexagon.
        let detail = self.tessellation
            .as_mut()
            .unwrap()
            .hex_mut(index)
            .unwrap();

        // 2. Change the danger state of the selected hexagon. Then drop to release `self`.
        detail.set_attacking();
        drop(detail);

        // 3. Fetch the coordinates of any threatened hexes
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

        // 4. Change the danger state of the threatened hexagons and return a record of
        //    their location for later use.
        let threatened: Vec<Threatened> = t_coords
            .into_iter()
            .map(|t_coord| {
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

                Threatened::new(threatened_index, t_coord)
            })
            .collect();

        // 5. Save the entire selection state.
        let selected = Selected::new(index, coordinate, threatened.as_slice());
        self.selected = Some(selected);
    }

    /// Select a hexagon where one is already `selected`. This could be a valid attacking
    /// move, deselecting the attacking hex or an invalid move which does nothing. Expects
    /// that the `selected` slot is `Some` and that the `coordinate` and `index` are valid.
    fn second_select_hexagon(
        &mut self, selection: Selected, coordinate: hexagon::Cube, index: usize
    ) {
        // 1. Determine if the hex is the attacking hex.
        if selection.index == index {
            jslog!("Attacking hexagon at {} stands down.", &coordinate);
            return self.deselect_hexagon(selection);
        }

        // 2. Otherwise check if a threatened hex was selected.
        let check = Threatened::new(index, coordinate);
        if selection.threatened.contains(&check) {
            jslog!("Hexagon at {} is attacked!", &coordinate);
            return self.attack_hexagon(check, selection);
        }

        // 3. The hex chosen was an invalid move. Log and set the selection back.
        jslog!("Invalid attacking move: {}", &coordinate);
        self.selected = Some(selection);
    }

    /// A hexagon has been attacked. This will advance game state.
    fn attack_hexagon(&mut self, attacked: Threatened, attacker: Selected) {
        // 1. Find the choice
        let action = game::Action::Attack(attacker.cube, attacked.cube);
        let choice = self.turn
            .as_ref()
            .unwrap()
            .choices()
            .iter()
            .find(|choice| *choice.action() == action)
            .unwrap();

        // 2. Advance session state.
        let new_state = self.session
            .advance(choice)
            .unwrap()
            .to_owned();

        // 3. Update internal game state.
        let tessellation = grid::generate_tessellation(
            &self.template, new_state.board(),
        );
        self.turn = Some(new_state);
        self.tessellation = Some(tessellation);
        self.selected = None;
    }

    /// Set the attacking hexagons danger level as well as any threatened hexagons to the
    /// `Safe` state in the `selection`.
    fn deselect_hexagon(&mut self, selection: Selected) {
        // 1. Get the hexagon.
        let detail = self.tessellation
            .as_mut()
            .unwrap()
            .hex_mut(selection.index)
            .unwrap();

        // 2. Deselect and drop.
        detail.set_safe();
        drop(detail);

        // 3. Loop through all threatened hexes and set them safe too.
        selection.threatened
            .into_iter()
            .for_each(|threatened| {
                let detail = self.tessellation
                    .as_mut()
                    .unwrap()
                    .hex_mut(threatened.index)
                    .unwrap();
                
                detail.set_safe();
            });
    }
}

#[wasm_bindgen]
impl Game {
    pub fn tessellation(&self) -> Tessellation {
        self.tessellation.clone().unwrap()
    }

    pub fn select_hex_with_pixel(&mut self, pixel: Point) {
        // Convert the pixel (x, y) into a hexagon axial coordinate.
        let coordinate = pixel.hexagon_axial(self.template.radius());
        self.select_hexagon(coordinate.into());
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
