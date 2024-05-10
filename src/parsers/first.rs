use crate::data_model::*;
use tracing::debug;

pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

fn parse_line((line_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let row = line_index + 1;
    let mut values: Vec<WsvValue> = Vec::new();
    let mut buf: String = String::new();
    let mut open_quotes: bool = false;
    let mut col = 0;
    for c in line.chars() {
        col += 1;
        match c {
            '"' => {
                if buf.is_empty() || buf.starts_with('\"') {
                    open_quotes = !open_quotes;
                    buf.push(c);
                } else {
                    return Err(dbg!(Error::new(
                        ErrorKind::NoLeadingWhitespace,
                        row,
                        col,
                        None
                    )));
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
                    values.push(parse_value(&mut buf, row, col)?);
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
        values.push(parse_value(&mut buf, row, col)?);
    }

    if open_quotes {
        Err(dbg!(Error::new(ErrorKind::OddDoubleQuotes, row, col, None)))
    } else {
        Ok(values)
    }
}

fn parse_value(buf: &mut str, row: usize, col: usize) -> Result<WsvValue, Error> {
    if buf == "-" {
        Ok(WsvValue::Null)
    } else if buf == "\"" {
        Err(dbg!(Error::new(ErrorKind::OddDoubleQuotes, row, col, None)))
    } else if buf.starts_with('"') && buf.ends_with('"') {
        Ok(WsvValue::V(
            buf[1..buf.len() - 1]
                .replace("\"/\"", "\n")
                .replace("\"\"", "\""),
        ))
    } else if buf.starts_with('"') && !buf.ends_with('"') {
        debug!(error = "No trailing whitespace", buf);
        Err(dbg!(Error::new(
            ErrorKind::NoTrailingWhitespace,
            row,
            col,
            None
        )))
    } else {
        Ok(WsvValue::V(buf.into()))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::parse;
    use crate::unit_tests::*;

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
    fn no_whitespace() {
        no_whitespace_test(&parse)
    }
    #[test]
    fn odd_quotes() {
        odd_quotes_test(&parse)
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
}
