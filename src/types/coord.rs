use core::fmt;
use std::str::FromStr;

use crate::ParseError;

const CHAR_A: u8 = b'A'; // ASCII value of 'A'

/// Coordinate left-top to right-bottom
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (self.x + CHAR_A) as char,
            (self.x + CHAR_A) as char
        )
    }
}

impl From<Coord> for String {
    fn from(coord: Coord) -> Self {
        format!("{}", coord)
    }
}

impl FromStr for Coord {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseError::InvalidCoordinate);
        }
        let (x, y) = match (s.chars().nth(0), s.chars().nth(1)) {
            (Some(x), Some(y)) if x.is_ascii_uppercase() && y.is_ascii_uppercase() => {
                (x as u8 - CHAR_A, y as u8 - CHAR_A)
            }
            _ => return Err(ParseError::InvalidCoordinate),
        };
        Ok(Coord { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_display() {
        let coord = Coord { x: 0, y: 0 };
        assert_eq!(format!("{}", coord), "AA");
        let coord = Coord { x: 18, y: 18 };
        assert_eq!(format!("{}", coord), "SS");
    }
    #[test]
    fn test_coord_from_str() {
        let coord: Coord = "AA".parse().unwrap();
        assert_eq!(coord.x, 0);
        assert_eq!(coord.y, 0);
        let coord: Coord = "SS".parse().unwrap();
        assert_eq!(coord.x, 18);
        assert_eq!(coord.y, 18);
    }
}
