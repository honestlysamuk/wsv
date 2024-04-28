pub(crate) use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::complete::char,
    combinator::{all_consuming, map, value as ifthen, verify},
    error::Error as nomError,
    multi::{many0, separated_list0},
    sequence::delimited,
    Err as nomErr, IResult,
};

use crate::data_model::*;

fn ws0(i: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace() && c != '\n')(i)
}

fn ws1(i: &str) -> IResult<&str, &str> {
    verify(
        take_while(|c: char| c.is_whitespace() && c != '\n'),
        |p: &str| !p.is_empty(),
    )(i)
}

fn new_line(i: &str) -> IResult<&str, &str> {
    ifthen("\n", tag("\"/\""))(i)
}

fn double_quote(i: &str) -> IResult<&str, &str> {
    ifthen("\"", tag("\"\""))(i)
}

fn string_part(i: &str) -> IResult<&str, &str> {
    verify(take_till(|c| c == '\n' || c == '\"'), |p: &str| {
        !p.is_empty()
    })(i)
}

fn null(i: &str) -> IResult<&str, WsvValue> {
    ifthen(WsvValue::Null, tag("-"))(i)
}

fn value(i: &str) -> IResult<&str, WsvValue> {
    map(
        verify(
            take_till(|c: char| c == '\"' || c == '#' || c.is_whitespace()),
            |s: &str| !s.is_empty(),
        ),
        |s| WsvValue::Value(String::from(s)),
    )(i)
}

#[tracing::instrument]
fn string(i: &str) -> IResult<&str, WsvValue> {
    map(
        delimited(
            char('\"'),
            many0(alt((string_part, double_quote, new_line))),
            char('\"'),
        ),
        |s| WsvValue::Value(s.iter().fold(String::new(), |acc, s| acc + s)),
    )(i)
}

fn comment(i: &str) -> IResult<&str, &str> {
    let (i, _) = tag("#")(i)?;
    take_till(|c| c == '\n')(i)
}
#[tracing::instrument]
fn line(i: &str) -> IResult<&str, Vec<WsvValue>> {
    let (i, _) = ws0(i)?;
    let (i, o) = separated_list0(ws1, alt((null, string, value)))(i)?;
    let (i, _) = ws0(i)?;
    let (i, _) = match comment(i) {
        Ok((i, o)) => (i, o),
        Err(_) => (i, ""),
    };
    Ok((i, o))
}

fn wsv(i: &str) -> IResult<&str, Vec<Vec<WsvValue>>> {
    separated_list0(char('\n'), line)(i)
}

pub fn parse(i: &str) -> Result<Wsv, WsvError> {
    match all_consuming(wsv)(i) {
        Ok((_, o)) => Ok(Wsv::from(o)),
        Err(e) => Err(WsvError::from(e)),
    }
}

impl From<nomErr<nomError<&str>>> for WsvError {
    fn from(value: nomErr<nomError<&str>>) -> Self {
        WsvError::Other(value.to_string())
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(value(VALUE), Ok(("", WsvValue::Value("1".to_owned()))));
    }

    const NULL: &str = r#"-"#;

    #[test]
    fn null_parses() {
        assert_eq!(null(NULL), Ok(("", WsvValue::Null)));
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
            Ok(("", WsvValue::Value("hello,world!".to_owned())))
        );
    }

    const STRING_WITH_SPACES: &str = r#""hello, world!""#;

    #[test]
    fn string_with_spaces_parses() {
        assert_eq!(
            string(STRING_WITH_SPACES),
            Ok(("", WsvValue::Value("hello, world!".to_owned())))
        );
    }

    const STRING_WITH_QUOTES: &str = r#""hello,""world!""#;

    #[test]
    fn string_with_quotes() {
        assert_eq!(
            string(STRING_WITH_QUOTES),
            Ok(("", WsvValue::Value("hello,\"world!".to_owned())))
        );
    }

    const STRING_WITH_NEWLINE: &str = r#""hello,"/"world!""#;

    #[test]
    fn string_with_newline() {
        assert_eq!(
            string(STRING_WITH_NEWLINE),
            Ok(("", WsvValue::Value("hello,\nworld!".to_owned())))
        );
    }

    const STRING_WITH_HASH: &str = r#""hello,#world!""#;

    #[test]
    fn string_with_hash() {
        assert_eq!(
            string(STRING_WITH_HASH),
            Ok(("", WsvValue::Value("hello,#world!".to_owned())))
        );
    }

    const EMPTY_STRING: &str = r#""""#;

    #[test]
    fn empty_string() {
        assert_eq!(
            string(EMPTY_STRING),
            Ok(("", WsvValue::Value("".to_owned())))
        );
    }

    const EASY_LINE: &str = r##"1 2"##;

    #[test]
    fn easy_line() {
        assert_eq!(
            line(EASY_LINE),
            Ok((
                "",
                vec![
                    WsvValue::Value("1".to_owned()),
                    WsvValue::Value("2".to_owned()),
                ]
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
                    WsvValue::Value("1".to_owned()),
                    WsvValue::Value("hello".to_owned()),
                    WsvValue::Value("world".to_owned()),
                    WsvValue::Value("\n".to_owned()),
                    WsvValue::Value("\"".to_owned()),
                    WsvValue::Value("".to_owned()),
                    WsvValue::Null,
                ]
            ))
        );
    }

    const WSV1: &str = r##"    1 hello "world" ""/"" """" "" -
"string" - null # other comment #comment
# comment
val#commentt
val# comment
"##;

    #[test]
    fn wsv1_test() {
        assert_eq!(
            wsv(WSV1),
            Ok((
                "",
                vec![
                    vec![
                        WsvValue::Value("1".to_owned()),
                        WsvValue::Value("hello".to_owned()),
                        WsvValue::Value("world".to_owned()),
                        WsvValue::Value("\n".to_owned()),
                        WsvValue::Value("\"".to_owned()),
                        WsvValue::Value("".to_owned()),
                        WsvValue::Null,
                    ],
                    vec![
                        WsvValue::Value("string".to_owned()),
                        WsvValue::Null,
                        WsvValue::Value("null".to_owned()),
                    ],
                    vec![],
                    vec![WsvValue::Value("val".to_owned())],
                    vec![WsvValue::Value("val".to_owned())],
                    vec![],
                ]
            ))
        );
    }

    const WSV2: &str = r##"
1 hello "world" ""/"" """" "" -   
#
"string" - null "#" # other comment #comment#
# comment
val#commentt
val# comment
"##;

    #[test]
    fn wsv2_test() {
        assert_eq!(
            wsv(WSV2),
            Ok((
                "",
                vec![
                    vec![],
                    vec![
                        WsvValue::Value("1".to_owned()),
                        WsvValue::Value("hello".to_owned()),
                        WsvValue::Value("world".to_owned()),
                        WsvValue::Value("\n".to_owned()),
                        WsvValue::Value("\"".to_owned()),
                        WsvValue::Value("".to_owned()),
                        WsvValue::Null,
                    ],
                    vec![],
                    vec![
                        WsvValue::Value("string".to_owned()),
                        WsvValue::Null,
                        WsvValue::Value("null".to_owned()),
                        WsvValue::Value("#".to_owned()),
                    ],
                    vec![],
                    vec![WsvValue::Value("val".to_owned())],
                    vec![WsvValue::Value("val".to_owned())],
                    vec![],
                ]
            ))
        );
    }
}
