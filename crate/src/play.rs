//! Game state handling stuff.
use std::collections::HashSet;

use wasm_bindgen::prelude::*;

use dicey_dice::{session, hexagon, game};
use crate::grid::{self, Template, Tessellation};
use crate::primitive::Point;
use crate::{log, jslog};

/// Rely on the choice scoring to move.
fn handle_ai_turn(choices: &[game::Choice]) -> usize {
    let (index, _) = choices
        .iter()
        .enumerate()
        .fold((0, game::Score::default()), |(index, best), (count, choice)| {
            let score = choice.score().unwrap();
            if score > best {
                (count, score)
            } else {
                (index, best)
            }
        });

    index
}


#[wasm_bindgen]
pub struct Advancement {
    ai_turn: bool,
}

impl Advancement {
    fn new(ai_turn: bool) -> Self {
        Advancement { ai_turn }
    }
}

#[wasm_bindgen]
impl Advancement {
    pub fn ai_turn(&self) -> bool {
        self.ai_turn
    }
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

#[wasm_bindgen]
pub struct Game {
    session: session::Session,
    ai_players: HashSet<game::Player>,
    ai_compute_horizon: usize,
    template: Template,    

    /// There will always be a tessellation. It is a bug if this field is left `None`.
    tessellation: Option<Tessellation>,

    /// There will always be a current turn.
    turn: Option<session::State>,

    /// Index of selected hex if any with threatened.
    selected: Option<Selected>,
}

impl Game {
    pub (crate) fn new(
        session: session::Session,
        ai_players: HashSet<game::Player>,
        ai_compute_horizon: usize,
        template: Template,
    ) -> Self {
        let tessellation = Some(grid::generate_tessellation(
            &template, session.current_turn().board(),
        ));
        let selected = None;
        let turn = Some(session.current_turn().to_owned());
        Game {
            session,
            ai_players,
            ai_compute_horizon,
            template,
            tessellation,
            turn,
            selected
        }
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
                game::Action::Attack(from_hex, to_hex, _, _) => {
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
        // 1. Find the choice.
        let choice = self.turn
            .as_ref()
            .unwrap()
            .choices()
            .iter()
            .enumerate()
            .find(|(_index, choice)| match choice.action() {
                game::Action::Attack(from_hex, to_hex, _, _) => {
                    (*from_hex == attacker.cube) && (*to_hex == attacked.cube)
                },
                _ => false
            })
            .map(|(index, _)| index)
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

    pub fn current_player_ai(&self) -> bool {
        let state = self.session.current_turn().to_owned();
        let curr_player = state.board().players().current();
        self.ai_players.contains(&curr_player)
    }

    pub fn current_player_id(&self) -> u8 {
        let state = self.turn.as_ref().unwrap();
        *state.board().players().current().number() as u8
    }

    pub fn current_player_moves_left(&self) -> u8 {
        let state = self.turn.as_ref().unwrap();
        let moves = state.board().moved();
        self.session.move_limit().get() - moves
    }

    pub fn current_player_dice_captured(&self) -> u8 {
        let state = self.turn.as_ref().unwrap();
        *state.board().captured_dice()
    }

    /// Attempts to advance the game without player input. This is possible if AI playing
    /// is activated and it's the AI's turn. A successful AI game advancement will return
    /// `true`. Otherwise `false` is returned signalling that the current player is human
    /// or the game is over.
    pub fn advance(&mut self) -> bool {
        let state = self.session.current_turn().to_owned();
        let curr_player = state.board().players().current();

        // Check if the game is over.
         match state.game() {
            session::Progression::PlayOn(_outcome) => (),
            session::Progression::GameOverWinner(player) => {
                jslog!("Game Over\nWinner is {}", &player);
                return false;
            },
            session::Progression::GameOverStalemate(players) => {
                jslog!("Game Over\nSTATELMATE between players {:?}", &players);
                return false;
            },
        }

        if self.ai_players.contains(&curr_player) {
            drop(state);
            let state = self.session.score_with_depth_horizon(self.ai_compute_horizon);
            let index = handle_ai_turn(state.choices().as_slice());
            drop(state);
            let state = self.session.advance(index).unwrap();
            let tessellation = grid::generate_tessellation(&self.template, state.board());
            self.turn = Some(state.to_owned());
            self.tessellation = Some(tessellation);
            true
        } else {
            false
        }
    }
}
