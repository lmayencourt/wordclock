/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use esp_idf_sys::{self as _, esp_restart};

use application::power_manager::PowerManager;

pub struct Esp32Soc;

impl PowerManager for Esp32Soc {
    fn reset(&self) {
        unsafe {
            esp_restart();
        }
    }
}