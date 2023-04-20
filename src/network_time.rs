/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::ffi::CString;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use log::*;

use esp_idf_svc::sntp::EspSntp;
use esp_idf_svc::sntp::SntpConf;
use esp_idf_svc::sntp::SyncStatus;

use crate::time::Time;

#[derive(Debug)]
enum NetworkTimeErrors {
    SyncTimeout,
}

impl std::fmt::Display for NetworkTimeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyncTimeout => write!(f, "Timeout when waiting for network time"),
        }
    }
}

pub fn init() -> Result<()> {
    // sntp_setoperatingmode()
    // sntp_set_sync_mode()
    // sntp_setservername()
    // sntp_set_time_sync_notification_cb()
    // sntp_init()
    let time = EspSntp::new(&SntpConf::default())?;

    unsafe {
        esp_idf_sys::sntp_set_sync_interval(15 * 100);
        esp_idf_sys::sntp_restart();
    };
    debug!("Sntp interval: {}", unsafe {
        esp_idf_sys::sntp_get_sync_interval()
    });

    let mut retries_before_timeout: u32 = 10;
    loop {
        let time_sync_status: SyncStatus = time.get_sync_status();
        debug!("Wait for time sync {:?}", time_sync_status);
        if time_sync_status == SyncStatus::Completed {
            break;
        }

        if retries_before_timeout > 0 {
            retries_before_timeout -= 1;
        } else {
            return Err(anyhow!(NetworkTimeErrors::SyncTimeout));
        }
        thread::sleep(Duration::from_secs(2));
    }

    get_time();
    configure_time_zone();

    Ok(())
}

pub fn configure_time_zone() {
    let tz = CString::new("TZ").expect("Failed to create CString");
    // TZ value for Geneva, Switzerland according to https://remotemonitoringsystems.ca/time-zone-abbreviations.php
    let time_zone = CString::new("CET-1CEST-2,M3.5.0/02:00:00,M10.5.0/03:00:00")
        .expect("Failed to create CString");

    unsafe {
        esp_idf_sys::setenv(tz.as_ptr(), time_zone.as_ptr(), 1);
        esp_idf_sys::tzset();
    }
}

pub fn get_epoch_time() -> i32 {
    let mut now: esp_idf_sys::time_t = 0;
    unsafe {
        esp_idf_sys::time(&mut now);
    }

    now
}

pub fn get_time() -> Time {
    let mut now: esp_idf_sys::time_t = 0;
    let mut time_info: esp_idf_sys::tm = Default::default();
    unsafe {
        esp_idf_sys::time(&mut now);
        esp_idf_sys::localtime_r(&now, &mut time_info);
    }

    debug!("Time info is {:?}", time_info);
    Time {
        second: time_info.tm_sec as u8,
        minute: time_info.tm_min as u8,
        hour: time_info.tm_hour as u8,
    }
}
