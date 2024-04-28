use crate::data_model::*;
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
    let mut values: Vec<WsvValue> = Vec::new();
    let mut buf: String = String::new();
    let mut open_quotes: bool = false;
    for c in line.chars() {
        match c {
            '"' => {
                if buf.is_empty() || buf.starts_with('\"') {
                    open_quotes = !open_quotes;
                    buf.push(c);
                } else {
                    debug!(error = "No leading whitespace", buf);
                    return Err(WsvError::MalformedInput(line_index + 1));
                }
            }
            '#' => {
                if open_quotes {
                    buf.push(c);
                } else {
                    break;
                }
            }
            c if c.is_whitespace() => {
                if open_quotes {
                    buf.push(c);
                } else if !buf.is_empty() {
                    values.push(parse_value(&mut buf, line_index + 1)?);
                    buf.clear();
                }
                // ignore otherwise
            }
            _ => {
                buf.push(c);
            }
        }
    }

    if !buf.is_empty() {
        values.push(parse_value(&mut buf, line_index + 1)?);
    }

    if dbg!(open_quotes) {
        Err(WsvError::DoubleQuotesMismatch(line_index + 1))
    } else {
        Ok(values)
    }
}

fn parse_value(buf: &mut str, line_index: usize) -> Result<WsvValue, WsvError> {
    if buf == "-" {
        Ok(WsvValue::Null)
    } else if buf.starts_with('"') && buf.ends_with('"') {
        Ok(WsvValue::Value(buf[1..buf.len() - 1]
            .replace("\"/\"", "\n")
            .replace("\"\"", "\"")))
    } else if buf.starts_with('"') && !buf.ends_with('"') {
        debug!(error = "No trailing whitespace", buf);
        Err(WsvError::MalformedInput(line_index + 1))
    } else {
        Ok(WsvValue::Value(buf.into()))
    }
}
