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
                if buf.is_empty() || buf.starts_with('\"') {
                    open_quotes = !open_quotes;
                    buf.push(c);
                } else {
                    return Err(WsvError::MalformedInput(line_number));
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

/*

fn parse_line2((line_number, line): (usize, &str)) -> Result<Vec<WsvValue>, WsvError> {
    let it = line.split('\"');
    if it.count() % 2 == 0 {
        Err(WsvError::DoubleQuotesMismatch(line_number))
    } else {
        it.
        todo!()
    }
}

split on hash. If two elements, Run parser on the first element. else run parser on the whole input

if three elements, run parser on the first input, else on first two, else on the whole input

push " " onto top and bottom of the string
split on "\""
first and last elements must be of the form

  aabb aabb - aabb    a

 split whitespace,  and add all values to the line

  second part must be part of the string. add all to buffer
  third part could be same as first and last, or could also be just / or empty

  third = / then push "\n" to the buf. = empty then push "\"" to the buf, else check for a hash, etc.

  fourth


  mmmm"mmmm"mmmm


Cannot split on whitespace
*/
