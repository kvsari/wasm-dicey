//! Grid specific operations

use wasm_bindgen::prelude::*;

use dicey_dice::game::{Holding, Board};

use crate::hex::{Blank, Detail};
use crate::primitive::Point;

/// A drawing template of the grid which will contain all the coordinates precomputed. We
/// use a rectangular shape in the `Template`. A `Template` is expected to have at least
/// one hex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    columns: u32,
    rows: u32,
    radius: u32,
    hexes: Vec<Blank>,
}

impl Template {
    pub fn new(columns: u32, rows: u32, radius: u32, hexes: &[Blank]) -> Self {
        Template {
            columns,
            rows,
            radius,
            hexes: hexes.into_iter().map(|i| *i).collect(),
        }
    }

    pub fn radius(&self) -> u32 {
        self.radius
    }
}

fn generate_row(
    columns: u32, start: Point, width: u32, radius: u32
) -> impl Iterator<Item = Blank> {
    let translate_by = Point::new(width as i32, 0);
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
pub fn generate_template(columns: u32, rows: u32, start: Point, radius: u32) -> Template {
    let width = (3_f64.sqrt() * radius as f64).round() as u32;
    let half_width = (width / 2) as i32;
    let height = 2 * radius;
    let three_quarter_height = (height as f64 * 0.75_f64).round() as i32;
    let one_and_half_height = (height as f64 * 1.5_f64).round() as i32;

    let even_row_start = Point::new(0, 0);
    let odd_row_start = Point::new(half_width, three_quarter_height);
    //let row_increment = Point::new(1, one_and_half_height);
    
    let mut blanks: Vec<Blank> = Vec::new();
    let mut even_rows = 0;
    let mut odd_rows = 0;
    
    (0..rows)
        .for_each(|row| {
            let offset = if row % 2 == 0 {
                // We have an even row
                let height_increment = one_and_half_height * even_rows;
                even_rows += 1;
                even_row_start + Point::new(0, height_increment)
            } else {
                // We have an odd row
                let height_increment = one_and_half_height * odd_rows;
                odd_rows += 1;
                odd_row_start + Point::new(0, height_increment)
            };
            
            // Generate each line from the start point.
            blanks.extend(generate_row(columns, offset + start, width, radius));
        });

    Template::new(rows, columns, radius, blanks.as_slice())
}
    

/// A grid meant to be viewable. Can also find the hexagon within which a point collides
/// with to help with selecting. A `Tessellation` is expected to have at least one hex.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tessellation {
    rows: u32,
    columns: u32,
    radius: u32,
    hexes: Vec<Detail>,
}

impl Tessellation {
    pub fn new(rows: u32, columns: u32, radius: u32, hexes: &[Detail]) -> Self {
        Tessellation {
            rows,
            columns,
            radius,
            hexes: hexes.into_iter().map(|i| *i).collect(),
        }
    }

    pub fn hex_mut(&mut self, index: usize) -> Option<&mut Detail> {
        self.hexes.get_mut(index)
    }
}

#[wasm_bindgen]
impl Tessellation {
    /// If index is out of bounds will wrap around to the beginning.
    pub fn hex(&self, index: usize) -> Detail {
        let index = index % self.hexes.len();
        self.hexes.get(index).map(|d| *d).unwrap()
    }
    
    pub fn len(&self) -> usize {
        self.hexes.len()
    }

    pub fn radius(&self) -> u32 {
        self.radius
    }

    pub fn start_hex_center(&self) -> Point {
        self.hexes
            .first()
            .map(|d| d.center())
            .unwrap_or_default()
    }    
}

/// Produce a new `Tessellation` by merging a `Template` and `Board`. The dimensions of the
/// `Board` must match the `Template` otherwise things will go awry.
pub (crate) fn generate_tessellation(template: &Template, board: &Board) -> Tessellation {
    let detail: Vec<Detail> = template.hexes
        .iter()
        .zip(board.grid().iter())
        .map(|(blank, hex)| Detail::new(
            blank.points(),
            blank.center(),
            *hex.data().owner().number(),
            hex.data().dice(),
        ))
        .collect();

    Tessellation {
        rows: template.rows,
        columns: template.columns,
        radius: template.radius,
        hexes: detail,
    }
}
