use unicode_bom::Bom;

pub use self::data_model::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn parse<P>(path: P) -> Result<Vec<WsvLine>, WsvError>
where
    P: AsRef<Path>,
{
    let bom = Bom::from(&mut File::open(&path)?);
    match bom {
        Bom::Null => parse_lossy(File::open(&path)?).into_iter().collect(),
        _ => Err(WsvError::BomPresent(bom)),
    }
}
pub fn parse_lossy(file: File) -> Vec<Result<WsvLine, WsvError>> {
    BufReader::new(file)
        .lines()
        .map(|line| match line {
            Ok(str) => parse_line(str),
            Err(e) => Err(WsvError::from(e)),
        })
        .collect()
}

fn parse_line(line: String) -> Result<WsvLine, WsvError> {
    let mut values: Vec<WsvValue> = Vec::new();
    let mut buf: String = String::new();
    let mut open_quotes: bool = false;
    dbg!(&line);

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
        Err(WsvError::DoubleQuotesMismatch(1))
    } else {
        Ok(values)
    }
}

mod data_model {
    use core::fmt;
    use std::io;

    use unicode_bom::Bom;

    pub type WsvLine = Vec<WsvValue>;

    #[derive(Default, Debug, PartialEq)]
    pub enum WsvValue {
        Value(String),
        #[default]
        Null,
        Comment(String),
    }
    impl fmt::Display for WsvValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }
    impl From<&mut String> for WsvValue {
        fn from(string: &mut String) -> WsvValue {
            if string == "-" {
                WsvValue::Null
            } else if string.starts_with('"') && string.ends_with('"') {
                WsvValue::Value(
                    string[1..string.len() - 1]
                        .replace("\"/\"", "\n")
                        .replace("\"\"", "\""),
                )
            } else {
                WsvValue::Value(string.clone())
            }
        }
    }

    #[derive(Debug)]
    pub enum WsvError {
        BomPresent(Bom),
        DoubleQuotesMismatch(i32),
        Other(io::Error),
    }
    impl fmt::Display for WsvError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }
    impl From<io::Error> for WsvError {
        fn from(cause: io::Error) -> WsvError {
            WsvError::Other(cause)
        }
    }
}
