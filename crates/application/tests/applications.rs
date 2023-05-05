/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use anyhow::Result;

use application::behaviour::*;
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

#[test]
fn display_time() {
    let display = FakeDisplay {
        state: FakeDisplayState::Clean,
    };
    let time_source = MockTime {
        curent: time::Time::new(11, 22, 33).unwrap(),
    };
    let mut app = Application::new(display, time_source);
    assert_eq!(app.display.state, FakeDisplayState::Clean);

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
