/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use anyhow::Result;

/// Interface to interact with network connection
/// # Errors
/// The functions will return an error if the hardware fails to carry the operation.
pub trait Network {
    fn connect(&mut self) -> Result<()>;
    fn disconnect(&mut self) -> Result<()>;
    fn is_connected(&self) -> bool;
}