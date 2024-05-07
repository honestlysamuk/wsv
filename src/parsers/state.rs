struct Automaton<State> {
    state: State,
    row: u32,
    col: u32,
    buf: String,
    input: Deque<char>,
    out: Vec<WsvValue>,
    err: Option<Error>,
}
enum State {
    Comment,
    Finished,
    Free,
    Value(char),
    EndOfValue,
    Error(ErrorKind),
    MayBeNull,
    Null,

    String(char),
    StringDelimiter,
    EscapedDoubleQuote,
    EscapedReturn,
}
struct Error {
    kind: ErrorKind,
    row: u32,
    col: u32,
}

#[derive(Error)]
enum ErrorKind {
    #[error("Cannot have hashes in values")]
    HashInValue,
    #[error("Odd number of double quotes detected")]
    OddDoubleQuotes,
    #[error("Cannot have new lines in values")]
    NewLineInValue,
    #[error("Cannot have double quotes in values")]
    DoubleQuoteInValue,
    #[error("Must have whitespace between values")]
    MissingWhitespace,
    #[error(r##"Can only escape double quotes with "" or new lines with "/""##)]
    IllFormattedEscape
}

impl Automaton {
    // we assume that the input has no \n, and that we are BufReading each line.
    fn new(input: String, row: u32) -> Automaton {
        Automaton {
            state: State::Free,
            row,
            col: 0,
            buf: String::new(),
            input: input.chars().collect(),
            output: vec![],
            err: None,
        }
    }
    fn parse_line(&mut self) -> (Result<Vec<WsvValue>>, String) {
        self.act();
        match self.err {
            Some(e) => (Err(e), self.input.collect())
            None => (self.output, self.input.collect())
        }
    }
    fn next(&mut self) -> Option<char> {
        self.pos += 1;
        input.pop_front()
    }
}

fn act(automaton: &mut Automaton, new_state: State) {
    self.state = new_state;
    self.act();
}

impl Automaton<Finished> {
    fn act(&self) {}
}

impl Automaton<Comment> {
    fn act(&mut self) {
        act(self, State::Finished);
    }
}

impl Automaton<Free> {
    fn act(&mut self) {
        match self.next() {
            Some(c) if c.is_whitespace() => act(self, State::Free),
            Some('#') => act(self, State::Comment),
            Some('-') => act(self, State::MayBeNull),
            Some('\"') => act(self, State::StartString),
            Some(c) => act(self, State::Value(c)),
            None => act(self, State::Finished),
        }
    }
}
impl Automaton<Value> {
    fn act(&mut self) {
        self.buf.push(self.state.0);
        match self.next() {
            Some('\"') => act(self, State::Error(ErrorKind::DoubleQuoteInValue)),
            Some('#') => act(self, State::Error(ErrorKind::HashInValue)),
            Some(c) if c.is_whitespace() => act(self, State::EndOfValue),
            Some(c) => act(self, State::Value(c)),
            None => act(self, State::EndOfValue),
        }
    }
}

impl Automaton<EndOfValue> {
    fn act(&mut self) {
        self.output.push(WsvValue::Value(buf));
        buf.clear();
        act(self, State::Free);
    }
}

impl Automaton<Error> {
    fn act(&mut self) {
        self.err = Some(Error {self.state.0, row: self.row, col: self.col});
        act(self, State::Finished);
    }
}
impl Automaton<MayBeNull> {
    fn act(&mut self) {
        self.buf.push('-');
        match self.next() {
            None => act(self, State::Null),
            Some(c) if c.is_whitespace() => act(self, State::Null),
            Some(c) => act(self, State::Value(c)),
        }
    }
}
impl Automaton<Null> {
    fn act(&mut self) {
        buf.clear();
        self.output.push(WsvValue::Null);
        act(self, State::Free); 
    }
}

impl Automaton<MayBeStringPart> {
    fn act(&mut self) {
        match self.next() {
            Some('\"') => act(self, State::EscapeOrEndOfString),
            Some(c) => act(self, State::StringPart(c)),
            None => act(self, State::Error(ErrorKind::OddDoubleQuotes)),
        }
    }
}
impl Automaton<StringPart> {
    fn act(&mut self) {
        self.buf.push(self.state.0);
        match self.next() {
            Some('\"') => act(self, State::EscapeOrEndOfString),
            Some(c) => act(self, State::StringPart(c)),
            None => act(self, State::Error(ErrorKind::OddDoubleQuotes)),
        }
    }
}

impl Automaton<EscapeOrEndOfString> {
    fn act(&mut self) {
        match self.next() {
            Some('\"') => act(self, State::EscapedDoubleQuote),
            Some('/') => act(self, State::MayBeEscapedReturn),
            Some(c) if c.is_whitespace() => act(self, State::EndOfValue),
            Some(c) => act(self, State::Error(ErrorKind::MissingWhitespace)),
            None =>  act(self, State::Error(ErrorKind::OddDoubleQuotes)),
        }
    }
}
impl Automaton<MayBeEscapedReturn> {
    fn act(&mut self) {
        match self.next() {
            Some('\"') => act(self, State::EscapedReturn),
            _ => act(self, State::Error(ErrorKind::IllFormattedEscapeSequence)),
        }
    }
}
impl Automaton<EscapedReturn> {
    fn act(&mut self) {
        self.buf.push('\n');
        act(self, State::MayBeStringPart);
    }
}
impl Automaton<EscapedDoubleQuote> {
    fn act(&mut self) {
        self.buf.push('\"');
        act(self, State::MayBeStringPart);
    }
}

mod tests {
    #[test]
    fn works_a_bit() -> {
        let val = Automaton::new("val".to_owned()).go();
        assert_eq!(val, vec![WsvValue::Value("val")]);
    }
}