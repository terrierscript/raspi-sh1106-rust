extern crate embedded_hal as hal;
use sh1106::interface::DisplayInterface;
use sh1106::mode::GraphicsMode;
use sh1106::mode::displaymode::DisplayModeTrait;

pub trait DevIntf{
    // fn setup_spi() -> hal::blocking::spi::Transfer<W, Error = CommE> + hal::blocking::spi::Write<W, Error = CommE>;
    fn generate_display<DI>(&self) -> GraphicsMode<DI> where DI: DisplayInterface;
}

