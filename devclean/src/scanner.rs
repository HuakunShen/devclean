use crate::{
    predicates::{
        languages::{
            git::GitDirtyRepoPredicate, node::NodeModulesPredicate, rust::RustTargetPredicate,
        },
        stop::{HiddenDirStop, IsFileStop, Stop},
        Reportable,
    },
    results::AnalyzeTarget,
};
use fs_extra::dir::get_size;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::DirEntry;
use std::path::Path;

/// A scanner walks through directories following the depth constraint and stop conditions
/// All valid paths are sent to the task_tx channel for further processing
pub struct Scanner {
    depth: u16,
    // task_tx: Option<Sender<AnalyzeTarget>>,
    // pub task_rx: Receiver<AnalyzeTarget>,
    /// Stop Scanning when any of the conditions are met
    stop_conditions: Vec<Box<dyn Stop>>,
    /// report to task_tx when any of the conditions are met
    report_conditions: Vec<Box<dyn Reportable>>,
    /// Whether to descend into symlinked directories (default: false, stop on symlinks)
    follow_symlinks: bool,
    pb: Option<ProgressBar>,
}

impl Scanner {
    pub fn new(
        depth: u16,
        // task_tx: Sender<AnalyzeTarget>,
        // task_rx: Receiver<AnalyzeTarget>,
        stop_conditions: Vec<Box<dyn Stop>>,
        report_conditions: Vec<Box<dyn Reportable>>,
        cmd_progress_bar: bool,
        follow_symlinks: bool,
    ) -> Self {
        let pb = if cmd_progress_bar {
            let pb = ProgressBar::new(1);
            let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}").unwrap();
            pb.set_style(spinner_style);
            Some(pb)
        } else {
            None
        };

        Scanner {
            depth,
            // task_tx: Some(task_tx),
            // task_rx,
            stop_conditions,
            report_conditions,
            follow_symlinks,
            pb,
        }
    }

    /// Whether the scanner should descend into a directory entry.
    /// Symlinked directories are only entered when `follow_symlinks` is enabled.
    fn should_enter_dir(&self, entry: &DirEntry) -> bool {
        match entry.file_type() {
            Ok(ft) if ft.is_dir() => true,
            Ok(ft) if ft.is_symlink() => self.follow_symlinks && entry.path().is_dir(),
            _ => false,
        }
    }

    pub fn _scan_recursive(&mut self, path: &Path, depth: u16) -> Vec<AnalyzeTarget> {
        if self.pb.is_some() && path.is_dir() {
            self.pb
                .as_ref()
                .unwrap()
                .set_message(format!("Scanning: {}", path.to_string_lossy()));
        }
        if depth > self.depth {
            return vec![];
        }
        let mut targets = vec![];
        for condition in &self.report_conditions {
            if condition.report(path) {
                targets.push(AnalyzeTarget::new(path.to_path_buf(), depth, None));
                return targets;
            }
        }
        for stop_condition in &self.stop_conditions {
            if stop_condition.stop(path) {
                return targets;
            }
        }
        if path.is_file() {
            return targets;
        }
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let is_symlink = entry.file_type().map(|ft| ft.is_symlink()).unwrap_or(false);
            if is_symlink && !self.follow_symlinks {
                continue;
            }
            targets.extend(self._scan_recursive(&entry.path(), depth + 1))
        }
        if depth == 0 {
            self.pb
                .as_ref()
                .unwrap()
                .finish_with_message("Scan Finished...");
        }
        targets
    }

    /// This function fills the size field of the AnalyzeTarget in parallel
    pub fn scan_recursive(&mut self, path: &Path, depth: u16) -> Vec<AnalyzeTarget> {
        // let start = std::time::Instant::now();
        let mut targets = self._scan_recursive(path, depth);
        // fill targets size field in parallel
        targets.par_iter_mut().for_each(|target| {
            target.size = Some(get_size(&target.path).unwrap_or(0));
        });
        targets
    }

    /// This is an alternative equivalent to scan_recursive
    /// Scan with rayon parallelism
    pub fn scan_parallel(&self, path: &Path, depth: u16) -> Vec<AnalyzeTarget> {
        if self.pb.is_some() && path.is_dir() {
            self.pb
                .as_ref()
                .unwrap()
                .set_message(format!("Scanning: {}", path.to_string_lossy()));
        }

        if depth > self.depth {
            return vec![];
        }

        let mut targets = vec![];
        for condition in &self.report_conditions {
            if condition.report(path) {
                targets.push(AnalyzeTarget::new(
                    path.to_path_buf(),
                    depth,
                    Some(get_size(path).unwrap_or(0)),
                ));
                return targets;
            }
        }
        for stop_condition in &self.stop_conditions {
            if stop_condition.stop(path) {
                return targets;
            }
        }

        if path.is_file() {
            return targets;
        }
        let entries: Vec<_> = path
            .read_dir()
            .unwrap()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let results: Vec<AnalyzeTarget> = entries
            .par_iter()
            .flat_map(|entry| {
                let path = entry.path();
                if self.should_enter_dir(entry) {
                    self.scan_parallel(&path, depth + 1)
                } else {
                    vec![]
                }
            })
            .collect();
        targets.extend(results);
        if depth == 0 && self.pb.is_some() {
            self.pb
                .as_ref()
                .unwrap()
                .finish_with_message("Scan Finished...");
        }

        targets
    }

    // pub fn scan(&mut self) {
    //     self.walk(&self.path.clone(), 0);
    //     drop(self.task_tx.take().unwrap());
    // }

    // pub fn scan_and_collect(&mut self) -> Vec<AnalyzeTarget> {
    //     let mut targets = vec![];
    //     self.scan();
    //     loop {
    //         match self.task_rx.recv() {
    //             Ok(target) => {
    //                 targets.push(target);
    //             }
    //             Err(_) => {
    //                 break;
    //             }
    //         }
    //     }
    //     targets
    // }

    pub fn set_depth(&mut self, depth: u16) {
        self.depth = depth;
    }
}

pub fn get_dirty_git_repo_scanner(depth: u16, pb: bool, follow_symlinks: bool) -> Scanner {
    // let (tx, rx) = mpsc::channel::<AnalyzeTarget>(); // Add type annotation for T
    let stop_conditions: Vec<Box<dyn Stop>> =
        vec![Box::new(IsFileStop {}), Box::new(HiddenDirStop {})];
    let report_conditions: Vec<Box<dyn Reportable>> = vec![Box::new(GitDirtyRepoPredicate {})];
    Scanner::new(
        // PathBuf::from(path),
        depth,
        // tx,
        // rx,
        stop_conditions,
        report_conditions,
        pb,
        follow_symlinks,
    )
}

/// Get a scanner for Rust and Node.js projects
/// pb is whether project bar should be displayed
pub fn get_project_garbage_scanner(depth: u16, pb: bool, follow_symlinks: bool) -> Scanner {
    // let (tx, rx) = mpsc::channel::<AnalyzeTarget>(); // Add type annotation for T
    let stop_conditions: Vec<Box<dyn Stop>> =
        vec![Box::new(IsFileStop {}), Box::new(HiddenDirStop {})];
    let report_conditions: Vec<Box<dyn Reportable>> = vec![
        Box::new(RustTargetPredicate {}),
        Box::new(NodeModulesPredicate {}),
    ];
    Scanner::new(
        // PathBuf::from(path),
        depth,
        // tx,
        // rx,
        stop_conditions,
        report_conditions,
        pb,
        follow_symlinks,
    )
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;

    /// root/
    ///   real/package.json
    ///   real/node_modules/
    ///   links/real-link -> ../real  (symlinked project)
    fn setup_fixture(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("devclean_test_{}_{}", std::process::id(), name));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("real/node_modules")).unwrap();
        fs::write(root.join("real/package.json"), "{}").unwrap();
        fs::create_dir_all(root.join("links")).unwrap();
        symlink(root.join("real"), root.join("links/real-link")).unwrap();
        root
    }

    fn count_targets(root: &Path, follow_symlinks: bool) -> usize {
        let scanner = get_project_garbage_scanner(10, false, follow_symlinks);
        scanner.scan_parallel(root, 0).len()
    }

    #[test]
    fn symlinked_projects_are_skipped_by_default() {
        let root = setup_fixture("skip");
        assert_eq!(count_targets(&root, false), 1);
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn symlinked_projects_are_scanned_when_follow_symlinks_enabled() {
        let root = setup_fixture("follow");
        assert_eq!(count_targets(&root, true), 2);
        fs::remove_dir_all(&root).unwrap();
    }
}
