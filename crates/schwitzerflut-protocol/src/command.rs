use crate::color::{Color, ParseColorError};
use crate::coordinates::{Coordinates, ParseCoordinatesError};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Command {
    GetCanvasSize(GetCanvasSizeCommand),
    SetPixel(SetPixelCommand),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.starts_with("PX") => Ok(Self::SetPixel(s.parse::<SetPixelCommand>()?)),
            "SIZE" => Ok(Self::GetCanvasSize(GetCanvasSizeCommand)),
            _ => Err(Self::Err::UnknownCommand),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetCanvasSize(cmd) => write!(f, "{cmd}"),
            Self::SetPixel(cmd) => write!(f, "{cmd}"),
        }
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseCommandError {
    #[error("Unknown command")]
    UnknownCommand,

    #[error("Unable to parse SetPixel command: {0}")]
    ParseSetPixelCommand(#[from] ParseSetPixelCommandError),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct SetPixelCommand {
    pub coordinates: Coordinates,
    pub color: Color,
}

impl FromStr for SetPixelCommand {
    type Err = ParseSetPixelCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..3] != "PX " {
            return Err(Self::Err::Syntax);
        }

        let (coordinates, color) = s[3..].rsplit_once(' ').ok_or(Self::Err::Syntax)?;
        let coordinates = coordinates.parse()?;
        let color = color.parse()?;

        Ok(Self { color, coordinates })
    }
}

impl Display for SetPixelCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PX {coordinates} {color}",
            coordinates = self.coordinates,
            color = self.color
        )
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseSetPixelCommandError {
    #[error("Invalid Syntax, Expected: 'PX <u32> <u32> <hex color>")]
    Syntax,

    #[error("Unable to parse coordinates: {0}")]
    ParseCoordinatesError(#[from] ParseCoordinatesError),

    #[error("Unable to parse color: {0}")]
    ParseColorError(#[from] ParseColorError),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GetCanvasSizeCommand;

impl Display for GetCanvasSizeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SIZE")
    }
}

#[cfg(test)]
mod tests {
    use crate::color::{Color, RgbColor, RgbaColor};
    use crate::command::{Command, GetCanvasSizeCommand, ParseCommandError, SetPixelCommand};
    use crate::coordinates::Coordinates;

    #[test]
    pub fn test_parse_get_canvas_size_command() {
        assert_eq!(
            "SIZE".parse(),
            Ok(Command::GetCanvasSize(GetCanvasSizeCommand))
        )
    }

    #[test]
    pub fn test_parse_set_rgb_pixel() {
        assert_eq!(
            "PX 1337 42 c0ffee".parse(),
            Ok(Command::SetPixel(SetPixelCommand {
                coordinates: Coordinates { x: 1337, y: 42 },
                color: Color::Rgb(RgbColor {
                    r: 0xc0,
                    g: 0xff,
                    b: 0xee
                }),
            }))
        )
    }

    #[test]
    pub fn test_parse_set_rgba_pixel() {
        assert_eq!(
            "PX 1337 42 c0ffeeff".parse(),
            Ok(Command::SetPixel(SetPixelCommand {
                coordinates: Coordinates { x: 1337, y: 42 },
                color: Color::Rgba(RgbaColor {
                    rgb: RgbColor {
                        r: 0xc0,
                        g: 0xff,
                        b: 0xee
                    },
                    alpha: 0xff
                }),
            }))
        )
    }

    #[test]
    pub fn test_parse_unknown_command() {
        assert_eq!(
            "FOOBAR fasdfa".parse::<Command>(),
            Err(ParseCommandError::UnknownCommand)
        )
    }
}
