//! Deliberately the most convoluted implementation I could think of.
//! This splits a given line by the special characters and performs
//! logic on the parts. Adding features to this is incredibly difficult, and fixing bugs is
//! all about tracing and minor tweaks here and there, none of hich make any superficial sense.
//! Enjoy!

use crate::data_model::*;
use std::collections::VecDeque;
use tracing::debug;

pub fn parse(i: &str) -> Result<Wsv, WsvError> {
    if i.is_empty() {
        Ok(Default::default())
    } else {
        match i
            .split('\n')
            .enumerate()
            .map(parse_line)
            .collect::<Result<Vec<Vec<WsvValue>>, WsvError>>()
        {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(e),
        }
    }
}

fn parse_line((line_index, line): (usize, &str)) -> Result<Vec<WsvValue>, WsvError> {
    let line = line.trim_matches(|c: char| c.is_whitespace());
    let mut input = String::new();
    for line_before_comment in line.split('#') {
        input.push_str(line_before_comment);

        match parse_line_without_comments((line_index + 1, &input)) {
            Ok(result) => return Ok(result),
            Err(WsvError::MalformedInput(e)) => return Err(WsvError::MalformedInput(e)),
            Err(_) => { /* try again */ }
        }
    }
    Err(WsvError::DoubleQuotesMismatch(line_index + 1))
}

fn parse_line_without_comments(
    (line_number, line): (usize, &str),
) -> Result<Vec<WsvValue>, WsvError> {
    let mut result: Vec<WsvValue> = Vec::new();
    let mut string = String::new();
    let mut index = 0;
    let count = line.split('\"').count();
    let parts = line.split('\"').enumerate();

    for (i, part) in parts {
        if let Err(e) = process_parts(
            i,
            part,
            count,
            line_number,
            &mut result,
            &mut string,
            &mut index,
        ) {
            return Err(e);
        }
    }

    if !string.is_empty() {
        result.push(WsvValue::new(&mut string));
        debug!(order = "sixth", line = ?result);
        string.clear();
    }
    if index % 2 == 1 {
        Err(WsvError::DoubleQuotesMismatch(line_number))
    } else {
        Ok(result)
    }
}

#[tracing::instrument]
fn process_parts(
    i: usize,
    part: &str,
    count: usize,
    y: usize,
    result: &mut Vec<WsvValue>,
    string: &mut String,
    index: &mut usize,
) -> Result<(), WsvError> {
    let first = i == 0 && count > 1;
    let last = i == count - 1 && count > 1;
    let only = i == 0 && count == 1;
    *index = i;

    tracing::debug!(only, first, last);

    if only {
        result.append(&mut process_part(part).0.into());
    } else if first {
        let (these_parts, _, trailing_ws) = process_part(part);
        if !trailing_ws && !part.is_empty() {
            debug!(error = "No trailing whitespace.", ?result);
            return Err(WsvError::MalformedInput(y));
        } else {
            result.append(&mut these_parts.into());
        }
    } else if last {
        let (these_parts, leading_ws, _) = process_part(part);
        if !leading_ws && !part.is_empty() {
            debug!(error = "No leading whitespace.", ?result);
            return Err(WsvError::MalformedInput(y));
        } else {
            result.push(WsvValue::new(string));
            string.clear();
            result.append(&mut these_parts.into());
        }
    } else {
        if i % 2 == 1 {
            string.push_str(part);
        } else {
            match process_string_part(part) {
                Decision::SpecialCharacter(ch) => string.push(ch),
                Decision::EndOfString => {
                    if !string.is_empty() {
                        result.push(WsvValue::new(string));
                        string.clear();
                    };
                    let (these_parts, leading_ws, trailing_ws) = process_part(part);
                    if leading_ws && trailing_ws {
                        result.append(&mut these_parts.into());
                    } else {
                        debug!(error = "No leading or trailing whitespace.", ?result);
                        return Err(WsvError::MalformedInput(y));
                    }
                }
            }
        }
    }
    Ok(())
}

fn process_part(main_part: &str) -> (VecDeque<WsvValue>, bool, bool) {
    let length = main_part.chars().count();
    let chars = main_part.chars().collect::<Vec<char>>();
    let leading_ws = length == 0 || chars.first().unwrap().is_whitespace();
    let trailing_ws = length == 0 || chars.last().unwrap().is_whitespace();

    let split = main_part.split_whitespace().collect::<VecDeque<&str>>();
    debug!(?split);
    (
        split.iter().map(|s| WsvValue::from(*s)).collect(),
        leading_ws,
        trailing_ws,
    )
}

fn process_string_part(string_part: &str) -> Decision {
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
