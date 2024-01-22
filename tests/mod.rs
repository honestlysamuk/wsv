use wsv::*;

#[test]
fn nulls() {
    let wsv = parse_wsv("./tests/nulls.wsv").unwrap();
    println!("{:?}", &wsv);
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
#[test]
fn comments() {
    let wsv = parse_wsv("./tests/comments.wsv").unwrap();
    println!("{:?}", &wsv);
    assert_eq!(wsv, vec![vec![Some("CommentExample".to_owned())]]);
}
#[test]
fn numbers() {
    let wsv = parse_wsv("./tests/numbers.wsv").unwrap();
    println!("{:?}", &wsv);
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
#[test]
fn strings() {
    let wsv = parse_wsv("./tests/strings.wsv").unwrap();
    println!("{:?}", &wsv);
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
#[test]
fn not_null() {
    let wsv = parse_wsv("./tests/not_null.wsv").unwrap();
    println!("{:?}", wsv);
    assert_eq!(wsv, vec![vec![Some("-".to_string())]]);
}

#[test]
#[should_panic]
fn odd_quotes() {
    let wsv = parse_wsv("./tests/odd_quotes.wsv").unwrap();
    println!("{:?}", &wsv);
}
#[test]
#[should_panic]
fn invalid_utf8() {
    let wsv = parse_wsv("./tests/invalid_utf8.wsv").unwrap();
    println!("{:?}", &wsv);
}
