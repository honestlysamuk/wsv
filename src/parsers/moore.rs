/// A Moore machine is a form of state machine which is defined by six variables.
/// For educational purposes, I am going to write code in such a way as to highlight
/// those six things as clearly as possible. It is important to note that these
/// structures are a consequence of a lot of iteration on my end, and so the hard
/// part becomes linking all this to your use case. That is why in this repository
/// I have
///
/// Astute readers may notice that there are a few details which do not line up nicely
/// My favourite is the while loop. You might forgive me for using a finish state when
/// it's not defined in the spec. The more fun one is that the input list is actually
/// unbounded, since inputs.next() eventually just returns None forever. I need the states
/// to transition more times than there are inputs in order to reach the Finished state.
///
/// It is important to realise that this is, strictly speaking, the only output set possible.
/// Consider using every possible variant of the Data struct as a set, or
/// every possible set of WsvValues. Both of these are infinite sets.
use crate::data_model::*;
pub fn parse(i: &str) -> Result<Vec<Vec<WsvValue>>, Error> {
    i.split('\n').enumerate().map(parse_line).collect()
}

// we assume that line has no `\n`.
fn parse_line((row_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    // File rows are one-indexed.
    let row = row_index + 1;

    let mut inputs = line.chars();
    let mut state = State::Default;
    let end_state = State::Finished;
    let mut data = Data::new(row);

    while end_state != state {
        state = transition(state, inputs.next());
        data = data.apply(g(state));
    }

    return data.reconcile();

    // // It is more 'idiomatic' in Rust to coerce your core logic into a chain of iterators.
    // // This is for good reason. It allows the compiler more freedom to optimise. In this case,
    // // however, it is slower, and far less readable for those not familiar with the idiom.
    // (0..)
    //     .map_while(|_| {
    //         if end_state == state {
    //             None
    //         } else {
    //             state = transition(state, inputs.next());
    //             Some(g(state))
    //         }
    //     })
    //     .fold(Data::new(row), |data, o| data.apply(o))
    //     .reconcile()
}

/// Imagine for this one that I actually have a `PushChar` variant for every `char`,
/// and an `AddError` variant for every `kind`. Then, each variant represents a
/// transformation on the `Data` struct, which I then use.
#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Transform {
    PushCharA,
    PushChar(char),
    AddError(ErrorKind),
    PushDash,
    PushQuote,
    PushNewline,
    AddValue,
    AddNull,
    IncrementColumnNumber,
}

/// The state space is the easiest to spot. Of course, during development
/// I was adding and removing states quite frequently until I fixed all the
/// bugs.
#[derive(Debug, PartialEq, Hash, Clone, Copy)]
enum State {
    Default,
    Comment,
    Finished,
    MayBeNull,
    Null,
    ValueA,
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

/// Named to match with the wikipedia article. This was originally
/// a method on Data called `modify_with` which took `&State` and returned nothing.
/// Now, it returns an element of the `OutputAlphabet` set.
fn g(state: State) -> Transform {
    match state {
        State::ValueA => Transform::PushCharA,
        State::Value(c) => Transform::PushChar(c),
        State::StringPart(c) => Transform::PushChar(c),
        State::Error(kind) => Transform::AddError(kind),
        State::MayBeNull => Transform::PushDash,
        State::EscapedReturn => Transform::PushNewline,
        State::EscapedDoubleQuote => Transform::PushQuote,
        State::EndOfValue => Transform::AddValue,
        State::Comment => Transform::AddValue,
        State::Null => Transform::AddNull,
        _ => Transform::IncrementColumnNumber,
    }
}

/// Here, I am explicitly defining the InputAlphabet as Option<char>, which is the set of all
/// unicode characters plus `None`.
type InputSet = Option<char>;

/// This is your delta function. It takes the current state, the next input value,
/// and returns the new state.
fn transition(state: State, event: InputSet) -> State {
    match (state, event) {
        // Example of how there are actually five error states, not one.
        (State::Error(ErrorKind::MissingWhitespace), _) => State::Finished,
        (State::Error(ErrorKind::OddDoubleQuotes), _) => State::Finished,
        (State::Error(ErrorKind::Nom), _) => State::Finished,
        // or I could just write
        // (State::Error(_), _) => State::Finished,

        // Similarly, I could define a row for every variant of the InputAlphabet, but since
        // Rust allows for the blanket pattern matching,
        (State::Value(_), None) => State::EndOfValue,
        (State::Value(_), Some('\"')) => State::Error(ErrorKind::MissingWhitespace),
        (State::Value(_), Some('#')) => State::Comment,
        (State::Value(_), Some(c)) if c.is_whitespace() => State::EndOfValue,
        (State::Value(_), Some('A')) => State::ValueA, // Extra
        // .. et cetera. I'm only giving one example because I add eleven more
        // transition rows for every additional state.
        (State::Value(_), Some(c)) => State::Value(c),

        (State::ValueA, None) => State::EndOfValue, // Extra
        (State::ValueA, Some('\"')) => State::Error(ErrorKind::MissingWhitespace), // Extra
        (State::ValueA, Some('#')) => State::Comment, // Extra
        (State::ValueA, Some(c)) if c.is_whitespace() => State::EndOfValue, // Extra
        (State::ValueA, Some('A')) => State::ValueA, // Extra
        (State::ValueA, Some(c)) => State::Value(c), // Extra

        (State::StringPart(_), None) => State::Error(ErrorKind::OddDoubleQuotes),
        (State::StringPart(_), Some('\"')) => State::EscapeOrEndOfString,
        (State::StringPart(_), Some(c)) => State::StringPart(c),

        (State::Finished, _) => State::Finished,
        (State::Comment, _) => State::Finished,

        (State::Default, None) => State::Finished,
        (State::Default, Some('#')) => State::Finished,
        (State::Default, Some('-')) => State::MayBeNull,
        (State::Default, Some('\"')) => State::StartString,
        (State::Default, Some(c)) if c.is_whitespace() => State::Default,
        (State::Default, Some('A')) => State::ValueA, // Extra
        (State::Default, Some(c)) => State::Value(c),

        (State::EndOfValue, None) => State::Finished,
        (State::EndOfValue, Some('#')) => State::Finished,
        (State::EndOfValue, Some('-')) => State::MayBeNull,
        (State::EndOfValue, Some('\"')) => State::StartString,
        (State::EndOfValue, Some(c)) if c.is_whitespace() => State::Default,
        (State::EndOfValue, Some('A')) => State::ValueA, // Extra
        (State::EndOfValue, Some(c)) => State::Value(c),

        (State::Null, None) => State::Finished,
        (State::Null, Some('#')) => State::Finished,
        (State::Null, Some('-')) => State::MayBeNull,
        (State::Null, Some('\"')) => State::StartString,
        (State::Null, Some(c)) if c.is_whitespace() => State::Default,
        (State::Null, Some('A')) => State::ValueA, // Extra
        (State::Null, Some(c)) => State::Value(c),

        (State::MayBeNull, None) => State::Null,
        (State::MayBeNull, Some(c)) if c.is_whitespace() => State::Null,
        (State::MayBeNull, Some('\"')) => State::Error(ErrorKind::MissingWhitespace),
        (State::MayBeNull, Some('A')) => State::ValueA, // Extra
        (State::MayBeNull, Some(c)) => State::Value(c),

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
        (State::EscapeOrEndOfString, _) => State::Error(ErrorKind::MissingWhitespace),

        (State::MayBeEscapedReturn, Some('\"')) => State::EscapedReturn,
        (State::MayBeEscapedReturn, _) => State::Error(ErrorKind::MissingWhitespace),
    }
}

/// The data struct is what contains everything relevant to the features I want from my parser.
/// The state machine is just a means to an end. It is a tool I have used to produce a list of
///  simple transformations on this struct. That's it.
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
    /// This function just takes the alphabet and performs simple transformations on
    /// my data for each "letter". This is another case where enums (sum types) help to
    /// shrink the code base by parameterising our output alphabet. Thank you, sum types.
    fn apply(mut self, transform: Transform) -> Self {
        match transform {
            Transform::PushCharA => {
                self.col += 1;
                self.buf.push('A');
            }
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
