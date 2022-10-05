use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone)]
pub enum ParseError {
    Expected(usize, String),
    Unexpected(usize, String),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ParseError::Expected(pos, expected) => {
                write!(f, "Expected '{}' at pos {}", expected, pos)
            }
            ParseError::Unexpected(pos, unexpected) => {
                write!(f, "Unexpected '{}' at pos {}", unexpected, pos)
            }
        }
    }
}
