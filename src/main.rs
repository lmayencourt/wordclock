use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use log::*;

// use embedded_svc::{
//     http::{
//         client::{Client, Request, RequestWrite, Response},
//         Status,
//     },
//     io::Read,
// };
// use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;

use esp_idf_svc::systime::EspSystemTime;

pub mod network;
pub mod network_time;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    info!("Hello, ESP32 world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let mut network = network::Network::new(peripherals.modem)?;
    let wifi_res = network.setup_and_connect();

    match wifi_res {
        Ok(()) => info!("Connected to wifi!"),
        Err(err) => error!("Failed to connect: {}", err),
    }

    network_time::init()?;

    loop {
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(500));
        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(500));

        info!("Network time epoch: {:?}", network_time::get_epoch_time());
        info!("Parsed network time: {}", network_time::get_time());
        info!("System time: {:?}", EspSystemTime {}.now());
    }
}
