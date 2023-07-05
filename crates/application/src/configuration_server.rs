/* SPDX-License-Identifier: MIT
* Copyright (c) 2023 Louis Mayencourt
*/

/// Interface to get a new pending configuration
pub trait ConfigurationServer {
    // Check if a new configuration was received from the user
    // Used as a condition to exit configuration mode.
    fn is_configuration_received(&self) -> bool;

    // Return the HTTP URI query string if `is_configuration_received()` returned `true`
    fn get_config_uri(&self) -> Option<String>;
}