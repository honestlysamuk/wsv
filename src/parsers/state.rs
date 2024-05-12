use crate::data_model::*;

pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

// we assume that line has no `\n`.
fn parse_line((row_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let row = row_index + 1;
    let mut inputs = line.chars();
    let mut state = State::Default;
    let finished_states = [State::Finished];
    let mut data = Data::new(row);

    while !finished_states.contains(&state) {
        state = state.transition(inputs.next());
        data.modify_with(&state);
    }

    data.reconcile()
}

#[derive(Debug)]
struct Data {
    row: usize,
    col: usize,
    buf: String,
    out: Vec<WsvValue>,
    err: Option<Error>,
}

impl Data {
    fn new(row: usize) -> Data {
        Data {
            row,
            col: 0,
            buf: String::new(),
            out: vec![],
            err: None,
        }
    }
    fn modify_with(&mut self, state: &State) {
        self.col += 1;
        match state {
            State::Value(c) => self.buf.push(*c),
            State::StringPart(c) => self.buf.push(*c),
            State::Error(kind) => self.err = Some(Error::new(*kind, self.row, self.col, None)),

            State::MayBeNull => self.buf.push('-'),
            State::EscapedReturn => self.buf.push('\n'),
            State::EscapedDoubleQuote => self.buf.push('\"'),
            State::EndOfValue => {
                self.out.push(WsvValue::V(self.buf.clone()));
                self.buf.clear();
            }
            State::Comment => {
                self.out.push(WsvValue::V(self.buf.clone()));
                self.buf.clear();
            }
            State::Null => {
                self.out.push(WsvValue::Null);
                self.buf.clear();
            }
            _ => {}
        }
    }
    fn reconcile(self) -> Result<Vec<WsvValue>, Error> {
        match self.err {
            Some(e) => Err(dbg!(e)),
            None => Ok(self.out),
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Default,
    Comment,
    Finished,
    MayBeNull,
    Null,
    Value(char),
    EndOfValue,
    Error(ErrorKind),
    StartString,
    EscapeOrEndOfString,
    MayBeEscapedReturn,
    EscapedReturn,
    EscapedDoubleQuote,
    StringPart(char),
}

impl State {
    fn transition(self, event: Option<char>) -> State {
        match (self, event) {
            (State::Finished, _) => State::Finished,
            (State::Comment, _) => State::Finished,
            (State::Error(_), _) => State::Finished,

            (State::Default, None) => State::Finished,
            (State::Default, Some('#')) => State::Finished,
            (State::Default, Some('-')) => State::MayBeNull,
            (State::Default, Some('\"')) => State::StartString,
            (State::Default, Some(c)) if c.is_whitespace() => State::Default,
            (State::Default, Some(c)) => State::Value(c),

            (State::EndOfValue, None) => State::Finished,
            (State::EndOfValue, Some('#')) => State::Finished,
            (State::EndOfValue, Some('-')) => State::MayBeNull,
            (State::EndOfValue, Some('\"')) => State::StartString,
            (State::EndOfValue, Some(c)) if c.is_whitespace() => State::Default,
            (State::EndOfValue, Some(c)) => State::Value(c),

            (State::Null, None) => State::Finished,
            (State::Null, Some('#')) => State::Finished,
            (State::Null, Some('-')) => State::MayBeNull,
            (State::Null, Some('\"')) => State::StartString,
            (State::Null, Some(c)) if c.is_whitespace() => State::Default,
            (State::Null, Some(c)) => State::Value(c),

            (State::MayBeNull, None) => State::Null,
            (State::MayBeNull, Some(c)) if c.is_whitespace() => State::Null,
            (State::MayBeNull, Some('\"')) => State::Error(ErrorKind::NoLeadingWhitespace),
            (State::MayBeNull, Some(c)) => State::Value(c),

            (State::Value(_), None) => State::EndOfValue,
            (State::Value(_), Some('\"')) => State::Error(ErrorKind::NoLeadingWhitespace),
            (State::Value(_), Some('#')) => State::Comment,
            (State::Value(_), Some(c)) if c.is_whitespace() => State::EndOfValue,
            (State::Value(_), Some(c)) => State::Value(c),

            (State::StartString, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StartString, Some('\"')) => State::EscapeOrEndOfString,
            (State::StartString, Some(c)) => State::StringPart(c),

            (State::EscapedReturn, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedReturn, Some('\"')) => State::EscapeOrEndOfString,
            (State::EscapedReturn, Some(c)) => State::StringPart(c),

            (State::EscapedDoubleQuote, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedDoubleQuote, Some('\"')) => State::EscapeOrEndOfString,
            (State::EscapedDoubleQuote, Some(c)) => State::StringPart(c),

            (State::EscapeOrEndOfString, None) => State::EndOfValue,
            (State::EscapeOrEndOfString, Some('#')) => State::Comment,
            (State::EscapeOrEndOfString, Some('\"')) => State::EscapedDoubleQuote,
            (State::EscapeOrEndOfString, Some('/')) => State::MayBeEscapedReturn,
            (State::EscapeOrEndOfString, Some(c)) if c.is_whitespace() => State::EndOfValue,
            (State::EscapeOrEndOfString, _) => State::Error(ErrorKind::NoTrailingWhitespace),

            (State::MayBeEscapedReturn, Some('\"')) => State::EscapedReturn,
            (State::MayBeEscapedReturn, _) => State::Error(ErrorKind::NoTrailingWhitespace),

            (State::StringPart(_), None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StringPart(_), Some('\"')) => State::EscapeOrEndOfString,
            (State::StringPart(_), Some(c)) => State::StringPart(c),
        }
    }
}

#[cfg(test)]
use crate::unit;
#[cfg(test)]
unit! {}
