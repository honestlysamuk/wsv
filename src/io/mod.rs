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

mod benchmarks;
