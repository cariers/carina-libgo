#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("invalid color")]
    InvalidColor,
    #[error("invalid coordinate")]
    InvalidCoordinate,
}
