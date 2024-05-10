use core::fmt;
use std::fmt::Display;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{Attribute, Cell, Color, Table};

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Wsv(pub Vec<Vec<WsvValue>>);

impl fmt::Debug for Wsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Wsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table.apply_modifier(UTF8_ROUND_CORNERS);

        for line in &self.0 {
            table.add_row(line.iter().map(|el| {
                match el {
                    WsvValue::V(val) => {
                        if val.is_empty() {
                            Cell::new("Empty")
                                .add_attribute(Attribute::Bold)
                                .fg(Color::Blue)
                        } else {
                            Cell::new(val)
                        }
                    }
                    WsvValue::Null => Cell::new("NULL")
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Green),
                }
            }));
        }
        write!(f, "{}", table)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WsvValue {
    V(String),
    Null
}

impl WsvValue {
    pub fn new(i: &str) -> Self {
        WsvValue::V(i.into())
    }
}
impl fmt::Display for WsvValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&mut String> for WsvValue {
    fn from(string: &mut String) -> WsvValue {
        WsvValue::V(string.to_owned())
    }
}
impl From<&str> for WsvValue {
    fn from(string: &str) -> WsvValue {
        WsvValue::V(string.to_owned())
    }
}
impl From<String> for WsvValue {
    fn from(string: String) -> WsvValue {
        WsvValue::V(string)
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub row: usize,
    pub col: usize,
    pub source: Option<Box<dyn std::error::Error>>,
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on row {}, position {}\nCaused by {:?}", self.kind, self.row, self.col, self.source)
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    OddDoubleQuotes,
    NoLeadingWhitespace,
    NoTrailingWhitespace,
    MissingWhitespace,
    Nom,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::OddDoubleQuotes => "Odd number of double quotes detected",
            Self::NoLeadingWhitespace => "Missing whitespace on the left side of this double quote",
            Self::NoTrailingWhitespace => "Missing whitespace on the right side of this double quote",
            Self::MissingWhitespace => "Missing whitespace on one side of this double quote",
            Self::Nom => "Nom Error",
        })
    }
}

impl Error {
    pub fn new(kind: ErrorKind, row: usize, col: usize, source: Option<Box<dyn std::error::Error>>) -> Error {
        Error {kind, row, col, source}
    }
}