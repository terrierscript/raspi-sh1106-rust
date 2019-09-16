
extern crate linux_embedded_hal as hal;
extern crate embedded_graphics;
extern crate sh1106;

use hal::I2cdev;
use hal::Spidev;
use hal::Pin;
use hal::Delay;
use hal::sysfs_gpio::Direction;
use hal::spidev::SpidevOptions;

use sh1106::prelude::*;
use sh1106::Builder;


pub struct SpidevSH1106 {
    spidev: Spidev
}

impl SpidevSH1106 {
    fn setup_spi() -> Spidev {
        let mut spi = Spidev::open("/dev/spidev0.0").expect("spi failed");
        let options = SpidevOptions::new().max_speed_hz(2_000_000).build();
        spi.configure(&options).expect("SPI configure error");
        return spi;
    }
}