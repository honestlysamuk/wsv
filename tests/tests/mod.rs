use std::fs::read_to_string;
use wsv::Wsv;
use wsv::WsvError;
use wsv::WsvValue;

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
    WsvValue::Value(inp.to_owned())
}

pub fn malformed_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/malformed.wsv").unwrap();
    match parse(&contents) {
        Err(WsvError::MalformedInput(1)) => println!("Successful"),
        Ok(v) => panic!("Parsed Malformed input: {v:?}"),
        Err(e) => panic!("Wrong error message. Got {e}"),
    }
}

pub fn odd_quotes_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/odd_quotes.wsv").unwrap();
    match parse(&contents) {
        Err(WsvError::DoubleQuotesMismatch(2)) => println!("successful"),
        Ok(v) => panic!("Parsed Odd Double Quotes: {v:?}"),
        Err(e) => panic!("Wrong error message. got {e}"),
    }
}

pub fn comments_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/comments.wsv").unwrap();
    let output = vec![
        vec![],
        vec![v("CommentExample")],
        vec![v("but"), v(" # "), v("is"), v("fine")],
    ];
    do_test!(parse, input, output);
}

pub fn not_null_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/not_null.wsv").unwrap();
    let output = vec![vec![v("-")]];
    do_test!(parse, input, output);
}

pub fn single_slash_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/single_slash.wsv").unwrap();
    let output = vec![vec![v("/")]];
    do_test!(parse, input, output);
}

pub fn empty_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/empty.wsv").unwrap();
    let output = vec![vec![]];
    do_test!(parse, input, output);
}

pub fn trailing_return_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/trailing_return.wsv").unwrap();
    let output = vec![vec![v("5")], vec![]];
    do_test!(parse, input, output);
}

pub fn empty_string_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/empty_string.wsv").unwrap();
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
    do_test!(parse, input, output);
}

pub fn null_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/nulls.wsv").unwrap();
    let output = vec![
        vec![v("nullExample"), WsvValue::Null, v("2")],
        vec![WsvValue::Null],
        vec![WsvValue::Null, v("2")],
        vec![v("3"), WsvValue::Null],
    ];
    do_test!(parse, input, output);
}

pub fn numbers_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/numbers.wsv").unwrap();
    let output = vec![vec![v("1"), v("2.0"), v("3.4.5"), v("6.789")]];
    do_test!(parse, input, output);
}

pub fn strings_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/strings.wsv").unwrap();
    let output = vec![
        vec![v("hello")],
        vec![v("it's"), v(""), v("me")],
        vec![v("I was wondering")],
        vec![v(" if \" after all these ")],
        vec![v("years"), v(""), v(" you'd like\nto meet")],
    ];
    do_test!(parse, input, output);
}

pub fn parse_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/parse_example.wsv").unwrap();
    let output = vec![
        vec![],
        vec![
            v("1"),
            v("hello"),
            v("world"),
            v("\n"),
            v("\""),
            v(""),
            WsvValue::Null,
        ],
        vec![v("string"), WsvValue::Null, v("null")],
        vec![],
        vec![],
        vec![v("val")],
        vec![v("val")],
        vec![],
    ];
    do_test!(parse, input, output);
}
