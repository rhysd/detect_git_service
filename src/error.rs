use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    GitCommandFailed { stderr: String, args: Vec<String> },
    CommandCannotRun(io::Error),
    BrokenUrl { url: String, msg: String },
    CannotDetect { reason: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::GitCommandFailed { stderr, args } => {
                if stderr.is_empty() {
                    write!(f, "`git")?;
                } else {
                    write!(f, "{}: `git", stderr)?;
                }
                for arg in args.iter() {
                    write!(f, " '{}'", arg)?;
                }
                write!(f, "` exited with non-zero status")
            }
            Error::CommandCannotRun(err) => write!(f, "{}: cannot run command", err),
            Error::BrokenUrl { url, msg } => write!(f, "Git URL {} is broken: {}", url, msg),
            Error::CannotDetect { reason } => write!(f, "Cannot detect service: {}", reason),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
