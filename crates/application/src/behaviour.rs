/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

/// Possible state of the device
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Created,
    Startup,
    DisplayTime,
    Configuration,
    Menu,
    NightMode,
    Error,
}

/// Possible event that may trigger a state transition of the device.
pub enum Event {
    Start,
    InvalidConfiguration,
    ValidConfiguration,
    Tick,
    EnterMenu,
    NextMenu,
    ExitMenu,
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
            state: State::Created,
        }
    }

    /// React to a given event
    pub fn handle_event(&mut self, event: Event) {
        match (&self.state, event) {
            (State::Created, Event::Start) => self.state = State::Startup,
            (State::Startup, Event::InvalidConfiguration) => self.state = State::Configuration,
            (State::Startup, Event::ValidConfiguration) => self.state = State::DisplayTime,
            (State::Configuration, Event::ValidConfiguration) => self.state = State::DisplayTime,
            (State::DisplayTime, Event::Tick) => (),
            (State::DisplayTime, Event::EnterMenu) => self.state = State::Menu,
            (State::DisplayTime, Event::Night) => self.state = State::NightMode,
            (State::Menu, Event::NextMenu) => (),
            (State::Menu, Event::ExitMenu) => self.state = State::DisplayTime,
            (State::NightMode, Event::Day) => self.state = State::DisplayTime,
            (_, Event::Error) => self.state = State::Error,
            (_, _) => self.state = State::Error,
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
        assert_eq!(state_machine.state, State::Created);

        state_machine.handle_event(Event::Start);
        assert_eq!(state_machine.state, State::Startup);

        state_machine.handle_event(Event::InvalidConfiguration);
        assert_eq!(state_machine.state, State::Configuration);

        state_machine.handle_event(Event::ValidConfiguration);
        assert_eq!(state_machine.state, State::DisplayTime);
    }
}
