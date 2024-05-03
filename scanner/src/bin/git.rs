use std::path::PathBuf;

use scanner::predicates::general::is_git_repo_clean;

fn main() {
    // let path = PathBuf::from("/Users/hacker/Dev/research/winden");
    // let path = PathBuf::from("/Users/hacker/Dev/projects/Jarvis");
    let path = PathBuf::from("/Users/hacker/Dev/projects/dev-cleaner");
    let clean = is_git_repo_clean(path.as_path()).unwrap();
    println!("Is git repo clean: {}", clean);
}

