use std::path::Path;

// This file contains predicates for stopping the scan process such as is_hidden
pub trait Stop: Send + Sync {
    /// Check if the scan process can be stopped
    fn stop(&self, path: &Path) -> bool;
}

/// Stop the scan process if the path is hidden
/// Add this object to scanner's stop_conditions
pub struct HiddenDirStop {}
impl Stop for HiddenDirStop {
    fn stop(&self, path: &Path) -> bool {
        path.is_dir()
            && path
                .file_name()
                .map(|name| name.to_string_lossy().starts_with('.'))
                .unwrap_or(false)
    }
}

unsafe impl Send for HiddenDirStop {}

pub struct IsFileStop;
impl Stop for IsFileStop {
    fn stop(&self, path: &Path) -> bool {
        path.is_file()
    }
}
