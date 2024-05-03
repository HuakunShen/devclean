use std::path::{Path, PathBuf};

// Git
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").is_dir()
}

// Rust
pub fn is_rust_project(path: &Path) -> bool {
    path.join("Cargo.toml").is_file()
}

pub fn is_rust_target(path: &Path) -> bool {
    is_rust_project(path) && path.join("target").is_dir()
}

// Node
pub fn is_node_project(path: &Path) -> bool {
    path.join("package.json").is_file()
}

pub fn is_node_modules(path: &Path) -> bool {
    is_node_project(path) && path.join("node_modules").is_dir()
}
