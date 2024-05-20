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

pub fn parse(i: &str) -> Vec<Result<Vec<WsvValue>, Error>> {
    i.split("n").enumerate().map(parse_line).collect()
}

pub fn parse_strict(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    let input_set = i.chars().map(Some).chain(vec![None]);

    let mut data = Data::new();
    WsvMachine::process(input_set).for_each(|o| {
        data.apply(o);
    });

    data.reconcile()
}

/// Note I can also use a loop and call `.next()` on the `chars` iterator to get the same behaviour, but
/// I wanted to make it clear that the input set is `Option<char>`, where the `None` indicates the end
/// of a row/file. It is not `char` on its own.
pub fn parse_line((row_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let input_set = line.chars().map(Some).chain(vec![None]);

    let mut data = Data::new().at_row(row_index + 1);
    WsvMachine::process(input_set).for_each(|o| {
        data.apply(o);
    });

    data.reconcile_row()
}

/// This trait encapsulates the raw definition of a Mealy Machine as closely as I can to the wikipedia entry.
///
/// There are two functions, three types, and one type constrait, making up the sextuple that is a
/// Mealy Machine. I am using the Default trait parameter on the StateSpace as the initial state requirement.
/// Notice that the function to combine all these parts is very short. Pure state machines are simple. The
/// hard part is working out what part of your problem fits in the StateSpace, InputAlphabet and OutputAlphabet.
/// The functions on them are the easy bit, relatively speaking.
trait Mealy {
    type StateSpace: Default;
    type InputAlphabet;
    type OutputAlphabet;
    fn transition(state: &Self::StateSpace, input: &Self::InputAlphabet) -> Self::StateSpace;
    fn output(state: &Self::StateSpace, input: &Self::InputAlphabet) -> Self::OutputAlphabet;

    /// This is the stateful version, with in and out being vectors instead.
    fn process_vec(input: Vec<Self::InputAlphabet>) -> Vec<Self::OutputAlphabet> {
        let mut state = Self::StateSpace::default();
        let mut output = vec![];
        for i in input {
            state = Self::transition(&state, &i);
            output.push(Self::output(&state, &i));
        }
        output
    }

    /// This is what I believe is the more "idiomatic" way. One can even argue for using
    /// `IntoIterator` instead of the `Iterator` trait.
    fn process(
        input: impl Iterator<Item = Self::InputAlphabet>,
    ) -> impl Iterator<Item = Self::OutputAlphabet> {
        input.scan(Self::StateSpace::default(), |state, i| {
            *state = Self::transition(state, &i);
            Some(Self::output(state, &i))
        })
    }
}

#[derive(Debug, Default)]
struct WsvMachine {}

#[derive(Debug, PartialEq, Hash, Clone, Copy, Default)]
enum State {
    #[default]
    Default,
    StartComment,
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
    EndOfValueAndEndOfLine,
    NullEndOfLine,
    EndOfLine,
    StringPart,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Transform {
    AddValue,
    AddValueAndRow,
    AddNull,
    AddNullAndRow,
    AddRow,
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
            (State::StartComment, _) => Transform::AddValue,
            (State::Null, _) => Transform::AddNull,
            (State::EndOfLine, _) => Transform::AddRow,
            (State::NullEndOfLine, _) => Transform::AddNullAndRow,
            (State::EndOfValueAndEndOfLine, _) => Transform::AddValueAndRow,

            (_, _) => Transform::IncrementColumnNumber,
        }
    }
    fn transition(state: &Self::StateSpace, input: &Self::InputAlphabet) -> Self::StateSpace {
        match (state, input) {
            (State::Finished, _) => State::Finished,
            (State::Error(_), _) => State::Finished,

            (State::Comment, Some('\n')) => State::EndOfLine,
            (State::Comment, None) => State::Finished,
            (State::Comment, _) => State::Comment,

            (State::StartComment, Some('\n')) => State::EndOfLine,
            (State::StartComment, None) => State::Finished,
            (State::StartComment, _) => State::Comment,

            (State::EndOfValueAndEndOfLine, None) => State::Finished,
            (State::EndOfValueAndEndOfLine, Some('\n')) => State::EndOfLine,
            (State::EndOfValueAndEndOfLine, Some('#')) => State::Comment,
            (State::EndOfValueAndEndOfLine, Some('-')) => State::MayBeNull,
            (State::EndOfValueAndEndOfLine, Some('\"')) => State::StartString,
            (State::EndOfValueAndEndOfLine, Some(c)) if c.is_whitespace() => State::Default,
            (State::EndOfValueAndEndOfLine, Some(_)) => State::Value,

            (State::NullEndOfLine, None) => State::Finished,
            (State::NullEndOfLine, Some('\n')) => State::EndOfLine,
            (State::NullEndOfLine, Some('#')) => State::Comment,
            (State::NullEndOfLine, Some('-')) => State::MayBeNull,
            (State::NullEndOfLine, Some('\"')) => State::StartString,
            (State::NullEndOfLine, Some(c)) if c.is_whitespace() => State::Default,
            (State::NullEndOfLine, Some(_)) => State::Value,

            (State::EndOfLine, None) => State::Finished,
            (State::EndOfLine, Some('\n')) => State::EndOfLine,
            (State::EndOfLine, Some('#')) => State::Comment,
            (State::EndOfLine, Some('-')) => State::MayBeNull,
            (State::EndOfLine, Some('\"')) => State::StartString,
            (State::EndOfLine, Some(c)) if c.is_whitespace() => State::Default,
            (State::EndOfLine, Some(_)) => State::Value,

            (State::Default, None) => State::Finished,
            (State::Default, Some('\n')) => State::EndOfLine,
            (State::Default, Some('#')) => State::Comment,
            (State::Default, Some('-')) => State::MayBeNull,
            (State::Default, Some('\"')) => State::StartString,
            (State::Default, Some(c)) if c.is_whitespace() => State::Default,
            (State::Default, Some(_)) => State::Value,

            (State::EndOfValue, None) => State::Finished,
            (State::EndOfValue, Some('\n')) => State::EndOfLine,
            (State::EndOfValue, Some('#')) => State::Comment,
            (State::EndOfValue, Some('-')) => State::MayBeNull,
            (State::EndOfValue, Some('\"')) => State::StartString,
            (State::EndOfValue, Some(c)) if c.is_whitespace() => State::Default,
            (State::EndOfValue, Some(_)) => State::Value,

            (State::Null, None) => State::Finished,
            (State::Null, Some('\n')) => State::EndOfLine,
            (State::Null, Some('#')) => State::EndOfLine,
            (State::Null, Some('-')) => State::MayBeNull,
            (State::Null, Some('\"')) => State::StartString,
            (State::Null, Some(c)) if c.is_whitespace() => State::Default,
            (State::Null, Some(_)) => State::Value,

            (State::MayBeNull, None) => State::Null,
            (State::MayBeNull, Some('\n')) => State::NullEndOfLine,
            (State::MayBeNull, Some(c)) if c.is_whitespace() => State::Null,
            (State::MayBeNull, Some('\"')) => State::Error(ErrorKind::MissingWhitespace),
            (State::MayBeNull, Some(_)) => State::Value,

            (State::Value, None) => State::EndOfValue,
            (State::Value, Some('\n')) => State::EndOfValueAndEndOfLine,
            (State::Value, Some('\"')) => State::Error(ErrorKind::MissingWhitespace),
            (State::Value, Some('#')) => State::StartComment,
            (State::Value, Some(c)) if c.is_whitespace() => State::EndOfValue,
            (State::Value, Some(_)) => State::Value,

            (State::StartString, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StartString, Some('\n')) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StartString, Some('\"')) => State::EscapeOrEndOfString,
            (State::StartString, Some(_)) => State::StringPart,

            (State::EscapedReturn, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedReturn, Some('\n')) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedReturn, Some('\"')) => State::EscapeOrEndOfString,
            (State::EscapedReturn, Some(_)) => State::StringPart,

            (State::EscapedDoubleQuote, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedDoubleQuote, Some('\n')) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedDoubleQuote, Some('\"')) => State::EscapeOrEndOfString,
            (State::EscapedDoubleQuote, Some(_)) => State::StringPart,

            (State::EscapeOrEndOfString, None) => State::EndOfValue,
            (State::EscapeOrEndOfString, Some('\n')) => State::EndOfValueAndEndOfLine,
            (State::EscapeOrEndOfString, Some('#')) => State::StartComment,
            (State::EscapeOrEndOfString, Some('\"')) => State::EscapedDoubleQuote,
            (State::EscapeOrEndOfString, Some('/')) => State::MayBeEscapedReturn,
            (State::EscapeOrEndOfString, Some(c)) if c.is_whitespace() => State::EndOfValue,
            (State::EscapeOrEndOfString, _) => State::Error(ErrorKind::MissingWhitespace),

            (State::MayBeEscapedReturn, Some('\"')) => State::EscapedReturn,
            (State::MayBeEscapedReturn, _) => State::Error(ErrorKind::MissingWhitespace),

            (State::StringPart, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StringPart, Some('\n')) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StringPart, Some('\"')) => State::EscapeOrEndOfString,
            (State::StringPart, Some(_)) => State::StringPart,
        }
    }
}

#[derive(Debug)]
struct Data {
    row: usize,
    col: usize,
    buf: String,
    out: Vec<Vec<WsvValue>>,
    err: Option<Error>,
}
impl Data {
    fn new() -> Data {
        Data {
            row: 1,
            col: 0,
            buf: String::new(),
            out: vec![vec![]],
            err: None,
        }
    }
    fn at_row(mut self, idx: usize) -> Self {
        self.row = idx + 1;
        self
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
                self.out
                    .last_mut()
                    .expect("initialised with one")
                    .push(WsvValue::V(self.buf.clone()));
                self.buf.clear();
            }
            Transform::AddNull => {
                self.col += 1;
                self.out
                    .last_mut()
                    .expect("initialised with one")
                    .push(WsvValue::Null);
                self.buf.clear();
            }
            Transform::IncrementColumnNumber => self.col += 1,
            Transform::AddValueAndRow => {
                self.col += 1;

                self.out
                    .last_mut()
                    .expect("initialised with one")
                    .push(WsvValue::V(self.buf.clone()));
                self.out.push(vec![]);

                self.buf.clear();
            }
            Transform::AddNullAndRow => {
                self.col += 1;

                self.out
                    .last_mut()
                    .expect("initialised with one")
                    .push(WsvValue::Null);
                self.out.push(vec![]);

                self.buf.clear();
            }
            Transform::AddRow => {
                self.col += 1;

                self.out.push(vec![])
            }
        }
    }
    fn reconcile(self) -> Result<Vec<Vec<WsvValue>>, Error> {
        match self.err {
            Some(e) => Err(dbg!(e)),
            None => Ok(self.out),
        }
    }
    fn reconcile_row(mut self) -> Result<Vec<WsvValue>, Error> {
        match self.err {
            Some(e) => Err(dbg!(e)),
            None => Ok(self.out.pop().unwrap()),
        }
    }
}

#[cfg(test)]
use crate::unit;
#[cfg(test)]
unit! {}
