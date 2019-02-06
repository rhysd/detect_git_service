use std::fmt;
use std::io;

/// Error caused by APIs in detect_git_service crate.
#[derive(Debug)]
pub enum Error {
    /// Error raised when underlying `git` execution has failed.
    GitCommandFailed {
        /// Stderr output from the failed command.
        stderr: String,
        /// Args used for the command execution.
        args: Vec<String>,
    },
    /// Error raised when a shell command cannot be run as child process.
    CommandCannotRun(io::Error),
    /// Error raised when trying to parse a broken Git URL.
    BrokenUrl {
        /// A broken URL as string.
        url: String,
        /// What was broken.
        msg: String,
    },
    /// Error raised when this library could not find any Git hosting service
    /// from Git URL of the repository.
    CannotDetect {
        /// The reason why Git hosting service cannot be detected
        reason: String,
    },
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

/// Result type dedicated for detect_git_service crate.
pub type Result<T> = std::result::Result<T, Error>;
