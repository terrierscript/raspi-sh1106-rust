extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate sh1106;

// use rppal::gpio::{Gpio, InputPin, Trigger};
// use key_event::hook_keyevent_test;
use key_event::CallbackFn;
use random_pos::random;
// use sh1106::interface::DisplayInterface;
use sh1106::prelude::*;
// use sh1106::Builder;
use spidev_sh1106::SpidevSH1106;
// use key_event::get_pins;
use key_event::hook_keyevent;
// use std::borrow::Borrow;
// use std::boxed::Box;
// use std::rc::Rc;

fn main() {
    spi2();
}

fn spi2() {
    // let sd = SpidevSH1106::new();
    let mut disp: GraphicsMode<_> = SpidevSH1106::gen_display();
    SpidevSH1106::reset(&mut disp).expect("cannot reset");

    // disp.set_rotation(DisplayRotation::Rotate180)
    //     .expect("failed rotation");
    disp.init().expect("not initialized");
    disp.flush().expect("not flushed");
    println!("flashed");

    for _ in 0..10 {
        disp.set_pixel(random(128), random(64), 1);
    }

    disp.flush().expect("cannot flushed");
    // let b = Rc::new(disp);
    // set_keys();
    let event_cb: CallbackFn = |name: String, lv: i8| {
        println!("{:?} {:?}", name, lv);
        let mut disp: GraphicsMode<_> = SpidevSH1106::gen_display();
        SpidevSH1106::draw_random(&mut disp);
        // b;
        // b.as_ref().set_pixel(random(128), random(64), 1)
        // render_dot(&mut disp);
        // for _ in 0..10 {
        //     disp.set_pixel(random(128), random(64), 1);
        // }
    };
    println!("hook");
    // hook_keyevent_test(event_cb);
    hook_keyevent(event_cb);
    // hook_keyevent(| | -> { //TOOD
    // //     println!("{:?} {:?}",  lv);
    // //     // if lv === HIGH
    // });
    println!("end");
    // loop {}
}

// fn render_dot<T>(disp: &mut GraphicsMode<T>)
// where T: DisplayInterface
// {
//     for _ in 0..10 {
//         disp.set_pixel(random(128), random(64), 1);
//     }
//     disp.flush().unwrap();
// }
