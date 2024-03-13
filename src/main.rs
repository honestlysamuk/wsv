use std::error::Error;
use std::fs::read_to_string;
use std::{env, process};
use tracing::subscriber::set_global_default as sgd;
use tracing::Level;
use tracing_subscriber::FmtSubscriber as sub;
use wsv::{Wsv, WsvValue};

fn run(input: String) -> Result<(), Box<dyn Error>> {
    let contents =
        read_to_string::<String>("./tests/example_files/".to_string() + &input + ".wsv")?;

    let wsv = Wsv::try_from(contents.as_str())?;

    println!("{wsv:#?}");
    let sum = wsv
        .into_iter()
        .map(|row| {
            if let Some(WsvValue::Value(string)) = row.first() {
                string.len()
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("done: {}", sum);
    println!();
    println!("{contents}");
    Ok(())
}

fn main() {
    sgd(sub::builder().with_max_level(Level::TRACE).finish()).unwrap();
    let input = env::args_os().nth(1).unwrap().into_string().unwrap();
    if let Err(err) = run(input) {
        println!("{:?}", err);
        process::exit(1);
    }
}
