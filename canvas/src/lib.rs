use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use embedded_graphics::primitives::Rect;
// use embedded_graphics::co
use embedded_graphics::fonts::Font12x16;
use key_event::keyenum::KeyEnum;

#[derive(Clone, Copy, Debug)]
pub struct Char {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Canvas {
    width: i32,
    height: i32,
    character: Char,
}

impl Canvas {
    pub fn new() -> Self {
        println!("canv new");
        Canvas {
            width: 128,
            height: 64,
            character: Char { x: 30, y: 30 },
        }
    }

    pub fn move_char(&mut self, key: KeyEnum) -> &mut Canvas {
        let pad = 2;
        println!("{:?}", self);
        match key {
            KeyEnum::KEY_UP_PIN => self.character.y = self.character.y - pad,
            KeyEnum::KEY_DOWN_PIN => self.character.y = self.character.y + pad,
            KeyEnum::KEY_LEFT_PIN => self.character.x = self.character.x - pad,
            KeyEnum::KEY_RIGHT_PIN => self.character.x = self.character.x + pad,
            // else => {}
            _ => (),
        };
        return self;
    }
    pub fn draw_char<D>(&self, mut drawable: D) -> D
    where
        D: Drawing<PixelColorU8>,
        // C: PixelColor + Clone,
    {
        let c = self.character;
        // let a = 1;
        // let fill = Some(PixelColorU8::from(1));
        // let fill2 = Some(Pixel);
        let p = 2;
        let z: Rect<PixelColorU8> = Rect::new(
            Coord::new(c.x - p, c.y - p),
            Coord::new(c.x + p, c.y + p),
            // 1,
        )
        .with_fill(Some(PixelColorU8(1u8)));
        // let d = Circle::new(Coord::new(self.character.x, self.character.y), 10);
        // let f = Font12x16::render_str("hello");
        drawable.draw(z.into_iter());
        return drawable;
    }
}
