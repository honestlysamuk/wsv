use wsv::{tabulate::tabulate, takepest::*};
fn main() {
    println!(
        "{}",
        tabulate(parse("./tests/nulls.wsv").expect("no errors"))
    );
}
