/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use log::*;

use behaviour::*;
use configuration::{Configuration, ConfigurationManager, PersistentStorage};
use display::Display;
use time_source::TimeSource;

pub mod behaviour;
pub mod configuration;
pub mod display;
pub mod time;
pub mod time_source;
pub mod version;

pub struct Application<D: Display, T: TimeSource, S: PersistentStorage> {
    pub display: D,
    pub time_source: T,
    pub configuration: Configuration,
    pub configuration_manager: ConfigurationManager<S>,
    behaviour: Behaviour,
    event_queue: Vec<Event>,
}

impl<D: Display, T: TimeSource, S: PersistentStorage> Application<D, T, S> {
    pub fn new(mut display: D, time_source: T, persistent_storage: S) -> Self {
        display.clear().unwrap();
        Application {
            display,
            time_source,
            configuration: Configuration::default(),
            configuration_manager: ConfigurationManager::new(persistent_storage),
            behaviour: Behaviour::new(),
            event_queue: vec![],
        }
    }

    pub fn publish_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    pub fn run(&mut self) {
        // loop {
        if let Some(event) = self.event_queue.pop() {
            self.behaviour.handle_event(event);
            self.state_action();
        }
        // }
    }

    pub fn state_action(&mut self) {
        match self.behaviour.current_state() {
            State::Startup => self.startup(),
            State::DisplayTime => self.display_time(),
            State::Configuration => self.configuration(),
            _ => self.error(),
        }
    }

    pub fn get_current_state(&self) -> State {
        self.behaviour.current_state()
    }

    fn startup(&mut self) {
        info!("Startup...");
        let _ = self.display.draw_progress(1);

        self.configuration = self.configuration_manager.load_from_persistent_storage();
        if self.configuration.is_valid() {
            debug!("Valid configuration");
            self.publish_event(Event::ValidConfiguration);
        } else {
            warn!("No valid configuration in persistent storage");
            self.publish_event(Event::InvalidConfiguration);
        }
    }

    fn configuration(&mut self) {
        info!("Configuration");
        let _ = self.display.draw_progress(2);

        if self.configuration.is_valid() {
            self.publish_event(Event::ValidConfiguration);
        } else {
            // todo!("Start configuration server");
            warn!("Use hard-coded config!");
            let hard_coded_config = Configuration::new(env!("RUST_ESP32_WIFI_SSID").to_string(), env!("RUST_ESP32_WIFI_PASSWORD").to_string() );
            self.configuration = hard_coded_config;
            // let _ = self.configuration_manager.store_to_persistent_storage(self.configuration.clone());
            self.publish_event(Event::ValidConfiguration);
        }
    }

    fn display_time(&mut self) {
        let time = self.time_source.get_time().unwrap();
        info!("Displaying time: {}", time);

        let _ = self.display.draw_time(time);
    }

    fn error(&mut self) {
        info!("Startup action");
        let _ = self.display.draw_error();
    }
}
