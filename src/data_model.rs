use core::fmt;
use std::fmt::Display;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{Attribute, Cell, Color, Table};
use thiserror::Error;

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Wsv(pub Vec<Vec<WsvValue>>);

impl Default for Wsv {
    fn default() -> Self {
        Self(vec![vec![]])
    }
}

impl fmt::Debug for Wsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Wsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table.apply_modifier(UTF8_ROUND_CORNERS);

        for line in self.into_iter() {
            table.add_row(line.iter().map(|el| {
                match el {
                    WsvValue::Value(val) => {
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
impl IntoIterator for Wsv {
    type Item = Vec<WsvValue>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl IntoIterator for &Wsv {
    type Item = Vec<WsvValue>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

impl From<Vec<Vec<WsvValue>>> for Wsv {
    fn from(input: Vec<Vec<WsvValue>>) -> Self {
        Wsv(input)
    }
}

impl PartialEq<Wsv> for Vec<Vec<WsvValue>> {
    fn eq(&self, other: &Wsv) -> bool {
        other.0.eq(self)
    }
}

impl PartialEq<Vec<Vec<WsvValue>>> for Wsv {
    fn eq(&self, other: &Vec<Vec<WsvValue>>) -> bool {
        self.0.eq(other)
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum WsvValue {
    Value(String),
    #[default]
    Null,
}

impl WsvValue {
    #[tracing::instrument]
    pub fn new(i: &str) -> Self {
        WsvValue::Value(i.into())
    }
}
impl fmt::Display for WsvValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&mut String> for WsvValue {
    fn from(string: &mut String) -> WsvValue {
        From::from(string.as_str())
    }
}
impl From<&str> for WsvValue {
    fn from(string: &str) -> WsvValue {
        if string == "-" {
            WsvValue::Null
        } else if string.starts_with('"') && string.ends_with('"') {
            // let val = string[1..string.len() - 1]
            //     .split("\"")
            //     .map(|c| match c {
            //         "" => "\"",
            //         "/" => "\n",
            //         _ => c,
            //     })
            //     .collect::<String>();
            WsvValue::Value(
                string[1..string.len() - 1]
                    .replace("\"/\"", "\n")
                    .replace("\"\"", "\""),
            )
        } else {
            WsvValue::Value(string.into())
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
