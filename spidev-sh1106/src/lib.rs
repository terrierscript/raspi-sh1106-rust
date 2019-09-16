extern crate linux_embedded_hal as hal;
extern crate sh1106;

// use hal::I2cdev;
use hal::Spidev;
use hal::Pin;
use hal::Delay;
use hal::sysfs_gpio::Direction;
use hal::spidev::SpidevOptions;
// use hal::sysfs_gpio::Pin;
// use embedded_hal::digital::v2::OutputPin;

// use sh1106::prelude::*;
// use sh1106::Builder;
use sh1106::interface::DisplayInterface;
use sh1106::mode::GraphicsMode;
use sh1106::Error;
use sh1106::interface::SpiInterface;
pub struct SpidevSH1106 {
    pub spidev: Spidev,
    // mut dc_pin: Pin,
    // mut reset_pin: Pin
}

impl SpidevSH1106 {
    pub fn new() -> Self{
        SpidevSH1106 {
            spidev : SpidevSH1106::setup_spi(),
            // dc_pin: SpidevSH1106::dc_pin(),
            // reset_pin: SpidevSH1106::reset_pin(),
        }
    }

    fn setup_spi() -> Spidev {
        let mut spi = Spidev::open("/dev/spidev0.0").expect("spi failed");
        let options = SpidevOptions::new().max_speed_hz(2_000_000).build();
        spi.configure(&options).expect("SPI configure error");
        return spi;
    }

    pub fn dc_pin() -> Pin {
        let dc = Pin::new(24);
        dc.export().expect("cannnot export dc pin");
        while !dc.is_exported() {}
        dc.set_direction(Direction::Out).expect("DC: cannot set out direction");
        return dc;
    }

    fn reset_pin() -> Pin {
        let reset = Pin::new(25);
        reset.export().expect("reset pin unwrap failed");
        while !reset.is_exported() {}
        reset.set_direction(Direction::Out).expect("reset direction failed");
        return reset;
    }

    // pub fn generate_display<DI>(&self) -> GraphicsMode<DI> where 
    //     DI: DisplayInterface
    // {
    //     let mut dc = Self::dc_pin();
    //     return Builder::new().connect_spi(self.spidev, dc)
    // }

    pub fn reset<DI>(disp: &mut GraphicsMode<DI>) -> Result<(), Error<(), ()>>
    where 
        DI: DisplayInterface,
    {
        let mut rpin = Self::reset_pin();
        let mut delay = Delay {};
        return disp.reset(&mut rpin, &mut delay);
    }
    // pub fn reset<DI: DisplayInterface>(&self) -> GraphicsMode<DI> {

    //     let mut reset = Pin::new(25);
    //     reset.export().expect("reset pin unwrap failed");
    //     while !reset.is_exported() {}
    //     reset.set_direction(Direction::Out).expect("reset direction failed");

    //     let mut disp: GraphicsMode<_> = Builder::new().connect_spi(self.spidev, dc).into();
        
    //     let mut delay = Delay {};

    //     disp.reset(&mut reset, &mut delay);
    //     disp.set_rotation(DisplayRotation::Rotate180);
    //     disp.init().expect("not initialized");
    //     disp.flush().expect("not flushed");
    //     return disp;
    // }
}