//! This was my first attempt at parsing a wsv file. There was no design in mind.
//!
//! It is pretty fast, passes every test and the worst to debug. Error detection is accurate,
//! and error correction is possible. Originally it had some logic after the loop, but I
//! took some inspiration from the state machines and chose to iterate over an `Option<char>`
//! which let me include that logic in the `None` case within the loop instead.
use crate::data_model::*;

pub fn parse_strict(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

pub fn parse(i: &str) -> Vec<Result<Vec<WsvValue>, Error>> {
    i.split('\n').enumerate().map(parse_line).collect()
}

pub fn parse_line((line_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let row = line_index + 1;
    let mut values: Vec<WsvValue> = Vec::new();
    let mut buf: String = String::new();
    let mut col = 0;
    // tracker variable which enables proper column labelling on error. It contains the position
    // of every other double quote. If it is `None` at the end, the file is invalid.
    let mut closing_quote_pos = Some(0);
    let mut chars = line.chars();

    loop {
        col += 1;
        match chars.next() {
            Some('"') => {
                if buf.is_empty() || buf.starts_with('"') {
                    closing_quote_pos = match closing_quote_pos {
                        None => Some(col),
                        Some(_) => None,
                    };
                    buf.push('"');
                } else {
                    return Err(Error::new(ErrorKind::MissingWhitespace, row, col, None));
                }
            }
            Some('#') => {
                if closing_quote_pos.is_none() {
                    buf.push('#');
                } else {
                    break;
                }
            }
            Some(c) if c.is_whitespace() => {
                if closing_quote_pos.is_none() {
                    buf.push(c);
                } else if !buf.is_empty() {
                    if buf == "\"" || (buf.starts_with('"') && !buf.ends_with('"')) {
                        return Err(Error::new(ErrorKind::OddDoubleQuotes, row, col + 1, None));
                    }
                    values.push(parse_value(&mut buf));
                    buf.clear();
                }
                // ignore otherwise
            }
            Some('/') => {
                buf.push('/');
            }
            Some(c) => {
                if let Some(pos) = closing_quote_pos {
                    // in other words, if we get a character (not matching the other options above) immediately
                    // after a closing double quote (pos + 1 == col), then we assume it's the start of a new value
                    // and we should have some whitespace here instead. `pos != 0` to exclude the beginning.
                    if pos + 1 == col && pos != 0 {
                        return Err(Error::new(ErrorKind::MissingWhitespace, row, col, None));
                    }
                }
                buf.push(c);
            }
            None => {
                if !buf.is_empty() {
                    if buf == "\"" || (buf.starts_with('"') && !buf.ends_with('"')) {
                        return Err(Error::new(ErrorKind::OddDoubleQuotes, row, col, None));
                    }
                    values.push(parse_value(&mut buf));
                }
                break;
            }
        }
    }
    Ok(values)
}

/// Assume we have a well-formed buf.
fn parse_value(buf: &mut str) -> WsvValue {
    if buf == "-" {
        WsvValue::Null
    } else if buf.starts_with('"') && buf.ends_with('"') {
        WsvValue::V(
            buf[1..buf.len() - 1]
                .replace("\"/\"", "\n")
                .replace("\"\"", "\""),
        )
    } else {
        WsvValue::V(buf.into())
    }
}

#[cfg(test)]
use crate::unit;
#[cfg(test)]
unit! {}
