/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::time::Instant;

pub trait TimeMonotonic {
    fn now(&self) -> Instant;
}

struct MonotonicSystemTime;

impl TimeMonotonic for MonotonicSystemTime{
    fn now(&self) -> Instant {
        return Instant::now();
    }
}