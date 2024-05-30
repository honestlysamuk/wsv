use crate::data_model::Error;
use crate::ErrorKind;
use crate::WsvValue;
use regex::Regex;

fn check() -> Regex {
    Regex::new(
        r##"^(-|"(?:""|"/"|[^\n"]+)*"|[^\s"#]+)?(?:[\s--\n]+(-|"(?:""|"/"|[^\n"]+)*"|[^\s"#]+))*[\s--\n]*(#.*$)?$"##
    ).unwrap()
}

fn data() -> Regex {
    Regex::new(r##"\s*#.*$|^\s*(?<first>-|"(?:""|"/"|[^\n"]+)*"|[^\s"#]+)|\s+(?<rest>-|"(?:""|"/"|[^\n"]+)*"|[^\s"#]+)"##).unwrap()
}

pub fn parse(i: &str) -> Vec<Result<Vec<WsvValue>, Error>> {
    i.split("n").enumerate().map(parse_line).collect()
}

pub fn parse_strict(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split("n").enumerate().map(parse_line).collect()
}

pub fn parse_line((_line_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    dbg!(check().captures(line));

    if check().is_match(line) {
        Ok(data()
            .captures_iter(line)
            .enumerate()
            .map(|(i, c)| dbg!(c).get(1 + i.min(1)).unwrap())
            .map(|m| WsvValue::convert(m.as_str()))
            .collect())
    } else {
        Err(Error::new(ErrorKind::OddDoubleQuotes, 0, 1, None))
    }
}

#[cfg(test)]
use crate::unit;
#[cfg(test)]
unit! {}
