extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use random_pos::random;
use sh1106::prelude::*;
use sh1106::Builder;
use spidev_sh1106::SpidevSH1106;

fn main() {
    spi2();
}

fn spi2() {
    let sd = SpidevSH1106::new();
    let dc = SpidevSH1106::dc_pin();
    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(sd.spidev, dc).into();

    SpidevSH1106::reset(&mut disp).expect("cannot reset");

    disp.set_rotation(DisplayRotation::Rotate180)
        .expect("failed rotation");
    disp.init().expect("not initialized");
    disp.flush().expect("not flushed");
    println!("flashed");

    for _ in 0..100 {
        disp.set_pixel(random(128), random(64), 1);
    }

    disp.flush().expect("cannot flushed");
    println!("end");
    // set_keys();
    // loop {}
}
