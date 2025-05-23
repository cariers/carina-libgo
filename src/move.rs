use core::fmt;
use std::str::FromStr;

use serde::Serialize;

use crate::ParseError;

const CHAR_LOWER_A: u8 = b'a'; // ASCII value of 'a'

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Pass,
    Coordinate { x: u8, y: u8 },
}

impl Move {
    #[inline]
    pub fn new(x: u8, y: u8) -> Self {
        Move::Coordinate { x, y }
    }

    #[inline]
    pub fn pass() -> Self {
        Move::Pass
    }

    #[inline]
    pub fn from_index(index: usize, board_size: usize) -> Self {
        Move::Coordinate {
            x: (index % board_size) as u8,
            y: (index / board_size) as u8,
        }
    }

    #[inline]
    pub fn index(&self, board_size: usize) -> usize {
        match self {
            Move::Pass => usize::MAX,
            Move::Coordinate { x, y } => (*y as usize) * board_size + (*x as usize),
        }
    }
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "pass" {
            return Ok(Move::Pass);
        }
        if s.len() != 2 {
            return Err(ParseError::InvalidCoordinate);
        }

        match (s.chars().nth(0), s.chars().nth(1)) {
            (Some(x), Some(y)) if x.is_ascii_lowercase() && y.is_ascii_lowercase() => {
                let x = x as u8 - CHAR_LOWER_A;
                let y = y as u8 - CHAR_LOWER_A;
                Ok(Move::Coordinate { x, y })
            }
            _ => Err(ParseError::InvalidCoordinate),
        }
    }
}

impl From<Move> for String {
    fn from(pos: Move) -> Self {
        match pos {
            Move::Pass => "pass".to_string(),
            Move::Coordinate { x, y } => format!(
                "{}{}",
                (x + CHAR_LOWER_A) as char,
                (y + CHAR_LOWER_A) as char
            ),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Pass => write!(f, "pass"),
            Move::Coordinate { x, y } => write!(
                f,
                "{}{}",
                (x + CHAR_LOWER_A) as char,
                (y + CHAR_LOWER_A) as char
            ),
        }
    }
}

impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Move {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
