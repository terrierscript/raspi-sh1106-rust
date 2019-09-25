use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rect;
use keyenum::KeyEnum;

#[derive(Debug)]
pub struct Character {
    x: i32,
    y: i32,
}
impl Character {
    fn immutalbe_move(&self, x: i32, y: i32) -> Character {
        Character{
            x: self.x + x,
            y: self.y + y
        }
    }
}

#[derive(Debug)]
pub struct Canvas {
    width: i32,
    height: i32,
    move_pad: i32,
    character: Character,
}

impl Canvas {
    pub fn new() -> Self {
        println!("canv new");
        Canvas {
            width: 128,
            height: 64,
            move_pad: 4,
            character: Character { x: 30, y: 30 },
        }
    }


    pub fn move_char(&mut self, key: KeyEnum) -> &mut Canvas {
        self.character = match key {
            KeyEnum::KeyUpPin => self.character.immutalbe_move(0, -self.move_pad),
            KeyEnum::KeyDownPin => self.character.immutalbe_move(0, self.move_pad),
            KeyEnum::KeyLeftPin => self.character.immutalbe_move(-self.move_pad, 0),
            KeyEnum::KeyRightPin => self.character.immutalbe_move(self.move_pad, 0),
            _ => self.character.immutalbe_move(0, 0),
        };
        return self;
    }
    pub fn draw_char<D>(&self, mut drawable: D) -> D
    where
        D: Drawing<PixelColorU8>,
    {
        let c = &self.character;
        let p = 4;
        let z: Rect<PixelColorU8> = Rect::new(
            Coord::new(c.x - p, c.y - p),
            Coord::new(c.x + p, c.y + p),
            // 1,
        )
        .with_fill(Some(PixelColorU8(1u8)));
        drawable.draw(z.into_iter());
        drawable
    }
}

mod test {

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
