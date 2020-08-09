// use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::pixelcolor::Gray8;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
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
        let c = match key {
            KeyEnum::KeyUpPin => self.character.immutalbe_move(0, -self.move_pad),
            KeyEnum::KeyDownPin => self.character.immutalbe_move(0, self.move_pad),
            KeyEnum::KeyLeftPin => self.character.immutalbe_move(-self.move_pad, 0),
            KeyEnum::KeyRightPin => self.character.immutalbe_move(self.move_pad, 0),
            _ => self.character.immutalbe_move(0, 0),
        };
        self.character = c;
        self
    }
    pub fn draw_char<D>(&self, mut drawable: D) -> D
    where
        D: DrawTarget<Gray8>,
    {
        let c = &self.character;
        let p = 4;
        // let f = PrimitiveStyleBuilder::new().fill_color()
        let z = Rectangle::new(
            Point::new(c.x - p, c.y - p),
            Point::new(c.x + p, c.y + p),
            // 1,
        );
        // .with_fill(Some(PixelColor(1u8)));
        drawable.draw_iter(z.into_iter());
        drawable
    }
}
impl Default for Canvas {
    fn default() -> Self{
        Self::new()
    }
}

mod test {

    #[test]
    fn test_move_char() {
        let mut cnv = crate::Canvas::new();
        println!("{:?}", cnv);
        assert_eq!(cnv.character.x, 30);
        assert_eq!(cnv.character.y, 30);
        cnv.move_char(keyenum::KeyEnum::KeyRightPin);
        assert_eq!(cnv.character.x, 34);
        assert_eq!(cnv.character.y, 30);
        cnv.move_char(keyenum::KeyEnum::KeyDownPin);
        assert_eq!(cnv.character.x, 34);
        assert_eq!(cnv.character.y, 34);
        // assert_eq!(cnv.character.x, 30);
        println!("{:?}", cnv);
    }
}
