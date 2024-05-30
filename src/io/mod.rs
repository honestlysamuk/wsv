use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use crate::data_model::Parser;
use crate::Error;
use crate::WsvValue;

pub fn from_reader(i: &mut impl Read) -> Vec<Result<Vec<WsvValue>, Error>> {
    from_reader_with_parser(i, Parser::default())
}

fn from_reader_with_parser(i: &mut impl Read, parser: Parser) -> Vec<Result<Vec<WsvValue>, Error>> {
    BufReader::new(i)
        .lines()
        .enumerate()
        .map(|(i, l)| (parser.fn_ptr())((i, &l.unwrap())))
        .collect()
}

pub fn from_string(i: &mut impl Read) -> Vec<Result<Vec<WsvValue>, Error>> {
    from_string_with_parser(i, Parser::default())
}

fn from_string_with_parser(i: &mut impl Read, parser: Parser) -> Vec<Result<Vec<WsvValue>, Error>> {
    let mut buf = String::new();
    i.read_to_string(&mut buf).unwrap();
    buf.split('\n').enumerate().map(parser.fn_ptr()).collect()
}

pub fn calculate_stuff(input: &Vec<Result<Vec<WsvValue>, Error>>) -> String {
    fn char_len(row: &Result<Vec<WsvValue>, Error>) -> usize {
        match row {
            Ok(data) => data.iter().map(|val| val.len()).sum(),
            Err(_) => 1,
        }
    }
    fn row_len(row: &Result<Vec<WsvValue>, Error>) -> usize {
        match row {
            Ok(data) => data.len(),
            Err(_) => 1,
        }
    }

    let character_count = input.iter().map(|row| char_len(row)).sum::<usize>();
    let used_cell_count = input.iter().map(|row| row_len(row)).sum::<usize>();
    let row_count = input.len();

    use std::fmt::Write;
    let mut out = String::new();
    write!(
        out,
        "Minimum Byte Count: {}",
        character_count + used_cell_count * row_count
    )
    .unwrap();
    write!(out, "\nUsed Cell Count: {}", used_cell_count).unwrap();
    write!(out, "\nRow Count: {}", row_count).unwrap();
    out
}

mod benchmarks;
