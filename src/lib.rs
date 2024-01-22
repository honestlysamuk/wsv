use std::{fs, path::Path};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nulls() {
        let wsv = parse_wsv("tests/nulls.wsv");
        println!("{:?}", wsv);
    }
    #[test]
    fn comments() {
        let wsv = parse_wsv("tests/comments.wsv");
        println!("{:?}", wsv);
    }
    #[test]
    fn numbers() {
        let wsv = parse_wsv("tests/numbers.wsv");
        println!("{:?}", wsv);
    }
    #[test]
    fn strings() {
        let wsv = parse_wsv("tests/strings.wsv");
        println!("{:?}", wsv);
    }
    #[test]
    fn not_null() {
        let wsv = parse_wsv("tests/not_null.wsv");
        println!("{:?}", wsv);
    }
    #[test]
    fn everything() {
        let wsv = parse_wsv("tests/everything.wsv");
        println!("{:?}", wsv);
    }
    #[test]
    #[should_panic]
    fn odd_quotes() {
        let wsv = parse_wsv("tests/odd_quotes.wsv");
        println!("{:?}", wsv);
    }
    #[test]
    #[should_panic]
    fn invalid_utf8() {
        let wsv = parse_wsv("tests/invalid_utf8.wsv");
        println!("{:?}", wsv);
    }
}

type WSV = Vec<Vec<Option<String>>>;
fn parse_wsv<P>(path: P) -> WSV
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(path).expect("UTF-8 encoding");

    let mut result: WSV = Vec::new();
    let mut values: Vec<Option<String>> = Vec::new();
    let mut value = String::new();
    let mut open_quotes: bool = false;

    for c in contents.chars() {
        match c {
            '#' => {
                if open_quotes {
                    value.push(c);
                } else {
                    break; // back to the line loop
                }
            }
            '\n' => {
                if open_quotes {
                    value.push(c);
                }
                if !open_quotes {
                    if !value.is_empty() {
                        values.push(Some(value));
                        value = String::new();
                    }
                    result.push(values);
                    values = Vec::new();
                }
            }
            c if c.is_whitespace() => {
                if open_quotes {
                    value.push(c);
                }
                if !open_quotes && !value.is_empty() {
                    values.push(Some(value));
                    value = String::new();
                }
            }
            '"' => {
                open_quotes = !open_quotes;
                value.push(c);
            }
            '-' => {
                if open_quotes {
                    value.push(c);
                } else {
                    values.push(None);
                }
            }
            _ => value.push(c),
        }

        //   values
        //        .into_iter()
        //      .map(|v| v.unwrap().replace("\"/\"", "\n").replace("\"\"", "\""));
    }
    result
}
