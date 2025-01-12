#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::delay::Delay;
use esp_hal::prelude::*;
use log::info;

const ADC_MIN: u16 = 3400; // Change value to match sensor
const ADC_MAX: u16 = 4095; // Change value to match sensor

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    let analog_pin = peripherals.GPIO2;
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(analog_pin, Attenuation::Attenuation11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);
    loop {
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();
        let humidity_percentage = map_to_percentage(pin_value);
        info!("Humidity: {}% {}", humidity_percentage, pin_value);
        delay.delay_millis(1500);
    }
}

fn map_to_percentage(adc_value: u16) -> u8 {
    if adc_value > ADC_MAX {
        return 0;
    }
    if adc_value < ADC_MIN {
        return 100;
    }

    let percentage = ((ADC_MAX - adc_value) as u32 * 100) / (ADC_MAX - ADC_MIN) as u32;
    percentage as u8
}
