use std::io;

#[derive(Debug)]
pub enum Error {
    GitCommandFailed { stderr: String, args: Vec<String> },
    CommandCannotRun(io::Error),
    BrokenUrl { url: String, msg: String },
    CannotDetect { msg: String },
}

pub type Result<T> = std::result::Result<T, Error>;
