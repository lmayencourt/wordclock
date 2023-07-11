/* SPDX-License-Identifier: MIT
* Copyright (c) 2023 Louis Mayencourt
*/

/// Interface to interact with board power mode
pub trait PowerManager {
    /// Restart the system
    fn reset(&self);
}
