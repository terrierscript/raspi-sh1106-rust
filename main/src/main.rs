extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use canvas::Canvas;
use key_event::hook_keyevent;
use random_pos::random;
use sh1106::prelude::*;
use spidev_sh1106::display::Display;
use spidev_sh1106::SpidevSH1106;

use embedded_graphics::fonts::Font12x16;
use embedded_graphics::prelude::*;
// use embedded_graphics::text_12x16;

fn main() {
    spi2();
}

fn spi2() {
    let d = Display::new();
    // let dd = d.gen_display();
    // let sd = SpidevSH1106::new();
    let cnv = Canvas::new();
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
    let event_cb = move |name: String, lv: i8| {
        println!("{:?} {:?}", name, lv);
        let mut disp: GraphicsMode<_> = SpidevSH1106::gen_display();
        // let f = Font12x16::render_str(name.as_str());
        &cnv.draw_char(disp);
        // disp.draw(f.into_iter());
        // disp.init().expect("iniiii");
        // for _ in 0..10 {
        //     // SpidevSH1106::draw_random(&mut disp);
        // }
        // disp.flush().expect("fllll");
    };
    println!("hook");
    hook_keyevent(event_cb);
    println!("end");
}
