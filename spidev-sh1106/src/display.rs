

pub struct Display {

  pub disp: SpidevInterface
}

pub impl Display{
  pub fn new() -> Self {
    let spidev = SpidevSH1106::setup_spi(),
    let dc_pin = SpidevSH1106::dc_pin(),

    Display {
      disp: gen_display()
    }
  }
  pub fn gen_display() -> GraphicsMode<SpidevInterface> {
      let d: GraphicsMode<_> = Builder::new()
          .connect_spi(SpidevSH1106::setup_spi(), SpidevSH1106::dc_pin())
          // .connect_spi(self.spidev, self.dc_pin)
          .into();
      return d;
  }
}