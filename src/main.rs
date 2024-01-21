use std::{fs, path::Path};

fn main() {
    let example = "/home/srarr/Projects/wsv/tests/example.wsv";

    type WSV = Vec<Vec<Option<String>>>;
    fn parse_wsv<P>(path: P) -> WSV
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(path).expect("UTF-8 encoding");
        let lines = contents.split("\n").collect::<Vec<&str>>();

        let mut result: WSV = Vec::new();

        for line in lines.iter() {
            let mut value = String::new();
            let mut values: Vec<Option<String>> = Vec::new();
            let mut open_quotes: bool = false;

            for c in line.chars() {
                match c {
                    '#' => {
                        if open_quotes {
                            value.push(c);
                        } else {
                            break; // back to the line loop
                        }
                    }
                    c if c.is_whitespace() => {
                        if !value.is_empty() {
                            values.push(Some(value));
                            value = String::new();
                        }
                    }
                    '"' => {
                        open_quotes = !open_quotes;
                        value.push(c);
                    }
                    _ => value.push(c),
                }
            }
            if !value.is_empty() {
                values.push(Some(value));
            }
            result.push(values);
            values = Vec::new();
        }
        result
    }
    let wsv = parse_wsv(example);
    println!("{:?}", wsv);
}
