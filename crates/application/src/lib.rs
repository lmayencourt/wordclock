/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use behaviour::*;
use display::Display;
use time_source::TimeSource;

pub mod behaviour;
pub mod display;
pub mod time;
pub mod time_source;
pub mod version;

pub struct Application<D: Display, T: TimeSource> {
    pub display: D,
    pub time_source: T,
    behaviour: Behaviour,
    event_queue: Vec<Event>,
}

impl<D: Display, T: TimeSource> Application<D, T> {
    pub fn new(mut display: D, time_source: T) -> Self {
        display.clear().unwrap();
        Application {
            display,
            time_source,
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
            State::DisplayTime => self.displayTime(),
            _ => self.error(),
        }
    }

    fn startup(&mut self) {
        println!("Startup action");
        let _ = self.display.draw_progress(1);
    }

    fn displayTime(&mut self) {
        let time = self.time_source.get_time().unwrap();
        let _ = self.display.draw_time(time);
    }

    fn error(&mut self) {
        println!("Startup action");
        let _ = self.display.draw_error();
    }
}
