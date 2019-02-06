extern crate diff_enum;
extern crate url;

mod error;
mod git;
mod service;

pub use crate::error::Error;
pub use crate::service::{detect, detect_with_git, GitService};
