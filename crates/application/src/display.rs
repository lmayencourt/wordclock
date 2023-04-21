/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use anyhow::{Result};

use crate::time::Time;

/// Interface to draw various things on a display.
/// # Errors
/// The functions will return an error if the hardware fails to carry the operation.
pub trait Display {
    /// Turn off all the pixels of display
    fn clear(&mut self) -> Result<()>;

    /// Draw the given time on the display.
    fn draw_time(&mut self, time: Time) -> Result<()>;
}