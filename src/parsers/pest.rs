use pest::error::Error as pestError;
use pest::Parser;
use pest_derive::Parser;

use crate::data_model::*;

#[derive(Parser)]
#[grammar = "parsers/wsv.pest"]
pub struct WsvParser;

pub fn parse(i: &str) -> Result<Wsv, WsvError> {
    Ok(Wsv::from(
        WsvParser::parse(Rule::Wsv, i)?
            .next()
            .expect("Parsing returns exactly one instance of Wsv")
            .into_inner()
            .filter(|line| line.as_rule() != Rule::EOI)
            .map(|l| {
                l.into_inner()
                    .map(
                        |item: pest::iterators::Pair<'_, Rule>| match item.as_rule() {
                            Rule::Value => WsvValue::Value(item.as_str().to_string()),
                            Rule::Null => WsvValue::Null,
                            Rule::String => WsvValue::Value(
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
            .collect::<Vec<Vec<WsvValue>>>(),
    ))
}

impl From<pestError<Rule>> for WsvError {
    fn from(value: pestError<Rule>) -> Self {
        WsvError::Other(value.to_string())
    }
}
