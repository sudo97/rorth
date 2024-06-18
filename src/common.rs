#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownToken {
        word: String,
        pos: usize,
        line: usize,
    },
}
