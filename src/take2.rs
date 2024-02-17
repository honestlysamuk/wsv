use self::data_model::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// This function takes only a path to the location of the data and reads the file into a
// single string for processing in one go. There is no error handling, only panics.
pub fn parse<P>(path: P) -> Result<WSV, std::io::Error>
where
    P: AsRef<Path>,
{
    let contents = BufReader::new(File::open(&path)?)
        .lines()
        .filter_map(|line| parse_line(line.ok()?))
        .collect::<WSV>();

    match contents[0].get(0) {
        None => {
            panic!("File is empty.")
        }
        Some(x) => match x {
            Some(first_value) => {
                let byte_order_mark: &[u8; 3] = &[239, 187, 191];
                if first_value.as_bytes().starts_with(byte_order_mark) {
                    panic!("This UTF-8 file is encoded with a byte-order mark.")
                }
                /* UTF-8 encoded value without the byte-order mark */
            }
            None => { /* First line starts with null value */ }
        },
    }

    Ok(contents)
}

fn parse_line(line: String) -> Option<Vec<Option<String>>> {
    let mut values: Vec<Option<String>> = Vec::new();
    let mut value: WsvValue = Default::default();
    let mut open_quotes: bool = false;
    let mut last = None;

    for c in line.chars() {
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
                    values.push(value.sanitise());
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
                    last = Some(c);
                }
            }
            _ => value.push(c),
        }
    }

    if open_quotes {
        panic!("odd number of double quotes");
    }

    if !value.is_empty() {
        values.push(value.sanitise());
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

mod data_model {
    pub type WSV = Vec<Vec<Option<String>>>;

    #[derive(Default)]
    pub struct WsvValue(String);

    impl WsvValue {
        pub fn sanitise(&mut self) -> Option<String> {
            let mut value = self.0.clone();
            self.0.clear();

            if value.starts_with('"') && value.ends_with('"') {
                value.pop();
                value.remove(0);
            }
            value = value.replace("\"/\"", "\n").replace("\"\"", "\"");
            Some(value)
        }
        pub fn push(&mut self, ch: char) -> () {
            self.0.push(ch);
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }
}
