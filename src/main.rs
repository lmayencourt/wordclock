/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use log::*;
use smart_leds::colors::*;

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

use crate::display::Display;
use crate::led_driver::WS2812;
use crate::persistent_settings::WifiConfiguration;

pub mod display;
pub mod led_driver;
pub mod network;
pub mod network_time;
pub mod persistent_settings;
pub mod time;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    info!("Hello, ESP32 world!");

    let mut config;

    info!("Check if wifi config already exist");
    if let Ok(true) = WifiConfiguration::do_exist() {
        info!("Read wifi configuration from persistent storage");
        config = WifiConfiguration::load()?;
    } else {
        info!("Write wifi configuration to persistent storage");
        config = WifiConfiguration::new(env!("RUST_ESP32_WIFI_SSID"), env!("RUST_ESP32_WIFI_PASSWORD"));
        config.store()?;
    }

    info!("Config {:?}", config);

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let led_driver = WS2812::new(114, peripherals.pins.gpio13, peripherals.rmt.channel0)?;
    let mut display = display::RgbLedStripMatrix::new(led_driver)?;

    let mut network = network::Network::new(peripherals.modem)?;
    let wifi_res = network.setup_and_connect(&config.ssid, &config.password);

    match wifi_res {
        Ok(()) => info!("Connected to wifi!"),
        Err(err) => error!("Failed to connect: {}", err),
    }

    network_time::init().expect("failed to get network time");

    loop {
        led.set_high().unwrap();
        display.draw_time(network_time::get_time())?;
        thread::sleep(Duration::from_millis(500));
        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(500));

        info!("Network time epoch: {:?}", network_time::get_epoch_time());
        info!("Parsed network time: {}", network_time::get_time());
        info!("System time: {:?}", EspSystemTime {}.now());
    }
}
