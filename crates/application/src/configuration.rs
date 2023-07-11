/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::str::FromStr;

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::time::{Time, TIME_STRING_LENGTH};

/// Key entries used as index for persistent storage.
const WIFI_SSID_KEY: &str = "wifi_ssid";
const WIFI_PASSWORD_KEY: &str = "wifi_password";
const NIGHT_START_KEY: &str = "night_start";
const NIGHT_END_KEY: &str = "night_end";

/// REGEX used to parse the http get query string containing the configuration
const CONFIGURATION_QUERY_STRING_REGEX: &str = r"^\/get\?input_wifi_ssid=(?P<ssid>.*)&input_wifi_password=(?P<password>.*)&input_night_mode_start=(?P<night_start>[\%3A0-9]*)&input_night_mode_end=(?P<night_end>[\%3A0-9]*)";

#[derive(Debug, Clone, PartialEq)]
struct ConfigurationFields {
    ssid: String,
    password: String,
    night_start: Option<Time>,
    night_end: Option<Time>,
}

#[derive(Debug, Clone, PartialEq)]
enum ConfigurationState {
    Invalid,
    Valid(ConfigurationFields),
}

/// Device configuration data type, representing validity and configuration fields.
#[derive(Debug, Clone, PartialEq)]
pub struct Configuration {
    state: ConfigurationState,
}

impl Configuration {
    /// Create a new valid configuration
    pub fn new(
        ssid: String,
        password: String,
        night_start: Option<Time>,
        night_end: Option<Time>,
    ) -> Self {
        Self {
            state: ConfigurationState::Valid(ConfigurationFields {
                ssid,
                password,
                night_start,
                night_end,
            }),
        }
    }

    pub fn from_uri_query_string(uri: &str) -> Result<Self> {
        let re = Regex::new(CONFIGURATION_QUERY_STRING_REGEX)?;

        let mut night_start: Option<Time> = None;
        let mut night_end: Option<Time> = None;
        if let Some(cap) = re.captures(uri) {
            let utf8_encoded_value: String = cap[1].parse()?;
            let ssid: String = url_escape::decode(&utf8_encoded_value).to_string();
            let utf8_encoded_value: String = cap[2].parse()?;
            let password: String = url_escape::decode(&utf8_encoded_value).to_string();
            if let Some(value) = cap.name("night_start") {
                if value.len() >= 5 {
                    let mut time = String::from(url_escape::decode(value.as_str()));
                    time.push_str(":00");
                    night_start = Some(Time::from_str(&time)?);
                }
            }
            if let Some(value) = cap.name("night_end") {
                if value.len() >= 5 {
                    let mut time = String::from(url_escape::decode(value.as_str()));
                    time.push_str(":00");
                    night_end = Some(Time::from_str(&time)?);
                }
            }

            Ok(Configuration {
                state: ConfigurationState::Valid(ConfigurationFields {
                    ssid,
                    password,
                    night_start,
                    night_end,
                }),
            })
        } else {
            Err(anyhow!("Failed to parse query string: {}", uri))
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.state {
            ConfigurationState::Valid(_) => true,
            _ => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self.state {
            ConfigurationState::Invalid => true,
            _ => false,
        }
    }

    pub fn get_ssid(&self) -> Option<String> {
        match &self.state {
            ConfigurationState::Valid(fields) => Some(fields.ssid.clone()),
            _ => None,
        }
    }

    pub fn get_password(&self) -> Option<String> {
        match &self.state {
            ConfigurationState::Valid(fields) => Some(fields.password.clone()),
            _ => None,
        }
    }

    pub fn get_night_start(&self) -> Option<Time> {
        match &self.state {
            ConfigurationState::Valid(fields) => fields.night_start,
            _ => None,
        }
    }

    pub fn get_night_end(&self) -> Option<Time> {
        match &self.state {
            ConfigurationState::Valid(fields) => fields.night_end,
            _ => None,
        }
    }
}

impl Default for Configuration {
    /// Default constructor, returning an invalid Configuration.
    fn default() -> Self {
        Self {
            state: ConfigurationState::Invalid,
        }
    }
}

/// Interface to store and load data to persistent memory.
pub trait PersistentStorage {
    /// Load a string identified by the provided key.
    ///
    /// # Error
    /// The functions will return an error if the hardware fails to carry the operation.
    fn load_string(&mut self, key: &str) -> Result<String>;

    /// Store a string identified by the provided key.
    ///
    /// # Error
    /// The functions will return an error if the hardware fails to carry the operation.
    fn store_string(&mut self, key: &str, value: &str) -> Result<()>;
}

/// Storage agnostic interface to load and store a Configuration.
pub struct ConfigurationManager<PersistentStorage> {
    storage_backend: PersistentStorage,
}

impl<P: PersistentStorage> ConfigurationManager<P> {
    pub fn new(storage_backend: P) -> Self {
        Self { storage_backend }
    }

    /// Load a Configuration from persistent storage.
    ///
    /// Returned Configuration can be in `Invalid`.
    pub fn load_from_persistent_storage(&mut self) -> Configuration {
        let ssid: String;
        match self.storage_backend.load_string(WIFI_SSID_KEY) {
            Ok(value) => ssid = value,
            _ => {
                return Configuration {
                    state: ConfigurationState::Invalid,
                }
            }
        }

        let password: String;
        match self.storage_backend.load_string(WIFI_PASSWORD_KEY) {
            Ok(value) => password = value,
            _ => {
                return Configuration {
                    state: ConfigurationState::Invalid,
                }
            }
        }

        let night_start: Option<Time>;
        match self.storage_backend.load_string(NIGHT_START_KEY) {
            Ok(value) => {
                if value.len() == TIME_STRING_LENGTH {
                    night_start = Some(Time::from_str(&value).unwrap());
                } else {
                    night_start = None;
                }
            }
            _ => {
                return Configuration {
                    state: ConfigurationState::Invalid,
                }
            }
        }

        let night_end: Option<Time>;
        match self.storage_backend.load_string(NIGHT_END_KEY) {
            Ok(value) => {
                if value.len() == TIME_STRING_LENGTH {
                    night_end = Some(Time::from_str(&value).unwrap());
                } else {
                    night_end = None;
                }
            }
            _ => {
                return Configuration {
                    state: ConfigurationState::Invalid,
                }
            }
        }

        Configuration {
            state: ConfigurationState::Valid(ConfigurationFields {
                ssid,
                password,
                night_start,
                night_end,
            }),
        }
    }

    /// Store the given Configuration to persistent memory.
    ///
    /// # Error
    /// The functions will return an error if the hardware fails to carry the operation.
    pub fn store_to_persistent_storage(&mut self, configuration: Configuration) -> Result<()> {
        if configuration.is_valid() {
            // It is safe to unwrap here, as configuration is guarantee to be valid.
            self.storage_backend
                .store_string(WIFI_SSID_KEY, &configuration.get_ssid().unwrap())?;
            self.storage_backend
                .store_string(WIFI_PASSWORD_KEY, &configuration.get_password().unwrap())?;
            if let Some(night_start) = configuration.get_night_start() {
                self.storage_backend
                    .store_string(NIGHT_START_KEY, night_start.to_string().as_str())?;
            }
            if let Some(night_end) = configuration.get_night_end() {
                self.storage_backend
                    .store_string(NIGHT_END_KEY, night_end.to_string().as_str())?;
            }
        } else {
            return Err(anyhow!("Can't store invalid configuration"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn from_uri_query_string() {
        let config = Configuration::from_uri_query_string("/get?input_wifi_ssid=myhomenetwork&input_wifi_password=1234&input_night_mode_start=&input_night_mode_end=").unwrap();
        assert_eq!(
            Configuration {
                state: ConfigurationState::Valid(ConfigurationFields {
                    ssid: String::from("myhomenetwork"),
                    password: String::from("1234"),
                    night_start: None,
                    night_end: None
                }),
            },
            config
        );
    }

    #[test]
    fn from_uri_query_string_with_night_mode() {
        let config = Configuration::from_uri_query_string("/get?input_wifi_ssid=myhomenetwork&input_wifi_password=1234&input_night_mode_start=23%3A30&input_night_mode_end=04%3A40").unwrap();
        assert_eq!(
            Configuration {
                state: ConfigurationState::Valid(ConfigurationFields {
                    ssid: String::from("myhomenetwork"),
                    password: String::from("1234"),
                    night_start: Some(Time::new(23, 30, 0).unwrap()),
                    night_end: Some(Time::new(4, 40, 0).unwrap()),
                }),
            },
            config
        );
    }

    #[test]
    fn from_uri_query_string_with_special_chars() {
        let config = Configuration::from_uri_query_string("/get?input_wifi_ssid=Solnet-1234&input_wifi_password=Secret%40-7&input_night_mode_start=&input_night_mode_end=").unwrap();
        assert_eq!(
            Configuration {
                state: ConfigurationState::Valid(ConfigurationFields {
                    ssid: String::from("Solnet-1234"),
                    password: String::from("Secret@-7"),
                    night_start: None,
                    night_end: None,
                }),
            },
            config
        );
    }
}
