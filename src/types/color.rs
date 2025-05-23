use core::fmt;
use std::str::FromStr;

use serde::Serialize;

use crate::ParseError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black => write!(f, "B"),
            Color::White => write!(f, "W"),
        }
    }
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        format!("{}", color)
    }
}

impl FromStr for Color {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Color::Black),
            "W" => Ok(Color::White),
            _ => Err(ParseError::InvalidColor),
        }
    }
}

impl core::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        self.opposite()
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}
