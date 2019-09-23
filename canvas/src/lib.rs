use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
// use embedded_graphics::co

#[derive(Clone,Copy)]
pub struct Char {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
pub struct Canvas {
    width: i32,
    height: i32,
    character: Char,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            width: 128,
            height: 64,
            character: Char { x: 30, y: 30 },
        }
    }
    pub fn draw_char<C, D>(&self, mut drawing: D)
    where
        C: PixelColor,
        D: Drawing<C>,
    {
        // let a = 1;
        let d = Circle::new(Coord::new(self.character.x, self.character.y), 2);
        drawing.draw(d.into_iter());
    }
}
