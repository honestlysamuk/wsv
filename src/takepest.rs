pub use self::data_model::*;
use pest::Parser;
use pest_derive::Parser;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Parser)]
#[grammar = "wsv.pest"]
pub struct WsvParser;

pub fn parse<P>(path: P) -> Result<Vec<WsvLine>, WsvError>
where
    P: AsRef<Path>,
{
    let input = read_to_string(&path).expect("cannot read file");

    Ok(WsvParser::parse(Rule::wsv, &input)?
        .next()
        .unwrap()
        .into_inner()
        .inspect(|l| println!("{l:?}"))
        .filter(|line| line.as_rule() != Rule::EOI)
        .map(|l| {
            l.into_inner()
                .map(|part| match part.as_rule() {
                    Rule::value | Rule::string => WsvValue::from(part.as_str().to_owned()),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect())
}

#[derive(Debug)]
pub enum WsvError {
    ParseError(String),
}
impl Display for WsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl From<pest::error::Error<Rule>> for WsvError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        WsvError::ParseError(value.to_string())
    }
}

mod data_model {
    use core::fmt;

    pub type WsvLine = Vec<WsvValue>;

    #[derive(Default, Debug, PartialEq)]
    pub enum WsvValue {
        Value(String),
        #[default]
        Null,
        Comment(String),
    }
    impl fmt::Display for WsvValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }
    impl From<String> for WsvValue {
        fn from(string: String) -> WsvValue {
            if string == "-" {
                WsvValue::Null
            } else if string.starts_with('"') && string.ends_with('"') {
                WsvValue::Value(
                    string[1..string.len() - 1]
                        .replace("\"/\"", "\n")
                        .replace("\"\"", "\""),
                )
            } else {
                WsvValue::Value(string)
            }
        }
    }
}
