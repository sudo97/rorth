#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownToken {
        word: String,
        pos: usize,
        line: usize,
    },
    ParseError {
        word: String,
        pos: usize,
        line: usize,
    },
    StackEmpty {
        pos: usize,
        line: usize,
    },
}
