use core::fmt;
use std::fmt::Display;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{Attribute, Cell, Color, Table};

#[derive(Default, Debug, Copy, Clone)]
pub(crate) enum Parser {
    Nom,
    State,
    Mealy,
    Moore,
    Split,
    #[default]
    First,
}

type ParserFn = &'static dyn Fn((usize, &str)) -> Result<Vec<WsvValue>, Error>;

impl Parser {
    pub fn fn_ptr(self) -> ParserFn {
        match self {
            Parser::First => &crate::first::parse_line,
            Parser::Nom => &crate::nom::parse_line,
            Parser::Split => &crate::split::parse_line,
            Parser::State => &crate::state::parse_line,
            Parser::Moore => &crate::moore::parse_line,
            Parser::Mealy => &crate::mealy::parse_line,
        }
    }
}

#[repr(transparent)]
pub struct Wsv(pub Vec<Result<Vec<WsvValue>, Error>>);

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
            match line {
                Err(e) => {
                    table.add_row(vec![Cell::new(e.to_string())
                        .add_attribute(Attribute::Bold)
                        .fg(Color::DarkRed)]);
                }
                Ok(line) => {
                    table.add_row(line.iter().map(|el| {
                        match el {
                            WsvValue::Null => Cell::new("NULL")
                                .add_attribute(Attribute::Bold)
                                .fg(Color::Green),
                            WsvValue::V(val) if val.is_empty() => Cell::new("Empty String")
                                .add_attribute(Attribute::Bold)
                                .fg(Color::Blue),
                            WsvValue::V(val) => Cell::new(val),
                        }
                    }));
                }
            }
        }
        write!(f, "{}", table)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WsvValue {
    V(String),
    Null,
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
        match &self.source {
            Some(val) => {
                write!(
                    f,
                    "{} on row {}, col {}\nCaused by {:?}",
                    self.kind, self.row, self.col, val
                )
            }
            None => {
                write!(f, "{} on row {}, col {}", self.kind, self.row, self.col)
            }
        }
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    OddDoubleQuotes,
    MissingWhitespace,
    Nom,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OddDoubleQuotes => "Odd number of double quotes detected",
                Self::MissingWhitespace => "Whitespace expected",
                Self::Nom => "Nom Error",
            }
        )
    }
}

impl Error {
    pub fn new(
        kind: ErrorKind,
        row: usize,
        col: usize,
        source: Option<Box<dyn std::error::Error>>,
    ) -> Error {
        Error {
            kind,
            row,
            col,
            source,
        }
    }
}
