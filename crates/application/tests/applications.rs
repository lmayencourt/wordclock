
use application::*;
use application::behaviour::*;

#[derive(PartialEq, Debug)]
enum DummyDisplayState {
    Clean,
    Progress(u8),
    Error,
    Time(time::Time),
}
struct DummyDisplay {
    state: DummyDisplayState,
}

impl display::Display for DummyDisplay {
    fn clear(&mut self) -> anyhow::Result<()> {
        self.state = DummyDisplayState::Clean;
        Ok(())
    }
    fn draw_error(&mut self) -> anyhow::Result<()> {
        self.state = DummyDisplayState::Error;
        Ok(())
    }
    fn draw_progress(&mut self, progress: u8) -> anyhow::Result<()> {
        self.state = DummyDisplayState::Progress(progress);
        Ok(())
    }
    fn draw_time(&mut self, time: time::Time) -> anyhow::Result<()> {
        self.state = DummyDisplayState::Time(time);
        Ok(())
    }
}

#[test]
fn build_and_run() {
    let display = DummyDisplay{state:DummyDisplayState::Clean};
    let mut app = Application::new(display);
    assert_eq!(app.display.state, DummyDisplayState::Clean);

    app.publish_event(Transition::ValidConfiguration);
    app.run();
    assert_eq!(app.display.state, DummyDisplayState::Time(time::Time { hour: 11, minute: 22, second: 33 }));

    // app.publish_event(Transition::Tick);
    // app.run();
    // assert_eq!(app.display.state, DummyDisplayState::Time(time::Time { hour: 11, minute: 23, second: 33 }));
}