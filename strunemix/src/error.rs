use thiserror::Error;

/// Strunemix errors in conversions
#[derive(Error, Debug, PartialEq)]
pub enum StrunemixFromError {
    #[error("Invalid input data, wrong order of data")]
    WrongOrder,
    #[error("Invalid input data, some data attributes are missing while others are present more than once")]
    AppearedMoreThanOnce,
    #[error("The string '{0}' is not a valid for the enum {1}")]
    NotAnEnumName(String, String),
}

/// General Strunemix error
#[derive(Error, Debug)]
pub enum StrunemixError {
    #[error("Parse error: {0}")]
    ParseError(#[from] StrunemixParseError),
    #[error("Incomplete form")]
    IncompleteForm,
    #[error("Conversion error: {0}")]
    ConversionError(#[from] StrunemixFromError),
}

/// Strunemix errors in data parsing
#[derive(Error, Debug)]
pub enum StrunemixParseError{
    #[error("Invalid data: {0}")]
    Other(Box<dyn std::error::Error>),
    #[error("Invalid Integer: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Invalid Float: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError), 
    #[error("Invalid Bool: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("Invalid Char: {0}")]
    ParseCharError(#[from] std::char::ParseCharError),
    #[error("Invalid String: {0}")]
    ParseError(#[from] std::string::ParseError),
}