use core::fmt;

use crate::pest::Rule;
use nom::error::Error as nomError;
use nom::Err as nomErr;
use pest::error::Error as pestError;
use thiserror::Error;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum WsvValue {
    Value(String),
    #[default]
    Null,
}
impl fmt::Display for WsvValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&mut String> for WsvValue {
    fn from(string: &mut String) -> WsvValue {
        if string == "-" {
            WsvValue::Null
        } else if string.starts_with('"') && string.ends_with('"') {
            WsvValue::Value(
                string[1..string.len() - 1]
                    .replace("\"/\"", "\n")
                    .replace("\"\"", "\""),
            )
        } else {
            WsvValue::Value(string.clone())
        }
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum WsvError {
    #[error("Double Quotes mismatch on line {0}.")]
    DoubleQuotesMismatch(usize),
    #[error("Malformed input on line {0}.")]
    MalformedInput(usize),
    #[error("Other Error: {0}.")]
    Other(String),
}

impl From<pestError<Rule>> for WsvError {
    fn from(value: pestError<Rule>) -> Self {
        WsvError::Other(value.to_string())
    }
}
impl From<nomErr<nomError<&str>>> for WsvError {
    fn from(value: nomErr<nomError<&str>>) -> Self {
        WsvError::Other(value.to_string())
    }
}
