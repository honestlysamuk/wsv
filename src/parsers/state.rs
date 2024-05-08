// we assume that the input has no \n, and that we are BufReading each line.
fn run<I>(mut inputs: I)
where
    I: Iterator<Item = char>,
{
    let mut state = dbg!(State::Default);
    let mut data = Data::for_row(1);
    loop {
        state = state.transition(dbg!(inputs.next()));
        data.modify_with(&state);
        if [State::Finished].contains(dbg!(&state)) {
            break;
        }
    }
    println!("OUT");
    println!("{:?}", data.reconcile())
}
#[derive(Debug)]
struct Data {
    row: u32,
    col: u32,
    buf: String,
    out: Vec<WsvValue>,
    err: Option<Error>,
}

#[derive(Debug)]
enum WsvValue {
    V(String),
    Null,
}

#[derive(Debug)]
struct Error {
    kind: ErrorKind,
    row: u32,
    col: u32,
}
///#[derive(Error)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum ErrorKind {
    //#[error("Odd number of double quotes detected")]
    OddDoubleQuotes,
    //#[error("Cannot have double quotes in values")]
    NoLeadingWhitespace,
    //#[error("Must have whitespace between values")]
    NoTrailingWhitespace,
}
impl Data {
    fn for_row(row: u32) -> Data {
        Data {
            row,
            col: 0,
            buf: String::new(),
            out: vec![],
            err: None,
        }
    }
    fn add_error(&mut self, kind: ErrorKind) {
        self.err = Some(Error {
            kind,
            row: self.row,
            col: self.col,
        });
    }
    fn modify_with(&mut self, state: &State) {
        self.col += 1;
        match state {
            State::Value(c) => self.buf.push(*c),
            State::StringPart(c) => self.buf.push(*c),
            State::EscapedReturn => self.buf.push('\n'),
            State::EscapedDoubleQuote => self.buf.push('\"'),
            State::MayBeNull => self.buf.push('-'),
            State::Error(kind) => self.add_error(*kind),
            State::EndOfValue => {
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
            Some(e) => Err(e),
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
            (State::Default, Some('-')) => State::MayBeNull,
            (State::Default, Some('#')) => State::Comment,
            (State::Default, Some('\"')) => State::StartString,
            (State::Default, Some(c)) if c.is_whitespace() => State::Default,
            (State::Default, Some(c)) => State::Value(c),

            (State::EndOfValue, None) => State::Finished,
            (State::EndOfValue, Some('-')) => State::MayBeNull,
            (State::EndOfValue, Some('#')) => State::Comment,
            (State::EndOfValue, Some('\"')) => State::StartString,
            (State::EndOfValue, Some(c)) if c.is_whitespace() => State::Default,
            (State::EndOfValue, Some(c)) => State::Value(c),

            (State::Null, None) => State::Finished,
            (State::Null, Some('-')) => State::MayBeNull,
            (State::Null, Some('#')) => State::Comment,
            (State::Null, Some('\"')) => State::StartString,
            (State::Null, Some(c)) if c.is_whitespace() => State::Default,
            (State::Null, Some(c)) => State::Value(c),

            (State::MayBeNull, None) => State::Null,
            (State::MayBeNull, Some('\"')) => State::Error(ErrorKind::NoLeadingWhitespace),
            (State::MayBeNull, Some(c)) if c.is_whitespace() => State::Null,
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
