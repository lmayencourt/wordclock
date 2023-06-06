/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use anyhow::Result;

use crate::time::Time;

#[derive(Debug)]
pub enum TimeSourceError {
    NotSynchronized,
    SynchronizationError,
}

/// Interface to get local time from a time source
/// # Errors
/// The functions will return an error if the hardware fails to carry the operation.
pub trait TimeSource {
    fn synchronize(&mut self) -> Result<(), TimeSourceError>;
    fn get_time(&self) -> Result<Time, TimeSourceError>;
}