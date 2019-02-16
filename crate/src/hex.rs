//! Hexagon operations

use wasm_bindgen::prelude::*;

use crate::primitive::{Colour, Colours, Point};

/// Determine a partical hexagon point/corner. Numbers higher than 6 wrap around.
#[wasm_bindgen]
pub fn pointy_hex_corner(center: &Point, radius: u32, corner: u8) -> Point {    
    let corner: f64 = corner.into();
    let radius: f64 = radius.into();
    let corner: f64 = corner.into();    
    let degrees = 60_f64 * corner - 30_f64;
    let radians = std::f64::consts::PI / 180_f64 * degrees;
    let new_x = (center.x() as f64) + radius * radians.cos();
    let new_y = (center.y() as f64) + radius * radians.sin();

    Point::new(new_x as i32, new_y as i32)
}

/// Blank hexagon. Used for templating.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Blank {
    points: [Point; 6],
}

impl Blank {
    pub fn new(center: &Point, radius: u32) -> Self {
        Blank {
            points: [
                pointy_hex_corner(center, radius, 1),
                pointy_hex_corner(center, radius, 2),
                pointy_hex_corner(center, radius, 3),
                pointy_hex_corner(center, radius, 4),
                pointy_hex_corner(center, radius, 5),
                pointy_hex_corner(center, radius, 6),
            ],
        }
    }
    
    pub fn points(&self) -> &[Point] {
        &self.points
    }
}

/// A hexagon tile with all the information needed to render it to a HTML5 canvas.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Detail {
    points: [Point; 6],
    colours: Colours,
    player_number: usize,
    selected: bool,

    // TODO: Add dice count here (+ coordinate array?).
}

impl Detail {
    /// Will panic if there are less than six points.
    pub fn new(
        points: &[Point],
        player_number: usize,
    ) -> Self {
        Detail {
            // Quick and simple array initialization from slice.
            points: [points[0], points[1], points[2], points[3], points[4], points[5]],
            colours: Colours::from_player_number(player_number),
            player_number,
            selected: false,
        }
    }
}

#[wasm_bindgen]
impl Detail {
    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn unselect(&mut self) {
        self.selected = false;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    // Doesn't work with `wasm-pack build`.
    //pub fn points(&self) -> [Point; 6] {
    //    self.points
    //}
    
    pub fn point1(&self) -> Point {
        self.points[0]
    }

    pub fn point2(&self) -> Point {
        self.points[1]
    }

    pub fn point3(&self) -> Point {
        self.points[2]
    }

    pub fn point4(&self) -> Point {
        self.points[3]
    }

    pub fn point5(&self) -> Point {
        self.points[4]
    }

    pub fn point6(&self) -> Point {
        self.points[5]
    }

    pub fn selected_colour(&self) -> Colour {
        self.colours.selected_colour
    }

    pub fn unselected_colour(&self) -> Colour {
        self.colours.unselected_colour
    }

    pub fn threatened_colour(&self) -> Colour {
        self.colours.threatened_colour
    }

    pub fn colour(&self) -> Colour {
        if self.selected {
            self.colours.selected_colour
        } else {
            self.colours.unselected_colour
        }
    }
}
