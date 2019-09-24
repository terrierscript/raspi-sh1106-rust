// use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::prelude::*;
// use embedded_graphics::primitives::Circle;
use embedded_graphics::primitives::Rect;
// use embedded_graphics::co
// use embedded_graphics::fonts::Font12x16;
use keyenum::KeyEnum;
// use std::clone::AssertParamIsClone;
// use std::clone::Clone::clone;

#[derive( Debug)]
pub struct Character {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Canvas {
    width: i32,
    height: i32,
    character: Character
}

impl Canvas {
    pub fn new() -> Self {
        println!("canv new");
        Canvas {
            width: 128,
            height: 64,
            character: Character { x: 30, y: 30 }
        }
    }

    pub fn move_char(&mut self, key: KeyEnum) -> &mut Canvas {
        let pad = 4;
        println!("{:?}", self);
        self.character = match key {
            KeyEnum::KeyUpPin => Character {
                x: &self.character.x + 0,
                y: &self.character.y - pad,
            },
            KeyEnum::KeyDownPin => Character {
                x: &self.character.x + 0,
                y: &self.character.y + pad,
            },
            KeyEnum::KeyLeftPin => Character {
                x: &self.character.x - pad,
                y: &self.character.y + 0,
            },
            KeyEnum::KeyRightPin => Character {
                x: &self.character.x + pad,
                y: &self.character.y + 0,
            },
            // else => {}
            _ => Character {
                x: &self.character.x + 0,
                y: &self.character.y + 0,
            },
        };
        return self;
    }
    pub fn draw_char<D>(&self, mut drawable: D) -> D
    where
        D: Drawing<PixelColorU8>,
        // C: PixelColor + Clone,
    {
        let c = &self.character;
        // let a = 1;
        // let fill = Some(PixelColorU8::from(1));
        // let fill2 = Some(Pixel);
        let p = 4;
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

mod test {
    use keyenum::KeyEnum;

    #[test]
    fn test_move_char() {
        let mut cnv = crate::Canvas::new();
        println!("{:?}", cnv);
        assert_eq!(cnv.character.x, 30);
        assert_eq!(cnv.character.y, 30);
        cnv.move_char(KeyEnum::KeyRightPin);
        assert_eq!(cnv.character.x, 34);
        assert_eq!(cnv.character.y, 30);
        cnv.move_char(KeyEnum::KeyDownPin);
        assert_eq!(cnv.character.x, 34);
        assert_eq!(cnv.character.y, 34);
        // assert_eq!(cnv.character.x, 30);
        println!("{:?}", cnv);
    }
}
