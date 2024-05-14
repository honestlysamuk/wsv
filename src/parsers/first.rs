use crate::data_model::*;

pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

fn parse_line((line_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let row = line_index + 1;
    let mut values: Vec<WsvValue> = Vec::new();
    let mut buf: String = String::new();
    let mut col = 0;
    let mut closing_quote_pos = Some(0);
    for c in line.chars() {
        col += 1;
        match c {
            '"' => {
                if buf.is_empty() || buf.starts_with('\"') {
                    closing_quote_pos = match closing_quote_pos {
                        None => Some(col),
                        Some(_) => None,
                    };
                    buf.push(c);
                } else {
                    dbg!("");
                    return Err(Error::new(ErrorKind::MissingWhitespace, row, col, None));
                }
            }
            '#' => {
                if closing_quote_pos.is_none() {
                    buf.push(c);
                } else {
                    break;
                }
            }
            c if c.is_whitespace() => {
                if closing_quote_pos.is_none() {
                    buf.push(c);
                } else if !buf.is_empty() {
                    if buf == "\"" || (buf.starts_with('"') && !buf.ends_with('"')) {
                        dbg!("");
                        return Err(Error::new(ErrorKind::OddDoubleQuotes, row, col + 1, None));
                    }
                    values.push(parse_value(&mut buf));
                    buf.clear();
                }
                // ignore otherwise
            }
            '/' => {
                buf.push(c);
            }
            _ => {
                if let Some(c) = closing_quote_pos {
                    if c + 1 == col && c != 0 {
                        dbg!("");
                        return Err(Error::new(ErrorKind::MissingWhitespace, row, col, None));
                    }
                }
                buf.push(c);
            }
        }
    }

    if !buf.is_empty() {
        if buf == "\"" || (buf.starts_with('"') && !buf.ends_with('"')) {
            dbg!("");
            return Err(Error::new(ErrorKind::OddDoubleQuotes, row, col + 1, None));
        }
        values.push(parse_value(&mut buf));
    }

    Ok(values)
}

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
