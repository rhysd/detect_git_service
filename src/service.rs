use crate::error::{Error, Result};
use crate::git::Git;
use std::path::Path;
use url::Url;

#[derive(Debug)]
pub enum GitService {
    GitHub {
        user: String,
        repo: String,
        branch: Option<String>,
    },
    GitHubEnterprise {
        user: String,
        repo: String,
        branch: Option<String>,
    },
    GitLab {
        user: String,
        repo: String,
        branch: Option<String>,
    },
    Bitbucket {
        user: String,
        repo: String,
        branch: Option<String>,
    },
}

impl GitService {
    fn from_remote_and_branch(remote_url: String, branch: Option<String>) -> Result<GitService> {
        let remote_url = remote_url.trim_right_matches(".git");
        let remote_url = Url::parse(remote_url).map_err(|e| Error::BrokenUrl {
            url: remote_url.to_string(),
            msg: format!("{}", e),
        })?;

        let host = remote_url.host_str().ok_or_else(|| Error::BrokenUrl {
            url: remote_url.to_string(),
            msg: "No host in URL".to_string(),
        })?;

        let mut path_entries = remote_url.path().split('/').filter(|s| !s.is_empty());
        let (user, repo) = match (path_entries.next(), path_entries.next()) {
            (Some(u), Some(r)) => (u.to_string(), r.to_string()),
            _ => {
                return Err(Error::CannotDetect {
                    reason: format!("Path of Git URL does not represent user/repo"),
                });
            }
        };

        match host {
            "github.com" => Ok(GitService::GitHub { user, repo, branch }),
            "gitlab.com" => Ok(GitService::GitLab { user, repo, branch }),
            "bitbucket.org" => Ok(GitService::Bitbucket { user, repo, branch }),
            host if host.starts_with("github.") => {
                Ok(GitService::GitHubEnterprise { user, repo, branch })
            }
            host if host.starts_with("gitlab.") => Ok(GitService::GitLab { user, repo, branch }),
            _ => Err(Error::CannotDetect {
                reason: format!("No service detected from URL {}", remote_url),
            }),
        }
    }

    pub fn detect<P: AsRef<Path>>(path: P) -> Result<GitService> {
        let path = path.as_ref();
        let git = Git::new(&path, None);
        let (remote_url, branch) = git.tracking_remote()?;
        GitService::from_remote_and_branch(remote_url, branch)
    }

    pub fn detect_with_git<P, S>(path: P, git_cmd: S) -> Result<GitService>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let path = path.as_ref();
        let git_cmd = git_cmd.as_ref();
        let git = Git::new(&path, Some(git_cmd));
        let (remote_url, branch) = git.tracking_remote()?;
        GitService::from_remote_and_branch(remote_url, branch)
    }
}
