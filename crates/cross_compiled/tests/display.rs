/* SPDX-License-Identifier: MIT
* Copyright (c) 2023 Louis Mayencourt
*/

use std::thread;
use std::time::Duration;

use anyhow::Result;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::prelude::*;

use cross_compiled::led_driver::WS2812;
use cross_compiled::rgb_led_strip_matrix;

use application::display::Display;

#[test]
fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let led_driver = WS2812::new(114, peripherals.pins.gpio15, peripherals.rmt.channel0)?;
    let mut display = rgb_led_strip_matrix::RgbLedStripMatrix::new(led_driver)?;

    loop {
        display.draw_all()?;
        thread::sleep(Duration::from_millis(250));
        display.clear()?;
        thread::sleep(Duration::from_millis(750));
    }
}
    