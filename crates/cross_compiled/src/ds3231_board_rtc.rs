/* SPDX-License-Identifier: MIT
 *
 * This files uses part of code from:
 * https://github.com/eldruin/ds323x-rs/tree/v0.5.1
 * Copyright (c) 2018-2023 Diego Barrios Romero
 *
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;

use esp_idf_hal::{delay::BLOCK, i2c::I2cDriver};

use application::time::Time;
use application::time_source::TimeSource;

/// Hardcoded I2C address of the DS3231 chip, according to the datasheet.
const DS3231_RTC_ADDRESS: u8 = 0x68;

/// DS3231 register addresses
const DS3231_RTC_SECONDES_REG: u8 = 0x00;
const DS3231_RTC_MINUTES_REG: u8 = 0x01;
const DS3231_RTC_HOURS_REG: u8 = 0x02;

/// DS3231 register bitfields
const DS3231_HOUR_H24_H12: u8 = 0b0100_0000;
const DS3231_HOUR_AM_PM: u8 = 0b0010_0000;

pub struct Ds3231Rtc<'a> {
    i2c_master: Rc<RefCell<I2cDriver<'a>>>,
}

impl<'a> Ds3231Rtc<'a> {
    pub fn new(i2c_master: I2cDriver<'a>) -> Self {
        Ds3231Rtc {i2c_master: Rc::new(RefCell::new(i2c_master))}
    }

    pub fn set_time(&self, time: Time) -> Result<()> {
        self.write_register(DS3231_RTC_SECONDES_REG, Self::decimal_to_packed_bcd(time.second))?;
        self.write_register(DS3231_RTC_MINUTES_REG, Self::decimal_to_packed_bcd(time.minute))?;
        self.write_register(DS3231_RTC_HOURS_REG, Self::decimal_to_packed_bcd(time.hour))?;

        Ok(())
    }

    /// Read a single register from the DS3231 memory
    fn read_register(&self, register_address: u8) -> Result<u8> {
        let tx_buff: [u8; 1] = [register_address];
        self.i2c_master.borrow_mut().write(DS3231_RTC_ADDRESS, &tx_buff, BLOCK)?;
        let mut read_value: [u8; 1] = [0; 1];
        self.i2c_master.borrow_mut().read(DS3231_RTC_ADDRESS, &mut read_value, BLOCK)?;
        Ok(read_value[0])
    }

    /// Write a value to single register of the DS3231 memory
    fn write_register(&self, register_address: u8, value: u8) -> Result<()> {
        let tx_buf: [u8; 2] = [register_address, value];
        self.i2c_master.borrow_mut().write(DS3231_RTC_ADDRESS, &tx_buf, BLOCK)?;
        Ok(())
    }

    /// Convert a decimal number to packed BCD format
    fn decimal_to_packed_bcd(dec: u8) -> u8 {
        ((dec / 10) << 4) | (dec % 10)
    }

    /// Convert a packed BCD encoded value into a decimal number
    fn packed_bcd_to_decimal(bcd: u8) -> u8 {
        (bcd >> 4) * 10 + (bcd & 0xF)
    }

    /// Convert a DS3231 hour register format to 
    fn hours_from_register(data: u8) -> u8 {
        if Self::is_24h_format(data) {
            Self::packed_bcd_to_decimal(data & !DS3231_HOUR_H24_H12)
        } else if Self::is_am(data) {
            Self::packed_bcd_to_decimal(data & !(DS3231_HOUR_H24_H12 | DS3231_HOUR_AM_PM))
        } else {
            Self::packed_bcd_to_decimal(data & !(DS3231_HOUR_H24_H12 | DS3231_HOUR_AM_PM)) + 12
        }
    }

    /// Return true if DS3231 returned hour is in 24h format
    fn is_24h_format(data: u8) -> bool {
        data & DS3231_HOUR_H24_H12 == 0
    }

    /// Return true if DS3231 returned hour is AM format
    fn is_am(data: u8) -> bool {
        data & DS3231_HOUR_AM_PM == 0
    }
}

impl<'a> TimeSource for Ds3231Rtc<'a> {
    fn get_time(&self) -> Result<Time, application::time_source::TimeSourceError> {
        let second = self.read_register(DS3231_RTC_SECONDES_REG).unwrap();
        let minute = self.read_register(DS3231_RTC_MINUTES_REG).unwrap();
        let hour = self.read_register(DS3231_RTC_HOURS_REG).unwrap();

        let second = Ds3231Rtc::packed_bcd_to_decimal(second);
        let minute = Ds3231Rtc::packed_bcd_to_decimal(minute);
        let hour = Ds3231Rtc::hours_from_register(hour);

        Ok(Time::new(hour, minute, second).unwrap())
    }

    fn synchronize(&mut self) -> Result<(), application::time_source::TimeSourceError> {
        Err(application::time_source::TimeSourceError::SynchronizationError)
    }
}