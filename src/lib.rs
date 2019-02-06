//! Detect Git hosting service from file path
//!
//! This library provides APIs to detect Git hosting service used for given
//! file path. Service is detected based on a URL of remote repository of the
//! path.
//!
//! ```
//! use std::path::Path;
//! use detect_git_service::GitService;
//!
//! let path = Path::new(".");
//! let service = detect_git_service::detect(&path).unwrap();
//!
//! assert_eq!(service.user(), "rhysd");
//! assert_eq!(service.repo(), "detect_git_service");
//! assert!(service.branch().is_some());
//!
//! if let GitService::GitHub{user, repo, branch} = service {
//!     assert_eq!(user, "rhysd");
//!     assert_eq!(repo, "detect_git_service");
//!     assert!(branch.is_some());
//! }
//! ```

#![deny(missing_docs)]

extern crate diff_enum;
extern crate url;

mod error;
mod git;
mod service;

pub use crate::error::Error;
pub use crate::service::{detect, detect_with_git, GitService};
