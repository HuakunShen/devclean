use git2::{Repository, StatusOptions};
use scanner::{predicates::general::is_git_repo_clean, scanner::get_dirty_git_repo_scanner};
use std::path::{Path, PathBuf};

fn main() {
    let p = PathBuf::from("/Users/hacker/Dev/");
    let mut scanner = get_dirty_git_repo_scanner(p.as_path(), 5, true);
    scanner.scan();
    while let Ok(target) = scanner.task_rx.recv() {
        println!("{:#?}", target);
    }
}
