/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use application::behaviour::*;
use application::configuration::Configuration;
use application::*;

#[derive(PartialEq, Debug)]
enum FakeDisplayState {
    Clean,
    Progress(u8),
    Error,
    Time(time::Time),
}
struct FakeDisplay {
    state: FakeDisplayState,
}

impl display::Display for FakeDisplay {
    fn clear(&mut self) -> anyhow::Result<()> {
        self.state = FakeDisplayState::Clean;
        Ok(())
    }
    fn draw_error(&mut self) -> anyhow::Result<()> {
        self.state = FakeDisplayState::Error;
        Ok(())
    }
    fn draw_progress(&mut self, progress: u8) -> anyhow::Result<()> {
        self.state = FakeDisplayState::Progress(progress);
        Ok(())
    }
    fn draw_time(&mut self, time: time::Time) -> anyhow::Result<()> {
        self.state = FakeDisplayState::Time(time);
        Ok(())
    }
}

struct MockTime {
    curent: time::Time,
}

impl MockTime {
    fn set_time(&mut self, time: time::Time) {
        self.curent = time;
    }
}

impl time_source::TimeSource for MockTime {
    fn get_time(&self) -> Result<time::Time> {
        Ok(self.curent)
    }
}

struct FakePersistentStorage {
    string_storage: HashMap<String, String>,
}

impl configuration::PersistentStorage for FakePersistentStorage {
    fn load_string(&mut self, key: &str) -> Result<String> {
        if self.string_storage.contains_key(key) {
            Ok(self.string_storage.get(key).unwrap().to_string())
        } else {
            Err(anyhow!("invalid query"))
        }
    }

    fn store_string(&mut self, key: &str, value: &str) -> Result<()> {
        self.string_storage
            .insert(key.to_string(), value.to_string());
        Ok(())
    }
}

fn get_application() -> Application<FakeDisplay, MockTime, FakePersistentStorage> {
    let display = FakeDisplay {
        state: FakeDisplayState::Clean,
    };
    let time_source = MockTime {
        curent: time::Time::new(11, 22, 33).unwrap(),
    };
    let persistent_storage = FakePersistentStorage {
        string_storage: HashMap::new(),
    };
    Application::new(display, time_source, persistent_storage)
}

#[test]
fn display_time() {
    let mut app = get_application();
    assert_eq!(app.display.state, FakeDisplayState::Clean);

    app.publish_event(Event::Start);
    app.run();
    assert_eq!(app.get_current_state(), State::Startup);

    app.publish_event(Event::ValidConfiguration);
    app.run();
    assert_eq!(
        app.display.state,
        FakeDisplayState::Time(time::Time::new(11, 22, 33).unwrap())
    );

    app.time_source
        .set_time(time::Time::new(11, 23, 33).unwrap());
    app.publish_event(Event::Tick);
    app.run();
    assert_eq!(
        app.display.state,
        FakeDisplayState::Time(time::Time::new(11, 23, 33).unwrap())
    );
}

#[test]
fn invalid_config_start_configuration() {
    let mut app = get_application();
    assert!(app.configuration.is_invalid());

    app.publish_event(Event::Start);
    app.run();
    assert_eq!(app.get_current_state(), State::Startup);

    app.run();
    assert_eq!(app.get_current_state(), State::Configuration);
}

#[test]
fn valid_config_start_displaying_time() {
    let mut app = get_application();
    let configuration = Configuration::new(String::from("home wifi"), String::from("secret"));
    app.configuration_manager
        .store_to_persistent_storage(configuration)
        .unwrap();
    assert!(app.configuration.is_invalid());

    app.publish_event(Event::Start);
    app.run();
    assert_eq!(app.get_current_state(), State::Startup);

    app.run();
    assert_eq!(app.get_current_state(), State::DisplayTime);
}
