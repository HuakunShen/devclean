use color_eyre::eyre::Result;
use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use prettytable::{Cell, Row, Table};
use scanner::predicates::general::{
    is_git_repo, is_node_modules, is_node_project, is_rust_project, is_rust_target,
};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug)]
struct ScanResult {
    path: PathBuf,
    size: u64,
    human_size: String,
    is_git_repo: bool,
    is_rust_project: bool,
    is_rust_target: bool,
    is_node_project: bool,
    is_node_modules: bool,
}

impl ScanResult {
    fn new(path: &Path) -> Self {
        let size = get_size(path).unwrap();
        Self {
            path: path.to_path_buf(),
            size,
            human_size: human_bytes(size as f64),
            is_git_repo: is_git_repo(path),
            is_rust_project: is_rust_project(path),
            is_rust_target: is_rust_target(path),
            is_node_project: is_node_project(path),
            is_node_modules: is_node_modules(path),
        }
    }

    /// Starting from this repo, stop scanning
    fn stop_scan(&self) -> bool {
        self.is_rust_target || self.is_node_modules
    }
}

#[derive(Debug)]
struct ScanResults(Vec<ScanResult>);
impl Display for ScanResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Path"),
            Cell::new("Size"),
            Cell::new("Git Repo"),
            Cell::new("Rust Project"),
            Cell::new("Rust Target"),
            Cell::new("Node Project"),
            Cell::new("Node Modules"),
        ]));

        for result in &self.0 {
            table.add_row(Row::new(vec![
                Cell::new(result.path.to_str().unwrap_or_default()),
                Cell::new(&result.human_size),
                Cell::new(&result.is_git_repo.to_string()),
                Cell::new(&result.is_rust_project.to_string()),
                Cell::new(&result.is_rust_target.to_string()),
                Cell::new(&result.is_node_project.to_string()),
                Cell::new(&result.is_node_modules.to_string()),
            ]));
        }

        write!(f, "{}", table)
    }
}

fn scan(path: &PathBuf) -> Result<ScanResults> {
    let mut results = vec![];
    for entry in WalkDir::new(path) {
        let entry: walkdir::DirEntry = entry?;
        let path = entry.path();
        // println!("{}", path.display());
        let res = ScanResult::new(path);
        // if res.stop_scan() {
        //     break;
        // }
        results.push(res);
    }

    Ok(ScanResults(results))
}

fn main() -> Result<()> {
    let target_path = PathBuf::from("/Users/hacker/Dev/magic-wormhole.rs");
    let results = scan(&target_path)?;
    println!("{}", results);
    Ok(())
}
