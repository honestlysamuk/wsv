use std::env;
use std::error::Error;
use std::fs::read_to_string;
use tracing::subscriber::set_global_default as sgd;
use tracing::Level;
use tracing_subscriber::FmtSubscriber as sub;
use wsv::{Wsv, WsvValue};
// rust-analyzer.inlayHints.typeHints.enable

fn main() {
    sgd(sub::builder().with_max_level(Level::TRACE).finish()).unwrap();
    let input = env::args().nth(1).unwrap();
    if let Err(err) = run(input) {
        println!("{:?}", err);
    }
}

// for record in WsvReader::builder().(file)?.into_iter() {
//     let country: Country = record.deserialize()?;
//     println!("{country:?}");
// }

fn run(input: String) -> Result<(), Box<dyn Error>> {
    let contents =
        read_to_string::<String>(["./tests/example_files/", &input, ".wsv"].concat())?;

    let wsv = Wsv::try_from(contents.as_str())?;

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

