pub use crate::data_model::*;

pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, WsvError> {
    if i.is_empty() {
        Ok(vec![vec![]])
    } else {
        i.lines()
            .enumerate()
            .map(parse_line)
            .collect::<Result<Vec<Vec<WsvValue>>, WsvError>>()
    }
}

fn parse_line((line_number, line): (usize, &str)) -> Result<Vec<WsvValue>, WsvError> {
    let mut values: Vec<WsvValue> = Vec::new();
    let mut buf: String = String::new();
    let mut open_quotes: bool = false;

    for c in line.chars() {
        match c {
            '"' => {
                open_quotes = !open_quotes;
                buf.push(c);
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
                    values.push(WsvValue::from(&mut buf));
                    buf.clear();
                }
                // ignore otherwise
            }
            _ => buf.push(c),
        }
    }

    if !buf.is_empty() {
        values.push(WsvValue::from(&mut buf));
    }

    if open_quotes {
        Err(WsvError::DoubleQuotesMismatch(line_number))
    } else {
        Ok(values)
    }
}
