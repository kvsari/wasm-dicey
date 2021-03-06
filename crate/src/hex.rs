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
    center: Point,
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
            center: *center,
        }
    }
    
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    pub fn center(&self) -> Point {
        self.center
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Danger {
    Safe,
    Attacking,
    Threatened,
}   

/// A hexagon tile with all the information needed to render it to a HTML5 canvas.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Detail {
    points: [Point; 6],
    center: Point,
    colours: Colours,
    player_number: usize,
    danger: Danger,
    dice: u8,
}

impl Detail {
    /// Will panic if there are less than six points.
    pub fn new(
        points: &[Point],
        center: Point,
        player_number: usize,
        dice: u8,
    ) -> Self {
        Detail {
            // Quick and simple array initialization from slice.
            points: [points[0], points[1], points[2], points[3], points[4], points[5]],
            center,
            colours: Colours::from_player_number(player_number),
            player_number,
            danger: Danger::Safe,
            dice,
        }
    }
}

#[wasm_bindgen]
impl Detail {
    pub fn center(&self) -> Point {
        self.center
    }
    
    // Doesn't work with `wasm-pack build`.
    //pub fn points(&self) -> [Point; 6] {
    //    self.points
    //}

    pub fn set_safe(&mut self) {
        self.danger = Danger::Safe;
    }

    pub fn set_attacking(&mut self) {
        self.danger = Danger::Attacking;
    }

    pub fn set_threatened(&mut self) {
        self.danger = Danger::Threatened;
    }

    /// If index is out of bounds, will wrap around to the beginning, like going around a
    /// circle. Of course, a hexagon has six sides so the indexes will be from 0 to 5.
    pub fn point(&self, index: usize) -> Point {
        let index = index % 6;
        self.points[index]
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
        match self.danger {
            Danger::Safe => self.colours.unselected_colour,
            Danger::Attacking => self.colours.selected_colour,
            Danger::Threatened => self.colours.threatened_colour,
        }
    }

    pub fn dice(&self) -> u8 {
        self.dice
    }
}
