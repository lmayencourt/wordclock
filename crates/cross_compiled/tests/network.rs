/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::thread;
use std::time::Duration;

use anyhow::Result;
use log::*;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::prelude::*;

use cross_compiled::network;

use application::color::Color;
use application::configuration::Configuration;
use application::network::Network;

#[test]
 fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let hard_coded_config = Configuration::new(env!("RUST_ESP32_WIFI_SSID").to_string(), env!("RUST_ESP32_WIFI_PASSWORD").to_string(), None, None, Color::default());

    let peripherals = Peripherals::take().unwrap();

    let mut network = network::WifiNetwork::new(peripherals.modem)?;
    network.configure(&hard_coded_config.get_ssid().unwrap(), &hard_coded_config.get_password().unwrap())?;
    let wifi_res = network.connect();

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

    assert!(network.is_connected());

    // if let Err(e) = network_time::init() {
    //     display.draw_error()?;
    //     return Err(e);
    // }

    // loop {
    //     led.set_high().unwrap();
    //     display.draw_time(network_time::get_time())?;
    //     thread::sleep(Duration::from_millis(500));
    //     led.set_low().unwrap();
    //     thread::sleep(Duration::from_millis(500));

    //     info!("Network time epoch: {:?}", network_time::get_epoch_time());
    //     info!("Parsed network time: {}", network_time::get_time());
    //     info!("System time: {:?}", EspSystemTime {}.now());
    // }

    Ok(())
 }
 