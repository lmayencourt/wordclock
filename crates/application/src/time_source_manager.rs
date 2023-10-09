/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

// use core::time::Duration;
use std::time::{Instant, Duration};

// use log::*;
use anyhow::Result;
use log::warn;

use crate::time::Time;
use crate::time_monotonic::TimeMonotonic;
use crate::time_source::{TimeSource, TimeSourceError};

/// Manage multiple time sources
///
/// Keep the CPU time in sync with board RTC and/or network time.
/// The accuracy of the time source is assumed to be CPU < board RTC < network time.
/// The availability of time source is assumed to be CPU > board RTC > network time.
/// The power cost of fetching time is assumed to be CPU < board RTC < network time.
///
/// To achieve high availability and low power, the time is always fetched in this order:
/// 1. CPU time
/// 2. board RTC
/// 3. Network
///
/// To achieve high accuracy, the time source are synchronized periodically:
/// - CPU time synchronized with RTC time every 5 min.
/// - RTC time synchronized with network time every every day (if available).\
///
/// It is difficult to set the cpu time manually, as the Time struct must be converted to epoch time...
/// This can be done apparently with mktime function!

pub const CPU_SYNC_TIMEOUT: Duration = Duration::from_secs(60*5);
pub const RTC_SYNC_TIMEOUT: Duration = Duration::from_secs(60*60*24);

pub struct TimeSourceManager<T: TimeMonotonic>{
    // The time sources are only public for testing
    // This is simpler than use sharable mutability
    pub time_monotonic: T,
    pub cpu_time: Box<dyn TimeSource>,
    pub board_time: Option<Box<dyn TimeSource>>,
    pub network_time: Option<Box<dyn TimeSource>>,
    last_cpu_sync: Option<Instant>,
    last_board_sync: Option<Instant>,
    last_network_sync: Option<Instant>,
}

impl<T: TimeMonotonic> TimeSourceManager<T>{
    pub fn new(
        time_monotonic: T,
        cpu_time: Box<dyn TimeSource>,
        board_time: Option<Box<dyn TimeSource>>,
        network_time: Option<Box<dyn TimeSource>>,
    ) -> Self {
        // Set duration with an offset to mark the struct as un-synchronized.
        Self {
            time_monotonic,
            cpu_time,
            board_time,
            network_time,
            last_cpu_sync: None,
            last_board_sync: None,
            last_network_sync: None,
        }
    }

    pub fn is_synchronized(&self) -> bool {
        let cpu_synched = self.is_cpu_synchronized();
        if let Some(board_synched) = self.is_board_synchronized() {
            return cpu_synched & board_synched
        }

        cpu_synched
    }

    pub fn is_cpu_synchronized(&self) -> bool {
        if let Some(cpu_sync) = self.last_cpu_sync {
            // Do not use cpu_sync.elapsed(), as it directly call Instant::now(),
            // which can't be tested.
            let elapsed = self.time_monotonic.now() - cpu_sync;
            if elapsed >= CPU_SYNC_TIMEOUT {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    pub fn is_board_synchronized(&self) -> Option<bool> {
        //board time can only be out of sync if network time and board time is available
        if self.network_time.is_none() || self.board_time.is_none() {
            return None
        }

        if let Some(board_sync) = self.last_board_sync {
            let elapsed = self.time_monotonic.now() - board_sync;
            if elapsed >= RTC_SYNC_TIMEOUT {
                return Some(false);
            }
        } else {
            return Some(false);
        }

        Some(true)
    }
}

impl<T:TimeMonotonic> TimeSource for TimeSourceManager<T> {
    fn synchronize(&mut self) -> Result<(), TimeSourceError> {
        // Only read network time if available and needed
        match self.is_board_synchronized() {
            Some(true) => (),
            _ => {
                let current_network_time: Result<Time, TimeSourceError>;
                // Try to read the network time if out of sync
                if let Some(network) = &self.network_time {
                    current_network_time = network.get_time();
                    if current_network_time.is_ok() {
                        // Network time is "read only", so reading it count as a sync
                        self.last_network_sync = Some(self.time_monotonic.now());
                    }
                } else {
                    current_network_time = Err(TimeSourceError::NotAvailable);
                }

                if let Ok(network_time) = current_network_time {
                    // Synchronize all time source with network time
                    self.cpu_time.set_time(network_time)?;
                    self.last_cpu_sync = Some(self.time_monotonic.now());
        
                    if let Some(rtc) = self.board_time.as_mut() {
                        rtc.set_time(network_time)?;
                        self.last_board_sync = Some(self.time_monotonic.now());
                    }
        
                    return Ok(())
                } else {
                    warn!("Can't fetch time from network, use board RTC time as primary source");
                }
            }
        }

        // Try to read the board time
        let current_board_time: Result<Time, TimeSourceError>;
        if let Some(rtc) = &self.board_time {
            current_board_time = rtc.get_time();
        } else {
            current_board_time = Err(TimeSourceError::NotAvailable);
        }

        if let Ok(board_time) = current_board_time {
            self.cpu_time.set_time(board_time)?;
            self.last_cpu_sync = Some(self.time_monotonic.now());
        } else {
            warn!("Can't fetch time from board RTC, use network time as primary source");
        }

        Ok(())
    }

    /// For now, always return board RTC time if available, or CPU time for simplicity
    ///
    /// It should be possible to save a bit of power by checking the last sync of the CPU
    /// time and returning it instead of the board RTC time, as no I2C transaction is needed
    /// for the cpu time.
    fn get_time(&self) -> Result<crate::time::Time, TimeSourceError> {
        if self.is_synchronized() {
            if let Some(rtc) = &self.board_time {
                rtc.get_time()
            } else {
                self.cpu_time.get_time()
            }
        } else {
            return Err(TimeSourceError::NotSynchronized);
        }
    }

    /// Set all time sources with the given time
    fn set_time(&mut self, now: crate::time::Time) -> Result<(), TimeSourceError> {
        self.cpu_time.set_time(now)?;

        if let Some(rtc) = self.board_time.as_mut() {
            rtc.set_time(now)?;
        }

        if let Some(network) = self.network_time.as_mut() {
            network.set_time(now)?;
        }

        let sync_time = Some(self.time_monotonic.now());
        self.last_cpu_sync = sync_time;
        self.last_board_sync = sync_time;
        self.last_network_sync = sync_time;

        Ok(())
    }
}
