//! Grid specific operations

use wasm_bindgen::prelude::*;

use dicey_dice::game::Board;

use crate::hex::{Blank, Detail};
use crate::primitive::Point;

/// A drawing template of the grid which will contain all the coordinates precomputed. We
/// use a rectangular shape in the `Template`.
#[derive(Debug, Clone, PartialEq, Eq)]
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
            hexes: hexes.into_iter().map(|i| *i).collect(),
        }
    }
}

fn generate_row(columns: u32, start: Point, radius: u32) -> impl Iterator<Item = Blank> {
    let translate_by = Point::new(radius as i32, 0);
    (0..columns)
        .map(move |column| {
            let translate_by = translate_by * column as i32;
            let start = start + translate_by;
            Blank::new(&start, radius)
        })
}

/// Generate a new template. Begins from `start` as the top left and works its way right
/// and down. There is implicit spacing between the hexes as a circle radius is used to
/// define their size. Thus the circle edges (which aren't drawn) will touch but the hex
/// edges which are within the circle (except for the corners that touch) won't touch.
fn generate_template(rows: u32, columns: u32, start: Point, radius: u32) -> Template {
    let mut blanks: Vec<Blank> = Vec::new();
    for row in (0..rows) {
        // TODO: Alter the start with each row.
        let start = start;

        // Generate each line from the start point.
        blanks.extend(generate_row(columns, start, radius));
    }

    Template::new(rows, columns, blanks.as_slice())
}
    

/// A grid meant to be viewable. Can also find the hexagon within which a point collides
/// with to help with selecting.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tessellation {
    rows: u32,
    columns: u32,
    hexes: Vec<Detail>,
}

impl Tessellation {
    pub fn new(rows: u32, columns: u32, hexes: &[Detail]) -> Self {
        Tessellation {
            rows,
            columns,
            hexes: hexes.into_iter().map(|i| *i).collect(),
        }
    }
}

/// Produce a new `Tessel;ation` by merging a `Template` and `Board`. The dimensions of the
/// `Board` must match the `Template` otherwise things will go awry.
fn generate_tessellation(template: &Template, board: &Board) -> Tessellation {
    let detail: Vec<Detail> = template.hexes
        .iter()
        .zip(board.grid().iter())
        .map(|(blank, hex)| Detail::new(blank.points(), *hex.data().owner().number()))
        .collect();

    Tessellation {
        rows: template.rows,
        columns: template.columns,
        hexes: detail,
    }
}
