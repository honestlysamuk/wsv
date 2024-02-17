use self::data_model::*;
use std::{fs, path::Path};

// This function takes only a path to the location of the data and reads the file into a
// single string for processing in one go. There is no error handling, only panics.
pub fn parse<P>(path: P) -> Result<WSV, std::io::Error>
where
    P: AsRef<Path>,
{
    let contents = fs::read_to_string(&path)
        .expect(format!("UTF-8 encoded file at location {:#?}", &path.as_ref()).as_ref());

    if contents.is_empty() {
        panic!("File is empty.")
    } else {
        let first_char = contents.chars().take(1).collect::<Vec<char>>()[0];

        if first_char.to_string().as_bytes() == [239, 187, 191] {
            panic!("This UTF8 file is encoded with a byte-order mark.")
        }
        println!("Contents: {}", contents);

        Ok(contents
            .lines()
            .filter_map(|line| parse_line(line))
            .collect::<WSV>())
    }
}

fn parse_line(line: &str) -> Option<Vec<Option<String>>> {
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
