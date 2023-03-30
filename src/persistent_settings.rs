/*
 * MIT license
 *
 * Copyright (c) 2021-2023 Michael Zill
 *
 * https://github.com/taunusflieger/anemometer/blob/master/anemometer-production/src/configuration.rs 
 */

use anyhow::{anyhow,Result, Context};
use log::*;

use esp_idf_svc::nvs::*;
use esp_idf_sys::*;

const NVS_STRING_READ_BUFFER_SIZE: usize = 180;

#[derive(Debug)]
pub struct WifiConfiguration {
    pub ssid: String,
    pub password: String,
}

impl WifiConfiguration {
    pub fn new(ssid: &str, password: &str) -> Self {
        debug!("Create empty wifi config");
        Self { ssid: String::from(ssid), password: String::from(password) }
    }

    pub fn do_exist() -> Result<bool> {
        let memory_partition = EspCustomNvsPartition::take("nvs").context("Partition `nvs`doesn't exist")?;
        let nvs = EspCustomNvs::new(memory_partition.clone(), "wifi_config", false).context("Partition `nvs` doesn't have a `wifi_config` namespace")?;

        if let Some(len) = nvs.length_str("ssid")? {
            Ok(len != 0)
        } else {
            Err(anyhow!("Wifi config not found in nvs"))
        }
    }

    pub fn load() -> Result<Self> {
        let memory_partition = EspCustomNvsPartition::take("nvs").context("Partition `nvs`doesn't exist")?;
        let nvs = EspCustomNvs::new(memory_partition.clone(), "wifi_config", false).context("Partition `nvs` doesn't have a `wifi_config` namespace")?;

        let mut nvs_str_buffer: [u8;100] = [0; 100];
        nvs.get_str("dummy", &mut nvs_str_buffer)?;

        Ok(Self{
            ssid: get_string_from_nvs(&nvs, "ssid")?,
            password: get_string_from_nvs(&nvs, "password")?,
        })
    }

    pub fn store(&mut self) -> Result<()> {
        let memory_partition = EspCustomNvsPartition::take("nvs").context("Partition `nvs`doesn't exist")?;
        let mut nvs = EspCustomNvs::new(memory_partition.clone(), "wifi_config", true).context("Partition `nvs` doesn't have a `wifi_config` namespace")?;

        nvs.set_str("ssid", &self.ssid)?;
        nvs.set_str("password", &self.password)?;

        Ok(())
    }
}

fn get_string_from_nvs(nvs: &EspCustomNvs, key: &str) -> Result<String, EspError> {
    let mut nvm_str_buffer: [u8; NVS_STRING_READ_BUFFER_SIZE] = [0; NVS_STRING_READ_BUFFER_SIZE];
    nvs.get_str(key, &mut nvm_str_buffer)?;

    // remove any tailing zeros
    Ok(String::from(
        core::str::from_utf8(
            &(nvm_str_buffer[0..nvm_str_buffer.iter().position(|&x| x == 0).unwrap()]),
        )
        .unwrap(),
    ))
}

use core::ptr;
use std::ffi::CString;
use esp_idf_svc::handle::RawHandle;

pub trait EspNvsExtention {
    fn length_str(&self, name: &str) -> Result<Option<usize>, EspError>;
    fn get_str<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, EspError>;
    fn set_str(&mut self, name: &str, val: &str) -> Result<bool, EspError>;
}

impl EspNvsExtention for EspCustomNvs {
    fn length_str(&self, name: &str) -> Result<Option<usize>, EspError> {
        let c_key = CString::new(name).unwrap();

        #[allow(unused_assignments)]
        let mut len = 0;

        match unsafe {
            nvs_get_str(
                self.handle(),
                c_key.as_ptr(),
                ptr::null_mut(),
                &mut len as *mut _,
            )
        } {
            ESP_ERR_NVS_NOT_FOUND => Ok(None),
            err => {
                // bail on error
                esp!(err)?;

                Ok(Some(len))
            }
        }
    }

    fn get_str<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, EspError> {
        let c_key = CString::new(name).unwrap();

        #[allow(unused_assignments)]
        let mut len = 0;
        match unsafe {
            len = buf.len();
            nvs_get_str(
                self.handle(),
                c_key.as_ptr(),
                buf.as_mut_ptr() as *mut _,
                &mut len as *mut _,
            )
        } {
            ESP_ERR_NVS_NOT_FOUND => Ok(None),
            err => {
                // bail on error
                esp!(err)?;

                Ok(Some(buf))
            }
        }
    }

    fn set_str(&mut self, name: &str, val: &str) -> Result<bool, EspError> {
        let c_key = CString::new(name).unwrap();
        let c_val = CString::new(val).unwrap();

        // start by just clearing this key
        unsafe { nvs_erase_key(self.handle(), c_key.as_ptr()) };

        esp!(unsafe { nvs_set_str(self.handle(), c_key.as_ptr(), c_val.as_ptr(),) })?;

        esp!(unsafe { nvs_commit(self.handle()) })?;

        Ok(true)
    }
}