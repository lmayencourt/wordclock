/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::collections::VecDeque;
use log::*;
use std::time::Duration;
use std::thread;

use behaviour::*;
use configuration::{Configuration, ConfigurationManager, PersistentStorage};
use configuration_server::ConfigurationServer;
use display::Display;
use network::Network;
use time_source::TimeSource;

pub mod behaviour;
pub mod configuration;
pub mod configuration_server;
pub mod display;
pub mod network;
pub mod time;
pub mod time_source;
pub mod version;

pub struct Application<D: Display, T: TimeSource, S: PersistentStorage, N: Network, C: ConfigurationServer> {
    pub display: D,
    pub time_source: T,
    pub configuration: Configuration,
    pub configuration_manager: ConfigurationManager<S>,
    pub network: N,
    pub configuration_server: C,
    behaviour: Behaviour,
    event_queue: VecDeque<Event>,
}

impl<D: Display, T: TimeSource, S: PersistentStorage, N: Network, C: ConfigurationServer> Application<D, T, S, N, C> {
    pub fn new(mut display: D, time_source: T, persistent_storage: S, network: N, configuration_server: C) -> Self {
        display.clear().unwrap();
        Application {
            display,
            time_source,
            configuration: Configuration::default(),
            configuration_manager: ConfigurationManager::new(persistent_storage),
            network,
            configuration_server,
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
            State::MenuFota => (),
            State::MenuCleanConfig => (),
            State::MenuExit => (),
            State::Fota => self.firmate_update(),
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

            if let Err(error) = self.network.configure(&self.configuration.get_ssid().unwrap(), &self.configuration.get_password().unwrap()) {
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
            // Network is already configured as access point by main.rs

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
                    },
                    Err(e) => error!("failed to parse config uri: {}", e),
                }
            }

            // warn!("Use hard-coded config!");
            // let hard_coded_config = Configuration::new(env!("RUST_ESP32_WIFI_SSID").to_string(), env!("RUST_ESP32_WIFI_PASSWORD").to_string(), None, None);
            // self.configuration = hard_coded_config;
            let _ = self.configuration_manager.store_to_persistent_storage(self.configuration.clone());

            self.publish_event(Event::ValidConfiguration);
        }
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

    fn firmate_update(&mut self) {

    }

    fn clean_config(&mut self) {
        self.configuration = Configuration::default();
        self.publish_event(Event::InvalidConfiguration);
    }

    fn error(&mut self) {
        let _ = self.display.draw_error();
    }
}
