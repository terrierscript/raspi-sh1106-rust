extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use canvas::Canvas;
use key_event::hook_keyevent;

use random_pos::random;
use sh1106::prelude::*;
use spidev_sh1106::display::Display;
use spidev_sh1106::SpidevSH1106;

// use embedded_graphics::text_12x16;

fn main() {
    spi2();
}

fn spi2() {
    let d = Display::new();
    // let dd = d.gen_display();
    // let sd = SpidevSH1106::new();
    let mut cnv : Mutex<Canvas> = Mutex::new(Canvas::new());
    let mut disp: GraphicsMode<_> = SpidevSH1106::gen_display();
    SpidevSH1106::reset(&mut disp).expect("cannot reset");

    disp.init().expect("not initialized");
    disp.flush().expect("not flushed");
    println!("flashed");

    for _ in 0..10 {
        disp.set_pixel(random(128), random(64), 1);
    }

    disp.flush().expect("cannot flushed");
    // let b = Rc::new(disp);
    // set_keys();
    hook_keyevent(
        // let event_cb =
        move |pin: u8| {
            println!("{:?}", pin);
            let disp: GraphicsMode<_> = SpidevSH1106::gen_display();
            let keyenum = keyenum::from_u8(pin).expect("invalid pin");
            let cnv2 = cnv.move_char(keyenum);
            let d = &mut cnv2.draw_char(disp);
            d.flush().expect("fllll");
        }, // event_cb
    );
    println!("hook");
    println!("end");
}
