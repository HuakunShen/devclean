pub mod node;
pub mod rust;
pub mod git;
use std::path::Path;

pub trait LanguagePredicate {
    fn is_in_project(&self, path: &Path) -> bool;
}
