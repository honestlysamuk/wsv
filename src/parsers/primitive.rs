use crate::data_model::*;

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
                    values.push(parse_value(&mut buf));
                    buf.clear();
                }
                // ignore otherwise
            }
            _ => buf.push(c),
        }
    }

    if !buf.is_empty() {
        values.push(parse_value(&mut buf));
    }

    if open_quotes {
        Err(WsvError::DoubleQuotesMismatch(line_index + 1))
    } else {
        Ok(values)
    }
}

fn parse_value(string: &mut str) -> WsvValue {
    if string == "-" {
        WsvValue::Null
    } else if string.starts_with('"') && string.ends_with('"') {
        WsvValue::Value(
            string[1..string.len() - 1]
                .replace("\"/\"", "\n")
                .replace("\"\"", "\""),
        )
    } else {
        WsvValue::Value(string.into())
    }
                    // let val = string[1..string.len() - 1]
            //     .split("\"")
            //     .map(|c| match c {
            //         "" => "\"",
            //         "/" => "\n",
            //         _ => c,
            //     })
            //     .collect::<String>();
}
