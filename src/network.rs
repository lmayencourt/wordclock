use anyhow::{anyhow, Result};
use esp_idf_sys::EspError;
use log::*;

use embedded_svc::wifi;

use esp_idf_hal::modem::*;
use esp_idf_svc::wifi::*;
// use esp_idf_svc::netif::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::nvs::*;

const SSID: &str = env!("RUST_ESP32_WIFI_SSID");
const PASS: &str = env!("RUST_ESP32_WIFI_PASSWORD");

pub struct Network<'a> {
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

impl<'a> Network<'a> {
    pub fn new(modem: Modem) -> Result<Self, EspError> {
        let sys_loop_stack = EspSystemEventLoop::take()?;
        let nvs = EspDefaultNvsPartition::take().ok();
        let wifi = EspWifi::new(modem, sys_loop_stack, nvs)?;
        Ok(Self { wifi })
    }

    pub fn setup_and_connect(&mut self) -> Result<()> {
        let scan_result = self.wifi.scan()?;
        let home_network = scan_result.into_iter().find(|a| a.ssid == SSID);
        if home_network.is_some() {
            info!("Detected home network {:?}", SSID);
        } else {
            error!("Error: Failed to detect home network {:?}", SSID);
            return Err(anyhow!(WifiErrors::CouldNotFindNetwork));
        }

        self.wifi
            .set_configuration(&wifi::Configuration::Client(wifi::ClientConfiguration {
                ssid: SSID.into(),
                password: PASS.into(),
                // channel,
                ..Default::default()
            }))?;

        self.wifi.start()?;
        self.wifi.connect()?;

        Ok(())
    }
}
