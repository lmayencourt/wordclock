/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::thread;
use std::time::Duration;

use log::*;
use anyhow::{Result};

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

use application::Application;
use application::behaviour::*;
use application::network::Network;
use application::version::Version;

// use cross_compiled::firmware_update;
use cross_compiled::led_driver::WS2812;
use cross_compiled::network;
use cross_compiled::network_time;
use cross_compiled::persistent_settings::NonVolatileStorage;
use cross_compiled::rgb_led_strip_matrix;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    info!("WordClock firmware ({}) - ESP32!", Version::from_string("99.99.99")?);

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let led_driver = WS2812::new(114, peripherals.pins.gpio15, peripherals.rmt.channel0)?;
    let display = rgb_led_strip_matrix::RgbLedStripMatrix::new(led_driver)?;

    let mut network = network::WifiNetwork::new(peripherals.modem)?;

    // Needed for now to avoid wifi error:
    // E (2691) phy_init: esp_phy_load_cal_data_from_nvs: NVS has not been initialized. Call nvs_flash_init before starting WiFi/BT.
    network.configure("dummy","1234")?;
    network.fake_connect()?;

    let time_source = network_time::NetworkTime::new();

    let persistent_storage = NonVolatileStorage;

    let mut application = Application::new(display, time_source, persistent_storage, network);

    application.publish_event(Event::Init);
    application.run();

    // Display check
    // for n in 0..1 {
    //     display.draw_all()?;
    //     thread::sleep(Duration::from_millis(1000));
    //     display.clear()?;
    //     thread::sleep(Duration::from_millis(2000));
    // }

    loop {
        application.run();

        // hearth beat
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(100));
        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(900));

        if application.get_current_state() == State::DisplayTime {
            application.publish_event(Event::Tick);

            info!("Network time epoch: {:?}", network_time::get_epoch_time());
            info!("Parsed network time: {}", network_time::get_time());
            info!("System time: {:?}", EspSystemTime {}.now());
        }
    }
}
