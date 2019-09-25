use crate::generator::Generator;
use crate::SpidevInterface;
use crate::SpidevSH1106;
use hal::Pin;
use hal::Spidev;
// use sh1106::interface::DisplayInterface;
use sh1106::mode::GraphicsMode;
use sh1106::prelude::*;
use sh1106::Builder;

pub struct Display {
    device: SpidevSH1106,
    // pub disp: GraphicsMode<SpidevInterface>,
    spidev: Spidev,
    dc_pin: Pin,
}

impl Display {
    pub fn new() -> Self {
        let sh = SpidevSH1106::new();
        let spidev = Generator::setup_spi();
        let dc_pin = Generator::dc_pin();

        Display {
            device: sh,
            spidev,
            dc_pin, // disp: Self::gen_display(),
        }
    }
    pub fn gen_display(self) -> GraphicsMode<SpidevInterface> {
        let d: GraphicsMode<_> = Builder::new()
            .with_rotation(DisplayRotation::Rotate180)
            .with_size(DisplaySize::Display128x64)
            .connect_spi(self.device.spidev, self.device.dc_pin)
            // .connect_spi(self.spidev, self.dc_pin)
            .into();
        d
    }
}

// mod test {
//   fn test() {
//     let d = Display::new().disp;
//   }
// }
