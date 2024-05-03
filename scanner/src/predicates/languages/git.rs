use super::LanguagePredicate;
use crate::predicates::{general::is_git_repo_clean, Reportable};
use color_eyre::eyre::Result;
use git2::{Repository, StatusOptions};
use std::path::Path;

pub struct GitDirtyRepoPredicate;
impl GitDirtyRepoPredicate {
    fn is_git_repo(&self, path: &Path) -> bool {
        Repository::open(path).is_ok()
    }

    /// no files to be commited
    fn is_git_repo_clean(&self, path: &Path) -> Result<bool> {
        let repo = Repository::open(path)?;
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        let statuses = repo.statuses(Some(&mut status_opts))?;
        Ok(statuses.is_empty())
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
        Repository::open(path).is_ok()
    }
}
