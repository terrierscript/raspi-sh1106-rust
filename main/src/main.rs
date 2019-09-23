extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use key_event::hook_keyevent;
use random_pos::random;
use sh1106::prelude::*;
use spidev_sh1106::SpidevSH1106;

fn main() {
    spi2();
}

fn spi2() {
    // let sd = SpidevSH1106::new();

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
    let event_cb = |name: String, lv: i8| {
        println!("{:?} {:?}", name, lv);
        let mut disp: GraphicsMode<_> = SpidevSH1106::gen_display();
        // disp.init().expect("iniiii");
        for _ in 0..10 {
            SpidevSH1106::draw_random(&mut disp);
        }
        disp.flush().expect("fllll");
    };
    println!("hook");
    hook_keyevent(event_cb);
    println!("end");
}
