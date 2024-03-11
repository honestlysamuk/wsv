use std::fs::read_to_string;
use wsv::Wsv;
use wsv::WsvError;
use wsv::WsvValue as w;

pub fn null_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/nulls.wsv").unwrap();
    match parse(&contents) {
        Ok(wsv) => {
            assert_eq!(
                wsv,
                Into::<Wsv>::into(vec![
                    vec![
                        w::Value("nullExample".to_owned()),
                        w::Null,
                        w::Value("2".to_owned())
                    ],
                    vec![w::Null],
                    vec![w::Null, w::Value("2".to_owned())],
                    vec![w::Value("3".to_owned()), w::Null]
                ])
            );
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn numbers_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/numbers.wsv").unwrap();
    match parse(&contents) {
        Ok(wsv) => {
            let outcome = vec![vec![
                w::Value("1".to_owned()),
                w::Value("2.0".to_owned()),
                w::Value("3.4.5".to_owned()),
                w::Value("6.789".to_owned()),
            ]];
            assert_eq!(wsv, outcome);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn strings_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/strings.wsv").unwrap();
    match parse(&contents) {
        Ok(wsv) => {
            assert_eq!(
                wsv,
                vec![
                    vec![w::Value("hello".to_owned())],
                    vec![w::Value("it's".to_owned()), w::Value("me".to_owned())],
                    vec![w::Value(
                        "I was wondering if \" after all these ".to_owned()
                    )],
                    vec![w::Value("years you'd like\nto meet".to_owned())]
                ]
            );
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn comments_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/comments.wsv").unwrap();
    match parse(&contents) {
        Ok(wsv) => {
            assert_eq!(
                wsv,
                vec![vec![], vec![w::Value("CommentExample".to_owned())]]
            );
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn not_null_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/not_null.wsv").unwrap();
    let wsv = parse(&contents).unwrap();
    println!("{:?}", wsv);
    assert_eq!(wsv, vec![vec![w::Value("-".to_string())]]);
}

pub fn empty_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/empty.wsv").unwrap();
    match parse(&contents) {
        Ok(wsv) => {
            let empty_vec: Vec<Vec<w>> = vec![vec![]];
            assert_eq!(wsv, empty_vec)
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

pub fn malformed_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/malformed.wsv").unwrap();
    match parse(&contents) {
        Ok(v) => panic!("Parsed Malformed input: {v:?}"),
        Err(e) => {
            println!("{e:?}")
        }
    }
}

pub fn odd_quotes_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let contents = read_to_string("./tests/example_files/odd_quotes.wsv").unwrap();
    match parse(&contents) {
        Ok(v) => panic!("Parsed Odd Double Quotes: {v:?}"),
        Err(e) => {
            println!("{e:?}")
        }
    }
}

pub fn parse_test(parse: &dyn Fn(&str) -> Result<Wsv, WsvError>) {
    let input: &str = r##"
1 hello "world" ""/"" """" "" -
  "string" - null # other comment #comment
# comment

val#commentt
val# comment
"##;
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
    match parse(input) {
        Ok(wsv) => {
            assert_eq!(wsv, output);
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}
