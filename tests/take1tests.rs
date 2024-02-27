use wsv::take1::parse;

#[test]
fn nulls() {

    
    match parse("./tests/example_files/nulls.wsv") {
        Ok(wsv) => {
            assert_eq!(
                wsv,
                vec![
                    vec![Some("nullExample".to_owned()), None, Some("2".to_owned())],
                    vec![None],
                    vec![None, Some("2".to_owned())],
                    vec![Some("3".to_owned()), None]
                ]
            );
        }
        Err(error) => {
            panic!("Function shouldn't throw errors yet. Got {error}")
        }
    }
}

#[test]
fn numbers() {
    match parse("./tests/example_files/numbers.wsv") {
        Ok(wsv) => {
            assert_eq!(
                wsv,
                vec![vec![
                    Some("1".to_owned()),
                    Some("2.0".to_owned()),
                    Some("3.4.5".to_owned()),
                    Some("6.789".to_owned())
                ],]
            );
        }
        Err(error) => {
            panic!("Function shouldn't throw errors yet. Got {error}")
        }
    }
}
#[test]
fn strings() {
    match parse("./tests/example_files/strings.wsv") {
        Ok(wsv) => {
            assert_eq!(
                wsv,
                vec![
                    vec![Some("hello".to_owned())],
                    vec![Some("it's".to_owned()), Some("me".to_owned())],
                    vec![Some("I was wondering if \" after all these ".to_owned())],
                    vec![Some("years you'd like\nto meet".to_owned())]
                ]
            );
        }
        Err(error) => {
            panic!("Function shouldn't throw errors yet. Got {error}")
        }
    }
}

#[test]
fn comments() {
    match parse("./tests/example_files/comments.wsv") {
        Ok(wsv) => {
            assert_eq!(wsv, vec![vec![Some("CommentExample".to_owned())]]);
        }
        Err(error) => {
            panic!("Function shouldn't throw errors yet. Got {error}")
        }
    }
}

#[test]
fn not_null() {
    let wsv = parse("./tests/example_files/not_null.wsv").unwrap();
    println!("{:?}", wsv);
    assert_eq!(wsv, vec![vec![Some("-".to_string())]]);
}

#[test]
#[should_panic]

fn odd_quotes() {
    parse("./tests/example_files/odd_quotes.wsv").expect_err("Odd Double Quotes");
}
#[test]
#[should_panic]

fn invalid_utf8() {
    parse("./tests/example_files/invalid_utf8.wsv").expect_err("Not UTF-8");
}
#[test]
#[should_panic]
fn empty() {
    parse("./tests/example_files/empty.wsv").expect_err("Other");
}
#[test]
#[should_panic]

fn utf8withbom() {
    parse("./tests/example_files/Untitled.txt").expect_err("BomPresent");
}
