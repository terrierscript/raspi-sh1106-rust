extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use std::sync::mpsc::channel;
use std::sync::Mutex;
use canvas::Canvas;
use key_event::hook_keyevent;

use random_pos::random;
use sh1106::prelude::*;
use spidev_sh1106::display::Display;
use spidev_sh1106::SpidevSH1106;
use std::sync::Arc;
// use embedded_graphics::text_12x16;

fn main() {
    spi2();
}

fn spi2() {
    let d = Display::new();
    // let dd = d.gen_display();
    // let sd = SpidevSH1106::new();
    let mut cnv = Arc::new(Mutex::new(Canvas::new()));
    let mut disp: GraphicsMode<_> = SpidevSH1106::gen_display();
    let (tx, rx) = channel();

    SpidevSH1106::reset(&mut disp).expect("cannot reset");
    

    disp.init().expect("not initialized");
    disp.flush().expect("not flushed");
    println!("flashed");

    for _ in 0..10 {
        disp.set_pixel(random(128), random(64), 1);
    }

    disp.flush().expect("cannot flushed");
    let dispMutex = Mutex::new(disp);
    // let b = Rc::new(disp);
    // set_keys();
    hook_keyevent(
        // let event_cb =
         move |pin: u8| {
            tx.send(pin);
            // let mut lock = cnv.try_lock();
            // let mut lock2 = dispMutex.try_lock();
            // if let Ok(ref mut canvas) = lock {
            // if let Ok(ref mut disp) = lock2 {
            //         println!("{:?}", pin);
            //         let disp: GraphicsMode<_> = SpidevSH1106::gen_display();
            //         let keyenum = keyenum::from_u8(pin).expect("invalid pin");
            //         canvas.move_char(keyenum);
            //         let d2 = canvas.draw_char(disp);
            //         d2.flush().expect("fllll");
            //     }
            // }
        }, // event_cb
    );
    loop {}
    println!("hook");
    println!("end");
}
