//! A Moore machine is another form of state machine which is defined by six other variables.
//!
//! The Moore variation is characterised by having "Moore" states than the Mealy one. Enjoyably, this distinction is not so obvious when comparing the code in Rust, since we have the option to use the _ as a match against "the rest".
//! The input set is still this number plus one for the None case.

use crate::data_model::*;

pub fn parse(i: &str) -> Vec<Result<Vec<WsvValue>, Error>> {
    i.split("n").enumerate().map(parse_line).collect()
}

pub fn parse_strict(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    let input_set = i.chars().map(Some).chain(vec![None]);

    let mut partially_constructed_wsv = WsvBuilder::new();
    WsvMachine::process(input_set).for_each(|transform| {
        partially_constructed_wsv.apply(transform);
    });

    partially_constructed_wsv.finish()
}

/// Note I can also use a loop and call `.next()` on the `chars` iterator to get the same behaviour, but I wanted to make it clear that the input set is `Option<char>`, where the `None` indicates the end of a row/file. It is not `char` on its own.
pub fn parse_line((row_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let input_set = line.chars().map(Some).chain(vec![None]);

    let mut partially_constructed_wsv = WsvBuilder::new().at_row(row_index + 1);
    WsvMachine::process(input_set).for_each(|transform| {
        partially_constructed_wsv.apply(transform);
    });

    partially_constructed_wsv.finish_row()
}

/// This trait encapsulates the raw definition of a Moore Machine as closely as I can to the wikipedia entry.
///
/// There are two functions, three types, and one type constrait, making up the sextuple that is a Moore Machine. I am using the Default trait parameter on the StateSpace as the initial state requirement.Notice that the function to combine all these parts is very short. Stateless machines are simple. The hard part is working out what part of your problem fits in the StateSpace, InputAlphabet and OutputAlphabet. The functions on them are the easy bit, relatively speaking. The only difference with Mealy here is the parametrisation of the output function.
trait Moore {
    type StateSpace: Default;
    type InputAlphabet;
    type OutputAlphabet;
    fn transition(state: &Self::StateSpace, input: &Self::InputAlphabet) -> Self::StateSpace;
    fn output(state: &Self::StateSpace) -> Self::OutputAlphabet;

    // /// This is the stateful version, with in and out being vectors instead.
    // fn process_vec(input: Vec<Self::InputAlphabet>) -> Vec<Self::OutputAlphabet> {
    //     let mut state = Self::StateSpace::default();
    //     let mut output = vec![];
    //     for i in input {
    //         state = Self::transition(&state, &i);
    //         output.push(Self::output(&state));
    //     }
    //     output
    // }

    /// This is is the stateless version, and could even be implemented as a dedicated iterator adapter. We could also use `IntoIterator` instead of the `Iterator` trait to be more flexible.
    fn process(
        input: impl Iterator<Item = Self::InputAlphabet>,
    ) -> impl Iterator<Item = Self::OutputAlphabet> {
        input.scan(Self::StateSpace::default(), |state, i| {
            *state = Self::transition(state, &i);
            Some(Self::output(state))
        })
    }
}

/*
An identifying feature of a state machine is the number of states used in the computation. It looks at first glance that this one only has 18 states, but enums are sum types. This means that the size (cardinality) of the enum is actually the sum of cardinalities of each variant. `State::Comment` is a unit variant and so has a cardinality of 1. Error has a cardinality equal to the cardinality of `ErrorKind`, which is 2. `StringPart` and `Value` have a cardinality equal to `char`. Since `char` can be any valid unicode symbol and there are 149,813 of those as of [Unicode 15.1.0](https://www.unicode.org/versions/Unicode15.1.0/), the total number of states is

> (18 - 3) + 2 + 149,813 + 149,813
> = 299,643

*/

#[derive(Debug, PartialEq, Hash, Clone, Copy, Default)]
enum State {
    Comment,
    #[default]
    Default,
    EndOfLine,
    EndOfValue,
    EndOfValueAndEndOfLine,
    Error(ErrorKind),
    EscapeOrEndOfString,
    EscapedReturn,
    EscapedDoubleQuote,
    Finished,
    MayBeEscapedReturn,
    MayBeNull,
    Null,
    NullEndOfLine,
    StartComment,
    StartString,
    StringPart(char),
    Value(char),
}

/*

# The Output Alphabet

This is my output alphabet. If you compare this whole implementation to the `State` implementation, you will see that this doesn't exist. This enum is an additional indirection to highlight exactly which part of the logic is controlled by the state machine. Each variant represents a transformation, which my WsvBuilder knows how to interpret. Remember, functions and data are symmetrical.

You may ask why the output set is not something else, such as a WsvValue directly. Go back to the definition of a state machine. The Output set is defined as *finite*. WsvValue has a variant `Value` which contains a `String`. String is an infinite set, and since enums are sum types, that makes the WsvValue have cardinality ∞ + 1 = ∞. This enum's cardinality, however, is the sum of the cardinalities of ErrorKind, char and 9 (There are 11 variants minus AddError and PushChar). `char` is a unicode character and there are 149_813 of those as of [Unicode 15.1.0](https://www.unicode.org/versions/Unicode15.1.0/), so Transform has a cardinality of 2 + 9 + 149_813 = 149_824. That is not infinite, and so is a valid Output Set.


*/

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Transform {
    AddError(ErrorKind),
    AddNull,
    AddNullAndRow,
    AddRow,
    AddValue,
    AddValueAndRow,
    IncrementColumnNumber,
    PushChar(char),
    PushDash,
    PushNewline,
    PushQuote,
}
/*
For this use case, I have no need to run arbitrary code between state transitions. I only want the complete list of outputs. Therefore, my machine does not need to hold any data. I could add a `state: State` attribute, and a function in the trait to interact with that attribute, then write a function which lets me encapsulate the logic of iterating through one transition at a time, but that is not useful for WSV here.
*/
struct WsvMachine {}

impl Moore for WsvMachine {
    type StateSpace = State;
    type InputAlphabet = Option<char>;
    type OutputAlphabet = Transform;

    /// The main difference between Moore and Mealy can be seen here. This function takes only one parameter, not two. Notice how Rust allows us to keep the expression simple using pattern matching. Otherwise, we would be required to define every state and transform explicitly, leading to statements like
    ///
    /// State::ValueA => Transform::PushCharA,
    /// State::Valuea => Transform::PushChara,
    /// State::ValueB => Transform::PushCharB,
    /// State::Valueb => Transform::PushCharb,
    /// and so on.
    ///
    fn output(state: &Self::StateSpace) -> Self::OutputAlphabet {
        match state {
            State::Value(c) => Transform::PushChar(*c),
            State::StringPart(c) => Transform::PushChar(*c),
            State::Error(kind) => Transform::AddError(*kind),
            State::MayBeNull => Transform::PushDash,
            State::EscapedReturn => Transform::PushNewline,
            State::EscapedDoubleQuote => Transform::PushQuote,
            State::EndOfValue => Transform::AddValue,
            State::StartComment => Transform::AddValue,
            State::Null => Transform::AddNull,
            State::EndOfLine => Transform::AddRow,
            State::NullEndOfLine => Transform::AddNullAndRow,
            State::EndOfValueAndEndOfLine => Transform::AddValueAndRow,
            _ => Transform::IncrementColumnNumber,
        }
    }
    /// The wiki article defines this with the delta symbol, δ. The only change between this function and the Mealy function is the addition of `(_)` after `State::Value` and `State::StringPart`.
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
            (State::EndOfValueAndEndOfLine, Some(c)) => State::Value(*c),
            (State::NullEndOfLine, None) => State::Finished,
            (State::NullEndOfLine, Some('\n')) => State::EndOfLine,
            (State::NullEndOfLine, Some('#')) => State::Comment,
            (State::NullEndOfLine, Some('-')) => State::MayBeNull,
            (State::NullEndOfLine, Some('\"')) => State::StartString,
            (State::NullEndOfLine, Some(c)) if c.is_whitespace() => State::Default,
            (State::NullEndOfLine, Some(c)) => State::Value(*c),
            (State::EndOfLine, None) => State::Finished,
            (State::EndOfLine, Some('\n')) => State::EndOfLine,
            (State::EndOfLine, Some('#')) => State::Comment,
            (State::EndOfLine, Some('-')) => State::MayBeNull,
            (State::EndOfLine, Some('\"')) => State::StartString,
            (State::EndOfLine, Some(c)) if c.is_whitespace() => State::Default,
            (State::EndOfLine, Some(c)) => State::Value(*c),
            (State::Default, None) => State::Finished,
            (State::Default, Some('\n')) => State::EndOfLine,
            (State::Default, Some('#')) => State::Comment,
            (State::Default, Some('-')) => State::MayBeNull,
            (State::Default, Some('\"')) => State::StartString,
            (State::Default, Some(c)) if c.is_whitespace() => State::Default,
            (State::Default, Some(c)) => State::Value(*c),
            (State::EndOfValue, None) => State::Finished,
            (State::EndOfValue, Some('\n')) => State::EndOfLine,
            (State::EndOfValue, Some('#')) => State::Comment,
            (State::EndOfValue, Some('-')) => State::MayBeNull,
            (State::EndOfValue, Some('\"')) => State::StartString,
            (State::EndOfValue, Some(c)) if c.is_whitespace() => State::Default,
            (State::EndOfValue, Some(c)) => State::Value(*c),
            (State::Null, None) => State::Finished,
            (State::Null, Some('\n')) => State::EndOfLine,
            (State::Null, Some('#')) => State::EndOfLine,
            (State::Null, Some('-')) => State::MayBeNull,
            (State::Null, Some('\"')) => State::StartString,
            (State::Null, Some(c)) if c.is_whitespace() => State::Default,
            (State::Null, Some(c)) => State::Value(*c),
            (State::MayBeNull, None) => State::Null,
            (State::MayBeNull, Some('\n')) => State::NullEndOfLine,
            (State::MayBeNull, Some(c)) if c.is_whitespace() => State::Null,
            (State::MayBeNull, Some('\"')) => State::Error(ErrorKind::MissingWhitespace),
            (State::MayBeNull, Some(c)) => State::Value(*c),
            (State::Value(_), None) => State::EndOfValue,
            (State::Value(_), Some('\n')) => State::EndOfValueAndEndOfLine,
            (State::Value(_), Some('\"')) => State::Error(ErrorKind::MissingWhitespace),
            (State::Value(_), Some('#')) => State::StartComment,
            (State::Value(_), Some(c)) if c.is_whitespace() => State::EndOfValue,
            (State::Value(_), Some(c)) => State::Value(*c),
            (State::StartString, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StartString, Some('\n')) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StartString, Some('\"')) => State::EscapeOrEndOfString,
            (State::StartString, Some(c)) => State::StringPart(*c),
            (State::EscapedReturn, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedReturn, Some('\n')) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedReturn, Some('\"')) => State::EscapeOrEndOfString,
            (State::EscapedReturn, Some(c)) => State::StringPart(*c),
            (State::EscapedDoubleQuote, None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedDoubleQuote, Some('\n')) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::EscapedDoubleQuote, Some('\"')) => State::EscapeOrEndOfString,
            (State::EscapedDoubleQuote, Some(c)) => State::StringPart(*c),
            (State::EscapeOrEndOfString, None) => State::EndOfValue,
            (State::EscapeOrEndOfString, Some('\n')) => State::EndOfValueAndEndOfLine,
            (State::EscapeOrEndOfString, Some('#')) => State::StartComment,
            (State::EscapeOrEndOfString, Some('\"')) => State::EscapedDoubleQuote,
            (State::EscapeOrEndOfString, Some('/')) => State::MayBeEscapedReturn,
            (State::EscapeOrEndOfString, Some(c)) if c.is_whitespace() => State::EndOfValue,
            (State::EscapeOrEndOfString, _) => State::Error(ErrorKind::MissingWhitespace),
            (State::MayBeEscapedReturn, Some('\"')) => State::EscapedReturn,
            (State::MayBeEscapedReturn, _) => State::Error(ErrorKind::MissingWhitespace),
            (State::StringPart(_), None) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StringPart(_), Some('\n')) => State::Error(ErrorKind::OddDoubleQuotes),
            (State::StringPart(_), Some('\"')) => State::EscapeOrEndOfString,
            (State::StringPart(_), Some(c)) => State::StringPart(*c),
        }
    }
}

/*
The WsvBuilder is the data structure which gets incrementally modified by the transforms, and produces the final parser output. There is no difference between this Builder and the one in the Mealy variant. However, I have kept each implementation with their own definitions to simplify maintenance.

*/
#[derive(Debug)]
struct WsvBuilder {
    row: usize,
    col: usize,
    buf: String,
    out: Vec<Vec<WsvValue>>,
    err: Option<Error>,
}
impl WsvBuilder {
    fn new() -> WsvBuilder {
        WsvBuilder {
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
    fn finish(self) -> Result<Vec<Vec<WsvValue>>, Error> {
        match self.err {
            Some(e) => Err(dbg!(e)),
            None => Ok(self.out),
        }
    }
    fn finish_row(mut self) -> Result<Vec<WsvValue>, Error> {
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
