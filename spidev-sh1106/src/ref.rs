
pub struct SpidevSH1106Ref<'a> {
    pub spidev: &'a Spidev,
    pub dc_pin: &'a Pin,
}
impl<'a> SpidevSH1106Ref<'a> {
    pub fn new() -> Self {
        SpidevSH1106Ref {
            spidev: &SpidevSH1106::setup_spi(),
            dc_pin: &SpidevSH1106::dc_pin(),
        }
    }
}