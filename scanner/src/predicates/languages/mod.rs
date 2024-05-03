pub mod node;
pub mod rust;
use std::path::Path;

pub trait LanguagePredicate {
    fn is_in_project(&self, path: &Path) -> bool;
}
