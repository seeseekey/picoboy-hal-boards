//! # Picoboy Display Example
//!
//! Blinks the LED on a Picoboy.
//!
//! This will draws a circle on the display.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use picoboy::entry;

// GPIO traits
use embedded_hal::digital::OutputPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use picoboy::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use picoboy::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use picoboy::hal;

use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::prelude::*;
use fugit::RateExtU32;
use rp2040_hal::Spi;
use sh1106::prelude::GraphicsMode;
use sh1106::Builder;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then draws a circle on the display.
#[entry]
fn main() -> ! {

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        picoboy::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);
    let pins = picoboy::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Define display width and height
    const DISPLAY_WIDTH: i32 = 128;
    const DISPLAY_HEIGHT: i32 = 64;

    // Configure SPI pins
    let spi_sclk = pins.sck.into_function::<rp2040_hal::gpio::FunctionSpi>(); // SCK
    let spi_mosi = pins.mosi.into_function::<rp2040_hal::gpio::FunctionSpi>(); // MOSI
    let spi_miso = pins.gpio16.into_function::<rp2040_hal::gpio::FunctionSpi>(); // MISO (not needed)

    // Create spi instance
    let spi = Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    // Init spi
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        8_000_000u32.Hz(), // 8 MHz
        embedded_hal::spi::MODE_0, // MODE_0 for SH1106
    );

    // Configure display pins
    let dc = pins.dc.into_push_pull_output(); // Data/command pin
    let mut rst = pins.reset.into_push_pull_output(); // Reset pin
    let cs = pins.cs.into_push_pull_output(); // Chip select

    // Reset display
    rst.set_low().unwrap();
    delay.delay_ms(10);
    rst.set_high().unwrap();
    delay.delay_ms(10);

    // Create display interface
    let mut display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc, cs).into();

    display.init().unwrap();
    display.flush().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("From Rust with love.", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    // Red led running indicator
    let mut led_pin = pins.led_red.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}

// End of file
