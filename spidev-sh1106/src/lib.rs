extern crate linux_embedded_hal as hal;
extern crate sh1106;

use hal::Delay;
use hal::Pin;
use hal::Spidev;
use random_pos::random;

use sh1106::interface::DisplayInterface;
use sh1106::mode::GraphicsMode;
use sh1106::prelude::*;
use sh1106::{NoOutputPin, Builder};
// use sh1106::Error;
// use hal::digital::v2::OutputPin;

pub mod display;
mod generator;
use generator::Generator;
use embedded_hal::digital::v2::OutputPin;

// pub type SpidevInterface = SpiInterface<Spidev, Pin, NoOutputPin>;
pub type SpidevInterface = SpiInterface<Spidev, Pin, NoOutputPin<hal::sysfs_gpio::Error>>;
// pub type SpidevInterface = SpiInterface<Spidev, Pin, hal::sysfs_gpio::Error>;

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

    pub fn reset<DI>(
        disp: &mut GraphicsMode<DI>,
    ) -> Result<(), sh1106::Error<(), hal::sysfs_gpio::Error>>
    where
        DI: DisplayInterface,
    {
        let mut rpin = Generator::reset_pin();
        let mut delay = Delay {};
        disp.reset(&mut rpin, &mut delay)
    }

    pub fn draw_random<DI>(disp: &mut GraphicsMode<DI>)
    // -> (Result<(), Error<(), ()>>)
    where
        DI: DisplayInterface,
    {
        disp.set_pixel(random(128), random(64), 1);
    }

    pub fn gen_display() -> GraphicsMode<SpidevInterface> {
        let np = NoOutputPin::new();
        let d:GraphicsMode<_>  = Builder::new()
            .with_rotation(DisplayRotation::Rotate180)
            .with_size(DisplaySize::Display128x64)
            .connect_spi(Generator::setup_spi(), Generator::dc_pin(),np)
            .into();
            
            // .into();
            
        return d;
    }
}
