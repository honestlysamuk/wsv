use std::fs::read_to_string;
use tracing::subscriber::set_global_default as sgd;
use tracing::Level;
use tracing_subscriber::FmtSubscriber as sub;
use wsv::Wsv;

fn main() {
    sgd(sub::builder().with_max_level(Level::TRACE).finish()).unwrap();

    let contents = read_to_string("./tests/example_files/strings.wsv").unwrap();

    println!("{}", Wsv::try_from(contents).unwrap())
}
