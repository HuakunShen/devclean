use crate::{
    predicates::{
        languages::{
            git::GitDirtyRepoPredicate, node::NodeModulesPredicate, rust::RustTargetPredicate,
        },
        stop::{HiddenDirStop, IsFileStop, Stop},
        Removable, Reportable,
    },
    results::AnalyzeTarget,
};
use fs_extra::dir::get_size;
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, Sender},
};

/// A scanner walks through directories following the depth constraint and stop conditions
/// All valid paths are sent to the task_tx channel for further processing
pub struct Scanner {
    depth: u16,
    path: PathBuf,
    task_tx: Option<Sender<AnalyzeTarget>>,
    pub task_rx: Receiver<AnalyzeTarget>,
    /// Stop Scanning when any of the conditions are met
    stop_conditions: Vec<Box<dyn Stop>>,
    /// report to task_tx when any of the conditions are met
    report_conditions: Vec<Box<dyn Reportable>>,
}

impl Scanner {
    pub fn new(
        path: PathBuf,
        depth: u16,
        task_tx: Sender<AnalyzeTarget>,
        task_rx: Receiver<AnalyzeTarget>,
        stop_conditions: Vec<Box<dyn Stop>>,
        report_conditions: Vec<Box<dyn Reportable>>,
    ) -> Self {
        Scanner {
            depth,
            path,
            task_tx: Some(task_tx),
            task_rx,
            stop_conditions,
            report_conditions,
        }
    }
    pub fn walk(&mut self, path: &PathBuf, depth: u16) {
        if depth > self.depth {
            return;
        }
        let task_tx = self.task_tx.as_ref().unwrap();
        for condition in &self.report_conditions {
            if condition.report(&path) {
                task_tx
                    .send(AnalyzeTarget::new(
                        path.clone(),
                        get_size(path).unwrap_or(0),
                        depth,
                    ))
                    .unwrap();
                return;
            }
        }

        for stop_condition in &self.stop_conditions {
            if stop_condition.stop(&path) {
                return;
            }
        }

        if path.is_file() {
            return;
        }
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            self.walk(&entry.path(), depth + 1);
        }
        return;
    }

    pub fn scan(&mut self) {
        self.walk(&self.path.clone(), 0);
        drop(self.task_tx.take().unwrap());
    }

    pub fn set_depth(&mut self, depth: u16) {
        self.depth = depth;
    }
}

pub fn get_dirty_git_repo_scanner(path: &Path, depth: u16) -> Scanner {
    let (tx, rx) = mpsc::channel::<AnalyzeTarget>(); // Add type annotation for T
    let stop_conditions: Vec<Box<dyn Stop>> =
        vec![Box::new(IsFileStop {}), Box::new(HiddenDirStop {})];
    let report_conditions: Vec<Box<dyn Reportable>> = vec![Box::new(GitDirtyRepoPredicate {})];
    Scanner::new(
        PathBuf::from(path),
        depth,
        tx,
        rx,
        stop_conditions,
        report_conditions,
    )
}

pub fn get_project_garbage_scanner(path: &Path, depth: u16) -> Scanner {
    let (tx, rx) = mpsc::channel::<AnalyzeTarget>(); // Add type annotation for T
    let stop_conditions: Vec<Box<dyn Stop>> =
        vec![Box::new(IsFileStop {}), Box::new(HiddenDirStop {})];
    let report_conditions: Vec<Box<dyn Reportable>> = vec![
        Box::new(RustTargetPredicate {}),
        Box::new(NodeModulesPredicate {}),
    ];
    Scanner::new(
        PathBuf::from(path),
        depth,
        tx,
        rx,
        stop_conditions,
        report_conditions,
    )
}
