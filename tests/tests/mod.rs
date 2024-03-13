use std::fs::read_to_string;
use wsv::Wsv;
use wsv::WsvError;
use wsv::WsvValue as w;

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
    let output = vec![vec![], vec![w::Value("CommentExample".to_owned())]];

    match parse(&input) {
        Ok(wsv) => {
            assert_eq!(wsv, output);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn not_null_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/not_null.wsv").unwrap();
    let output = vec![vec![w::Value("-".to_owned())]];

    match parse(&input) {
        Ok(wsv) => {
            assert_eq!(wsv, output);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn empty_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/empty.wsv").unwrap();
    let output: Vec<Vec<w>> = vec![vec![]];

    match parse(&input) {
        Ok(wsv) => {
            assert_eq!(wsv, output);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn null_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/nulls.wsv").unwrap();
    let output = vec![
        vec![
            w::Value("nullExample".to_owned()),
            w::Null,
            w::Value("2".to_owned()),
        ],
        vec![w::Null],
        vec![w::Null, w::Value("2".to_owned())],
        vec![w::Value("3".to_owned()), w::Null],
    ];

    match parse(&input) {
        Ok(wsv) => {
            assert_eq!(wsv, output);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn numbers_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/numbers.wsv").unwrap();
    let output = vec![vec![
        w::Value("1".to_owned()),
        w::Value("2.0".to_owned()),
        w::Value("3.4.5".to_owned()),
        w::Value("6.789".to_owned()),
    ]];

    match parse(&input) {
        Ok(wsv) => {
            assert_eq!(wsv, output);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn strings_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/strings.wsv").unwrap();
    let output = vec![
        vec![w::Value("hello".to_owned())],
        vec![w::Value("it's".to_owned()), w::Value("me".to_owned())],
        vec![w::Value(
            "I was wondering if \" after all these ".to_owned(),
        )],
        vec![w::Value("years you'd like\nto meet".to_owned())],
    ];

    match parse(&input) {
        Ok(wsv) => {
            assert_eq!(wsv, output);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn parse_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input = read_to_string("./tests/example_files/parse_example.wsv").unwrap();
    let output = vec![
        vec![],
        vec![
            w::Value("1".to_owned()),
            w::Value("hello".to_owned()),
            w::Value("world".to_owned()),
            w::Value("\n".to_owned()),
            w::Value("\"".to_owned()),
            w::Value("".to_owned()),
            w::Null,
        ],
        vec![
            w::Value("string".to_owned()),
            w::Null,
            w::Value("null".to_owned()),
        ],
        vec![],
        vec![],
        vec![w::Value("val".to_owned())],
        vec![w::Value("val".to_owned())],
        vec![],
    ];

    match parse(&input) {
        Ok(wsv) => {
            assert_eq!(wsv, output);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}
