use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

impl Coordinates {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Coordinates {
    type Err = ParseCoordinatesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(' ')
            .ok_or(ParseCoordinatesError::SyntaxError)?;
        let x = u32::from_str(x)?;
        let y = u32::from_str(y)?;

        Ok(Self { x, y })
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseCoordinatesError {
    #[error("Expected two integers seperated by a single whitespace")]
    SyntaxError,

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use crate::coordinates::{Coordinates, ParseCoordinatesError};

    #[test]
    fn test_parse_coordinates() {
        assert_eq!("1337 42".parse(), Ok(Coordinates { x: 1337, y: 42 }))
    }

    #[test]
    fn test_parse_invalid_syntax() {
        assert_eq!(
            "1337,42".parse::<Coordinates>(),
            Err(ParseCoordinatesError::SyntaxError)
        )
    }

    #[test]
    fn test_parse_invalid_integer() {
        assert!("foobar 32".parse::<Coordinates>().is_err())
    }
}
