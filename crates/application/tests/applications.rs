/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use application::behaviour::*;
use application::color::Color;
use application::configuration::Configuration;
use application::configuration_server::ConfigurationServer;
use application::firmware_update::FirmwareUpdate;
use application::power_manager::PowerManager;
use application::time_source::TimeSourceError;
use application::*;
use time::Time;

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
    fn synchronize(&mut self) -> Result<(), TimeSourceError> {
        Ok(())
    }

    fn get_time(&self) -> Result<time::Time, TimeSourceError> {
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

struct FakeNetwork {
    is_configured: bool,
    is_connected: bool,
    is_access_point: bool,
}

impl network::Network for FakeNetwork {
    fn configure(&mut self, _ssid: &str, _password: &str) -> Result<()> {
        self.is_configured = true;
        Ok(())
    }

    fn connect(&mut self) -> Result<()> {
        if self.is_configured == false {
            return Err(anyhow!("Network not configured properly"));
        }

        self.is_access_point = false;
        self.is_connected = true;
        Ok(())
    }

    fn disconnect(&mut self) -> Result<()> {
        self.is_connected = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.is_connected
    }

    fn setup_access_point(&mut self, ssid: &str) -> Result<()> {
        self.is_connected = false;
        self.is_access_point = true;
        Ok(())
    }
}

struct FakeConfigServer {
    is_config_received: bool,
}

impl FakeConfigServer {
    fn set_receive_config(&mut self) {
        self.is_config_received = true;
    }
}

impl ConfigurationServer for FakeConfigServer {
    fn is_configuration_received(&self) -> bool {
        self.is_config_received
    }

    fn get_config_uri(&mut self) -> Option<String> {
        if self.is_config_received {
            self.is_config_received = false;
            Some(String::from("/get?input_wifi_ssid=myhomenetwork&input_wifi_password=1234&input_night_mode_start=23%3A30&input_night_mode_end=04%3A40&favcolor=%2300ff00"))
        } else {
            None
        }
    }
}

struct FakePowerManager;

impl PowerManager for FakePowerManager {
    fn reset(&self) {}
}

struct FakeFirmwareUpdate;

impl FirmwareUpdate for FakeFirmwareUpdate {
    fn read_update_version(&self) -> Result<version::Version> {
        Ok(version::Version::new(1, 1, 0, None))
    }

    fn download_update(&self) -> Result<()> {
        Ok(())
    }

    fn reboot_to_new_image(&self) {}
}

fn get_application() -> Application<
    FakeDisplay,
    MockTime,
    FakePersistentStorage,
    FakeNetwork,
    FakeConfigServer,
    FakePowerManager,
    FakeFirmwareUpdate,
> {
    let display = FakeDisplay {
        state: FakeDisplayState::Clean,
    };
    let time_source = MockTime {
        curent: time::Time::new(11, 22, 33).unwrap(),
    };
    let persistent_storage = FakePersistentStorage {
        string_storage: HashMap::new(),
    };
    let network = FakeNetwork {
        is_configured: false,
        is_connected: false,
        is_access_point: true, // to reflect Anomaly-002
    };
    let configuration_server = FakeConfigServer {
        is_config_received: false,
    };
    let power_manager = FakePowerManager;
    let firmware_update = FakeFirmwareUpdate;

    Application::new(
        display,
        time_source,
        persistent_storage,
        network,
        configuration_server,
        power_manager,
        firmware_update,
    )
}

fn run_startup(
    app: &mut Application<
        FakeDisplay,
        MockTime,
        FakePersistentStorage,
        FakeNetwork,
        FakeConfigServer,
        FakePowerManager,
        FakeFirmwareUpdate,
    >,
) {
    assert_eq!(app.get_current_state(), State::Initial);
    app.publish_event(Event::Init);
    app.run();
    assert_eq!(app.get_current_state(), State::Startup);
}

fn preset_configuration(
    app: &mut Application<
        FakeDisplay,
        MockTime,
        FakePersistentStorage,
        FakeNetwork,
        FakeConfigServer,
        FakePowerManager,
        FakeFirmwareUpdate,
    >,
) {
    let configuration = Configuration::new(
        String::from("home wifi"),
        String::from("secret"),
        Some(Time::new(22, 0, 0).unwrap()),
        Some(Time::new(4, 30, 0).unwrap()),
        Color::new(0, 0, 0),
    );
    app.configuration_manager
        .store_to_persistent_storage(configuration)
        .unwrap();
}

fn goto_display_time(
    app: &mut Application<
        FakeDisplay,
        MockTime,
        FakePersistentStorage,
        FakeNetwork,
        FakeConfigServer,
        FakePowerManager,
        FakeFirmwareUpdate,
    >,
) {
    preset_configuration(app);
    run_startup(app);
    app.run();
    assert_eq!(app.get_current_state(), State::DisplayTime);
}

fn goto_menu(
    app: &mut Application<
        FakeDisplay,
        MockTime,
        FakePersistentStorage,
        FakeNetwork,
        FakeConfigServer,
        FakePowerManager,
        FakeFirmwareUpdate,
    >,
) {
    goto_display_time(app);
    app.publish_event(Event::EnterShortPush);
    app.run();
    assert_eq!(app.get_current_state(), State::MenuFota);
}

#[test]
fn display_time() {
    let mut app = get_application();
    assert_eq!(app.display.state, FakeDisplayState::Clean);
    goto_display_time(&mut app);

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
    app.configuration_server.set_receive_config();

    run_startup(&mut app);
    assert!(app.network.is_access_point);

    app.run();
    assert_eq!(app.get_current_state(), State::Configuration);
    assert!(app.network.is_configured);
}

#[test]
fn valid_config_start_displaying_time() {
    let mut app = get_application();
    preset_configuration(&mut app);
    assert!(app.configuration.is_invalid());

    run_startup(&mut app);
    assert!(app.configuration.is_valid());

    app.run();
    assert_eq!(app.get_current_state(), State::DisplayTime);
}

#[test]
fn valid_config_from_configuration_start_displaying_time() {
    let mut app = get_application();
    assert!(app.configuration.is_invalid());

    run_startup(&mut app);

    app.configuration_server.set_receive_config();
    app.run();
    assert_eq!(app.get_current_state(), State::Configuration);

    app.run();
    assert_eq!(app.get_current_state(), State::Startup);
    assert_eq!(app.configuration_server.get_config_uri(), None);

    app.run();
    assert_eq!(app.get_current_state(), State::DisplayTime);
}

#[test]
fn network_is_ready_in_display_time() {
    let mut app = get_application();
    // // app.time_source.set_time(None);
    assert_eq!(app.network.is_connected, false);
    goto_display_time(&mut app);

    assert!(app.network.is_configured);
}

#[test]
fn enter_menu_with_short_push() {
    let mut app = get_application();
    goto_display_time(&mut app);

    app.publish_event(Event::EnterShortPush);
    app.publish_event(Event::Tick);
    app.run();

    assert_eq!(app.get_current_state(), State::MenuFota);
}

#[test]
fn enter_menu_with_long_push() {
    let mut app = get_application();
    goto_display_time(&mut app);

    app.publish_event(Event::EnterLongPush);
    app.publish_event(Event::Tick);
    app.run();

    assert_eq!(app.get_current_state(), State::MenuFota);
}

#[test]
fn short_push_goes_to_next_menu_item() {
    let mut app = get_application();
    goto_menu(&mut app);

    app.publish_event(Event::EnterShortPush);
    app.run();

    assert_eq!(app.get_current_state(), State::MenuCleanConfig);

    app.publish_event(Event::EnterShortPush);
    app.run();

    assert_eq!(app.get_current_state(), State::MenuExit);

    app.publish_event(Event::EnterShortPush);
    app.run();

    assert_eq!(app.get_current_state(), State::MenuFota);
}

#[test]
fn long_push_enter_menu_item() {
    let mut app = get_application();
    goto_menu(&mut app);

    app.publish_event(Event::EnterLongPush);
    app.run();

    assert_eq!(app.get_current_state(), State::Fota);
}

#[test]
fn menu_clean_configuration() {
    let mut app = get_application();
    goto_menu(&mut app);

    app.publish_event(Event::EnterShortPush);
    app.run();

    app.publish_event(Event::EnterLongPush);
    app.run();

    assert_eq!(app.get_current_state(), State::CleanConfig);
    assert!(app.configuration.is_invalid());
    assert_eq!(
        app.configuration_manager.load_from_persistent_storage(),
        Configuration::default()
    );

    app.run();
    assert_eq!(app.get_current_state(), State::Startup);
}

#[test]
fn enter_night_mode() {
    let mut app = get_application();
    goto_display_time(&mut app);

    app.time_source
        .set_time(time::Time::new(22, 00, 00).unwrap());

    app.publish_event(Event::Tick);
    app.run();
    // Need an extra run to process the "Night" event
    app.run();

    assert_eq!(app.get_current_state(), State::NightMode);
}

#[test]
fn exit_night_mode() {
    let mut app = get_application();
    goto_display_time(&mut app);

    app.time_source
        .set_time(time::Time::new(22, 00, 00).unwrap());

    app.publish_event(Event::Tick);
    app.run();
    // Need an extra run to process the "Night" event
    app.run();

    assert_eq!(app.get_current_state(), State::NightMode);
    app.time_source
        .set_time(time::Time::new(4, 30, 00).unwrap());

    // app.publish_event(Event::Tick);
    // app.run();
    // Need an extra run to process the "Night" event
    app.run();

    assert_eq!(app.get_current_state(), State::DisplayTime);
}
