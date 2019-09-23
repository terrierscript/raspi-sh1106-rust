use embedded_graphics::prelude::*;

pub struct Char {
    x: u32,
    y: u32
}

pub struct Canvas {
    width: u32,
    height: u32,
    character: Char
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            width: 128,
            height: 64,
            character: Char {
                x: 30, 
                y: 30
            }
        }
    }
    pub fn draw_char() {
        // let a = 1;
    }
}
