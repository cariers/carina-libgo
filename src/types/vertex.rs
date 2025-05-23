use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::ParseError;

use super::Coord;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Vertex {
    Pass,
    Coord(Coord),
}

impl Vertex {
    pub fn is_pass(&self) -> bool {
        matches!(self, Vertex::Pass)
    }

    pub fn coord(&self) -> Option<Coord> {
        match self {
            Vertex::Coord(coord) => Some(*coord),
            Vertex::Pass => None,
        }
    }

    pub fn into_coord(self) -> Option<Coord> {
        match self {
            Vertex::Coord(coord) => Some(coord),
            Vertex::Pass => None,
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Vertex::Pass => write!(f, "PASS"),
            Vertex::Coord(coord) => write!(f, "{}", coord),
        }
    }
}

impl From<Vertex> for String {
    fn from(mv: Vertex) -> Self {
        format!("{}", mv)
    }
}

impl From<Coord> for Vertex {
    fn from(coord: Coord) -> Self {
        Vertex::Coord(coord)
    }
}

impl FromStr for Vertex {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PASS" => Ok(Vertex::Pass),
            s => Coord::from_str(s).map(Vertex::Coord),
        }
    }
}

impl Serialize for Vertex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Vertex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Vertex::from_str(&s).map_err(serde::de::Error::custom)
    }
}
