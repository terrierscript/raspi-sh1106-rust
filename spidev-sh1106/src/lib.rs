#[no_std]
#[no_main]
extern crate linux_embedded_hal as hal;
extern crate sh1106;

use hal::spidev::SpidevOptions;
use hal::sysfs_gpio::Direction;
use hal::Delay;
use hal::Pin;
use hal::Spidev;

use sh1106::Builder;
use sh1106::interface::DisplayInterface;
use sh1106::mode::displaymode::DisplayModeTrait;
use sh1106::mode::GraphicsMode;
use sh1106::Error;
use sh1106::prelude::*;

// use intf::DevIntf;
pub mod intf;

// use sh1106::interface::SpiInterface;

        // SPI: hal::blocking::spi::Transfer<u8, Error = CommE>
        //     + hal::blocking::spi::Write<u8, Error = CommE>,
        // DC: OutputPin<Error = PinE>,

pub struct SpidevSH1106 {
    pub spidev: Spidev,
    pub dc_pin: Pin
}


impl SpidevSH1106 {
    pub fn new() -> Self {
        SpidevSH1106 {
            spidev: SpidevSH1106::setup_spi(),
            dc_pin: SpidevSH1106::dc_pin(),
        }
    }

    // pub fn display<DI: DisplayInterface>(&self) -> GraphicsMode<DI>
    
    // // where
    // //     DI: DisplayInterface ,
    // //     // // NMODE: DisplayModeTrait<
    // //     // //     DI2,
    // //     // //     // SpiInterface<hal::Spidev, hal::Pin, NoOutputPin>
    // //     // // >
    // //     // NPIN:
    // //     // NMODE: sh1106::mode::displaymode::DisplayModeTrait<sh1106::interface::SpiInterface<hal::Spidev, hal::Pin, NPIN>>
    // //     // DI: DisplayModeTrait<sh1106::interface::SpiInterface<hal::Spidev, hal::Pin, sh1106::builder::NoOutputPin>>
    
    // pub fn display<DI, CommE>(&self) -> DisplayModeTrait<DI> where
    //     DI: DisplayInterface<Error=CommE>
    // {
    //     let mut d : GraphicsMode<_> = Builder::new().connect_spi(
    //         SpidevSH1106::setup_spi(),
    //         SpidevSH1106::dc_pin()
    //     );
    //     return d;
    // }


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

    // pub fn generate_display<DI, CommE>(&self) -> GraphicsMode<DI> where
    //     DI: DisplayInterface<Error=CommE>
    // {
    //     return Builder::new().connect_spi(self.spidev, self.dc)
    // }

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

}

// impl intf::DevIntf for SpidevSH1106 {
//     fn generate_display<DI>(&self) -> GraphicsMode<DI>
//     where DI: DisplayInterface{
 
//         let d : GraphicsMode<_> = Builder::new().connect_spi(
//             SpidevSH1106::setup_spi(),
//             SpidevSH1106::dc_pin()
//         ).into();
//         return d;
//     }
// }

pub struct SpidevSH1106Display<DI> where DI : DisplayInterface{
    display: GraphicsMode<DI>
}

// impl SpidevSH1106Display<DisplayInterface<Error=CommE>> {
//      pub fn new() -> Self {
//         let spidev = SpidevSH1106::new();
//         let d : GraphicsMode<_> = Builder::new().connect_spi(
//             SpidevSH1106::setup_spi(),
//             SpidevSH1106::dc_pin()
//         ).into();
//         SpidevSH1106Display{
//             display: d
//         }
        
//     }
   
// }