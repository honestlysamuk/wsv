use wsv::takepest::*;

fn main() {
    let elements = parse("./tests/odd_quotes.wsv").expect_err("no errors");

    println!("{elements:?}");
}
