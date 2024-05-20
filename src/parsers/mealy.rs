//! A Mealy machine is another form of state machine which is defined by six other variables.
//!
//! For educational purposes, I am going to write code in such a way as to highlight those six things as clearly as possible.

//! The Mealy variation actually changes very little, cosmetically. All that has happened is
//! instead of storing the current character on the Value or StringPart state, we
//! are just using the input again in the g function. This reduces the number of possible states
//! significantly. Remember that there are 149,878 unicode characters (as of Unicode v15.1),
//! each of which had its own state. This change reduces the number of states by 299,756.
//! Fortunately, since Rust has the _ character in pattern matching, this change is mostly transparent.
//!
//! The input set is still this number plus one for the None case.

use crate::data_model::*;
pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}
fn parse_line((row_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    // File rows are one-indexed.
    let mut data = Data::new(row_index + 1);

    // I can get the same behaviour if I just call next() on the chars iterator, but I've made the input
    // set explicit to indicate that I'm not just iterating over the characters of the input. I've had
    // remove all '\n's and add a None to indicate termination of the row. This is a simplification on my
    // part. It is so I can use

    let input_set = line.chars().map(Some).chain(vec![None]);

    let mut wsv = WsvMachine::default();
    for i in input_set {
        data.apply(wsv.step(&i));
    }

    let input_set = line.chars().map(Some).chain(vec![None]);

    WsvMachine::process(input_set).for_each(|o| {
        data.apply(o);
    });

    data.reconcile()
}

trait Mealy {
    type StateSpace: Default;
    type InputAlphabet;
    type OutputAlphabet;
    fn transition(state: &Self::StateSpace, input: &Self::InputAlphabet) -> Self::StateSpace;
    fn output(state: &Self::StateSpace, input: &Self::InputAlphabet) -> Self::OutputAlphabet;
    fn state(&mut self) -> &mut Self::StateSpace;

    fn step(&mut self, input: &Self::InputAlphabet) -> Self::OutputAlphabet {
        let state = self.state();
        *state = Self::transition(&state, input);
        Self::output(&state, input)
    }

    fn process(
        input: impl Iterator<Item = Self::InputAlphabet>,
    ) -> impl Iterator<Item = Self::OutputAlphabet> {
        input.scan(Self::StateSpace::default(), |state, i| {
            *state = Self::transition(state, &i);
            Some(Self::output(&state, &i))
        })
    }
}

#[derive(Debug, Default)]
struct WsvMachine {
    state: State,
}

#[derive(Debug, PartialEq, Hash, Clone, Copy, Default)]
enum State {
    #[default]
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

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Transform {
    AddValue,
    AddNull,
    AddError(ErrorKind),
    PushChar(char),
    PushDash,
    PushQuote,
    PushNewline,
    IncrementColumnNumber,
}

impl Mealy for WsvMachine {
    type StateSpace = State;
    type InputAlphabet = Option<char>;
    type OutputAlphabet = Transform;

    fn state(&mut self) -> &mut Self::StateSpace {
        &mut self.state
    }
    fn output(state: &Self::StateSpace, input: &Self::InputAlphabet) -> Self::OutputAlphabet {
        match (state, input) {
            (State::Value, Some(c)) => Transform::PushChar(*c),
            (State::Value, _) => Transform::IncrementColumnNumber,
            (State::StringPart, Some(c)) => Transform::PushChar(*c),
            (State::StringPart, _) => Transform::IncrementColumnNumber,
            (State::Error(kind), _) => Transform::AddError(*kind),
            (State::MayBeNull, _) => Transform::PushDash,
            (State::EscapedReturn, _) => Transform::PushNewline,
            (State::EscapedDoubleQuote, _) => Transform::PushQuote,
            (State::EndOfValue, _) => Transform::AddValue,
            (State::Comment, _) => Transform::AddValue,
            (State::Null, _) => Transform::AddNull,
            (_, _) => Transform::IncrementColumnNumber,
        }
    }
    fn transition(state: &Self::StateSpace, input: &Self::InputAlphabet) -> Self::StateSpace {
        match (state, input) {
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

    fn apply(&mut self, transform: Transform) {
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

use crate::unit_bench;
unit_bench! {}
