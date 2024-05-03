use color_eyre::eyre::Result;
use git2::Repository;
use std::path::{Path, PathBuf};

// Git
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").is_dir()
}

pub fn is_git_repo_clean(path: &Path) -> Result<bool> {
    let repo = Repository::open(path)?;
    let statuses = repo.statuses(None)?;
    Ok(statuses.is_empty())
}
