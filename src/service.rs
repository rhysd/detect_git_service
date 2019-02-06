use crate::error::{Error, Result};
use crate::git::Git;
use std::path::Path;
use url::{Host, Url};

/// Enum variants of Git hosting services which this library supports.
#[diff_enum::common_fields{
    /// User name in the service
    user: String,
    /// Repository name in the service
    repo: String,
    /// Current branch name if available
    branch: Option<String>,
}]
#[derive(Debug)]
pub enum GitService {
    /// GitHub http://github.com
    GitHub,
    /// GitHub Enterprise https://github.com/enterprise
    GitHubEnterprise,
    /// GitLab https://gitlab.com/
    GitLab,
    /// Bitbucket https://bitbucket.org/
    Bitbucket,
}

fn detect_with_remote_and_branch(remote_url: String, branch: Option<String>) -> Result<GitService> {
    let remote_url = remote_url.trim_right_matches(".git");
    let remote_url = Url::parse(remote_url).map_err(|e| Error::BrokenUrl {
        url: remote_url.to_string(),
        msg: format!("{}", e),
    })?;

    let host = match remote_url.host() {
        Some(Host::Domain(h)) => h,
        Some(_) => {
            return Err(Error::CannotDetect {
                reason: format!("Domain name must be contained in URL {}", remote_url),
            });
        }
        None => {
            return Err(Error::BrokenUrl {
                url: remote_url.to_string(),
                msg: "No host in URL".to_string(),
            });
        }
    };

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

/// Detect Git hosting service from a file path. Path can be both file path
/// and directory path. It returns an error when input was invalid or no service
/// was detected.
pub fn detect<P: AsRef<Path>>(path: P) -> Result<GitService> {
    let path = path.as_ref();
    let git = Git::new(&path, None);
    let (remote_url, branch) = git.tracking_remote()?;
    detect_with_remote_and_branch(remote_url, branch)
}

/// Almost the same as `detect`, but with explicitly specifying Git command.
pub fn detect_with_git<P, S>(path: P, git_cmd: S) -> Result<GitService>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let path = path.as_ref();
    let git_cmd = git_cmd.as_ref();
    let git = Git::new(&path, Some(git_cmd));
    let (remote_url, branch) = git.tracking_remote()?;
    detect_with_remote_and_branch(remote_url, branch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_this_repo() {
        let p = Path::new(".");
        let service = detect(&p).unwrap();
        match service {
            GitService::GitHub {
                ref user, ref repo, ..
            } => {
                assert_eq!(user, "rhysd");
                assert_eq!(repo, "detect_git_service");
            }
            _ => assert!(false, "unexpected service: {:?}", service),
        }
        assert_eq!(service.user(), "rhysd");
        assert_eq!(service.repo(), "detect_git_service");
    }

    #[test]
    fn detect_this_repo_from_file_path() {
        let p = Path::new(".").join("LICENSE");
        let service = detect(&p).unwrap();
        match service {
            GitService::GitHub {
                ref user, ref repo, ..
            } => {
                assert_eq!(user, "rhysd");
                assert_eq!(repo, "detect_git_service");
            }
            _ => assert!(false, "unexpected service: {:?}", service),
        }
        assert_eq!(service.user(), "rhysd");
        assert_eq!(service.repo(), "detect_git_service");
    }

    #[test]
    fn detect_this_repo_with_git() {
        let p = Path::new(".");
        let service = detect_with_git(&p, "git").unwrap();
        match service {
            GitService::GitHub {
                ref user, ref repo, ..
            } => {
                assert_eq!(user, "rhysd");
                assert_eq!(repo, "detect_git_service");
            }
            _ => assert!(false, "unexpected service: {:?}", service),
        }
        assert_eq!(service.user(), "rhysd");
        assert_eq!(service.repo(), "detect_git_service");
    }

    macro_rules! test_case_ok {
        ($test_case:ident, $url:expr, $service:ident, $user:expr, $repo:expr) => {
            #[test]
            fn $test_case() {
                let service = detect_with_remote_and_branch($url.to_string(), None).unwrap();
                if let GitService::$service { user, repo, branch } = service {
                    assert_eq!(branch, None);
                    assert_eq!(user, $user.to_string());
                    assert_eq!(repo, $repo.to_string());
                } else {
                    assert!(false, "unexpected service: {:?}", service);
                }
            }
        };
        ($test_case:ident, $url:expr, $service:ident, $user:expr, $repo:expr,) => {
            test_case_ok!($test_case, $url, $service, $user, $repo);
        };
    }

    test_case_ok!(
        github_https,
        "https://github.com/rhysd/detect_git_service",
        GitHub,
        "rhysd",
        "detect_git_service",
    );

    test_case_ok!(
        github_https_with_git_ext,
        "https://github.com/rhysd/detect_git_service.git",
        GitHub,
        "rhysd",
        "detect_git_service",
    );

    test_case_ok!(
        github_ssh,
        "ssh://git@github.com:22/rhysd/detect_git_service.git",
        GitHub,
        "rhysd",
        "detect_git_service",
    );

    test_case_ok!(
        github_enterprise,
        "https://github.mycompany.com/rhysd/detect_git_service.git",
        GitHubEnterprise,
        "rhysd",
        "detect_git_service",
    );

    test_case_ok!(
        gitlab_https,
        "https://gitlab.com/Linda_pp/detect_git_service",
        GitLab,
        "Linda_pp",
        "detect_git_service",
    );

    test_case_ok!(
        gitlab_https_with_git_ext,
        "https://gitlab.com/Linda_pp/detect_git_service.git",
        GitLab,
        "Linda_pp",
        "detect_git_service",
    );

    test_case_ok!(
        gitlab_ssh,
        "ssh://git@gitlab.com:22/Linda_pp/detect_git_service.git",
        GitLab,
        "Linda_pp",
        "detect_git_service",
    );

    test_case_ok!(
        gitlab_local,
        "https://gitlab.myinstance.net/Linda_pp/detect_git_service",
        GitLab,
        "Linda_pp",
        "detect_git_service",
    );

    test_case_ok!(
        bitbucket_https,
        "https://bitbucket.org/rhysd/detect_git_service",
        Bitbucket,
        "rhysd",
        "detect_git_service",
    );

    test_case_ok!(
        bitbucket_https_with_git_ext,
        "https://rhysd@bitbucket.org/rhysd/detect_git_service.git",
        Bitbucket,
        "rhysd",
        "detect_git_service",
    );

    test_case_ok!(
        bitbucket_ssh,
        "ssh://git@bitbucket.org:22/rhysd/detect_git_service.git",
        Bitbucket,
        "rhysd",
        "detect_git_service",
    );

    macro_rules! test_case_error {
        ($test_case:ident, $url:expr, $err:ident, $expected:expr) => {
            #[test]
            fn $test_case() {
                let err = detect_with_remote_and_branch($url.to_string(), None).unwrap_err();
                assert!(
                    format!("{}", err).contains($expected),
                    "unexpected error message: {}",
                    err
                );
            }
        };
        ($test_case:ident, $url:expr, $err:ident, $expected:expr,) => {
            test_case_error!($test_case, $url, $err, $expected);
        };
    }

    test_case_error!(
        broken_url,
        "https://",
        BrokenUrl,
        "Git URL https:// is broken"
    );

    test_case_error!(no_host, "foo:/foo", BrokenUrl, "No host in URL");

    test_case_error!(
        no_path,
        "https://github.com",
        CannotDetect,
        "Path of Git URL does not represent user/repo",
    );

    test_case_error!(
        no_repo,
        "https://github.com/foo",
        CannotDetect,
        "Path of Git URL does not represent user/repo",
    );

    test_case_error!(
        no_domain_name,
        "https://1.2.3.4/foo/bar",
        CannotDetect,
        "Domain name must be contained in URL https://1.2.3.4/foo/bar",
    );

    test_case_error!(
        unknown_service,
        "https://my.awesome.service.example.com/foo/bar",
        CannotDetect,
        "No service detected from URL https://my.awesome.service.example.com/foo/bar",
    );
} // mod tests
