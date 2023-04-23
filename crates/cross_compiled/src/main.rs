/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::thread;
use std::time::Duration;

use anyhow::{Result};
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

use crate::led_driver::WS2812;
use crate::persistent_settings::WifiConfiguration;

use application::display::Display;
use application::version::Version;

pub mod firmware_update;
pub mod led_driver;
pub mod network;
pub mod network_time;
pub mod persistent_settings;
pub mod rgb_led_strip_matrix;

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
    let mut display = rgb_led_strip_matrix::RgbLedStripMatrix::new(led_driver)?;
    display.draw_progress(1)?;

    let mut network = network::Network::new(peripherals.modem)?;
    let wifi_res = network.setup_and_connect(&config.ssid, &config.password);
    display.draw_progress(2)?;

    match wifi_res {
        Ok(()) => info!("Connected to wifi!"),
        Err(err) => error!("Failed to connect: {}", err),
    }
    thread::sleep(Duration::from_millis(2000));
    while network.is_connected() == false {
        info!("Waiting for network connection...");
        network.disconnect()?;
        thread::sleep(Duration::from_millis(500));
        network.connect()?;
        thread::sleep(Duration::from_millis(4000));
    }

    display.draw_progress(3)?;

    if let Err(e) = network_time::init() {
        display.draw_error()?;
        return Err(e);
    }

    info!("available version {}", firmware_update::read_update_version()?);

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
