pub use self::data_model::*;
use pest::Parser;
use std::fs::read_to_string;
use std::path::Path;

pub fn parse<P>(path: P) -> Result<Vec<WsvLine>, WsvError>
where
    P: AsRef<Path>,
{
    let input = read_to_string(&path).expect("cannot read file");

    Ok(WsvParser::parse(Rule::Wsv, &input)?
        .next()
        .unwrap()
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
        .collect())
}

mod data_model {
    use core::fmt;
    use pest_derive::Parser;
    use thiserror::Error;

    pub type WsvLine = Vec<WsvValue>;

    #[derive(Parser)]
    #[grammar = "wsv.pest"]
    pub struct WsvParser;

    #[derive(Default, Debug, PartialEq, Clone)]
    pub enum WsvValue {
        Value(String),
        #[default]
        Null,
        Comment(String),
    }
    impl fmt::Display for WsvValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Error, Debug, Clone, PartialEq)]
    pub enum WsvError {
        #[error("Parse Error: {0}.")]
        ParseError(String),
    }
    impl From<pest::error::Error<Rule>> for WsvError {
        fn from(value: pest::error::Error<Rule>) -> Self {
            WsvError::ParseError(value.to_string())
        }
    }
}
