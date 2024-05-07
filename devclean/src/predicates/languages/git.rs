use super::LanguagePredicate;
use crate::predicates::{
    general::{is_git_repo, is_git_repo_clean},
    Reportable,
};
use color_eyre::eyre::Result;
// use git2::{Repository, StatusOptions};
use std::path::Path;

pub struct GitDirtyRepoPredicate;
impl GitDirtyRepoPredicate {
    fn is_git_repo(&self, path: &Path) -> bool {
        // Repository::open(path).is_ok()
        is_git_repo(path)
    }

    /// no files to be commited
    fn is_git_repo_clean(&self, path: &Path) -> Result<bool> {
        is_git_repo_clean(path)
        // let repo = Repository::open(path)?;
        // let mut status_opts = StatusOptions::new();
        // status_opts.include_untracked(true);
        // let statuses = repo.statuses(Some(&mut status_opts))?;
        // Ok(statuses.is_empty())
    }
}

impl Reportable for GitDirtyRepoPredicate {
    fn report(&self, path: &std::path::Path) -> bool {
        // the default value is true here because there is a negate operator, making the final default value false
        self.is_git_repo(path) && !(self.is_git_repo_clean(path).unwrap_or(true))
    }
}

impl LanguagePredicate for GitDirtyRepoPredicate {
    fn is_in_project(&self, path: &std::path::Path) -> bool {
        // iterate over the parent directories to find the .git directory
        let mut current_path = path.to_path_buf();
        loop {
            if current_path.join(".git").is_dir() {
                return true;
            }
            if !current_path.pop() {
                break;
            }
        }
        false

        // Repository::open(path).is_ok()
    }
}
