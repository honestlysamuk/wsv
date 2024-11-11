use std::env;
use std::error::Error as stdError;
use std::fs::File;
// use tracing::subscriber::set_global_default as sgd;
// use tracing_subscriber::FmtSubscriber as sub;
// use tracing::Level;
use wsv::*;

fn main() {
    //sgd(sub::builder().with_max_level(Level::TRACE).finish()).unwrap();
    let input = env::args().nth(1).unwrap_or(String::from("welcome"));
    if let Err(err) = run(input) {
        println!("{:?}", err);
    }
}

fn run(input: String) -> Result<(), Box<dyn stdError>> {
    let mut file: File = File::open(["./tests/example_files/", &input, ".wsv"].concat())?;

    println!("{}", Wsv(from_reader(&mut file)));
    Ok(())
}
