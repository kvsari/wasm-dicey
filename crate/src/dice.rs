//! Dice drawing
use std::ops::Neg;

use wasm_bindgen::prelude::*;

use crate::primitive::{Colour, Point};

/// Square mostly mirroring the `rect()` method call on HTML5 `<canvas>`.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square {
    top_left: Point,
    length: i32,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dot {
    center: Point,
    radius: u32,
}

fn dice_len_from_radius(radius: u32) -> i32 {
    (radius as f64 / 2.5_f64).round() as i32
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
    Center = 4,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DiceTemplate {
    square: Square,
}

/// A dice that sits on the top left from the center. Thus the center is this dice' bottom
/// right corner.
#[wasm_bindgen]
impl DiceTemplate {
    pub fn new(hex_center: &Point, hex_radius: u32, position: Position) -> Self {
        let len = dice_len_from_radius(hex_radius);
        let square_top_left = match position {
            Position::TopLeft => *hex_center + Point::new(len.neg(), len.neg()),
            Position::TopRight => *hex_center + Point::new(0, len.neg()),
            Position::BottomLeft => *hex_center + Point::new(len.neg(), 0),
            Position::BottomRight => *hex_center + Point::new(0, 0),
            Position::Center => *hex_center + Point::new(len.neg() / 2, len.neg() / 2),
        };

        DiceTemplate {
            square: Square {
                top_left: square_top_left,
                length: len,
            }
        }
    }

    pub fn x(&self) -> i32 {
        self.square.top_left.x()
    }

    pub fn y(&self) -> i32 {
        self.square.top_left.y()
    }

    pub fn width(&self) -> i32 {
        self.square.length
    }

    pub fn height(&self) -> i32 {
        self.square.length
    }
}

