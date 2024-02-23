use wsv::takepest::parse;
use wsv::takepest::WsvValue as w;

#[test]
fn nulls() {
    match parse("./tests/nulls.wsv") {
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
    match parse("./tests/numbers.wsv") {
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
    match parse("./tests/strings.wsv") {
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
    match parse("./tests/comments.wsv") {
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
    let wsv = parse("./tests/not_null.wsv").unwrap();
    println!("{:?}", wsv);
    assert_eq!(wsv, vec![vec![w::Value("-".to_string())]]);
}

#[test]
fn empty() {
    match parse("./tests/empty.wsv") {
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
    match parse("./tests/odd_quotes.wsv") {
        Ok(v) => panic!("Parsed Odd Double Quotes: {v:?}"),
        Err(_) => {}
    }
}
