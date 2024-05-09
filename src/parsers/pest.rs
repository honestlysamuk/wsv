//! This is the implementation of the pest deserialisation parser for WSV.
//!
//! It uses the wsv.pest grammar in the same file to produce a tree that is processed easily in the function. The error handling could be improved, but the Pest Error type has a generic and I don't yet know how to handle that in a pretty way, so I'm just .to_string()ing it for now. It's also heavily nested, but I don't see any value in splitting off an internal function to handle the string parsing, for example, unless it aids in Error handling. I am working my way through Luca Palmieri's article on errors and am looking to apply the same principles here.

use pest::error::Error as pestError;
use pest::Parser;
use pest_derive::Parser;
use pest::error::LineColLocation::{Pos, Span};
use crate::data_model::*;

#[derive(Parser)]
#[grammar = "parsers/wsv.pest"]
struct WsvParser;

pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    Ok(
        WsvParser::parse(Rule::Wsv, i)?
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
            .collect()
    )
}

impl From<pestError<Rule>> for Error {
    fn from(value: pestError<Rule>) -> Self {
        let (row, col) =
            match value.line_col {
                Pos((a, b)) => (a, b),
                Span((a, b), (_, _)) => (a, b),
            };
        Error::new(ErrorKind::Pest, row, col, Some(Box::new(value)))
    }
}
