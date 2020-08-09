use canvas::{Event, Canvas};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::{
    keyboard::{Keycode, Mod}
};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(
        Size::new(128, 64), 
    );
    let setting = OutputSettingsBuilder::new().theme(BinaryColorTheme::OledBlue).build();

        // .theme(DisplayTheme::OledBlue)
        // .size(128, 64)
        // .build();
    let mut window = Window::new("test", &setting);
    let mut canvas = Canvas::new();
    // let d2 = canvas.draw_char(display);
    // let d2 = canvas.draw_char(display);
    // // d2.flush().expect("fllll");
    'running: loop {
        display.clear(BinaryColor::Off)?;
        // display.clear(BinaryColor::Off);
        let rect = canvas.char_rect();
        display.draw_iter(rect.into_iter());
        window.update(&display);
        for event in window.events() {
            match event {   
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    println!("Click event at ({}, {})", point.x, point.y);
                }
                SimulatorEvent::KeyDown { keycode, ..} => {
                    let ev = key_to_event(keycode);
                    canvas.move_char(ev);
                    // display.draw_iter(rect.into_iter());
                    println!("{}", keycode);
                    
                }
                SimulatorEvent::Quit => break 'running Ok(()),
                SimulatorEvent::KeyUp { keycode, keymod, repeat } => {}
                SimulatorEvent::MouseButtonDown { mouse_btn, point } => {}
                SimulatorEvent::MouseWheel { scroll_delta, direction } => {}
                SimulatorEvent::MouseMove { point } => {}
            }

            thread::sleep(Duration::from_millis(50));
         }
    }
}

fn key_to_event(key: Keycode) -> Event {
    return match key {
        Keycode::Up => Event::Up,
        Keycode::Down => Event::Down,
        Keycode::Left => Event::Left,
        Keycode::Right => Event::Right,
        _ => Event::UnknownEvent,
    };
}
