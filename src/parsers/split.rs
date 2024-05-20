//! Deliberately the most convoluted implementation I could think of.
//!
//! This splits a given line by the special characters and performs logic on the parts. Because of
//! the requirement that we have an even number of double quotes, there are different considerations
//! to make depending on the parity of the string part. This was very unpleasant to write, fix and
//! improve, but it makes for an interesting comparison with is all about tracing and minor
//! tweaks here and there, none of which make any superficial sense.
//! Enjoy!

use crate::data_model::*;
use itertools::{Itertools, Position};
use tracing::debug;

pub fn parse_strict(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

pub fn parse(i: &str) -> Vec<Result<Vec<WsvValue>, Error>> {
    i.split('\n').enumerate().map(parse_line).collect()
}

/// Handles comments.
/// If we have an input ike `value "with # hash" then # comment` and we split on `#`, the first part
/// of the split (`value " with `)is not valid, since we only have one double quote. So we stick the
/// next part on and try that: `value "with # hash" then `, and this is valid if and only if the hash
/// we split on signals a comment.
pub fn parse_line((line_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let line_number = line_index + 1;
    let mut parts_without_comment = Vec::new();
    let mut line_without_comment = String::new();
    // Since we haven't identified those hashes which are parts of strings yet, it's not clear if
    // the part before the first hash is the whole real input. If parsing that fails, it might be
    // because we excluded too much, so we append the next string part and check that.
    for part_between_hashes in line.split('#') {
        parts_without_comment.push(part_between_hashes);
        line_without_comment = parts_without_comment.iter().join("#");

        let even_number_of_quotes = line_without_comment.split('\"').count() % 2 == 1;
        if even_number_of_quotes {
            return parse_line_without_comments((line_number, &line_without_comment));
        }
    }
    Err(Error::new(
        ErrorKind::OddDoubleQuotes,
        line_number,
        line_without_comment.len() + 1,
        None,
    ))
}

fn parse_line_without_comments((line_number, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let mut result: Vec<WsvValue> = Vec::new();
    let mut string = String::new();
    let mut col = 0;

    for (i, (position, part)) in line.split('\"').with_position().enumerate() {
        let even_part = i % 2 == 1; // i is 0-indexed, so even parts are odd-indexed.
                                    // Only, First and Last are all odd parts anyway, but I added it into the match for symmetry.
        match (position, even_part) {
            (Position::Only, _) => result.append(&mut process_part(part).0),
            (Position::First, _) => {
                let (mut these_parts, _, leading_ws) = process_part(part);
                col += part.len() + 1;
                if !leading_ws && !part.is_empty() {
                    return Err(Error::new(
                        ErrorKind::MissingWhitespace,
                        line_number,
                        col,
                        None,
                    ));
                } else {
                    result.append(&mut these_parts);
                }
            }
            (Position::Last, _) => {
                let (mut these_parts, trailing_ws, _) = process_part(part);
                if !trailing_ws && !part.is_empty() {
                    return Err(Error::new(
                        ErrorKind::MissingWhitespace,
                        line_number,
                        col + 1,
                        None,
                    ));
                } else {
                    result.push(WsvValue::new(&string));
                    string.clear();
                    result.append(&mut these_parts);
                }
            }
            (Position::Middle, true) => {
                col += part.len() + 1; // + 1 for the missing quote
                string.push_str(part);
            }
            (Position::Middle, false) => match identify_string_part(part) {
                Decision::SpecialCharacter(ch) => string.push(ch),
                Decision::EndOfString => {
                    result.push(WsvValue::new(&string));
                    string.clear();
                    let (mut these_parts, trailing_ws, leading_ws) = process_part(part);
                    if !leading_ws {
                        return Err(Error::new(
                            ErrorKind::MissingWhitespace,
                            line_number,
                            col + part.len() + 1,
                            None,
                        ));
                    } else if !trailing_ws {
                        return Err(Error::new(
                            ErrorKind::MissingWhitespace,
                            line_number,
                            col + 1,
                            None,
                        ));
                    } else {
                        col += part.len() + 1;
                        result.append(&mut these_parts);
                    }
                }
            },
        }
    }
    Ok(result)
}

/// returns the processed part but remembers if there was leading or trailing whitespace.
/// Assumes that the main_part only contains well-formed values and nulls.
fn process_part(main_part: &str) -> (Vec<WsvValue>, bool, bool) {
    let len = main_part.chars().count();
    let chars = main_part.chars().collect::<Vec<char>>();
    (
        // main part only consists of values and nulls, defined by the original spec.
        main_part
            .split_whitespace()
            .map(|str| match str {
                "-" => WsvValue::Null,
                _ => WsvValue::new(str),
            })
            .collect(),
        len == 0 || chars[0].is_whitespace(),
        len == 0 || chars[len - 1].is_whitespace(),
    )
}

fn identify_string_part(string_part: &str) -> Decision {
    debug!(string_part);
    if string_part == "/" {
        Decision::SpecialCharacter('\n')
    } else if string_part.is_empty() {
        Decision::SpecialCharacter('\"')
    } else {
        Decision::EndOfString
    }
}

enum Decision {
    SpecialCharacter(char),
    EndOfString,
}

#[cfg(test)]
use crate::unit;
#[cfg(test)]
unit! {}
