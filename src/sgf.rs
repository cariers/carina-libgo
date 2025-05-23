// re-export sgf_parse
pub use sgf_parse::*;
pub mod iter;

impl From<crate::Color> for Color {
    fn from(c: crate::Color) -> Self {
        match c {
            crate::Color::Black => Color::Black,
            crate::Color::White => Color::White,
        }
    }
}

impl From<Color> for crate::Color {
    fn from(c: Color) -> Self {
        match c {
            Color::Black => crate::Color::Black,
            Color::White => crate::Color::White,
        }
    }
}

impl From<crate::Coord> for go::Point {
    fn from(c: crate::Coord) -> Self {
        Self { x: c.x, y: c.y }
    }
}

impl From<go::Point> for crate::Coord {
    fn from(c: go::Point) -> Self {
        Self { x: c.x, y: c.y }
    }
}
