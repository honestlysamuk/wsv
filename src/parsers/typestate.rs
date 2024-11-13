use crate::ErrorKind;
use crate::{Error as wsvError, WsvValue};
fn main() {
    let mut inputs = vec!['c'].into_iter();
    let mut state: Box<dyn State> = Box::new(Comment(Scaffold::at_row(0)));
    let mut ops = vec![];

    loop {
        let next_input = inputs.next();
        state = state.transition(next_input);
        ops.push(state.modify_with(next_input));
        if next_input.is_none() {
            break;
        }
    }
}

trait State {
    fn transition(self, input: Option<char>) -> Box<dyn State>;
    fn modify_with(&mut self, input: Option<char>);
}

struct Comment(Scaffold);
impl State for Comment {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            Some('\n') => Box::new(EndOfLine(self.0)),
            None => Box::new(Finished(self.0)),
            _ => Box::new(self),
        }
    }
    fn modify_with(&mut self, input: Option<char>) {
        self.0.col += 1;
        self.0.buf.push(input.unwrap());
    }
}

struct Default(Scaffold);
impl State for Default {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            Some('\n') => Box::new(EndOfLine(self.0)),
            None => Box::new(Finished(self.0)),
            _ => Box::new(self),
        }
    }
    fn modify_with(&mut self, input: Option<char>) {
        self.0.col += 1;
        self.0.buf.push(input.unwrap());
    }
}

struct EndOfLine(Scaffold);
impl State for EndOfLine {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Finished(self.0)),
            Some('\n') => Box::new(State::EndOfLine(self.0)),
            Some('#') => Box::new(State::Comment(self.0)),
            Some('-') => Box::new(State::MayBeNull(self.0)),
            Some('\"') => Box::new(State::StartString(self.0)),
            Some(c) if c.is_whitespace() => Box::new(State::Default(self.0)),
            Some(_) => Box::new(State::Value(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0.out.push(vec![]);
    }
}

struct EndOfValue(Scaffold);
impl State for EndOfValue {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Finished(self.0)),
            Some('\n') => Box::new(State::EndOfLine(self.0)),
            Some('#') => Box::new(State::Comment(self.0)),
            Some('-') => Box::new(State::MayBeNull(self.0)),
            Some('\"') => Box::new(State::StartString(self.0)),
            Some(c) if c.is_whitespace() => Box::new(State::Default(self.0)),
            Some(_) => Box::new(State::Value(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0
            .out
            .last_mut()
            .expect("initialised with one")
            .push(WsvValue::V(self.0.buf.clone()));
        self.0.buf.clear();
    }
}

struct EndOfValueAndEndOfLine(Scaffold);
impl State for EndOfValueAndEndOfLine {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Finished(self.0)),
            Some('\n') => Box::new(State::EndOfLine(self.0)),
            Some('#') => Box::new(State::Comment(self.0)),
            Some('-') => Box::new(State::MayBeNull(self.0)),
            Some('\"') => Box::new(State::StartString(self.0)),
            Some(c) if c.is_whitespace() => Box::new(State::Default(self.0)),
            Some(_) => Box::new(State::Value(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0
            .out
            .last_mut()
            .expect("initialised with one")
            .push(WsvValue::V(self.0.buf.clone()));
        self.0.out.push(vec![]);
        self.0.buf.clear();
    }
}

struct Error(Scaffold, ErrorKind);
impl State for Error {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        State::Finished
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0.err = Some(wsvError::new(self.1, self.0.row, self.0.col, None));
    }
}

struct EscapeOrEndOfString(Scaffold);
impl State for EscapeOrEndOfString {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::EndOfValue(self.0)),
            Some('\n') => Box::new(State::EndOfValueAndEndOfLine(self.0)),
            Some('#') => Box::new(State::StartComment(self.0)),
            Some('\"') => Box::new(State::EscapedDoubleQuote(self.0)),
            Some('/') => Box::new(State::MayBeEscapedReturn(self.0)),
            Some(c) if c.is_whitespace() => Box::new(State::EndOfValue(self.0)),
            _ => Box::new(State::Error(self.0, ErrorKind::MissingWhitespace)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
    }
}

struct EscapedDoubleQuote(Scaffold);
impl State for EscapedDoubleQuote {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Error(self.0, ErrorKind::OddDoubleQuotes)),
            Some('\n') => Box::new(State::Error(self.0, ErrorKind::OddDoubleQuotes)),
            Some('\"') => Box::new(State::EscapeOrEndOfString(self.0)),
            Some(_) => Box::new(State::StringPart(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0.buf.push('\"');
    }
}

struct EscapedReturn(Scaffold);
impl State for EscapedReturn {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Error(self.0, ErrorKind::OddDoubleQuotes)),
            Some('\n') => Box::new(State::Error(self.0, ErrorKind::OddDoubleQuotes)),
            Some('\"') => Box::new(State::EscapeOrEndOfString(self.0)),
            Some(_) => Box::new(State::StringPart(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0.buf.push('\n');
    }
}

struct Finished(Scaffold);
impl State for Finished {
    fn transition(self, _input: Option<char>) -> Box<dyn State> {
        Box::new(self)
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
    }
}

struct MayBeEscapedReturn(Scaffold);
impl State for MayBeEscapedReturn {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            Some('\"') => Box::new(State::EscapedReturn(self.0)),
            _ => Box::new(State::Error(self.0, ErrorKind::MissingWhitespace)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
    }
}

struct MayBeNull(Scaffold);
impl State for MayBeNull {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Null(self.0)),
            Some('\n') => Box::new(State::NullEndOfLine(self.0)),
            Some(c) if c.is_whitespace() => Box::new(State::Null(self.0)),
            Some('\"') => Box::new(State::Error(self.0, ErrorKind::MissingWhitespace)),
            Some(_) => Box::new(State::Value(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0.buf.push('-');
    }
}

struct Null(Scaffold);
impl State for Null {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Finished(self.0)),
            Some('\n') => Box::new(State::EndOfLine(self.0)),
            Some('#') => Box::new(State::EndOfLine(self.0)),
            Some('-') => Box::new(State::MayBeNull(self.0)),
            Some('\"') => Box::new(State::StartString(self.0)),
            Some(c) if c.is_whitespace() => Box::new(State::Default(self.0)),
            Some(_) => Box::new(State::Value(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0
            .out
            .last_mut()
            .expect("initialised with one")
            .push(WsvValue::Null);
        self.0.buf.clear();
    }
}

struct NullEndOfLine(Scaffold);
impl State for NullEndOfLine {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Finished(self.0)),
            Some('\n') => Box::new(State::EndOfLine(self.0)),
            Some('#') => Box::new(State::Comment(self.0)),
            Some('-') => Box::new(State::MayBeNull(self.0)),
            Some('\"') => Box::new(State::StartString(self.0)),
            Some(c) if c.is_whitespace() => Box::new(State::Default(self.0)),
            Some(_) => Box::new(State::Value(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0
            .out
            .last_mut()
            .expect("initialised with one")
            .push(WsvValue::V(self.0.buf.clone()));
        self.0.out.push(vec![]);
        self.0.buf.clear();
    }
}

struct StartComment(Scaffold);
impl State for StartComment {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            Some('\n') => Box::new(State::EndOfLine(self.0)),
            None => Box::new(State::Finished(self.0)),
            _ => Box::new(State::Comment(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
        self.0
            .out
            .last_mut()
            .expect("initialised with one")
            .push(WsvValue::V(self.0.buf.clone()));
        self.0.buf.clear();
    }
}

struct StartString(Scaffold);
impl State for StartString {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Error(self.0, ErrorKind::OddDoubleQuotes)),
            Some('\n') => Box::new(State::Error(self.0, ErrorKind::OddDoubleQuotes)),
            Some('\"') => Box::new(State::EscapeOrEndOfString(self.0)),
            Some(_) => Box::new(State::StringPart(self.0)),
        }
    }
    fn modify_with(&mut self, _input: Option<char>) {
        self.0.col += 1;
    }
}

struct StringPart(Scaffold);
impl State for StringPart {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => Box::new(State::Error(self.0, ErrorKind::OddDoubleQuotes)),
            Some('\n') => Box::new(State::Error(self.0, ErrorKind::OddDoubleQuotes)),
            Some('\"') => Box::new(State::EscapeOrEndOfString(self.0)),
            Some(_) => Box::new(State::StringPart(self.0)),
        }
    }
    fn modify_with(&mut self, input: Option<char>) {
        self.0.col += 1;
        if let Some(c) = input {
            self.0.buf.push(c);
        }
    }
}

struct Value(Scaffold);
impl State for Value {
    fn transition(self, input: Option<char>) -> Box<dyn State> {
        match input {
            None => State::EndOfValue,
            Some('\n') => Box::new(State::EndOfValueAndEndOfLine(self.0)),
            Some('\"') => Box::new(State::Error(self.0, ErrorKind::MissingWhitespace)),
            Some('#') => Box::new(State::StartComment(self.0)),
            Some(c) if c.is_whitespace() => Box::new(State::EndOfValue(self.0)),
            Some(_) => Box::new(State::Value(self.0)),
        }
    }
    fn modify_with(&mut self, input: Option<char>) {
        self.0.col += 1;
        if let Some(c) = input {
            self.0.buf.push(c);
        }
    }
}

/// The data structure which gets incrementally modified by the outputs, and produces the final parser output.
#[derive(Debug, Default)]
struct Scaffold {
    row: usize,
    col: usize,
    buf: String,
    out: Vec<Vec<WsvValue>>,
    err: Option<wsvError>,
}
impl Scaffold {
    fn at_row(idx: usize) -> Scaffold {
        Scaffold {
            row: 1 + idx,
            col: 0,
            buf: String::new(),
            out: vec![vec![]],
            err: None,
        }
    }
    fn finish(self) -> Result<Vec<Vec<WsvValue>>, wsvError> {
        match self.err {
            Some(e) => Err(dbg!(e)),
            None => Ok(self.out),
        }
    }
    fn finish_row(mut self) -> Result<Vec<WsvValue>, wsvError> {
        match self.err {
            Some(e) => Err(dbg!(e)),
            None => Ok(self.out.pop().unwrap()),
        }
    }
}
