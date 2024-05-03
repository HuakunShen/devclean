use super::LanguagePredicate;
use crate::predicates::{Removable, Reportable, Stop};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct NodeModulesPredicate;

impl NodeModulesPredicate {
    fn is_in_node_project(&self, path: &Path) -> bool {
        path.parent()
            .map_or(false, |p| p.join("package.json").is_file())
    }

    fn is_node_modules(&self, path: &Path) -> bool {
        path.is_dir() && path.file_name().map_or(false, |f| f == "node_modules")
    }
}

impl LanguagePredicate for NodeModulesPredicate {
    /// Check whether the path is in a Node project by checking if there is a package.json file in the parent directory
    fn is_in_project(&self, path: &Path) -> bool {
        self.is_in_node_project(path)
    }
}

impl Removable for NodeModulesPredicate {
    /// Check whether the path is a node_modules directory in a Node project
    fn is_removable(&self, path: &Path) -> bool {
        self.is_in_project(path) && self.is_node_modules(path)
    }
}

impl Stop for NodeModulesPredicate {
    fn stop(&self, path: &Path) -> bool {
        self.is_in_project(path) && self.is_node_modules(path)
    }
}

impl Reportable for NodeModulesPredicate {
    fn report(&self, path: &Path) -> bool {
        self.is_removable(path)
    }
}
