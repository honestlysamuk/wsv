use std::{fs, path::Path};

type WSV = Vec<Vec<Option<String>>>;
pub fn parse_wsv<P>(path: P) -> Result<WSV, std::io::Error>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(&path)
        .expect(format!("UTF-8 encoded file at location {:#?}", &path.as_ref()).as_ref());

    println!("{}", contents);

    Ok(contents
        .lines()
        .map(|line| parse_line(line))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<WSV>())
}

fn parse_line(line: &str) -> Option<Vec<Option<String>>> {
    let mut values: Vec<Option<String>> = Vec::new();
    let mut value = String::new();
    let mut open_quotes: bool = false;
    let mut last = None;

    let chars = line.chars();
    for c in chars {
        match c {
            '#' => {
                if open_quotes {
                    value.push(c);
                } else {
                    break;
                }
            }
            c if c.is_whitespace() => {
                if last == Some('-') {
                    values.push(None);
                    last = None;
                }
                if open_quotes {
                    value.push(c);
                }
                if !open_quotes && !value.is_empty() {
                    value = value.replace("\"/\"", "\n").replace("\"\"", "\"");
                    if value.starts_with('"') {
                        value.pop();
                        value.remove(0);
                    }
                    values.push(Some(value));
                    value = String::new();
                }
            }
            '"' => {
                open_quotes = !open_quotes;
                value.push(c);
            }
            '-' => {
                if !open_quotes {
                    last = Some(c);
                } else {
                    value.push(c);
                }
            }
            _ => value.push(c),
        }
    }

    if open_quotes {
        panic!("odd number of double quotes");
    }

    if !value.is_empty() {
        value = value.replace("\"/\"", "\n").replace("\"\"", "\"");
        if value.starts_with('"') {
            value.pop();
            value.remove(0);
        }
        values.push(Some(value));
    }

    if last == Some('-') {
        values.push(None);
    }

    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}
