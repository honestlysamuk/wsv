use crate::data_model::*;
pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}
fn parse_line((row_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    // File rows are one-indexed.
    let row = row_index + 1;

    let mut inputs = line.chars();
    let mut state = State::Default;
    let end_state = State::Finished;
    let mut data = Data::new(row);

    while end_state != state {
        let next_input = inputs.next();
        state = transition(state, next_input);
        data = data.apply(g(state, &next_input)); // not much difference.
    }

    data.reconcile()
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Transform {
    PushChar(char),
    AddError(ErrorKind),
    PushDash,
    PushQuote,
    PushNewline,
    AddValue,
    AddNull,
    IncrementColumnNumber,
}
#[derive(Debug, PartialEq, Hash, Clone, Copy)]
enum State {
    Default,
    Comment,
    Finished,
    MayBeNull,
    Null,
    Value,
    EndOfValue,
    Error(ErrorKind),
    StartString,
    EscapeOrEndOfString,
    MayBeEscapedReturn,
    EscapedReturn,
    EscapedDoubleQuote,
    StringPart,
}
fn g(state: State, input: &Option<char>) -> Transform {
    match (state, input) {
        (State::Value, Some(c)) => Transform::PushChar(*c),
        (State::Value, _) => Transform::IncrementColumnNumber,
        (State::StringPart, Some(c)) => Transform::PushChar(*c),
        (State::StringPart, _) => Transform::IncrementColumnNumber,
        (State::Error(kind), _) => Transform::AddError(kind),
        (State::MayBeNull, _) => Transform::PushDash,
        (State::EscapedReturn, _) => Transform::PushNewline,
        (State::EscapedDoubleQuote, _) => Transform::PushQuote,
        (State::EndOfValue, _) => Transform::AddValue,
        (State::Comment, _) => Transform::AddValue,
        (State::Null, _) => Transform::AddNull,
        (_, _) => Transform::IncrementColumnNumber,
    }
}
type InputSet = Option<char>;
fn transition(state: State, event: InputSet) -> State {
    match (state, event) {
        (State::Error(ErrorKind::MissingWhitespace), _) => State::Finished,
        (State::Error(ErrorKind::OddDoubleQuotes), _) => State::Finished,
        (State::Error(_), _) => State::Finished,
        (State::Value, None) => State::EndOfValue,
        (State::Value, Some('\"')) => State::Error(ErrorKind::MissingWhitespace),
        (State::Value, Some('#')) => State::Comment,
        (State::Value, Some(c)) if c.is_whitespace() => State::EndOfValue,
        (State::Value, Some(_)) => State::Value,
        (State::StringPart, None) => State::Error(ErrorKind::OddDoubleQuotes),
        (State::StringPart, Some('\"')) => State::EscapeOrEndOfString,
        (State::StringPart, Some(_)) => State::StringPart,
        (State::Finished, _) => State::Finished,
        (State::Comment, _) => State::Finished,
        (State::Default, None) => State::Finished,
        (State::Default, Some('#')) => State::Finished,
        (State::Default, Some('-')) => State::MayBeNull,
        (State::Default, Some('\"')) => State::StartString,
        (State::Default, Some(c)) if c.is_whitespace() => State::Default,
        (State::Default, Some(_)) => State::Value,
        (State::EndOfValue, None) => State::Finished,
        (State::EndOfValue, Some('#')) => State::Finished,
        (State::EndOfValue, Some('-')) => State::MayBeNull,
        (State::EndOfValue, Some('\"')) => State::StartString,
        (State::EndOfValue, Some(c)) if c.is_whitespace() => State::Default,
        (State::EndOfValue, Some(_)) => State::Value,
        (State::Null, None) => State::Finished,
        (State::Null, Some('#')) => State::Finished,
        (State::Null, Some('-')) => State::MayBeNull,
        (State::Null, Some('\"')) => State::StartString,
        (State::Null, Some(c)) if c.is_whitespace() => State::Default,
        (State::Null, Some(_)) => State::Value,
        (State::MayBeNull, None) => State::Null,
        (State::MayBeNull, Some(c)) if c.is_whitespace() => State::Null,
        (State::MayBeNull, Some('\"')) => State::Error(ErrorKind::MissingWhitespace),
        (State::MayBeNull, Some(_)) => State::Value,
        (State::StartString, None) => State::Error(ErrorKind::OddDoubleQuotes),
        (State::StartString, Some('\"')) => State::EscapeOrEndOfString,
        (State::StartString, Some(_)) => State::StringPart,
        (State::EscapedReturn, None) => State::Error(ErrorKind::OddDoubleQuotes),
        (State::EscapedReturn, Some('\"')) => State::EscapeOrEndOfString,
        (State::EscapedReturn, Some(_)) => State::StringPart,
        (State::EscapedDoubleQuote, None) => State::Error(ErrorKind::OddDoubleQuotes),
        (State::EscapedDoubleQuote, Some('\"')) => State::EscapeOrEndOfString,
        (State::EscapedDoubleQuote, Some(_)) => State::StringPart,
        (State::EscapeOrEndOfString, None) => State::EndOfValue,
        (State::EscapeOrEndOfString, Some('#')) => State::Comment,
        (State::EscapeOrEndOfString, Some('\"')) => State::EscapedDoubleQuote,
        (State::EscapeOrEndOfString, Some('/')) => State::MayBeEscapedReturn,
        (State::EscapeOrEndOfString, Some(c)) if c.is_whitespace() => State::EndOfValue,
        (State::EscapeOrEndOfString, _) => State::Error(ErrorKind::MissingWhitespace),
        (State::MayBeEscapedReturn, Some('\"')) => State::EscapedReturn,
        (State::MayBeEscapedReturn, _) => State::Error(ErrorKind::MissingWhitespace),
    }
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

    fn apply(mut self, transform: Transform) -> Self {
        match transform {
            Transform::PushChar(char) => {
                self.col += 1;
                self.buf.push(char);
            }
            Transform::AddError(kind) => {
                self.col += 1;
                self.err = Some(Error::new(kind, self.row, self.col, None));
            }
            Transform::PushDash => {
                self.col += 1;
                self.buf.push('-');
            }
            Transform::PushQuote => {
                self.col += 1;
                self.buf.push('\"');
            }
            Transform::PushNewline => {
                self.col += 1;
                self.buf.push('\n');
            }
            Transform::AddValue => {
                self.col += 1;
                self.out.push(WsvValue::V(self.buf.clone()));
                self.buf.clear();
            }
            Transform::AddNull => {
                self.col += 1;
                self.out.push(WsvValue::Null);
                self.buf.clear();
            }
            Transform::IncrementColumnNumber => self.col += 1,
        }
        self
    }
    fn reconcile(self) -> Result<Vec<WsvValue>, Error> {
        match self.err {
            Some(e) => Err(dbg!(e)),
            None => Ok(self.out),
        }
    }
}

#[cfg(test)]
use crate::unit;
#[cfg(test)]
unit! {}
