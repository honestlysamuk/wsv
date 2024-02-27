use unicode_bom::Bom;

pub use crate::data_model::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn parse<P>(path: P) -> Result<Vec<WsvLine>, WsvError>
where
    P: AsRef<Path>,
{
    let bom = Bom::from(&mut File::open(&path)?);
    match bom {
        Bom::Bom::Null => parse_lossy(File::open(&path)?).into_iter().collect(),
        _ => Err(WsvError::BomPresent(bom)),
    }
}
pub fn parse_lossy(file: File) -> Vec<Result<WsvLine, WsvError>> {
    BufReader::new(file)
        .lines()
        .map(|line| match line {
            Ok(str) => parse_line(str),
            Err(e) => Err(WsvError::from(e)),
        })
        .collect()
}
