//! Tiny stuff from which others are built out of.
use std::ops;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

/// Cheesy `ops::Add` impl that would have been seen in so many "learn now to program in
/// rust" tutorials on the web.
impl ops::Add for Point {
    type Output = Point;
    
    fn add(self, rhs: Point) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a> ops::Add<&'a Point> for Point {
    type Output = Point;
    
    fn add(self, rhs: &Point) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    Black = 0,
    White = 1,
    
    LightBlue = 2,
    Blue = 3,
    DarkBlue = 4,
    
    LightGreen = 5,
    Green = 6,
    DarkGreen = 7,
    
    LightRed = 8,
    Red = 9,
    DarkRed = 10,
    
    LightBrown = 11,
    Brown = 12,
    DarkBrown = 13,

    LightOrange = 14,
    Orange = 15,
    DarkOrange = 16,

    LightPurple = 17,
    Purple = 18,
    DarkPurple = 19,
}

/// Convenience struct for grouping together colours for a player.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Colours {
    pub unselected_colour: Colour,
    pub selected_colour: Colour,
    pub threatened_colour: Colour,
}

impl Colours {
    fn new(uc: Colour, sc: Colour, tc: Colour) -> Self {
        Colours {
            unselected_colour: uc,
            selected_colour: sc,
            threatened_colour: tc,
        }
    }
}

/// Generate the `Colours` pack from a player number. Assumes max num of 6. Wraps around.
#[wasm_bindgen]
impl Colours {
    pub (crate) fn from_player_number(player_number: usize) -> Self {

        // We could match from 0 but then it wouldn't line up with the `Player` struct.
        let player_number = (player_number % 6) + 1; 
                                                   
        match player_number {
            1 => Colours::new(Colour::Blue, Colour::DarkBlue, Colour::LightBlue),
            2 => Colours::new(Colour::Green, Colour::DarkGreen, Colour::LightGreen),
            3 => Colours::new(Colour::Red, Colour::DarkRed, Colour::LightRed),
            4 => Colours::new(Colour::Brown, Colour::DarkBrown, Colour::LightBrown),
            5 => Colours::new(Colour::Orange, Colour::DarkOrange, Colour::LightOrange),
            6 => Colours::new(Colour::Purple, Colour::DarkPurple, Colour::LightPurple),
            _ => unreachable!(),
        }
    }
}
    
