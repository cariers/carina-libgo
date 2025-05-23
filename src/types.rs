mod color;
mod coord;
mod vertex;

use core::fmt;
use serde::{Serialize, ser::SerializeTuple};

pub use color::Color;
pub use coord::Coord;
pub use vertex::Vertex;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Move(pub Color, pub Vertex);

impl From<(Color, Vertex)> for Move {
    fn from((color, vertex): (Color, Vertex)) -> Self {
        Move(color, vertex)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.0, self.1)
    }
}

impl From<(Color, Coord)> for Move {
    fn from((color, coord): (Color, Coord)) -> Self {
        Move(color, Vertex::from(coord))
    }
}

impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&self.0)?;
        state.serialize_element(&self.1)?;
        state.end()
    }
}
