use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::parse_line;
use crate::Error;
use crate::WsvValue;

pub fn parse_from_memory(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

pub fn parse_from_whole_file<P>(i: &P) -> Result<Vec<Vec<WsvValue>>, Error>
where
    P: AsRef<Path>,
{
    fs::read_to_string(i)
        .unwrap()
        .split('\n')
        .enumerate()
        .map(parse_line)
        .collect()
}

pub fn parse_from_buf_file<P>(i: &P) -> Result<Vec<Vec<WsvValue>>, Error>
where
    P: AsRef<Path>,
{
    BufReader::new(File::open(i).unwrap())
        .lines()
        .enumerate()
        .map(|(i, l)| parse_line((i, &l.expect("input is utf-8"))))
        .collect()
}
