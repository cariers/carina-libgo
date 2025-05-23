use core::fmt;
use std::str::FromStr;

use crate::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Pass,
    Coordinate(char, u8),
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Pass => write!(f, "pass"),
            Move::Coordinate(x, y) => write!(f, "{}{}", x, y),
        }
    }
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.is_empty() {
            return Err(Self::Err::EmptyString);
        }
        let str = str.to_uppercase();
        if str == "PASS" {
            return Ok(Move::Pass);
        }
        let c = str
            .as_bytes()
            .get(0)
            .ok_or_else(|| ParseError::InvalidCoordinate)?;
        let s = str
            .get(1..)
            .ok_or_else(|| ParseError::InvalidCoordinate)?
            .parse::<u8>()
            .map_err(|_| ParseError::InvalidCoordinate)?;

        Ok(Move::Coordinate(*c as char, s))
    }
}

impl crate::Move {
    pub fn to_gtp(self, board_size: u8) -> Move {
        match self {
            crate::Move::Pass => Move::Pass,
            crate::Move::Coordinate { x, y } => {
                let x = if x > 7 { x + 2 + b'A' } else { x + 1 + b'A' };
                let y = board_size - y;
                Move::Coordinate(x as char, y)
            }
        }
    }

    pub fn from_gtp(m: Move, board_size: u8) -> Self {
        match m {
            Move::Pass => crate::Move::Pass,
            Move::Coordinate(x, y) => {
                let x = x as u8;
                let x = if x > b'I' { x - b'A' - 2 } else { x - b'A' - 1 };
                let y = board_size - y - 1;
                crate::Move::Coordinate { x, y }
            }
        }
    }
}
