// use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::{egrectangle,primitive_style};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
// use keyenum::KeyEnum;
// use std::{convert::{TryFrom, Infallible}, fmt::Error};

#[warn(unused_must_use)]
    
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

pub enum Event {
    UnknownEvent,
    Up,
    Down,
    Left,
    Right
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

    pub fn move_char(&mut self, event: Event) -> &mut Canvas {
        let c = match event {
            Event::Up => self.character.immutalbe_move(0, -self.move_pad),
            Event::Down => self.character.immutalbe_move(0, self.move_pad),
            Event::Left => self.character.immutalbe_move(-self.move_pad, 0),
            Event::Right => self.character.immutalbe_move(self.move_pad, 0),
            _ => self.character.immutalbe_move(0, 0),
        };
        self.character = c;
        self
    }

    // pub fn key_to_event(&mut self, key: KeyEnum) -> Event {
    //     return match key {
    //         KeyEnum::KeyUpPin => Event::Up,
    //         KeyEnum::KeyDownPin => Event::Down,
    //         KeyEnum::KeyLeftPin => Event::Left,
    //         KeyEnum::KeyRightPin => Event::Right,
    //         _ => Event::UnknownEvent,
    //     };
    // }

    // pub fn move_char_from_key(&mut self, key: KeyEnum) -> &mut Canvas {
    //     let ev = self.key_to_event(key);
    //     self.move_char(ev);
    //     self
    // }


    pub fn draw_char<D>(&self, mut drawable: D) -> D
    where
        D: DrawTarget<BinaryColor>,
    {
        let c = &self.character;
        let p = 4;
        // let style = PrimitiveStyleBuilder::new().fill_color(Some(BinaryColor::On));
        // let z = Rectangle::new(
        //     Point::new(c.x - p, c.y - p),
        //     Point::new(c.x + p, c.y + p),
        //     // 1,
        // ).into_(style);
        let zz = egrectangle!(
            top_left = (c.x - p, c.y - p),
            bottom_right = (c.x + p, c.y + p),
            style = primitive_style!(
                fill_color = BinaryColor::On
            )
        );

        // zz.draw(&mut drawable);
        // z.draw(&mut drawable);
        // ;
        // .with_fill(Some(PixelColor(1u8)));
        // let r = 
        drawable.draw_iter(zz.into_iter());

        //     .and(Ok(drawable))
        // ;
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
