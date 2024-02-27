use std::fs::read_to_string;

use wsv::{parse, tabulate::tabulate};
fn main() {
    let contents = read_to_string("./tests/example_files/strings.wsv").unwrap();

    println!("{}", tabulate(parse(&contents).unwrap()))

    // println!(
    //     "{}",
    //     tabulate(parse("./tests/nulls.wsv").expect("no errors"))
    // );
}
