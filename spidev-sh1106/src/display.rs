use crate::generator::Generator;
use crate::SpidevInterface;
use sh1106::interface::DisplayInterface;
use sh1106::mode::GraphicsMode;
use sh1106::prelude::*;
use sh1106::Builder;

pub struct Display {
  pub disp: GraphicsMode<SpidevInterface>,
}

impl Display {
  pub fn new() -> Self {
    let spidev = Generator::setup_spi();
    let dc_pin = Generator::dc_pin();

    Display {
      disp: Self::gen_display(),
    }
  }
  fn gen_display() -> GraphicsMode<SpidevInterface> {
    let d: GraphicsMode<_> = Builder::new()
      .with_rotation(DisplayRotation::Rotate180)
      .with_size(DisplaySize::Display128x64)
      .connect_spi(Generator::setup_spi(), Generator::dc_pin())
      // .connect_spi(self.spidev, self.dc_pin)
      .into();
    return d;
  }
}

// mod test {
//   fn test() {
//     let d = Display::new().disp;
//   }
// }
