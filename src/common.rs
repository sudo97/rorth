#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownToken {
        word: String,
        pos: usize,
        line: usize,
    },
    Parse {
        word: String,
        pos: usize,
        line: usize,
        comment: String,
    },
    StackEmpty {
        pos: usize,
        line: usize,
    },
}
