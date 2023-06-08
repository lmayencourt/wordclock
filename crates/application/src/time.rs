/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use anyhow::{anyhow, Result};
use std::{fmt, str::FromStr};

/// Time in 24 hour clock representation
#[derive(PartialEq, Debug, Clone, Copy)]
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
        if hour > 23 || minute > 59 || second > 59 {
            return Err(anyhow!("invalid input {}:{}{}", hour, minute, second));
        }
        Ok(Time {
            hour,
            minute,
            second,
        })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
    }
}

impl FromStr for Time {
    type Err = std::num::ParseIntError;

    /// Parse a string with the "hh:mm::ss" format and return an time struct
    ///
    /// # Errors
    /// An error is return if provided arguments doesn't represent a valid time.
    fn from_str(time: &str) -> std::result::Result<Self, Self::Err> {
        let hour = u8::from_str_radix(&time[0..2], 10)?;
        let minute = u8::from_str_radix(&time[3..5], 10)?;
        let second = u8::from_str_radix(&time[6..8], 10)?;
        Ok(Time {
            hour,
            minute,
            second,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::time::Time;

    #[test]
    fn time_to_string() {
        assert_eq!("00:00:00", Time::new(0, 0, 0).unwrap().to_string());
        assert_eq!("12:03:04", Time::new(12, 3, 4).unwrap().to_string());
        assert_eq!("23:59:59", Time::new(23, 59, 59).unwrap().to_string());
    }

    #[test]
    fn string_to_time() {
        assert_eq!(Time::new(0, 0, 0).unwrap(), Time::from_str("00:00:00").unwrap());
        assert_eq!(Time::new(12, 3, 4).unwrap(), Time::from_str("12:03:04").unwrap());
        assert_eq!(Time::new(23, 59, 59).unwrap(), Time::from_str("23:59:59").unwrap());
    }
}