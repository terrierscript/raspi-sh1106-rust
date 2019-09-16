//! Print "Hello world!" with "Hello rust!" underneath. Uses the `embedded_graphics` crate to draw
//! the text with a 6x8 pixel font.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB9
//! (green)  SCL -> PB8
//! ```
//!
//! Run on a Blue Pill with `cargo run --example text`.

// #![no_std]
// #![no_main]

extern crate linux_embedded_hal as hal;
extern crate embedded_graphics;
extern crate sh1106;

// use hal::I2cdev;
// use hal::Spidev;
// use hal::Pin;
// use hal::Delay;
// use hal::sysfs_gpio::Direction;
// use hal::spidev::SpidevOptions;

// use embedded_hal::digital::v2::OutputPin;
// use embedded_graphics::primitives::{Line};
// use embedded_graphics::pixelcolor::BinaryColor;

// use embedded_graphics::prelude::*;
// use embedded_graphics::Drawing;
use sh1106::prelude::*;
use sh1106::Builder;
use random_pos::random;
use spidev_sh1106::SpidevSH1106;

fn main() {
    spi2();
}

fn spi2()  {
    let sd = SpidevSH1106::new();
    let dc = SpidevSH1106::dc_pin();
    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(sd.spidev, dc).into();
    
    SpidevSH1106::reset(&mut disp).expect("cannot reset");
    
    disp.set_rotation(DisplayRotation::Rotate180).expect("failed rotation");
    disp.init().expect("not initialized");
    disp.flush().expect("not flushed");
    println!("flashed");

    for _ in 0..100 {
        disp.set_pixel(random(128),random(64), 1);
    }
    
    disp.flush().expect("cannot flushed");
    println!("end");
    // loop {}
}

