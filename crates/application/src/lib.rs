/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use log::*;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

use anyhow::Result;

use behaviour::*;
use configuration::{Configuration, ConfigurationManager, PersistentStorage};
use configuration_server::ConfigurationServer;
use display::Display;
use firmware_update::FirmwareUpdate;
use network::Network;
use power_manager::PowerManager;
use time_source::TimeSource;

pub mod behaviour;
pub mod build_version;
pub mod color;
pub mod configuration;
pub mod configuration_server;
pub mod display;
pub mod firmware_update;
pub mod network;
pub mod power_manager;
pub mod time;
pub mod time_monotonic;
pub mod time_source;
pub mod time_source_manager;
pub mod version;

pub struct Application<
    D: Display,
    T: TimeSource,
    S: PersistentStorage,
    N: Network,
    C: ConfigurationServer,
    P: PowerManager,
    F: FirmwareUpdate,
> {
    pub display: D,
    pub time_source: T,
    pub configuration: Configuration,
    pub configuration_manager: ConfigurationManager<S>,
    pub network: N,
    pub configuration_server: C,
    pub power_manager: P,
    pub firmware_update: F,
    behaviour: Behaviour,
    event_queue: VecDeque<Event>,
}

impl<
        D: Display,
        T: TimeSource,
        S: PersistentStorage,
        N: Network,
        C: ConfigurationServer,
        P: PowerManager,
        F: FirmwareUpdate,
    > Application<D, T, S, N, C, P, F>
{
    pub fn new(
        mut display: D,
        time_source: T,
        persistent_storage: S,
        network: N,
        configuration_server: C,
        power_manager: P,
        firmware_update: F,
    ) -> Self {
        display.clear().unwrap();
        Application {
            display,
            time_source,
            configuration: Configuration::default(),
            configuration_manager: ConfigurationManager::new(persistent_storage),
            network,
            configuration_server,
            power_manager,
            firmware_update,
            behaviour: Behaviour::new(),
            event_queue: VecDeque::new(),
        }
    }

    pub fn publish_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    pub fn run(&mut self) {
        // loop {
        if let Some(event) = self.event_queue.pop_front() {
            info!("Handling event {:?}", event);
            self.behaviour.handle_event(event);
            self.state_action();
        }
        // }
    }

    pub fn state_action(&mut self) {
        info!("Executing {:?} action", self.behaviour.current_state());
        match self.behaviour.current_state() {
            State::Startup => self.startup(),
            State::DisplayTime => self.display_time(),
            State::NightMode => self.night_mode(),
            State::Configuration => self.configuration(),
            State::MenuFota => {
                let _ = self.display.draw_progress(1);
            }
            State::MenuCleanConfig => {
                let _ = self.display.draw_progress(2);
            }
            State::MenuExit => {
                let _ = self.display.draw_progress(3);
            }
            State::Fota => self.firmware_update(),
            State::CleanConfig => self.clean_config(),
            _ => self.error(),
        }
        info!("{:?} action Done", self.behaviour.current_state());
    }

    pub fn get_current_state(&self) -> State {
        self.behaviour.current_state()
    }

    fn startup(&mut self) {
        let _ = self.display.draw_progress(1);

        if self.configuration.is_invalid() {
            info!("Load configuration");
            self.configuration = self.configuration_manager.load_from_persistent_storage();
        }

        if self.configuration.is_valid() {
            info!("Valid configuration");

            self.display.set_default_color(self.configuration.get_display_color().unwrap());

            if let Err(error) = self.network.configure(
                &self.configuration.get_ssid().unwrap(),
                &self.configuration.get_password().unwrap(),
            ) {
                error!("Failed to configure wifi: {}", error);
                self.publish_event(Event::Error);
            }
            if let Err(error) = self.network.connect() {
                error!("Failed to connect to network: {}", error);
                self.publish_event(Event::Error);
            }
            if let Err(_) = self.time_source.synchronize() {
                error!("Failed to synch time source");
                self.publish_event(Event::Error);
            }
            if let Err(error) = self.network.disconnect() {
                error!("Failed to disconnect to network: {}", error);
                self.publish_event(Event::Error);
            }
            self.publish_event(Event::Start);
        } else {
            warn!("No valid configuration in persistent storage");
            self.publish_event(Event::InvalidConfiguration);
        }
    }

    fn configuration(&mut self) {
        let _ = self.display.draw_progress(2);

        if self.configuration.is_valid() {
            self.publish_event(Event::ValidConfiguration);
        } else {
            // Network is already configured as access point by main.rs, see Anomaly-002

            loop {
                thread::sleep(Duration::from_millis(200));
                if self.configuration_server.is_configuration_received() {
                    break;
                }
            }

            let config_uri = self.configuration_server.get_config_uri();
            if let Some(uri) = config_uri {
                match Configuration::from_uri_query_string(&uri) {
                    Ok(config) => {
                        info!("New config is {:?}", config);
                        self.configuration = config;
                    }
                    Err(e) => {
                        error!("failed to parse config uri: {}", e);
                        self.publish_event(Event::Error);
                        return;
                    }
                }
            }

            if let Err(e) = self.verify_network_configuration() {
                error!("Invalid Network configuration provided: {}", e);
                self.publish_event(Event::Error);
                return;
            }

            if let Err(e) = self
                .configuration_manager
                .store_to_persistent_storage(self.configuration.clone())
            {
                error!("Failed to write to persistent storage: {}", e);
                self.publish_event(Event::Error);
            }

            self.publish_event(Event::ValidConfiguration);
        }
    }

    fn verify_network_configuration(&mut self) -> Result<()>{
        if let Err(error) = self.network.configure(
            &self.configuration.get_ssid().unwrap(),
            &self.configuration.get_password().unwrap(),
        ) {
            error!("Failed to configure wifi: {}", error);
            self.publish_event(Event::Error);
        }
        if let Err(error) = self.network.connect() {
            error!("Failed to connect to network: {}", error);
            self.publish_event(Event::Error);
        }
        if let Err(error) = self.network.disconnect() {
            error!("Failed to disconnect to network: {}", error);
            self.publish_event(Event::Error);
        }

        Ok(())
    }

    fn display_time(&mut self) {
        let time = self.time_source.get_time().unwrap();
        info!("Displaying time: {}", time);

        let _ = self.display.draw_time(time);

        if let Some(night_start) = self.configuration.get_night_start() {
            if time.hour >= night_start.hour {
                if time.minute >= night_start.minute {
                    self.publish_event(Event::Night);
                }
            }
        }
    }

    fn night_mode(&mut self) {
        let time = self.time_source.get_time().unwrap();
        info!("Currently in night {}", time);

        if let Some(night_start) = self.configuration.get_night_start() {
            if time.hour >= night_start.hour {
                if time.minute >= night_start.minute {
                    self.publish_event(Event::Day);
                }
            }
        }
    }

    fn firmware_update(&mut self) {
        if let Err(error) = self.network.connect() {
            error!("Failed to connect to network: {}", error);
            self.publish_event(Event::Error);
        }

        match self.firmware_update.read_update_version() {
            Ok(version) => info!("Available version {}", version),
            Err(e) => {
                error!("Failed to read update version: {}", e);
                self.publish_event(Event::Error);
            }
        }

        match self.firmware_update.download_update() {
            Ok(()) => {
                info!("Update ready, restart device");
                // esp_idf_hal::delay::FreeRtos::delay_ms(5000);

                if let Err(error) = self.network.disconnect() {
                    error!("Failed to disconnect to network: {}", error);
                    self.publish_event(Event::Error);
                }

                self.firmware_update.reboot_to_new_image();
            }
            Err(e) => {
                error!("Failed to download update: {}", e);
                self.publish_event(Event::Error);
            }
        }
    }

    fn clean_config(&mut self) {
        self.configuration = Configuration::default();
        if let Ok(()) = self.configuration_manager.clean_persistent_storage() {
            self.publish_event(Event::InvalidConfiguration);
            self.power_manager.reset();
        } else {
            self.publish_event(Event::Error);
        }
    }

    fn error(&mut self) {
        let _ = self.display.draw_error();
    }
}
