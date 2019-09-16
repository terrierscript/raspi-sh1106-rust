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
use hal::spidev::SpidevOptions;

use embedded_hal::digital::v2::OutputPin;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line};
use embedded_graphics::Drawing;
// use embedded_graphics::pixelcolor::BinaryColor;
use sh1106::prelude::*;
use sh1106::Builder;
use random_pos::random;

fn main() {
    spi2();
}

fn setup_spi() -> Spidev {
    let mut spi = Spidev::open("/dev/spidev0.0").expect("spi failed");
    let options = SpidevOptions::new().max_speed_hz(2_000_000).build();
    spi.configure(&options).expect("SPI configure error");
    return spi;
}
fn spi2()  {
    let spi = setup_spi();
    let mut dc = Pin::new(24);
    dc.export().expect("cannnot export dc pin");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("DC: cannot set out direction");


    let mut reset = Pin::new(25);
    reset.export().expect("reset pin unwrap failed");
    while !reset.is_exported() {}
    reset.set_direction(Direction::Out).expect("reset direction failed");

    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();
    
    let mut delay = Delay {};

    disp.reset(&mut reset, &mut delay);
    disp.set_rotation(DisplayRotation::Rotate180);
    disp.init().expect("not initialized");
    disp.flush().expect("not flushed");
    println!("flashed");
    // disp.reset().expect("not flushed");


    // let line = Line::new(Coord)::new(0, 0), Coord::new(64, 64)
    // //     // .translate(Point::new(128 + PADDING * 2, 0))
    //     .with_stroke(Some(BinaryColor::On));
    // disp.draw(line.into_iter());

    (0..10).map(|| {

        disp.set_pixel(random(128),random(64), 1);
    })
    // disp.set_pixel(10,10, 1);
    // disp.set_pixel(20,10, 1);
    // disp.set_pixel(20,20, 1);
    
    
    // // disp.set_pixel(10, 11, 1);
    // // disp.set_pixel(10, 12, 1);
    
    disp.flush().expect("cannot flushed");
    // println!("{:?}", disp.buffer);
    println!("end");
    // loop {}
}

// #[exception]
// fn HardFault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }

// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
