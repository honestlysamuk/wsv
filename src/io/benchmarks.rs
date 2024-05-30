const PARSERS: [Parser; 6] = [
    Parser::Nom,
    Parser::State,
    Parser::Mealy,
    Parser::Moore,
    Parser::Split,
    Parser::First,
];

use std::fs::File;

use super::*;
use crate::data_model::Parser;
use divan::black_box;

#[divan::bench(args = PARSERS, sample_count = 1000)]
fn reader_one_big_line(parser: Parser) {
    from_reader_with_parser(
        black_box(&mut File::open("./tests/example_files/1bigline.wsv").unwrap()),
        parser,
    );
}
#[divan::bench(args = PARSERS, sample_count = 10)]
fn reader_lines_100_000(parser: Parser) {
    from_reader_with_parser(
        black_box(&mut File::open("./tests/example_files/100000lines.wsv").unwrap()),
        parser,
    );
}
#[divan::bench(args = PARSERS, sample_count = 10)]
fn reader_comments_100_000(parser: Parser) {
    from_reader_with_parser(
        black_box(&mut File::open("./tests/example_files/100000comments.wsv").unwrap()),
        parser,
    );
}

// #[divan::bench(args = PARSERS, sample_count = 1000)]
// fn string_one_big_line(parser: Parser) {
//     from_string_with_parser(
//         black_box(&mut File::open("./tests/example_files/1bigline.wsv").unwrap()),
//         parser,
//     );
// }
// #[divan::bench(args = PARSERS, sample_count = 10)]
// fn string_lines_100_000(parser: Parser) {
//     from_string_with_parser(
//         black_box(&mut File::open("./tests/example_files/100000lines.wsv").unwrap()),
//         parser,
//     );
// }

// count length of every string
// sum every number
// count every string with the letter A

// let sum = wsv
// .iter()
// .map(|row| {
//     if let Some(WsvValue::V(string)) = row.first() {
//         string.len()
//     } else {
//         0
//     }
// })
// .sum::<usize>();

// println!("done: {}", sum);
// println!();

// for record in WsvReader::builder().(file)?.into_iter() {
//     let country: Country = record.deserialize()?;
//     println!("{country:?}");
// }
