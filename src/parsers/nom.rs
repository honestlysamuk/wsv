pub(crate) use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::complete::char,
    combinator::{all_consuming, map, value as ifthen, verify},
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult,
};

use crate::data_model::*;

// pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
//     match all_consuming(wsv)(i) {
//         Ok((_, o)) => Ok(o),
//         Err(e) => Err(Error::new(ErrorKind::Nom, 0, 0, Some(e.to_string().into()))),
//     }
// }

// fn wsv(i: &str) -> IResult<&str, Vec<Vec<WsvValue>>> {
//     separated_list0(char('\n'), line)(i)
// }

pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

// we assume that line has no `\n`.
pub fn parse_line((row_index, input): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let row = row_index + 1;
    match all_consuming(line)(input) {
        Ok((_, o)) => Ok(o),
        Err(e) => Err(Error::new(
            ErrorKind::Nom,
            row,
            0,
            Some(e.to_string().into()),
        )),
    }
}

#[tracing::instrument]
fn line(i: &str) -> IResult<&str, Vec<WsvValue>> {
    let (i, _) = ws0(i)?;
    let (i, o) = separated_list0(ws1, alt((nul, string, value)))(i)?;
    let (i, _) = ws0(i)?;
    let (i, _) = match comment(i) {
        Ok((i, o)) => (i, o),
        Err(_) => (i, ""),
    };
    Ok((i, o))
}

fn ws0(i: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace() && c != '\n')(i)
}

fn ws1(i: &str) -> IResult<&str, &str> {
    verify(
        take_while(|c: char| c.is_whitespace() && c != '\n'),
        |p: &str| !p.is_empty(),
    )(i)
}

fn nul(i: &str) -> IResult<&str, WsvValue> {
    ifthen(WsvValue::Null, tag("-"))(i)
}

#[tracing::instrument]
fn string(i: &str) -> IResult<&str, WsvValue> {
    map(
        delimited(
            char('\"'),
            many0(alt((string_part, double_quote, new_line))),
            char('\"'),
        ),
        |s| WsvValue::V(s.iter().fold(String::new(), |acc, s| acc + s)),
    )(i)
}

fn string_part(i: &str) -> IResult<&str, &str> {
    verify(take_till(|c| c == '\n' || c == '\"'), |p: &str| {
        !p.is_empty()
    })(i)
}

fn double_quote(i: &str) -> IResult<&str, &str> {
    ifthen("\"", tag("\"\""))(i)
}

fn new_line(i: &str) -> IResult<&str, &str> {
    ifthen("\n", tag("\"/\""))(i)
}

fn value(i: &str) -> IResult<&str, WsvValue> {
    map(
        verify(
            take_till(|c: char| c == '\"' || c == '#' || c.is_whitespace()),
            |s: &str| !s.is_empty(),
        ),
        |s| WsvValue::V(String::from(s)),
    )(i)
}

fn comment(i: &str) -> IResult<&str, &str> {
    let (i, _) = tag("#")(i)?;
    take_till(|c| c == '\n')(i)
}

#[cfg(test)]
mod nom_tests {
    use super::*;

    fn ws(i: &str) -> IResult<&str, &str> {
        take_while(|c: char| c.is_whitespace() && c != '\n')(i)
    }

    #[test]
    fn whitespace_parses() {
        assert_eq!(ws(" "), Ok(("", " ")));
    }

    #[test]
    fn whitespace_parses_nothing() {
        assert_eq!(ws(""), Ok(("", "")));
    }

    #[test]
    fn whitespace_does_not_parse_newline() {
        assert_eq!(ws("\n"), Ok(("\n", "")));
    }

    const NEW_LINE: &str = r##""/""##;

    #[test]
    fn newline_parses() {
        assert_eq!(new_line(NEW_LINE), Ok(("", "\n")));
    }

    const QUOTE: &str = r##""""##;

    #[test]
    fn quote_parses() {
        assert_eq!(double_quote(QUOTE), Ok(("", "\"")));
    }

    const VALUE: &str = "1";

    #[test]
    fn value_parses() {
        assert_eq!(value(VALUE), Ok(("", WsvValue::V("1".to_owned()))));
    }

    const NULL: &str = r#"-"#;

    #[test]
    fn null_parses() {
        assert_eq!(nul(NULL), Ok(("", WsvValue::Null)));
    }

    const COMMENT: &str = "# This is a comment";

    #[test]
    fn comment_parses() {
        assert_eq!(comment(COMMENT), Ok(("", " This is a comment")));
    }

    const STRING: &str = r##""hello,world!""##;

    #[test]
    fn string_parses() {
        assert_eq!(
            string(STRING),
            Ok(("", WsvValue::V("hello,world!".to_owned())))
        );
    }

    const STRING_WITH_SPACES: &str = r#""hello, world!""#;

    #[test]
    fn string_with_spaces_parses() {
        assert_eq!(
            string(STRING_WITH_SPACES),
            Ok(("", WsvValue::V("hello, world!".to_owned())))
        );
    }

    const STRING_WITH_QUOTES: &str = r#""hello,""world!""#;

    #[test]
    fn string_with_quotes() {
        assert_eq!(
            string(STRING_WITH_QUOTES),
            Ok(("", WsvValue::V("hello,\"world!".to_owned())))
        );
    }

    const STRING_WITH_NEWLINE: &str = r#""hello,"/"world!""#;

    #[test]
    fn string_with_newline() {
        assert_eq!(
            string(STRING_WITH_NEWLINE),
            Ok(("", WsvValue::V("hello,\nworld!".to_owned())))
        );
    }

    const STRING_WITH_HASH: &str = r#""hello,#world!""#;

    #[test]
    fn string_with_hash() {
        assert_eq!(
            string(STRING_WITH_HASH),
            Ok(("", WsvValue::V("hello,#world!".to_owned())))
        );
    }

    const EMPTY_STRING: &str = r#""""#;

    #[test]
    fn empty_str() {
        assert_eq!(string(EMPTY_STRING), Ok(("", WsvValue::V("".to_owned()))));
    }

    const EASY_LINE: &str = r##"1 2"##;

    #[test]
    fn easy_line() {
        assert_eq!(
            line(EASY_LINE),
            Ok((
                "",
                vec![WsvValue::V("1".to_owned()), WsvValue::V("2".to_owned()),]
            ))
        );
    }

    #[test]
    fn empty_line() {
        assert_eq!(line(""), Ok(("", vec![])));
    }

    const HARD_LINE: &str = r##"1 hello "world" ""/"" """" "" -"##;

    #[test]
    fn hard_line() {
        assert_eq!(
            line(HARD_LINE),
            Ok((
                "",
                vec![
                    WsvValue::V("1".to_owned()),
                    WsvValue::V("hello".to_owned()),
                    WsvValue::V("world".to_owned()),
                    WsvValue::V("\n".to_owned()),
                    WsvValue::V("\"".to_owned()),
                    WsvValue::V("".to_owned()),
                    WsvValue::Null,
                ]
            ))
        );
    }
}

#[cfg(test)]
use crate::unit;
#[cfg(test)]
unit! {}
