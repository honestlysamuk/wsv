//! Deliberately the most convoluted implementation I could think of.
//! This splits a given line by the special characters and performs
//! logic on the parts. Adding features to this is incredibly difficult, and fixing bugs is
//! all about tracing and minor tweaks here and there, none of hich make any superficial sense.
//! Enjoy!

use crate::data_model::*;
use itertools::{Itertools, Position};
use tracing::debug;

pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

fn parse_line((line_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let line_number = line_index + 1;
    let mut line_without_comment = String::new();
    // Since we haven't identified those hashes which are parts of strings yet, it's not clear if
    // the part before the first hash is the whole real input. If parsing that fails, it might be
    // because we excluded too much of the initial string.
    for line_before_comment in line.split('#') {
        if line_without_comment.is_empty() {
            line_without_comment += line_before_comment;
        } else {
            line_without_comment = line_without_comment + "#" + line_before_comment;
        }
        let even_number_of_quotes = line_without_comment.split('\"').count() % 2 == 1;
        if even_number_of_quotes {
            return parse_line_without_comments((line_number, &line_without_comment));
        }
    }
    Err(dbg!(Error::new(ErrorKind::OddDoubleQuotes, line_number, 0, None)))
}

fn parse_line_without_comments(
    (line_number, line): (usize, &str),
) -> Result<Vec<WsvValue>, Error> {
    let mut result: Vec<WsvValue> = Vec::new();
    let mut string = String::new();

    for (i, (position, part)) in line.split('\"').with_position().enumerate() {
        match position {
            Position::Only => result.append(&mut process_part(part).0.into()),
            Position::First => {
                let (these_parts, _, trailing_ws) = process_part(part);
                if !trailing_ws && !part.is_empty() {
                    debug!(error = "No trailing whitespace", part);
                    return Err(dbg!(Error::new(ErrorKind::NoTrailingWhitespace, line_number, 0, None)));
                } else {
                    result.append(&mut these_parts.into());
                }
            }
            Position::Last => {
                let (these_parts, leading_ws, _) = process_part(part);
                if !leading_ws && !part.is_empty() {
                    debug!(error = "No leading whitespace", part);
                    return Err(dbg!(Error::new(ErrorKind::NoLeadingWhitespace, line_number, 0, None)));
                } else {
                    result.push(WsvValue::new(&mut string));
                    string.clear();
                    result.append(&mut these_parts.into());
                }
            }
            Position::Middle => {
                if i % 2 == 1 {
                    string.push_str(part);
                } else {
                    match identify_string_part(part) {
                        Decision::SpecialCharacter(ch) => string.push(ch),
                        Decision::EndOfString => {
                            result.push(WsvValue::new(&mut string));
                            string.clear();
                            let (these_parts, leading_ws, trailing_ws) = process_part(part);
                            if !leading_ws {
                                debug!(error = "No leading whitespace", part);
                                return Err(dbg!(Error::new(ErrorKind::NoLeadingWhitespace, line_number, 0, None)));
                              }  else if  !trailing_ws {
                                debug!(error = "No trailing whitespace", part);
                                return Err(dbg!(Error::new(ErrorKind::NoTrailingWhitespace, line_number, 0, None)));
                            } else {
                                result.append(&mut these_parts.into());
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(result)
}

/// returns the processed part but remembers if there was leading or trailing whitespace.
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
