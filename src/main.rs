#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, text::Text, Drawable, Pixel};

use epd_spectra::{Display2in9, Epd, TriColor};
use hal::gpio::p0::*;
use hal::gpio::Level;
use hal::gpio::*;
use hal::spim::Spim;
use hal::timer::Timer;
use nrf52811_hal::{self as hal};
use profont::PROFONT_24_POINT;
use tinybmp::Bmp;

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
use defmt::info;

#[entry]
fn main() -> ! {
    info!("START");
    let peripherals = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let mut timer = Timer::new(peripherals.TIMER0);
    //led
    //let mut led_red = port0.p0_16.into_push_pull_output(Level::High);
    //let mut led_blue = port0.p0_18.into_push_pull_output(Level::High);
    //let mut led_green = port0.p0_17.into_push_pull_output(Level::High);

    let cs: P0_06<hal::gpio::Output<PushPull>> = port0.p0_06.into_push_pull_output(Level::High);
    let dc = port0.p0_05.into_push_pull_output(Level::High);
    let spiclk = port0.p0_19.into_push_pull_output(Level::High).degrade();
    let spimosi = port0.p0_20.into_push_pull_output(Level::High).degrade();

    let busy = port0.p0_03.into_floating_input();
    let reset = port0.p0_04.into_push_pull_output(Level::High); //low?)

    let pins = hal::spim::Pins {
        sck: Some(spiclk),
        miso: None,
        mosi: Some(spimosi),
    };
    let spi = Spim::new(
        peripherals.SPIM1,
        pins,
        hal::spim::Frequency::K500,
        hal::spim::MODE_0,
        0,
    );

    let mut spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs);
    let epd = Epd::new(&mut spi_device, busy, dc, reset, &mut timer, 0);
    let mut epd = epd.init(&mut spi_device, &mut timer).unwrap();
    let mut display = Display2in9::default();
    display.set_rotation(epd_spectra::DisplayRotation::Rotate270);
    //inverted colors
    display.clear(TriColor::Black).unwrap();
    Text::new(
        "TRACYSPACY's TAG",
        Point::new(64, 155),
        MonoTextStyle::new(&PROFONT_24_POINT, TriColor::Red),
    )
    .draw(&mut display)
    .unwrap();

    //image
    let img1 = include_bytes!("../assets/repo.bmp");
    let bmp: Bmp<BinaryColor> = Bmp::from_slice(img1).unwrap();

    for Pixel(coord, color) in bmp.pixels() {
        let tri_color = match color {
            BinaryColor::On => TriColor::Black,  // Map black
            BinaryColor::Off => TriColor::White, // Map white
        };
        Pixel(coord + Point::new(142, 10), tri_color)
            .draw(&mut display)
            .unwrap();
    }

    epd.update(&display, &mut spi_device, &mut timer).unwrap();

    let _inactive_epd = epd.power_off(&mut spi_device, &mut timer).unwrap();

    loop {}
}
