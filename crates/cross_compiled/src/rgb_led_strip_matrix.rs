/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

 use anyhow::{anyhow, Result};
 use log::*;

use smart_leds::RGB8;
use smart_leds::colors::*;

use crate::led_driver::RgbLedStrip;

use application::display::Display;
use application::time::Time;

/// LEDs matrix have a given size of 11x10 (+4 dots)
/// Use `usize` type, as this value are only used for slice indexing
const LEDS_MATRIX_WIDTH:usize = 11;
const LEDS_MATRIX_HEIGTH: usize =10;
const LEDS_MATRIX_PIXEL_COUNT: usize = LEDS_MATRIX_HEIGTH*LEDS_MATRIX_WIDTH + 4;

/// Hour to text lookup table for Bärn dialect.
/// Stored as start_x, start_y, length
const BARN_HOURS_LOOKUP_TABLE:[[usize; 3]; 12] = 
    [
        [0, 4, 3], // eis
		[3, 4, 4], // zwöi
		[8, 4, 3], // drü
		[0, 5, 5], // vieri
		[5, 5, 4], // füfi
		[0, 6, 6], // sächsi
		[6, 6, 5], // sibni
		[0, 7, 5], // achti
		[5, 7, 4], // nüni
		[0, 8, 4], // zäni
		[7, 8, 4], // eufi
		[0, 9, 6], // zwöufi 
    ];

/// Index for minute lookup table
const MIN_5:usize = 1;
const MIN_10:usize = 2;
const MIN_15:usize = 3;
const MIN_20:usize = 4;
const MIN_30:usize = 5;

/// Minute to text lookup table for Bärn dialect.
/// Stored as start_x, start_y, length
const BARN_MINUTES_LOOKUP_TABLE:[[usize; 3]; 6] = 
    [
        [8, 9, 3], // uhr
        [8, 0, 3], // fÜf
        [8, 1, 3], // zää
        [0, 1, 6], // viertu
        [0, 2, 6], // zwänzg
        [3, 3, 5], // haubi
    ];

/// Preposition to text lookup table for Bärn dialect.
/// Stored as start_x, start_y, length
const BARN_PREPOSITION_LOOKUP_TABLE:[[usize; 3]; 2] =
    [
        [0, 3, 2], // ab
        [8, 2, 3], // vor
    ];

/// Word to text lookup table for Bärn dialect.
/// Stored as start_x, start_y, length
const BARN_WORD_LOOKUP_TABLE:[[usize; 3]; 3] =
    [
        [0, 0, 2], // es
        [3, 0, 4], // isch
        [8, 9, 3], // uhr
    ];

/// Red circle with cross sign.
const ERROR_SIGN:[RGB8; 114] =
    [
                                RED, RED, RED, RED,
        BLACK, BLACK, RED, RED, BLACK, BLACK, BLACK, RED, RED, BLACK, BLACK,
        BLACK, RED, RED, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, BLACK,
        RED, RED, BLACK, RED, BLACK, BLACK, BLACK, RED, BLACK, RED, RED,
        RED, BLACK, BLACK, BLACK, RED, BLACK, RED, BLACK, BLACK, BLACK, RED,
        RED, BLACK, BLACK, BLACK, BLACK, RED, BLACK, BLACK, BLACK, BLACK, RED,
        RED, BLACK, BLACK, BLACK, RED, BLACK, RED, BLACK, BLACK, BLACK, RED,
        RED, RED, BLACK, RED, BLACK, BLACK, BLACK, RED, BLACK, RED, RED,
        BLACK, RED, RED, BLACK, BLACK, BLACK, BLACK, BLACK, RED, RED, BLACK,
        BLACK, BLACK, RED, RED, BLACK, BLACK, BLACK, RED, RED, BLACK, BLACK,
        BLACK, BLACK, BLACK, RED, RED, RED, RED, RED, BLACK, BLACK, BLACK,
    ];

pub struct RgbLedStripMatrix<T: RgbLedStrip> {
    driver: T,
    frame: [RGB8; LEDS_MATRIX_PIXEL_COUNT]
}

impl<T: RgbLedStrip> RgbLedStripMatrix<T>
{
    /// Create a RGB LED strip matrix of `LEDS_MATRIX_HEIGTH` by `LEDS_MATRIX_WIDTH`.
    ///
    /// The display is initialized with a new `frame` cleared.
    /// 
    /// # Errors
    /// The function will return an error if the hardware fails to carry the operation.
    pub fn new(mut driver: T) -> Result<Self> {
        driver.clear()?;

        Ok(RgbLedStripMatrix{driver, frame:[BLACK; LEDS_MATRIX_PIXEL_COUNT]})
    }

    fn set_pixel_from_lut(&mut self, lut: &[[usize; 3]], idx: usize, color: RGB8) {
        for n in 0..lut[idx][2] {
            self.set_pixel(lut[idx][0]+n, lut[idx][1], color);
        }
    }

    fn set_pixel(&mut self, x_cor:usize, y_cor:usize, color: RGB8) {
        let mut corrected_x_cor = x_cor;
        let corrected_y_cor = LEDS_MATRIX_HEIGTH-1-y_cor;
        let mut pixel_num;

        if y_cor%2 == 1 {
            // we are on a odd line
            corrected_x_cor = LEDS_MATRIX_WIDTH-1-x_cor;
        }
        pixel_num = LEDS_MATRIX_WIDTH * corrected_y_cor + corrected_x_cor;

        // minutes dots are the 4 first pixels. Map them at [10, 0-3]
        if y_cor < LEDS_MATRIX_HEIGTH {
            pixel_num += 4;
        } else {
            pixel_num = x_cor;
        }

        debug!("setting pixel {} for x:{} y:{}", pixel_num, x_cor, y_cor);
        self.frame[pixel_num] = color;
    }

    fn set_dots(&mut self, start:usize, number:usize, color: RGB8) {
        for n in start..start+number {
            self.frame[n] = color;
        }
    }

    fn draw_frame(&mut self) -> Result<()> {
        self.driver.write(&self.frame)?;
        Ok(())
    }

    fn new_frame(&mut self) {
        self.frame = [BLACK; LEDS_MATRIX_PIXEL_COUNT];
    }

}

impl<T: RgbLedStrip> Display for RgbLedStripMatrix<T> {
    fn clear(&mut self) -> Result<()> {
        self.frame = [BLACK; LEDS_MATRIX_PIXEL_COUNT];
        self.driver.clear()
    }

    fn draw_time(&mut self, time: Time) -> Result<()> {
        // No need to check provided `time` parameter, as it can only represent a valid time.

        self.new_frame();

        // Compute hour to display for German
        let mut hour_to_display = time.hour;
        if time.minute >=25 {
            hour_to_display = time.hour+1;
        }
        // 24 -> 12
		if hour_to_display > 12 {
			hour_to_display = hour_to_display - 12 ;
		}
		// midnight
		if hour_to_display == 0 {
			hour_to_display = 12;
		}
        self.set_pixel_from_lut(&BARN_HOURS_LOOKUP_TABLE, (hour_to_display-1) as usize, BLUE);

        // Display minutes
		let minutes_word:usize;
		if time.minute < 5 {
			// Es isch
			self.set_pixel_from_lut(&BARN_WORD_LOOKUP_TABLE, 0, BLUE);
			self.set_pixel_from_lut(&BARN_WORD_LOOKUP_TABLE, 1, BLUE);
			// uhr
			minutes_word = (time.minute/5) as usize;
			debug!(" <5 id: {}", minutes_word);
			self.set_pixel_from_lut(&BARN_MINUTES_LOOKUP_TABLE, minutes_word, BLUE);
		} else if time.minute < 25 {
			// uhr, füf, zää, zwanzg ab
			minutes_word = (time.minute/5) as usize;
			debug!(" <25 id: {}", minutes_word);
			self.set_pixel_from_lut(&BARN_MINUTES_LOOKUP_TABLE, minutes_word, BLUE);
			self.set_pixel_from_lut(&BARN_PREPOSITION_LOOKUP_TABLE, 0, BLUE);
		} else if time.minute < 30 {
			// füf vor halbi h+1
			debug!(" <30");
			self.set_pixel_from_lut(&BARN_MINUTES_LOOKUP_TABLE, MIN_5, BLUE);
			self.set_pixel_from_lut(&BARN_PREPOSITION_LOOKUP_TABLE, 1, BLUE);
			self.set_pixel_from_lut(&BARN_MINUTES_LOOKUP_TABLE, MIN_30, BLUE);

		} else if time.minute < 35 {
			// halbi h+1
			debug!(" <35");
			self.set_pixel_from_lut(&BARN_MINUTES_LOOKUP_TABLE, MIN_30, BLUE);
		} else if time.minute < 40 {
			// füf ab halbi h+1
			debug!(" <40");
			self.set_pixel_from_lut(&BARN_MINUTES_LOOKUP_TABLE, MIN_5, BLUE);
			self.set_pixel_from_lut(&BARN_PREPOSITION_LOOKUP_TABLE, 0, BLUE);
			self.set_pixel_from_lut(&BARN_MINUTES_LOOKUP_TABLE, MIN_30, BLUE);
		} else {
			// füf, zää, zwanzg vor
			minutes_word = ((60 - time.minute-1)/5 +1) as usize;
			debug!(" >40 id: {}", minutes_word);
			self.set_pixel_from_lut(&BARN_MINUTES_LOOKUP_TABLE, minutes_word, BLUE);
			self.set_pixel_from_lut(&BARN_PREPOSITION_LOOKUP_TABLE, 1, BLUE);
		}

		// display inter'minutes
        if time.minute%5 != 0 {
            self.set_dots(0, (time.minute%5) as usize, BLUE);
        }

        self.draw_frame()?;

        Ok(())   
    }

    fn draw_error(&mut self) -> Result<()> {
        self.new_frame();

        self.frame = ERROR_SIGN;
        self.draw_frame()?;

        Ok(())
    }

    fn draw_progress(&mut self, progress:u8) -> Result<()> {
        if progress > 4 {
            return Err(anyhow!("Can't handle more than 4 levels"));
        }

        self.new_frame();
        self.set_dots(0, progress as usize, BLUE);
        self.draw_frame()?;

        Ok(())
    }
}
