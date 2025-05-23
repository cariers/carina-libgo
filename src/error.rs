#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("empty string")]
    EmptyString,
    #[error("invalid color")]
    InvalidColor,
    #[error("invalid coordinate")]
    InvalidCoordinate,
}
