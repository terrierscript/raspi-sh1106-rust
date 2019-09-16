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

use hal::I2cdev;
use hal::Spidev;
use hal::Pin;
use hal::Delay;
use hal::sysfs_gpio::Direction;
use embedded_hal::digital::v2::OutputPin;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line};
use embedded_graphics::Drawing;
use embedded_graphics::pixelcolor::BinaryColor;
use sh1106::prelude::*;
use sh1106::Builder;

fn main() {
    // i2c();
    // spi();
    spi2();
}

fn i2c() {
    println!("i2c start");
    let i2c = I2cdev::new("/dev/i2c-1").expect("open i2c");
    println!("open");

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    println!("disp");

    disp.init().expect("init");
    println!("init");
    disp.flush().expect("flush 1");
    println!("flush");
    disp.set_pixel(10, 10, 1);
    disp.flush().expect("flush 2");

}


fn spi2() -> ! {
    println!("start");
    // let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let spi = Spidev::open("/dev/spidev0.0").expect("spi failed");
    println!("spi connected");
    // let b = Builder::new();
    let mut dc = Pin::new(24);
    dc.export().expect("dc pin unwrap failed");
    // while !dc.is_exported() {}
    // println!("dc pin");
    // dc.set_high();
    dc.set_direction(Direction::Out).expect("dc direction failed");

    let mut reset = Pin::new(25);
    reset.export().expect("reset pin unwrap failed");
    while !reset.is_exported() {}
    println!("reset pin");
    reset.set_direction(Direction::Out).expect("reset direction failed");

    println!("pin");
    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();
    println!("disp");
    // let mut delay = Delay {};

    // disp.reset(&mut reset, &mut delay);
    
    disp.init().expect("not initialized");
    println!("initialized");
    println!("{:?}", disp.get_dimensions());
    disp.flush().expect("not flushed");
    println!("flashed");
    // disp.reset().expect("not flushed");


    // let line = Line::new(Point::new(0, 0), Point::new(64, 64))
    // //     // .translate(Point::new(128 + PADDING * 2, 0))
    //     .stroke(Some(BinaryColor::On));


    // disp.draw(line.into_iter());
    disp.set_pixel(60, 60, 1);
    println!("pixel");
    // // disp.set_pixel(10, 11, 1);
    // // disp.set_pixel(10, 12, 1);
    
    // disp.flush().expect("cannot flushed");
    // println!("{:?}", disp.buffer);
    println!("end");
    loop {}
}

// #[exception]
// fn HardFault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }

// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
