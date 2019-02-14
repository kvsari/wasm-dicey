//! Grid specific operations.
use dicey_dice::game::Board;

/// A drawing template of the grid which will contain all the coordinates precomputed. We
/// use a rectangular shape in the `Template`.
pub struct Template {
    rows: u32,
    columns: u32,
    hexes: Vec<Blank>,
}

impl Template {
    pub fn new(rows: u32, columns: u32, hexes: &[Blank]) -> Self {
        Template {
            rows,
            columns,
            hexes: hexes.into_iter().collect(),
        }
    }
}

/// A grid meant to be viewable. Can also find the hexagon within which a point collides
/// with to help with selecting.
#[wasm_bindgen]
pub struct Tesselation {
    rows: u32,
    columns: u32,
    hexes: Vec<Detail>,
}

impl Tesselation {
    pub fn new(rows: u32, columns: u32, hexes: &[Detail]) -> Self {
        Tesselation {
            rows,
            columns,
            hexes: hexes.into_iter().collect(),
        }
    }
}

/// Emit the colours for a player.
fn colours_to_player(

/// Produce a new `Tesselation` by merging a `Template` and `Board`. The dimensions of the
/// `Board` must match the `Template` otherwise things will go awry.
pub fn generate(template: &Template, board: &Board) -> Tesselation {
    
}
