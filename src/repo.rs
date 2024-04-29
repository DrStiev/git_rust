use configparser::ini::Ini;
use std::fs::{create_dir, metadata, read_dir, File};
use std::io::Write;
use std::path::Path;

pub struct GitRepository {
    pub worktree: &'static Path,
    pub gitdir: &'static Path,
    pub conf: Ini,
}

// FIXME I'M BAD AS SHIT!
fn repo_path(path: &[&str], gitdir: &'static Path) -> &'static Path {
    &Path::new(gitdir).join(path.join("/"))
}

fn repo_file(path: &[&str], gitdir: &'static Path, mkdir: bool) -> Option<&'static Path> {
    if repo_dir(path, gitdir, mkdir).is_some() {
        return Some(repo_path(path, gitdir));
    }
    None
}

fn repo_dir(path: &[&str], gitdir: &'static Path, mkdir: bool) -> Option<&'static Path> {
    let path = repo_path(path, gitdir);
    if Path::new(path).exists() {
        if Path::new(path).is_dir() {
            return Some(path);
        }
        panic!("Not a directory {}", path.to_str().unwrap());
    }

    if mkdir {
        create_dir(path);
        return Some(path);
    } else {
        None
    }
}

fn repo_default_config() -> Ini {
    let mut ret = Ini::new();

    ret.set("core", "repositoryformatversion", Some("0".to_owned()));
    ret.set("core", "filemode", Some("false".to_owned()));
    ret.set("core", "bare", Some("false".to_owned()));

    ret
}

impl GitRepository {
    fn new(path: &'static str, force: bool) -> Self {
        let worktree = Path::new(path);
        let gitdir: &Path = &Path::new(path).join(".git");

        if !(force || metadata(gitdir).unwrap().is_dir()) {
            panic!("Not a Git repository {}", path);
        }

        let mut conf = Ini::new();
        let cf = repo_file(&["config"], gitdir, false).unwrap().to_str();

        if cf.is_some() && Path::new(cf.unwrap()).exists() {
            conf.read(cf.unwrap().to_owned());
        } else if !force {
            panic!("Configuration file missing");
        }

        if !force {
            let vers = conf
                .get("core", "repositoryformatversion")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            if vers != 0 {
                panic!("Unsupported repositoryformatversion {}", vers);
            }
        }

        Self {
            worktree,
            gitdir,
            conf,
        }
    }

    pub fn repo_create(path: &'static str) -> Self {
        let repo = GitRepository::new(path, true);
        if Path::new(repo.worktree).exists() {
            if !Path::new(repo.worktree).is_dir() {
                panic!("{} is not a directory!", path);
            }
            if Path::new(repo.gitdir).exists() {
                match read_dir(repo.gitdir) {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(entry) => {
                                    panic!("{} is not empty!", repo.gitdir.to_str().unwrap())
                                }
                                Err(e) => panic!("Error: {}", e),
                            }
                        }
                    }
                    Err(e) => panic!("Error: {}", e),
                }
            }
        } else {
            create_dir(path);
        }

        repo_dir(&["branches"], repo.gitdir, true);
        repo_dir(&["objects"], repo.gitdir, true);
        repo_dir(&["refs", "tags"], repo.gitdir, true);
        repo_dir(&["refs", "heads"], repo.gitdir, true);

        // .git/description
        let mut file = File::open(
            repo_file(&["description"], repo.gitdir, false)
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .unwrap();
        file.write_all(b"Unnamed repository; edit this file 'desription' to name the repository\n");

        // .git/HEAD
        let mut file = File::open(
            repo_file(&["HEAD"], repo.gitdir, false)
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .unwrap();
        file.write_all(b"ref: refs/heads/master\n");

        let mut config = repo_default_config();
        config.write(repo_file(&["config"], repo.gitdir, false).unwrap());

        repo
    }

    // Rust do not handle recursion very well.
    // It's always better to make something iterative
    pub fn repo_find(path: &'static &str, required: bool) -> Option<Self> {
        let mut path = Path::new(path);

        loop {
            if path.join(".git").is_dir() {
                return Some(GitRepository::new(path.to_str().unwrap(), false));
            }

            // recurse in parent
            let parent = Path::new(&path.join(".."));

            if parent == path {
                // path is root
                if required {
                    panic!("No git direcctory");
                }
                return None;
            }

            path = parent;
        }
    }
}
