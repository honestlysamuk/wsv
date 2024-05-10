pub use crate::data_model::Error;
pub use crate::data_model::WsvValue;
pub use crate::data_model::WsvValue::Null;


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

pub fn v(inp: &str) -> WsvValue {
    WsvValue::V(inp.to_owned())
}

pub fn no_whitespace_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    //let contents = read_to_string("./tests/example_files/malformed.wsv").unwrap();
    const INPUT: &str = r##"mmm"mmm"mmm"##;
    match parse(INPUT) {
        Err(_) => println!("Successful"),
        Ok(v) => panic!("Parsed Malformed input: {v:?}"),
    }
}

pub fn no_leading_whitespace_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = r##"mmm"mmm" mmm"##;
    match parse(INPUT) {
        Err(_) => println!("Successful"),
        Ok(v) => panic!("Parsed Malformed input: {v:?}"),
    }
}

pub fn no_trailing_whitespace_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = r##"mmm "mmm"mmm"##;
    match parse(INPUT) {
        Err(_) => println!("Successful"),
        Ok(v) => panic!("Parsed Malformed input: {v:?}"),
    }
}

pub fn odd_quotes_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = r##"""##;
    match parse(INPUT) {
        Err(_) => println!("successful"),
        Ok(v) => panic!("Parsed Odd Double Quotes: {v:?}"),
    }
}

pub fn comments_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = "# This is a comment\nCommentExample # and this\nbut \" # \" is fine";
    let output = vec![
        vec![],
        vec![v("CommentExample")],
        vec![v("but"), v(" # "), v("is"), v("fine")],
    ];
    do_test!(parse, INPUT, output);
}

pub fn not_null_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = r#""-""#;
    let output = vec![vec![v("-")]];
    do_test!(parse, INPUT, output);
}

pub fn single_slash_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = r#""/""#;
    let output = vec![vec![v("/")]];
    do_test!(parse, INPUT, output);
}

pub fn empty_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = "";
    let output = vec![vec![]];
    do_test!(parse, INPUT, output);
}

pub fn trailing_return_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = "5\n";
    let output = vec![vec![v("5")], vec![]];
    do_test!(parse, INPUT, output);
}

pub fn empty_string_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
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

pub fn null_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = "-";
    let output = vec![vec![Null]];
    do_test!(parse, INPUT, output);
}

pub fn numbers_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
    const INPUT: &str = "1 2.0 3.4.5 6.789";
    let output = vec![vec![v("1"), v("2.0"), v("3.4.5"), v("6.789")]];
    do_test!(parse, INPUT, output);
}

pub fn strings_test(parse: &dyn Fn(&str) -> Result<Vec<Vec<WsvValue>>, Error>) {
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