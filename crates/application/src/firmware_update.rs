/* SPDX-License-Identifier: MIT
* Copyright (c) 2023 Louis Mayencourt
*/

use anyhow::{anyhow, Result};

use crate::version::Version;

/// Interface to perform a firmware update
pub trait FirmwareUpdate {
    /// Return the version of the latest update available to download.
    ///
    /// # Error
    /// Return an error if HTTP request or file parsing fails.
    fn read_update_version(&self) -> Result<Version>;

    fn download_update(&self) -> Result<()>;

    fn reboot_to_new_image(&self);
}
