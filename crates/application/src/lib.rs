use behaviour::*;
use display::Display;

pub mod behaviour;
pub mod display;
pub mod time;
pub mod version;

pub struct Application<D: Display> {
    pub display: D,
    behaviour: Behaviour,
    event_queue: Vec<Event>,
}

impl<D:Display> Application<D> {
    pub fn new(mut display: D) -> Self {
        display.clear().unwrap();
        Application {
            display,
            behaviour: Behaviour::new(),
            event_queue: vec![],
        }
    }

    pub fn publish_event(&mut self, event:Event) {
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
        let time = time::Time::new(11, 22, 33).unwrap();
        let _ = self.display.draw_time(time);
    }

    fn error(&mut self) {
        println!("Startup action");
        let _ = self.display.draw_error();
    }
}
