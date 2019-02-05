use crate::error::{Error, Result};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;
use std::process::Command;
use std::str;

pub struct Git<'a> {
    command: &'a str,
    dir: &'a Path,
}

impl<'a> Git<'a> {
    pub fn command<S: AsRef<OsStr> + Debug>(&self, args: &[S]) -> Result<String> {
        let out = Command::new(&self.command)
            .current_dir(self.dir)
            .arg("-C")
            .arg(self.dir)
            .args(args)
            .output()
            .map_err(|e| Error::CommandCannotRun(e))?;

        if out.status.success() {
            let s = str::from_utf8(&out.stdout)
                .expect("Failed to convert git command stdout from UTF8");
            Ok(s.trim().to_string())
        } else {
            let stderr = str::from_utf8(&out.stderr)
                .expect("Failed to convert git command stderr from UTF8")
                .trim()
                .to_string();
            Err(Error::GitCommandFailed {
                stderr,
                args: args
                    .iter()
                    .map(|a| a.as_ref().to_string_lossy().to_string())
                    .collect(),
            })
        }
    }

    pub fn remote_url<S: AsRef<str>>(&self, name: S) -> Result<String> {
        // XXX:
        // `git remote get-url {name}` is not available because it's added recently (at 2.6.1).
        // Note that git installed in Ubuntu 14.04 is 1.9.1.
        let mut url =
            self.command(&["config", "--get", &format!("remote.{}.url", name.as_ref())])?;

        if url.starts_with("git@") {
            // Note: Convert SSH protocol URL
            //  git@service.com:user/repo.git -> ssh://git@service.com:22/user/repo.git
            if let Some(i) = url.find(':') {
                url.insert_str(i + 1, "22/");
            }
            url.insert_str(0, "ssh://");
        }

        Ok(url)
    }

    pub fn tracking_remote(&self) -> Result<(String, Option<String>)> {
        let (url, branch) = match self.command(&["rev-parse", "--abbrev-ref", "--symbolic", "@{u}"])
        {
            Ok(stdout) => {
                // stdout is formatted as '{remote-name}/{branch-name}'
                let mut entries = stdout.splitn(2, '/');
                if let (Some(ref name), branch) = (entries.next(), entries.next()) {
                    (self.remote_url(name), branch.map(str::to_string))
                } else {
                    (self.remote_url("origin"), None)
                }
            }
            Err(_) => (self.remote_url("origin"), None),
        };
        url.map(|u| (u, branch))
    }
}

impl<'a> Git<'a> {
    pub fn new<'b: 'a, 'c: 'a, P: AsRef<Path>>(dir: &'b P, git_cmd: Option<&'c str>) -> Git<'a> {
        Git {
            command: git_cmd.unwrap_or("git"),
            dir: dir.as_ref(),
        }
    }
}
