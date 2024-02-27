pub use crate::data_model::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "wsv.pest"]
pub struct WsvParser;

pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, WsvError> {
    Ok(WsvParser::parse(Rule::Wsv, i)?
        .next()
        .expect("Parsing returns exectly one instance of Wsv")
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
