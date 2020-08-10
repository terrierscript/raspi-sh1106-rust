extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use canvas::{Event, Canvas};
use key_event::hook_keyevent;

use random_pos::random;
use sh1106::prelude::*;
use spidev_sh1106::{SpidevInterface, SpidevSH1106};
use keyenum::KeyEnum;

fn main() {
    spi2();
}


fn spi2() {
    let mut canvas = Canvas::new();
    let mut disp: GraphicsMode<_> = SpidevSH1106::gen_display();

    SpidevSH1106::reset(&mut disp).expect("cannot reset");

    disp.init().expect("not initialized");
    disp.flush().expect("not flushed");
    println!("flashed");

    for _ in 0..10 {
        disp.set_pixel(random(128), random(64), 1);
    }

    hook_keyevent(move |pin: u8| {
        let mut disp: GraphicsMode<SpidevInterface> = SpidevSH1106::gen_display();
        let keyenum = keyenum::from_u8(pin).expect("invalid pin");
        canvas.move_char(key_to_event(keyenum));
        let mut d2 = canvas.draw_char(&mut disp);
        
        disp.flush().expect("fllll");
    });
    println!("hook");
    println!("end");
}

fn key_to_event( key:KeyEnum) -> Event{
    return match key {
        KeyEnum::KeyUpPin => Event::Up,
        KeyEnum::KeyDownPin => Event::Down,
        KeyEnum::KeyLeftPin => Event::Left,
        KeyEnum::KeyRightPin => Event::Right,
        _ => Event::UnknownEvent,
    };
}
