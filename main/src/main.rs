extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use canvas::Canvas;
use key_event::hook_keyevent;

use random_pos::random;
use sh1106::prelude::*;
use spidev_sh1106::SpidevSH1106;

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

    hook_keyevent(
         move |pin: u8| {
            let disp: GraphicsMode<_> = SpidevSH1106::gen_display();
            let keyenum = keyenum::from_u8(pin).expect("invalid pin");
            canvas.move_char(keyenum);
            let mut d2 = canvas.draw_char(disp);
            d2.flush().expect("fllll");
        }
    );
    println!("hook");
    println!("end");
}
