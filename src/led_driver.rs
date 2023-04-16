/* SPDX-License-Identifier: MIT
 *
 * This files uses part of code from:
 * https://github.com/taunusflieger/anemometer/blob/master/anemometer-calibration/src/neopixel.rs
 * Copyright (c) 2021-2023 Michael Zill
 * 
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use smart_leds::RGB8;

use esp_idf_hal::gpio::*;
use esp_idf_hal::rmt::CHANNEL0;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::{VariableLengthSignal, PinState, Pulse, TxRmtDriver};

const RGB_FRAME_BIT_LENGTH: u32 = 24;

pub struct WS2812<'d> {
    led_count: u32,
    tx_rmt: TxRmtDriver<'d>,
    high: [Pulse; 2],
    low: [Pulse; 2],
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}

impl<'d> WS2812<'d> {
    pub fn new(led_count: u32, data_pin: Gpio13, channel: CHANNEL0) -> Result<Self> {

        let config = TransmitConfig::new().clock_divider(1);
        let tx_rmt = TxRmtDriver::new(channel, data_pin, &config)
            .context("Creation of Tx RMT driver")?;

        let ticks_hz = tx_rmt.counter_clock()?;
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(800))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(700))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(600))?;

        Ok(Self {
            led_count,
            tx_rmt,
            high: [t1h, t1l],
            low: [t0h, t0l],
        })
    }

    pub fn write(&mut self, pixels: &[RGB8]) -> Result<()> {
        if pixels.len() > self.led_count.try_into()? {
            return Err(anyhow!("Provided buffer length {} bigger than available leds {}", pixels.len(), self.led_count));
        }

        let mut signal = VariableLengthSignal::new();
        for pixel in pixels {
            for i in 0..self::RGB_FRAME_BIT_LENGTH {
                let color = Self::rgb_to_u32(pixel.r, pixel.g, pixel.b);
                let bit = 2_u32.pow(i%24) & color != 0;
                let pulse = if bit { self.high } else { self.low };
                signal.push(&pulse);
            }
        }

        self.tx_rmt
        .start_blocking(&signal)
        .context("Rmt sending sequence failed")?;

        Ok(())
    }

    fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
        (b as u32) << 16 | (r as u32) << 8 | g as u32
    }
}