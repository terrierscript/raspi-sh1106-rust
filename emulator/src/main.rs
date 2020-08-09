use canvas::Canvas;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    BinaryColorTheme, SimulatorDisplay, SimulatorEvent, Window, OutputSettingsBuilder,
};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(
        Size::new(128, 64), 
    );
    let setting = OutputSettingsBuilder::new().theme(BinaryColorTheme::OledBlue).build();

        // .theme(DisplayTheme::OledBlue)
        // .size(128, 64)
        // .build();
    let mut window = Window::new("test", &setting);
    let mut canvas = Canvas::new();
    let d2 = canvas.draw_char(display);
    // // d2.flush().expect("fllll");
    'running: loop {
        window.update(&d2);
        for event in window.events() {
            match event {   
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    println!("Click event at ({}, {})", point.x, point.y);
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }

            thread::sleep(Duration::from_millis(200));
         }
    }
}
