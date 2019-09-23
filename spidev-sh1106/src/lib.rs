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
// use std::
// use intf::DevIntf;
pub mod intf;

type SpidevInterface = SpiInterface<Spidev, Pin, NoOutputPin>;

pub struct SpidevSH1106 {
    pub spidev: Spidev,
    pub dc_pin: Pin,
}

impl SpidevSH1106 {
    pub fn new() -> Self {
        SpidevSH1106 {
            spidev: SpidevSH1106::setup_spi(),
            dc_pin: SpidevSH1106::dc_pin(),
        }
    }

    fn dc_pin() -> Pin {
        let dc = Pin::new(24);
        dc.export().expect("cannnot export dc pin");
        while !dc.is_exported() {}
        dc.set_direction(Direction::Out)
            .expect("DC: cannot set out direction");
        return dc;
    }

    fn reset_pin() -> Pin {
        let reset = Pin::new(25);
        reset.export().expect("reset pin unwrap failed");
        while !reset.is_exported() {}
        reset
            .set_direction(Direction::Out)
            .expect("reset direction failed");
        return reset;
    }

    pub fn reset<DI>(disp: &mut GraphicsMode<DI>) -> Result<(), Error<(), ()>>
    where
        DI: DisplayInterface,
    {
        let mut rpin = Self::reset_pin();
        let mut delay = Delay {};
        return disp.reset(&mut rpin, &mut delay);
    }

    fn setup_spi() -> Spidev {
        let mut spi = Spidev::open("/dev/spidev0.0").expect("spi failed");
        let options = SpidevOptions::new().max_speed_hz(2_000_000).build();
        spi.configure(&options).expect("SPI configure error");
        return spi;
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
            .connect_spi(SpidevSH1106::setup_spi(), SpidevSH1106::dc_pin())
            // .connect_spi(self.spidev, self.dc_pin)
            .into();
        return d;
    }
}
