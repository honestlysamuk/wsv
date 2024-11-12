use crate::data_model::*;
use crate::{Error, WsvValue};

fn main() {
    let inputs = vec!['c'].iter();
    let state: Box<dyn State> = Box::new(Comment(Scaffold::at_row(0)));
    let ops = vec![];

    loop {
        let next_input = inputs.next();
        state = state.transition(next_input);
        ops.push(state.output(next_input));
        if next_input.is_none() {
            break;
        }
    }
}

struct EndOfLine(Scaffold);
impl State for EndOfLine {
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

struct Finished(Scaffold);
impl State for Finished {
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
trait State {
    fn transition(self, input: Option<char>) -> Box<dyn State>;
    fn modify_with(&mut self, input: Option<char>);
}

/// The data structure which gets incrementally modified by the outputs, and produces the final parser output.
#[derive(Debug, Default)]
struct Scaffold {
    row: usize,
    col: usize,
    buf: String,
    out: Vec<Vec<WsvValue>>,
    err: Option<Error>,
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
