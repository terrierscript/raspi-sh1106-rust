extern crate linux_embedded_hal as hal;
extern crate embedded_graphics;
extern crate sh1106;
// extern crate machine_ip;

use hal::{Spidev, Pin};
use hal::sysfs_gpio::Direction;
use hal::spidev::SpidevOptions;
use hal::Delay;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, };
// use embedded_graphics::geometry::Point;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::drawable::Drawable;
// use embedded_graphics::Drawable;
use embedded_graphics::coord::Coord;
use sh1106::prelude::*;
use sh1106::Builder;

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.0").expect("Could not open SPI device");
    let options = SpidevOptions::new().max_speed_hz(50_000).build();
    spi.configure(&options).expect("SPI configure error");

    let mut reset = Pin::new(25);
    reset.export().unwrap();
    while !reset.is_exported() {}
    reset.set_direction(Direction::Out).unwrap();

    let dc = Pin::new(24);
    dc.export().unwrap();
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).unwrap();

    let mut delay = Delay {};

    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

    disp.reset(&mut reset, &mut delay);
    disp.init().unwrap();
    disp.flush().unwrap();
    
    disp.set_pixel(10, 10, 1);
    disp.set_pixel(20, 20, 1);
    disp.set_pixel(30, 30, 1);
    // disp.draw(Line::new(Coord::new(8, 16 + 16), Coord::new(8 + 16, 16 + 16)).into_iter());
    // disp.draw(Line::new(Coord::new(8, 16 + 16),Coord::new(8 + 8, 16), Style::with_stroke(PixelColor)).into_iter());
    // disp.draw(Line::new((8 + 16, 16 + 16), (8 + 8, 16), 1).into_iter());

    // disp.draw(Rect::new((48, 16), (48 + 16, 16 + 16), 1u8).into_iter());

    // disp.draw(Circle::new((96, 16 + 8), 8, 1u8).into_iter());

    // let local_addr = machine_ip::get().unwrap();

    // disp.draw(
    //         Font6x8::render_str(&format!("IP: {}", local_addr.to_string()))
    //             .translate((0, 56))
    //             .into_iter(),
    //     );

    disp.flush().unwrap();
}
