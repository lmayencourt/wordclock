use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};

// use embedded_svc::{
//     http::{
//         client::{Client, Request, RequestWrite, Response},
//         Status,
//     },
//     io::Read,
// };
// use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration};

use embedded_svc::wifi;
// use embedded_svc::wifi::Configuration;
// use esp_idf_svc::eventloop::EspSystemEventLoop;
// use esp_idf_svc::netif::EspNetif;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::*;

use esp_idf_svc::wifi::*;
// use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::eventloop::*;

const SSID: &str = env!("RUST_ESP32_WIFI_SSID");
const PASS: &str = env!("RUST_ESP32_WIFI_PASSWORD");

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    let sys_loop_stack = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take().ok();
    let mut wifi = EspWifi::new(peripherals.modem, sys_loop_stack, nvs)?;

    let wifi_res = wifi_setup_and_connect(&mut wifi);
    match wifi_res {
        Ok(()) => println!("Connected to wifi!"),
        Err(err) => println!("Failed to connect: {}", err),
    }

    loop {
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(500));
        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(500));
    }
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

fn wifi_setup_and_connect(wifi:&mut EspWifi) -> Result<()> {
    let scan_result = wifi.scan()?;
    let home_network = scan_result.into_iter().find(|a| a.ssid == SSID);
    if home_network.is_some() {
        println!("Detected home network {:?}", SSID);
    }
    else {
        println!("Error: Failed to detect home network {:?}", SSID);
        return Err(anyhow!(WifiErrors::CouldNotFindNetwork));
    }

    wifi.set_configuration(&wifi::Configuration::Client(
        wifi::ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            // channel,
            ..Default::default()
        }
    ))?;

    wifi.start()?;
    wifi.connect()?;

    Ok(())
}