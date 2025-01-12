#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::prelude::*;
use log::info;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();
    let mut led = Output::new(peripherals.GPIO8, Level::High);

    let delay = Delay::new();
    loop {
        info!("Hello rusty ESP !");
        led.toggle();
        delay.delay_millis(1000);
    }
}
