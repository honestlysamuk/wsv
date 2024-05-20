//! This is the implementation of the pest deserialisation parser for WSV.
//!
//! It uses the wsv.pest grammar in the same file to produce a tree that is processed easily in the function. The standard
//! errors returned have enough information in them to parse into `wsv::Error`s, but it's a bit hacky.
//!
//! While pest is among the slowest to execute and definitely to slowest to compile, it was the most likely to pass new tests
//! and was one of the fastest to implement from ignorance.
//!
//! There is no `fn parse` here, since that would require a separate `.pest` grammar, which defeats the point of a grammar for me.

use crate::data_model::*;
use pest::error::Error as pestError;
use pest::error::ErrorVariant;
use pest::error::LineColLocation::{Pos, Span};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parsers/wsv.pest"]
struct WsvParser;

pub fn parse_strict(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    Ok(WsvParser::parse(Rule::Wsv, i)?
        .next()
        .expect("Parsing returns exactly one instance of Wsv")
        .into_inner()
        .filter(|line| line.as_rule() != Rule::EOI)
        .map(|l| {
            l.into_inner()
                .map(
                    |item: pest::iterators::Pair<'_, Rule>| match item.as_rule() {
                        Rule::Value => WsvValue::V(item.as_str().to_string()),
                        Rule::Null => WsvValue::Null,
                        Rule::String => WsvValue::V(
                            item.into_inner()
                                .map(|part| match part.as_rule() {
                                    Rule::NewLine => "\n",
                                    Rule::DoubleQuote => "\"",
                                    Rule::StringPart => part.as_str(),
                                    _ => unreachable!(),
                                })
                                .fold("".to_owned(), |string, part| string + part),
                        ),
                        _ => unreachable!(),
                    },
                )
                .collect()
        })
        .collect())
}

impl From<pestError<Rule>> for Error {
    fn from(value: pestError<Rule>) -> Self {
        let (row, col) = match value.line_col {
            Pos((a, b)) => (a, b),
            Span((a, b), (_, _)) => (a, b),
        };
        let kind = match value.variant {
            ErrorVariant::ParsingError { positives, .. } => match positives.len() {
                3 => ErrorKind::OddDoubleQuotes,
                _ => ErrorKind::MissingWhitespace,
            },
            _ => ErrorKind::MissingWhitespace,
        };
        Error::new(kind, row, col, None)
    }
}

#[cfg(test)]
use crate::unit;
#[cfg(test)]
unit! {}
