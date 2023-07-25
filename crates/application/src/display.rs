/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use anyhow::Result;

use crate::{time::Time, color::Color};

/// Interface to draw various things on a display.
/// # Errors
/// The functions will return an error if the hardware fails to carry the operation.
pub trait Display {
    /// Turn off all the pixels of display
    fn clear(&mut self) -> Result<()>;

    /// Draw the given time on the display.
    fn draw_time(&mut self, time: Time) -> Result<()>;

    /// Draw an error sign on the display.
    fn draw_error(&mut self) -> Result<()>;

    /// Draw a progress bar, with 4 levels.
    ///
    /// # Errors
    /// Fails early if progress higher than 4 is provided.
    fn draw_progress(&mut self, progress: u8) -> Result<()>;

    /// Set the default color to be used to draw on the display.
    fn set_default_color(&mut self, color: Color);
}
