use crate::data_model::*;

/*

fn parse_line2((line_number, line): (usize, &str)) -> Result<Vec<WsvValue>, WsvError> {
    let it = line.split('\"');
    if it.count() % 2 == 0 {
        Err(WsvError::DoubleQuotesMismatch(line_number))
    } else {
        it.
        todo!()
    }
}

split on hash. If two elements, Run parser on the first element. else run parser on the whole input

if three elements, run parser on the first input, else on first two, else on the whole input

push " " onto top and bottom of the string
split on "\""
first and last elements must be of the form

  aabb aabb - aabb    a

 split whitespace,  and add all values to the line

  second part must be part of the string. add all to buffer
  third part could be same as first and last, or could also be just / or empty

  third = / then push "\n" to the buf. = empty then push "\"" to the buf, else check for a hash, etc.

  fourth



part: &str =



Cannot split on whitespace
*/

enum Decision {
    SpecialCharacter(char),
    EndOfString,
}

fn process_part(part: &str) -> Vec<WsvValue> {
    part.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(WsvValue::from)
        .collect()
}

fn process_string_part(part: &str) -> Decision {
    if part == "/" {
        Decision::SpecialCharacter('\n')
    } else if part.is_empty() {
        Decision::SpecialCharacter('\"')
    } else {
        Decision::EndOfString
    }
}

#[tracing::instrument]
fn parse_line_without_comments(
    (line_number, line): (usize, &str),
) -> Result<Vec<WsvValue>, WsvError> {
    let mut result: Vec<WsvValue> = Vec::new();
    let mut string = String::new();
    let mut index = 0;

    tracing::debug!("{line}");

    for (i, part) in line.split('\"').enumerate() {
        index = i;
        if i % 2 == 0 {
            string.push_str(part);
        } else {
            match process_string_part(part) {
                Decision::SpecialCharacter(ch) => string.push(ch),
                Decision::EndOfString => {
                    result.push(WsvValue::from(&mut string));
                    string.clear();
                    result.append(&mut process_part(part));
                }
            }
        }
    }
    if index % 2 == 0 {
        Err(WsvError::DoubleQuotesMismatch(line_number))
    } else {
        Ok(result)
    }
}

fn parse_line((line_number, line): (usize, &str)) -> Result<Vec<WsvValue>, WsvError> {
    for line_before_comment in line.split('#') {
        if let Ok(result) = parse_line_without_comments((line_number, line_before_comment)) {
            return Ok(result);
        }
    }
    Err(WsvError::DoubleQuotesMismatch(line_number))
}

pub fn parse(i: &str) -> Result<Wsv, WsvError> {
    if i.is_empty() {
        Ok(Default::default())
    } else {
        match i
            .split('\n')
            .enumerate()
            .map(parse_line)
            .collect::<Result<Vec<Vec<WsvValue>>, WsvError>>()
        {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(e),
        }
    }
}
// The odd case:

// oes fij " soifjf seoij " oir joirjse " oirjefoe ij " sodifj sefj "sduih""siuhf" siueh
