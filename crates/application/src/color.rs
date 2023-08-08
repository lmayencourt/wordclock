/* SPDX-License-Identifier: MIT
* Copyright (c) 2023 Louis Mayencourt
*/

use std::fmt;

use anyhow::{anyhow, Result};
use regex::Regex;
use rgb::RGB8;

/// Length of a color in string representation
pub const COLOR_AS_STRING_LENGTH: usize = 6;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    pub rgb: RGB8,
}

impl Color {
    pub fn new(r:u8, g:u8, b:u8) -> Self {
        Color { rgb: RGB8{r, g, b} }
    }

    /// Create a color from a RGB hexadecimal string, like provided by html
    ///
    /// # Error
    /// Fails if input is not 3 hex words as RRGGBB.
    pub fn from_rgb_hex_string(rgb: &str) -> Result<Self> {
        let re = Regex::new(r"^([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})$")?;

        if let Some(cap) = re.captures(rgb) {
            let r:u8 = u8::from_str_radix(&cap[1], 16)?;
            let g:u8 = u8::from_str_radix(&cap[2], 16)?;
            let b:u8 = u8::from_str_radix(&cap[3], 16)?;

            Ok(Color { rgb: RGB8{r, g, b} })

        } else {
            Err(anyhow!("Provided input is not a valid hex rgb string RRGGBB: {}", rgb))
        }
    }

    pub fn is_black(&self) -> bool {
        if self.rgb.r == 0 && self.rgb.g == 0 && self.rgb.b == 0 {
            true
        } else {
            false
        }
    }
}

impl Default for Color {
    /// Default screen color is blue
    fn default() -> Self {
        Color { rgb: RGB8 {r:0, g:0, b:255} }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X}{:02X}{:02X}",
            self.rgb.r, self.rgb.g, self.rgb.b
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_valid_rgb_hex_string() {
        let color = Color::from_rgb_hex_string("000000").unwrap();
        assert_eq!(
            color,
            Color {
                rgb: RGB8{r:00, g:00, b:00}
            }
        );

        let color = Color::from_rgb_hex_string("AABBCC").unwrap();
        assert_eq!(
            color,
            Color {
                rgb: RGB8{r:170, g:187, b:204}
            }
        );

        let color = Color::from_rgb_hex_string("00FF00").unwrap();
        assert_eq!(
            color,
            Color {
                rgb: RGB8{r:00, g:255, b:00}
            }
        );
    }

    #[test]
    fn from_invalid_rgb_hex_string() {
        // empty string
        let color = Color::from_rgb_hex_string("");
        assert!(color.is_err());

        // to short entry
        let color = Color::from_rgb_hex_string("00");
        assert!(color.is_err());

        // invalid char
        let color = Color::from_rgb_hex_string("AAQQCC");
        assert!(color.is_err());

        // to big entry
        let color = Color::from_rgb_hex_string("00112233");
        assert!(color.is_err());
    }

    #[test]
    fn color_to_string() {
        let color = Color::new(170, 187, 204);
        assert_eq!("AABBCC", color.to_string());
    }

    #[test]
    fn color_is_black() {
        let color = Color::new(0,0,0);
        assert!(color.is_black());
    }
}