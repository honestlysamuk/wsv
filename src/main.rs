use wsv::{pest::*, tabulate::tabulate};
fn main() {
    println!(
        "{}",
        tabulate(parse("./tests/nulls.wsv").expect("no errors"))
    );
}
