use canvas::Canvas;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    BinaryColorTheme, SimulatorDisplay, SimulatorEvent, Window, OutputSettingsBuilder,
};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = SimulatorDisplay::new(
        Size::new(128, 64), 
    );
        // .theme(DisplayTheme::OledBlue)
        // .size(128, 64)
        // .build();
    // let mut canvas = Canvas::new();
    // let d2 = canvas.draw_char(display);
    // // d2.flush().expect("fllll");
    // loop {
    //     let end = display.run_once();

    //     if end {
    //         break;
    //     }

    //     thread::sleep(Duration::from_millis(200));
    // }
}
