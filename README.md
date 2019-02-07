`detect_git_service` crate
==========================
[![crates.io][crate-badge]][crate]
[![documentation][doc-badge]][doc]
[![CI on Linux and macOS][travis-ci-badge]][travis-ci]
[![CI on Windows][appveyor-badge]][appveyor]

[`detect_git_service`][crate] is a small crate to detect the Git hosting service from a path.

The service is detected based on a URL of remote repository of the path.

```rust
extern crate detect_git_service;

use std::path::Path;
use detect_git_service::GitService;

let path = Path::new(".");
let service = detect_git_service::detect(&path).unwrap();

assert_eq!(service.user(), "rhysd");
assert_eq!(service.repo(), "detect_git_service");
assert!(service.branch().is_some());

if let GitService::GitHub{user, repo, branch} = service {
    assert_eq!(user, "rhysd");
    assert_eq!(repo, "detect_git_service");
    assert!(branch.is_some());
}
```

Please read [the documentation][doc] for more details.



## Installation

Add [`detect_git_service`][crate] to your crate's dependencies.

```toml
[dependencies]
detect_git_service = "1"
```



## License

Distributed under [the MIT License](LICENSE).

[proj]: https://github.com/rhysd/detect_git_service
[crate]: https://crates.io/crates/detect_git_service
[crate-badge]: https://img.shields.io/crates/v/detect_git_service.svg
[doc-badge]: https://docs.rs/detect_git_service/badge.svg
[doc]: https://docs.rs/detect_git_service
[appveyor-badge]: https://ci.appveyor.com/api/projects/status/fgkdy3ufjgbrg1xy?svg=true
[appveyor]: https://ci.appveyor.com/project/rhysd/detect_git_service/branch/master
[travis-ci-badge]: https://travis-ci.org/rhysd/detect_git_service.svg?branch=master
[travis-ci]: https://travis-ci.org/rhysd/detect_git_service

