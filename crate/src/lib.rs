use std::num::NonZeroU8;
use std::collections::HashSet;

use wasm_bindgen::prelude::*;

use dicey_dice::{session, game};

mod utils;
pub mod hex;
pub mod dice;
pub mod grid;
pub mod primitive;
pub mod play;

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
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[macro_export]
macro_rules! jslog {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
pub fn init() {
    jslog!("Setting default panic hook.");
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn game_3x3_init(board_top_left: Point, hex_radius: u32) -> play::Game {
    let template = grid::generate_template(3, 3, board_top_left, hex_radius);
    let board = game::canned_3x3_start01();
    jslog!("Getting board");
    let session = match session::Setup::new()
        .set_board(board)
        .set_move_limit(NonZeroU8::new(3).unwrap())
        .session() {
            Ok(session) => session,
            Err(e) => {
                jslog!("Failed to create game session: {}", &e);
                panic!("Failed to start session.");
            },
        };
    jslog!("Created session");
    play::Game::new(session, template)
}

#[wasm_bindgen]
pub fn start_new_game(
    board_size: u32,
    board_top_left: Point,
    hex_radius: u32,
    move_limit: u8,
    player1_code: u8,
    player2_code: u8,
    player3_code: u8,
    player4_code: u8,
) -> play::Game {
    let template = grid::generate_template(
        board_size, board_size, board_top_left, hex_radius
    );
    jslog!("Generating new random board.");

    let player_codes = [player1_code, player2_code, player3_code, player4_code];

    // The `Players` struct is a bit awkward...
    let players = player_codes
        .iter()
        .fold(game::Players::new(4), |players, code| {
            if *code == 1 || *code == 2 {
                players.next()
            } else {
                players.remove_current()
            }
        });

    // So is the `Player` struct. Mistakes were made...
    let ai_players = player_codes
        .iter()
        .enumerate()
        .map(|(num, code)| (num + 1, code))
        .fold(HashSet::new(), |mut ai, (num, code)| {
            if *code == 2 {
                ai.insert(game::Player::new(num, (64 + num as u8).into()));
            }
            ai
        });
    
    let move_limit = NonZeroU8::new(move_limit).unwrap_or(NonZeroU8::new(2).unwrap());
    let board = game::generate_random_board(board_size, board_size, players);
    let session = match session::Setup::new()
        .set_board(board)
        .set_move_limit(move_limit)
        .session() {
            Ok(session) => session,
            Err(e) => {
                jslog!("Failed to create game session: {}", &e);
                panic!("Failed to start session.");
            },
        };
    jslog!("Created session");
    play::Game::new(session, template)
}
