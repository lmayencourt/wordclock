/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::time::Instant;

use log::*;

use esp_idf_sys::{self as _, esp_restart};

use application::power_manager::PowerManager;
use application::time::Time;
use application::time_monotonic::TimeMonotonic;
use application::time_source::TimeSource;

pub struct Esp32Soc;

impl PowerManager for Esp32Soc {
    fn reset(&self) {
        unsafe {
            esp_restart();
        }
    }
}

pub struct Esp32SocSystemTime;

impl Esp32SocSystemTime {
    pub fn new() -> Self {
        Esp32SocSystemTime{}
    }
}

impl TimeMonotonic for Esp32SocSystemTime {
    fn now(&self) -> Instant {
        Instant::now()
    }
}
pub struct Esp32SocCpuTime;

impl Esp32SocCpuTime {
    pub fn new() -> Self {
        Esp32SocCpuTime{}
    }
}

impl TimeSource for Esp32SocCpuTime {
    fn synchronize(&mut self) -> anyhow::Result<(), application::time_source::TimeSourceError> {
        // todo!();
        Ok(())
    }

    fn get_time(&self) -> anyhow::Result<Time, application::time_source::TimeSourceError> {
        let mut now: esp_idf_sys::time_t = 0;
        let mut time_info: esp_idf_sys::tm = Default::default();
        unsafe {
            esp_idf_sys::time(&mut now);
            esp_idf_sys::localtime_r(&now, &mut time_info);
        }

        debug!("Time info is {:?}", time_info);
        Ok(
            Time {
                second: time_info.tm_sec as u8,
                minute: time_info.tm_min as u8,
                hour: time_info.tm_hour as u8,
            }
        )
    }

    fn set_time(&mut self, now: Time) -> anyhow::Result<(), application::time_source::TimeSourceError> {
        let mut time_info: esp_idf_sys::tm = Default::default();
        time_info.tm_sec = now.second as i32;
        time_info.tm_min = now.minute as i32;
        time_info.tm_hour = now.hour as i32;

        let current_unix_time: esp_idf_sys::time_t;
        unsafe {
            current_unix_time = esp_idf_sys::mktime(&mut time_info);
        }

        let tv_sec: *const esp_idf_sys::timeval = &esp_idf_sys::timeval{tv_sec:current_unix_time, tv_usec:0};
        let tz: *const esp_idf_sys::timezone = &esp_idf_sys::timezone::default();

        unsafe {
            esp_idf_sys::settimeofday(tv_sec, tz);
        }
        Ok(())
    }
}