  
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

#![no_std]
#![no_main]

extern crate linux_embedded_hal as hal;
extern crate embedded_graphics;
extern crate sh1106;

use hal::I2cdev;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line};
use sh1106::prelude::*;
use sh1106::Builder;
// use cortex_m_rt::ExceptionFrame;
// use cortex_m_rt::{entry, exception};
// use embedded_graphics::fonts::Font6x8;
// use embedded_graphics::Drawing;
// use hal::i2c::{BlockingI2c, DutyCycle, Mode};
// use hal::prelude::*;
// use hal::stm32;


fn main() {
    // let dp = stm32::Peripherals::take().unwrap();

    // let mut flash = dp.FLASH.constrain();
    // let mut rcc = dp.RCC.constrain();

    // let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    // let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // let i2c = BlockingI2c::i2c1(
    //     dp.I2C1,
    //     (scl, sda),
    //     &mut afio.mapr,
    //     Mode::Fast {
    //         frequency: 400_000,
    //         duty_cycle: DutyCycle::Ratio2to1,
    //     },
    //     clocks,
    //     &mut rcc.apb1,
    //     1000,
    //     10,
    //     1000,
    //     1000,
    // );
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let mut disp: GraphicsMode<
        I2cInterface<hal::I2cdev>
    > = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();
    disp.flush().unwrap();
    
    disp.set_pixel(10, 10, 1);
    disp.set_pixel(10, 11, 1);
    disp.set_pixel(10, 12, 1);
    // let line = Line::new((8, 16 + 16), (8 + 16, 16 + 16)).into_iter();
    // disp.draw(line);
    // disp.draw(
    //     Font6x8::render_str("Hello world!")
    //         .stroke_width(1)
    //         .into_iter(),
    // );
    // disp.draw(
    //     Font6x8::render_str("Hello Rust!")
    //         .stroke_width(1)
    //         .translate(Point::new(0, 16))
    //         .into_iter(),
    // );

    disp.flush().unwrap();
}

// #[exception]
// fn HardFault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }