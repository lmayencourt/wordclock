/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use log::info;

/// Possible state of the device
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Initial,
    Startup,
    DisplayTime,
    Configuration,
    MenuFota,
    MenuCleanConfig,
    MenuExit,
    Fota,
    CleanConfig,
    NightMode,
    Error,
}

/// Possible event that may trigger a state transition of the device.
#[derive(Debug)]
pub enum Event {
    Init,
    Start,
    InvalidConfiguration,
    ValidConfiguration,
    Tick,
    EnterShortPush,
    EnterLongPush,
    Night,
    Day,
    Error,
}

/// Device state-machine implementation
pub struct Behaviour {
    state: State,
}

impl Behaviour {
    pub fn new() -> Self {
        Self {
            state: State::Initial,
        }
    }

    /// React to a given event
    pub fn handle_event(&mut self, event: Event) {
        let old_state = self.state;
        match (&self.state, event) {
            (State::Initial, Event::Init) => self.state = State::Startup,
            (State::Startup, Event::InvalidConfiguration) => self.state = State::Configuration,
            (State::Startup, Event::Start) => self.state = State::DisplayTime,
            (State::Configuration, Event::ValidConfiguration) => self.state = State::Startup,
            (State::Configuration, Event::Tick) => (),
            (State::DisplayTime, Event::Tick) => (),
            (State::DisplayTime, Event::EnterShortPush) => self.state = State::MenuFota,
            (State::DisplayTime, Event::EnterLongPush) => self.state = State::MenuFota,
            (State::DisplayTime, Event::Night) => self.state = State::NightMode,
            (State::MenuFota, Event::EnterShortPush) => self.state = State::MenuCleanConfig,
            (State::MenuFota, Event::EnterLongPush) => self.state = State::Fota,
            (State::MenuFota, _) => (),
            (State::MenuCleanConfig, Event::EnterShortPush) => self.state = State::MenuExit,
            (State::MenuCleanConfig, Event::EnterLongPush) => self.state = State::CleanConfig,
            (State::MenuCleanConfig, _) => (),
            (State::MenuExit, Event::EnterShortPush) => self.state = State::MenuFota,
            (State::MenuExit, Event::EnterLongPush) => self.state = State::DisplayTime,
            (State::Fota, _) => (),
            (State::CleanConfig, Event::InvalidConfiguration) => self.state = State::Startup,
            (State::NightMode, Event::Day) => self.state = State::DisplayTime,
            (_, Event::Error) => self.state = State::Error,
            (_, _) => self.state = State::Error,
        }

        if old_state != self.state {
            info!("{:?} -> {:?}", old_state, self.state);
        }
    }

    pub fn current_state(&self) -> State {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_events() {
        let mut state_machine = Behaviour::new();
        assert_eq!(state_machine.state, State::Initial);

        state_machine.handle_event(Event::Init);
        assert_eq!(state_machine.state, State::Startup);

        state_machine.handle_event(Event::InvalidConfiguration);
        assert_eq!(state_machine.state, State::Configuration);

        state_machine.handle_event(Event::ValidConfiguration);
        assert_eq!(state_machine.state, State::Startup);

        state_machine.handle_event(Event::Start);
        assert_eq!(state_machine.state, State::DisplayTime);
    }
}
