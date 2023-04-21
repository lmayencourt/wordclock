/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::fmt;
use anyhow::{anyhow, Result};

/// Time representation
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    /// Return a time structure
    /// 
    /// # Errors
    /// An error is return if provided arguments doesn't represent a valid time.
    pub fn new(hour: u8, minute: u8, second: u8) -> Result<Self> {
        if hour > 12 || minute > 59 || second > 59 {
            return Err(anyhow!("invalid input {}:{}{}", hour, minute, second))
        }
        Ok(Time{hour, minute, second})
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    }
}
