use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for RgbColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err(Self::Err::UnexpectedInputLength { length: s.len() });
        }

        let r = u8::from_str_radix(&s[0..2], 16)?;
        let g = u8::from_str_radix(&s[2..4], 16)?;
        let b = u8::from_str_radix(&s[4..6], 16)?;

        Ok(Self { r, g, b })
    }
}

impl Display for RgbColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RgbaColor {
    pub rgb: RgbColor,
    pub alpha: u8,
}

impl FromStr for RgbaColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err(Self::Err::UnexpectedInputLength { length: s.len() });
        }

        let rgb = s[..6].parse::<RgbColor>()?;
        let alpha = u8::from_str_radix(&s[6..8], 16)?;

        Ok(Self { rgb, alpha })
    }
}

impl Display for RgbaColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:02x}", self.rgb, self.alpha)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Color {
    Rgb(RgbColor),
    Rgba(RgbaColor),
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            6 => Ok(Self::Rgb(s.parse()?)),
            8 => Ok(Self::Rgba(s.parse()?)),
            n => Err(Self::Err::UnexpectedInputLength { length: n }),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rgb(rgb) => write!(f, "{rgb}"),
            Self::Rgba(rgba) => write!(f, "{rgba}"),
        }
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseColorError {
    #[error("Expected 6 or 8 chars of input, got {length}")]
    UnexpectedInputLength { length: usize },

    #[error("Not a valid hexadecimal value")]
    InvalidHex(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::RgbColor;
    use crate::color::{Color, ParseColorError, RgbaColor};
    use std::num::ParseIntError;

    #[test]
    fn test_parse_rgb() {
        assert_eq!(
            "c0ffee".parse(),
            Ok(Color::Rgb(RgbColor {
                r: 0xc0,
                g: 0xff,
                b: 0xee
            }))
        )
    }

    #[test]
    fn test_parse_rgba() {
        assert_eq!(
            "c0ffeeff".parse(),
            Ok(Color::Rgba(RgbaColor {
                rgb: RgbColor {
                    r: 0xc0,
                    g: 0xff,
                    b: 0xee
                },
                alpha: 0xff
            }))
        )
    }

    #[test]
    fn test_parse_invalid_length() {
        assert_eq!(
            "c00".parse::<Color>(),
            Err(ParseColorError::UnexpectedInputLength { length: 3 })
        )
    }

    #[test]
    fn test_parse_invalid_hex() {
        assert!("xxxxxx".parse::<Color>().is_err())
    }
}
