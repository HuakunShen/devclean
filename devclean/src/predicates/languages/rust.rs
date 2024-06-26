use super::LanguagePredicate;
use crate::predicates::{Removable, Reportable, Stop};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct RustTargetPredicate;

impl RustTargetPredicate {
    /// Check whether the path is in a Rust project by checking if there is a Cargo.toml file in the parent directory
    pub fn is_in_rust_project(&self, path: &Path) -> bool {
        path.parent()
            .map_or(false, |p| p.join("Cargo.toml").is_file())
    }

    /// Check whether the path is a target directory
    pub fn is_target(&self, path: &Path) -> bool {
        path.is_dir() && path.file_name().map_or(false, |f| f == "target")
    }
}

impl LanguagePredicate for RustTargetPredicate {
    fn is_in_project(&self, path: &Path) -> bool {
        self.is_in_rust_project(path)
    }
}

impl Removable for RustTargetPredicate {
    fn is_removable(&self, path: &Path) -> bool {
        self.is_in_rust_project(path) && self.is_target(path)
    }
}

impl Stop for RustTargetPredicate {
    fn stop(&self, path: &Path) -> bool {
        self.is_in_rust_project(path) && self.is_target(path)
    }
}

impl Reportable for RustTargetPredicate {
    fn report(&self, path: &Path) -> bool {
        self.is_removable(path)
    }
}
