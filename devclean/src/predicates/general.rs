use color_eyre::eyre::Result;
// use git2::Repository;
use std::path::Path;

// Git
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").is_dir()
}

/// Check if the git repo has any uncommitted changes with "git status --porcelain"
pub fn is_git_repo_clean(path: &Path) -> Result<bool> {
    let output = std::process::Command::new("git")
        .current_dir(path)
        .args(&["status", "--porcelain"])
        .output()
        .expect("failed to execute process");
    let output_str =
        std::str::from_utf8(&output.stdout).expect("Failed to convert bytes to string");
    let line_count = output_str
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .count();
    Ok(line_count == 0)
}

pub fn is_dir_empty(path: &Path) -> bool {
    path.read_dir().map_or(true, |mut dir| dir.next().is_none())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_repo_clean() {
        let x = is_git_repo_clean(Path::new("/Users/hacker/Dev/projects/tauri-demo"));
        // let x = is_git_repo_clean(Path::new("/Users/hacker/Dev/projects/Nowtu"));
        println!("{:?}", x);
        // assert!(is_git_repo_clean(Path::new("tests/fixtures/git")));
        // assert!(!is_git_repo_clean(Path::new("tests/fixtures/git_dirty")));
    }
}
