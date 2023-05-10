/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use anyhow::{anyhow, Result};

/// Key entries used as index for persistent storage.
const WIFI_SSID_KEY: &str = "wifi_ssid";
const WIFI_PASSWORD_KEY: &str = "wifi_password";

#[derive(Clone)]
struct ConfigurationFields {
    ssid: String,
    password: String,
}

enum ConfigurationState {
    Invalid,
    Valid(ConfigurationFields),
}

/// Device configuration data type, representing validity and configuration fields.
pub struct Configuration {
    state: ConfigurationState,
}

impl Configuration {
    /// Create a new valid configuration
    pub fn new(ssid: String, password: String) -> Self {
        Self {
            state: ConfigurationState::Valid(ConfigurationFields { ssid, password }),
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

        Configuration {
            state: ConfigurationState::Valid(ConfigurationFields { ssid, password }),
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
        } else {
            return Err(anyhow!("Can't store invalid configuration"));
        }

        Ok(())
    }
}
