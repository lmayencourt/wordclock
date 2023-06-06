/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use esp_idf_sys::EspError;
use log::*;

use embedded_svc::wifi;

use esp_idf_hal::modem::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::nvs::*;

use application::network::Network;

pub struct WifiNetwork<'a> {
    ssid: Option<String>,
    password: Option<String>,
    wifi: EspWifi<'a>,
}

#[derive(Debug)]
enum WifiErrors {
    CouldNotFindNetwork,
}

impl std::fmt::Display for WifiErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CouldNotFindNetwork => write!(f, "Could not find Network"),
        }
    }
}

impl<'a> WifiNetwork<'a> {
    pub fn new(modem: Modem) -> Result<Self, EspError> {
        let sys_loop_stack = EspSystemEventLoop::take()?;
        let nvs = EspDefaultNvsPartition::take().ok();
        let wifi = EspWifi::new(modem, sys_loop_stack, nvs)?;
        Ok(Self { ssid:None, password:None, wifi })
    }

    pub fn setup_and_connect(&mut self, ssid: &str, password: &str) -> Result<()> {
        let scan_result = self.wifi.scan()?;
        let home_network = scan_result.into_iter().find(|a| a.ssid == ssid);
        if home_network.is_some() {
            info!("Detected home network {:?}", ssid);
        } else {
            error!("Error: Failed to detect home network {:?}", ssid);
            return Err(anyhow!(WifiErrors::CouldNotFindNetwork));
        }

        self.wifi
            .set_configuration(&wifi::Configuration::Client(wifi::ClientConfiguration {
                ssid: ssid.into(),
                password: password.into(),
                // channel,
                ..Default::default()
            }))?;

        self.wifi.start()?;
        self.wifi.connect()?;

        Ok(())
    }
}

impl<'a> Network for WifiNetwork<'a> {
    fn configure(&mut self, ssid: &str, password: &str) -> Result<()> {
        self.ssid = Some(String::from(ssid));
        self.password = Some(String::from(password));

        self.wifi
            .set_configuration(&wifi::Configuration::Client(wifi::ClientConfiguration {
                ssid: ssid.into(),
                password: password.into(),
                // channel,
                ..Default::default()
            }))?;

        self.wifi.start()?;

        Ok(())
    }

    fn connect(&mut self) -> Result<()> {
        self.wifi.connect()?;
        thread::sleep(Duration::from_millis(10000));

        while self.is_connected() == false {
            info!("Waiting for network connection...");
            self.disconnect()?;
            thread::sleep(Duration::from_millis(500));
            self.connect()?;
            thread::sleep(Duration::from_millis(10000));
        }

        Ok(())
    }

    fn disconnect(&mut self) -> Result<()> {
        self.wifi.disconnect()?;

        Ok(())
    }

    fn is_connected(&self) -> bool {
        match self.wifi.is_up() {
            Ok(connected) => connected,
            Err(e) => {
                error!("Failed to get wifi status: {}", e);
                return false
            }
        }
    }

}
