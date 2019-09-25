use hal::spidev::SpidevOptions;
use hal::sysfs_gpio::Direction;
use hal::Pin;
use hal::Spidev;

pub struct Generator {}

impl Generator {
    pub fn dc_pin() -> Pin {
        let dc = Pin::new(24);
        dc.export().expect("cannnot export dc pin");
        while !dc.is_exported() {}
        dc.set_direction(Direction::Out)
            .expect("DC: cannot set out direction");
        return dc;
    }

    pub fn reset_pin() -> Pin {
        let reset = Pin::new(25);
        reset.export().expect("reset pin unwrap failed");
        while !reset.is_exported() {}
        reset
            .set_direction(Direction::Out)
            .expect("reset direction failed");
        return reset;
    }

    pub fn setup_spi() -> Spidev {
        let mut spi = Spidev::open("/dev/spidev0.0").expect("spi failed");
        let options = SpidevOptions::new().max_speed_hz(2_000_000).build();
        spi.configure(&options).expect("SPI configure error");
        return spi;
    }
}
