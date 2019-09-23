#[no_std]
#[no_main]
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use hal::spidev::SpidevOptions;
use hal::sysfs_gpio::Direction;
use hal::Delay;
use hal::Pin;
use hal::Spidev;
use random_pos::random;

use sh1106::builder::NoOutputPin;
use sh1106::interface::DisplayInterface;
use sh1106::mode::displaymode::DisplayModeTrait;
use sh1106::mode::GraphicsMode;
use sh1106::prelude::*;
use sh1106::properties::DisplayProperties;
use sh1106::Builder;
use sh1106::Error;
use std::boxed::Box;
use std::rc::Rc;
// use crate::generator::Generator;
// extern crate generator;
// use std::
// use intf::DevIntf;
pub mod intf;
mod generator;
use generator::Generator;

pub type SpidevInterface = SpiInterface<Spidev, Pin, NoOutputPin>;

pub struct SpidevSH1106 {
    pub spidev: Spidev,
    pub dc_pin: Pin,
}

impl SpidevSH1106 {
    pub fn new() -> Self {
        SpidevSH1106 {
            spidev: Generator::setup_spi(),
            dc_pin: Generator::dc_pin(),
        }
    }

    pub fn reset<DI>(disp: &mut GraphicsMode<DI>) -> Result<(), Error<(), ()>>
    where
        DI: DisplayInterface,
    {
        let mut rpin = Generator::reset_pin();
        let mut delay = Delay {};
        return disp.reset(&mut rpin, &mut delay);
    }

    pub fn draw_random<DI>(disp: &mut GraphicsMode<DI>)
    // -> (Result<(), Error<(), ()>>)
    where
        DI: DisplayInterface,
    {
        disp.set_pixel(random(128), random(64), 1);
    }

    pub fn gen_display() -> GraphicsMode<SpidevInterface> {
        let d: GraphicsMode<_> = Builder::new()
            .connect_spi(Generator::setup_spi(), Generator::dc_pin())
            // .connect_spi(self.spidev, self.dc_pin)
            .into();
        return d;
    }
}
