use serde::{Deserialize, Serialize};

use crate::ParseError;
use core::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        match self {
            Color::Black => "B",
            Color::White => "W",
        }
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

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        color.as_ref().to_string()
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::from_str(&s).map_err(serde::de::Error::custom)
    }
}
