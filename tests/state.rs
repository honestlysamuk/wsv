mod tests;
use crate::tests::*;
use wsv::state::parse;

#[test]
fn null() {
    null_test(&parse)
}
#[test]
fn numbers() {
    numbers_test(&parse)
}
#[test]
fn strings() {
    strings_test(&parse)
}
#[test]
fn comments() {
    comments_test(&parse)
}
#[test]
fn not_null() {
    not_null_test(&parse)
}
#[test]
fn empty() {
    empty_test(&parse)
}
#[test]
fn malformed() {
    malformed_test(&parse)
}
#[test]
fn odd_quotes() {
    odd_quotes_test(&parse)
}
#[test]
fn parse_example() {
    parse_test(&parse)
}
#[test]
fn single_slash() {
    single_slash_test(&parse)
}
#[test]
fn empty_string() {
    empty_string_test(&parse)
}
#[test]
fn trailing_return() {
    trailing_return_test(&parse)
}
#[test]
fn no_leading_whitespace() {
    no_leading_whitespace_test(&parse)
}
#[test]
fn no_trailing_whitespace() {
    no_trailing_whitespace_test(&parse)
}