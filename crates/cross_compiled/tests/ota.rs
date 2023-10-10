/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use application::time_source::TimeSource;
use log::*;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;

use cross_compiled::led_driver::WS2812;
use cross_compiled::network;
use cross_compiled::network_time;
use cross_compiled::ota_update::OtaUpdate;
use cross_compiled::rgb_led_strip_matrix;

use application::configuration::Configuration;
use application::display::Display;
use application::firmware_update::FirmwareUpdate;
use application::network::Network;
// use application::version::Version;
 
#[test]
fn display() {
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();

    let led_driver = WS2812::new(114, peripherals.pins.gpio15, peripherals.rmt.channel0).unwrap();
    let mut display = rgb_led_strip_matrix::RgbLedStripMatrix::new(led_driver).unwrap();

    // Display check
    for n in 0..1 {
        display.draw_all().expect("Failed to draw on the screen");
        thread::sleep(Duration::from_millis(1000));
        display.clear().expect("Failed to draw on the screen");
        thread::sleep(Duration::from_millis(2000));
    }
    assert!(true);
}

 fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    info!("Hello, ESP32 world!");
    // info!("OTA successful!");
    // let peripherals = Peripherals::take().unwrap();
    // let mut led = PinDriver::output(peripherals.pins.gpio2)?;
    // loop {
    //     led.set_high().unwrap();
    //     thread::sleep(Duration::from_millis(500));
    //     led.set_low().unwrap();
    //     thread::sleep(Duration::from_millis(500));
    // }

    // info!("Running WordClock firmware {}", Version::from_string("99.99.1")?);

    let hard_coded_config = Configuration::new(env!("RUST_ESP32_WIFI_SSID").to_string(), env!("RUST_ESP32_WIFI_PASSWORD").to_string(), None, None, Color::default());

    info!("Config {:?}", hard_coded_config);

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let led_driver = WS2812::new(114, peripherals.pins.gpio15, peripherals.rmt.channel0)?;
    let mut display = rgb_led_strip_matrix::RgbLedStripMatrix::new(led_driver)?;

    display.draw_progress(1)?;

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

    display.draw_progress(3)?;

    let network_time = Box::new(network_time::NetworkTime::new());
    if let Err(e) = network_time.synchronize() {
        display.draw_error()?;
        return Err(anyhow!{"Failed to get network time"});
    }

    let firmware_update = OtaUpdate;
    info!("available version {}", firmware_update.read_update_version()?);

    firmware_update.download_update()?;
    info!("Update ready, restart device");
    esp_idf_hal::delay::FreeRtos::delay_ms(5000);
    firmware_update.reboot_to_new_image();

    Ok(())
 }
 