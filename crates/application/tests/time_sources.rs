/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::time::{Instant, Duration};

use application::*;
use application::time::Time;
use application::time_monotonic::TimeMonotonic;
use application::time_source::{TimeSource,TimeSourceError};
use application::time_source_manager::{TimeSourceManager, CPU_SYNC_TIMEOUT, RTC_SYNC_TIMEOUT};

const INITIAL_RTC_TIME: Time = Time{hour:1, minute:2, second:3};
const INITIAL_NETWORK_TIME: Time = Time{hour:12, minute:34, second:56};

const ELAPSED_RTC_TIME: Time = Time{hour:2, minute:3, second:4};

struct MockMonotonicTime {
    now: Instant,
}

impl MockMonotonicTime {
    fn elapsed(&mut self, duration: Duration) {
        self.now = self.now + duration;
    }
}

impl TimeMonotonic for MockMonotonicTime {
    fn now(&self) -> std::time::Instant {
        return self.now
    }
}

struct MockTime {
    current: Time,
}

impl MockTime {
    fn set_time(&mut self, time: Time) {
        self.current = time;
    }
}

impl time_source::TimeSource for MockTime {
    fn synchronize(&mut self) -> Result<(), TimeSourceError> {
        Ok(())
    }

    fn get_time(&self) -> Result<Time, TimeSourceError> {
        Ok(self.current)
    }

    fn set_time(&mut self, now: Time) -> Result<(), TimeSourceError> {
        self.current = now;
        Ok(())
    }
}

mod board_time_only {
    use super::*;

    #[test]
    fn start_unsynchronized() {

        let time_source_manager = get_time_source_manager();

        assert!(!time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time(), Err(TimeSourceError::NotSynchronized));
    }

    #[test]
    fn set_time() {
        let mut time_source_manager = get_time_source_manager();
        let new_time = Time::new(1,2,3).unwrap();

        time_source_manager.set_time(new_time).unwrap();

        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), new_time);
    }

    #[test]
    fn sync_and_get_time() {
        let mut time_source_manager = get_time_source_manager();

        assert!(time_source_manager.synchronize().is_ok());
        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), INITIAL_RTC_TIME);
    }

    #[test]
    fn out_of_sync_detection() {
        let mut time_source_manager = get_time_source_manager();

        // first sync should always return network time.
        time_source_manager.synchronize().unwrap();
        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), INITIAL_RTC_TIME);

        // reach CPU out of sync
        time_source_manager.time_monotonic.elapsed(CPU_SYNC_TIMEOUT);
        assert!(!time_source_manager.is_synchronized());
        // this should sync with the board time
        time_source_manager.board_time.as_mut().unwrap().set_time(ELAPSED_RTC_TIME).unwrap();
        time_source_manager.synchronize().unwrap();
        assert_eq!(time_source_manager.get_time().unwrap(), ELAPSED_RTC_TIME);
    }

    fn get_time_source_manager() -> TimeSourceManager<MockMonotonicTime> {
        let sys_time = MockMonotonicTime{now:Instant::now()};
        let cpu_time = Box::new(MockTime{current:Time::new(0, 0, 0).unwrap()});
        let board_time = Box::new(MockTime{current:INITIAL_RTC_TIME});

        TimeSourceManager::new(sys_time, cpu_time, Some(board_time), None)
    }
}

mod network_time_only {
    use super::*;

    #[test]
    fn start_unsynchronized() {

        let time_source_manager = get_time_source_manager();

        assert!(!time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time(), Err(TimeSourceError::NotSynchronized));
    }

    #[test]
    fn set_time() {
        let mut time_source_manager = get_time_source_manager();
        let new_time = Time::new(1,2,3).unwrap();

        time_source_manager.set_time(new_time).unwrap();

        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), new_time);
    }

    #[test]
    fn sync_and_get_time() {
        let mut time_source_manager = get_time_source_manager();

        assert!(time_source_manager.synchronize().is_ok());
        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), INITIAL_NETWORK_TIME);
    }

    #[test]
    fn out_of_sync_detection() {
        let mut time_source_manager = get_time_source_manager();

        // first sync should always return network time.
        time_source_manager.synchronize().unwrap();
        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), INITIAL_NETWORK_TIME);

        // reach CPU out of sync
        time_source_manager.time_monotonic.elapsed(CPU_SYNC_TIMEOUT);
        assert!(!time_source_manager.is_synchronized());
        // this should sync with the network time
        time_source_manager.network_time.as_mut().unwrap().set_time(ELAPSED_RTC_TIME).unwrap();
        time_source_manager.synchronize().unwrap();
        assert_eq!(time_source_manager.get_time().unwrap(), ELAPSED_RTC_TIME);
    }

    fn get_time_source_manager() -> TimeSourceManager<MockMonotonicTime> {
        let sys_time = MockMonotonicTime{now:Instant::now()};
        let cpu_time = Box::new(MockTime{current:Time::new(0, 0, 0).unwrap()});
        let network_time = Box::new(MockTime{current:INITIAL_NETWORK_TIME});

        TimeSourceManager::new(sys_time, cpu_time, None, Some(network_time))
    }
}

mod board_and_network {
    use super::*;

    #[test]
    fn start_unsynchronized() {

        let time_source_manager = get_time_source_manager();

        assert!(!time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time(), Err(TimeSourceError::NotSynchronized));
    }

    #[test]
    fn set_time() {
        let mut time_source_manager = get_time_source_manager();
        let new_time = Time::new(1,2,3).unwrap();

        time_source_manager.set_time(new_time).unwrap();

        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), new_time);
    }

    #[test]
    fn sync_and_get_time() {
        let mut time_source_manager = get_time_source_manager();

        assert!(time_source_manager.synchronize().is_ok());
        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), INITIAL_NETWORK_TIME);
    }

    #[test]
    fn out_of_sync_detection() {
        let mut time_source_manager = get_time_source_manager();

        // first sync should always return network time.
        time_source_manager.synchronize().unwrap();
        assert!(time_source_manager.is_synchronized());
        assert_eq!(time_source_manager.get_time().unwrap(), INITIAL_NETWORK_TIME);

        // reach CPU out of sync
        time_source_manager.time_monotonic.elapsed(CPU_SYNC_TIMEOUT);
        assert!(!time_source_manager.is_synchronized());
        // this should sync with the board time
        time_source_manager.board_time.as_mut().unwrap().set_time(ELAPSED_RTC_TIME).unwrap();
        time_source_manager.synchronize().unwrap();
        assert_eq!(time_source_manager.get_time().unwrap(), ELAPSED_RTC_TIME);

        // reach board out of sync
        time_source_manager.time_monotonic.elapsed(RTC_SYNC_TIMEOUT);
        assert!(!time_source_manager.is_synchronized());
        // this should sync with network time
        time_source_manager.synchronize().unwrap();
        assert_eq!(time_source_manager.get_time().unwrap(), INITIAL_NETWORK_TIME);
    }

    fn get_time_source_manager() -> TimeSourceManager<MockMonotonicTime> {
        let sys_time:MockMonotonicTime = MockMonotonicTime{now:Instant::now()};
        let cpu_time = Box::new(MockTime{current:Time::new(0, 0, 0).unwrap()});
        let board_time = Box::new(MockTime{current:INITIAL_RTC_TIME});
        let network_time = Box::new(MockTime{current:INITIAL_NETWORK_TIME});
    
        TimeSourceManager::new(sys_time, cpu_time, Some(board_time), Some(network_time))
    }
}
