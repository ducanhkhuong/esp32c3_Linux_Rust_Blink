#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay,gpio::{Io, Level, Output},prelude::*};
use esp_println::logger::init_logger_from_env;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio8, Level::High);
    let delay = Delay::new();
    init_logger_from_env();

    loop {
        led.toggle();
        log::info!("LED is {:?}", led.is_set_high());
        delay.delay_millis(1000);
    }
}