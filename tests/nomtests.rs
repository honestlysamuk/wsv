use std::fs::read_to_string;
use wsv::parse;
use wsv::WsvValue as w;

#[test]
fn nulls() {
    let contents = read_to_string("./tests/example_files/nulls.wsv").unwrap();
    match parse(&contents) {
        Ok(wsv) => {
            assert_eq!(
                wsv,
                vec![
                    vec![
                        w::Value("nullExample".to_owned()),
                        w::Null,
                        w::Value("2".to_owned())
                    ],
                    vec![w::Null],
                    vec![w::Null, w::Value("2".to_owned())],
                    vec![w::Value("3".to_owned()), w::Null]
                ]
            );
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}

#[test]
fn numbers() {
    let contents = read_to_string("./tests/example_files/numbers.wsv").unwrap();
    match parse(&contents) {
        Ok(wsv) => {
            assert_eq!(
                wsv,
                vec![vec![
                    w::Value("1".to_owned()),
                    w::Value("2.0".to_owned()),
                    w::Value("3.4.5".to_owned()),
                    w::Value("6.789".to_owned())
                ],]
            );
        }
        Err(error) => {
            panic!("Shouldn't Err. Got {error}")
        }
    }
}
#[test]
fn strings() {
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

#[test]
fn comments() {
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

#[test]
fn not_null() {
    let contents = read_to_string("./tests/example_files/not_null.wsv").unwrap();
    let wsv = parse(&contents).unwrap();
    println!("{:?}", wsv);
    assert_eq!(wsv, vec![vec![w::Value("-".to_string())]]);
}

#[test]
fn empty() {
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

#[test]
fn odd_quotes() {
    let contents = read_to_string("./tests/example_files/odd_quotes.wsv").unwrap();
    match parse(&contents) {
        Ok(v) => panic!("Parsed Odd Double Quotes: {v:?}"),
        Err(e) => {
            println!("{e:?}")
        }
    }
}
