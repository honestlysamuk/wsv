#[macro_export]
macro_rules! unit {
    () => {
        #[cfg(test)]
        mod tests {
            use super::parse;
            use crate::data_model::Error;
            use crate::data_model::ErrorKind::*;
            use crate::data_model::WsvValue;
            use crate::data_model::WsvValue::Null;

            macro_rules! do_test {
                ($parser:ident, $input:expr, $output:expr) => {
                    match $parser(&$input) {
                        Ok(wsv) => {
                            assert_eq!(wsv, $output);
                        }
                        Err(error) => {
                            panic!("Shouldn't Err. Got {error}")
                        }
                    }
                };
            }

            fn v(inp: &str) -> WsvValue {
                WsvValue::V(inp.to_owned())
            }

            #[test]
            fn no_whitespace_test() {
                const INPUT: &str = r##"mmm"AAA"mmm"##;
                match parse(INPUT) {
                    Err(Error{kind: MissingWhitespace, row: 1, col: 4, ..}) => {
                        println!("Successful");
                    },
                    Err(e) => panic!("Wrong error. Expected\nMissingWhitespace, 1, 4\nGot\n{e:?}"),                    Ok(v) => panic!("Parsed Malformed input: {v:?}"),
                }
            }

            #[test]
            fn no_leading_whitespace_test() {
                const INPUT: &str = r##"mmm"mmm" mmm"##;
                match parse(INPUT) {
                    // Err(Error{row: 1, col: 4, ..}) => {
                    //     eprintln!("Successful");
                    // },
                    Err(Error{kind: MissingWhitespace, row: 1, col: 4, ..}) => {
                        println!("Successful");
                    },
                    Err(e) => panic!("Wrong error. Expected\nMissingWhitespace, 1, 4\nGot\n{e:?}"),
                    Ok(v) => panic!("Parsed Malformed input: {v:?}"),
                }
            }

            #[test]
            fn no_trailing_whitespace_test() {
                const INPUT: &str = r##"mmm "mmm"mmm"##;
                match parse(INPUT) {
                    Err(Error{kind: MissingWhitespace, row: 1, col: 10, ..}) => {
                        println!("Successful");
                    },
                    Err(e) => panic!("Wrong error. Expected\nMissingWhitespace, 1, 10\nGot\n{e:?}"),
                    Ok(v) => panic!("Parsed Malformed input: {v:?}"),
                }
            }

            #[test]
            fn no_trailing_whitespace_and_odd_quotes_test() {
                const INPUT: &str = r##"mmm "mmm"mmm""##;
                match parse(INPUT) {
                    Err(Error{kind: MissingWhitespace, row: 1, col: 10, ..}) => {
                        println!("Successful");
                    },
                    Err(Error{kind: OddDoubleQuotes, row: 1, col: 14, ..}) => {
                        println!("Successful");
                    },
                    Err(e) => panic!("Wrong error. Expected\nMissingWhitespace, 1, 10\nOr\nOddDoubleQuotes, 1, 14\nGot\n{e:?}"),
                    Ok(v) => panic!("Parsed Malformed input: {v:?}"),
                }
            }

            #[test]
            fn one_quote_test() {
                const INPUT: &str = r##"""##;
                match parse(INPUT) {
                    Err(Error{kind: OddDoubleQuotes, row: 1, col: 2, ..}) => {
                        println!("Successful");
                    },
                    Err(e) => panic!("Wrong error. Expected\nOddDoubleQuotes, 1, 2\nGot\n{e:?}"),
                    Ok(v) => panic!("Parsed Odd Double Quotes: {v:?}"),
                }
            }

            #[test]
            fn odd_quotes_test() {
                const INPUT: &str = r##"somthing " somethingelse"##;
                match parse(INPUT) {
                    Err(Error{kind: OddDoubleQuotes, row: 1, col: 25, ..}) => {
                        println!("Successful");
                    },
                    Err(e) => panic!("Wrong error. Expected\nOddDoubleQuotes, 1, 25\nGot\n{e:?}"),
                    Ok(v) => panic!("Parsed Odd Double Quotes: {v:?}"),
                }
            }

            #[test]
            fn no_leading_whitespace_and_odd_quotes_test() {
                const INPUT: &str = r##"somthing" somethingelse"##;
                match parse(INPUT) {
                    Err(Error{kind: MissingWhitespace, row: 1, col: 9, ..}) => {
                        println!("Successful");
                    },
                    Err(Error{kind: OddDoubleQuotes, row: 1, col: 24, ..}) => {
                        println!("Successful");
                    },
                    Err(e) => panic!("Wrong error. Expected\nMissingWhitespace, 1, 9\nGot\n{e:?}"),
                    Ok(v) => panic!("Parsed Odd Double Quotes: {v:?}"),
                }
            }

            #[test]
            fn comments_test() {
                const INPUT: &str = "# This is a comment\nCommentExAmple # and this\nbut \" # \" is fine";
                let output = vec![
                    vec![],
                    vec![v("CommentExAmple")],
                    vec![v("but"), v(" # "), v("is"), v("fine")],
                ];
                do_test!(parse, INPUT, output);
            }

            #[test]
            fn not_null_test() {
                const INPUT: &str = r#""-""#;
                let output = vec![vec![v("-")]];
                do_test!(parse, INPUT, output);
            }

            #[test]
            fn single_slash_test() {
                const INPUT: &str = r#""/""#;
                let output = vec![vec![v("/")]];
                do_test!(parse, INPUT, output);
            }

            #[test]
            fn empty_test() {
                const INPUT: &str = "";
                let output = vec![vec![]];
                do_test!(parse, INPUT, output);
            }

            #[test]
            fn trailing_return_test() {
                const INPUT: &str = "5\n";
                let output = vec![vec![v("5")], vec![]];
                do_test!(parse, INPUT, output);
            }

            #[test]
            fn null_test() {
                const INPUT: &str = "-";
                let output = vec![vec![Null]];
                do_test!(parse, INPUT, output);
            }

            #[test]
            fn numbers_test() {
                const INPUT: &str = "1 2.0 3.4.5 6.789";
                let output = vec![vec![v("1"), v("2.0"), v("3.4.5"), v("6.789")]];
                do_test!(parse, INPUT, output);
            }

            #[test]
            fn empty_string_test() {
                const INPUT: &str = r#"""
            "1"
            "2" "3"
            "4" ""
            "" "5"
            "" ""
            "6" ""  7
                8  "" "9"
            "a" "" "b""#;
                let output = vec![
                    vec![v("")],
                    vec![v("1")],
                    vec![v("2"), v("3")],
                    vec![v("4"), v("")],
                    vec![v(""), v("5")],
                    vec![v(""), v("")],
                    vec![v("6"), v(""), v("7")],
                    vec![v("8"), v(""), v("9")],
                    vec![v("a"), v(""), v("b")],
                ];
                do_test!(parse, INPUT, output);
            }

            #[test]
            fn strings_test() {
                const INPUT: &str = r##"hello it's "me" "" 
                "I'was" "wondering#if" "after/all" 
                "these""years" "you'd"/"like" 
                ""/"" """" "#" "/"      "##;
                let output = vec![
                    vec![v("hello"), v("it's"), v("me"), v("")],
                    vec![v("I'was"), v("wondering#if"), v("after/all")],
                    vec![v("these\"years"), v("you'd\nlike")],
                    vec![v("\n"), v("\""), v("#"), v("/")],
                ];
                do_test!(parse, INPUT, output);
            }
        }
    };
}
