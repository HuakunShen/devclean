use fs_extra::dir::get_size;
use crate::predicates::{
    languages::{node::NodeModulesPredicate, rust::RustTargetPredicate},
    stop::{HiddenDirStop, IsFileStop, Stop},
    Removable,
};
use std::{
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

#[derive(Debug)]
pub struct AnalyzeTarget {
    path: PathBuf,
    size: u64,
    depth: u16,
}

pub struct Scanner {
    depth: u16,
    path: PathBuf,
    task_tx: Option<Sender<AnalyzeTarget>>,
    general_stop_conditions: Vec<Box<dyn Stop>>,
    stop_conditions: Vec<Box<dyn Stop>>,
    remove_conditions: Vec<Box<dyn Removable>>,
}

impl Scanner {
    pub fn new(
        path: PathBuf,
        depth: u16,
        task_tx: Sender<AnalyzeTarget>,
        stop_conditions: Vec<Box<dyn Stop>>,
        remove_conditions: Vec<Box<dyn Removable>>,
    ) -> Self {
        Scanner {
            depth,
            path,
            task_tx: Some(task_tx),
            general_stop_conditions: vec![Box::new(HiddenDirStop {}), Box::new(IsFileStop {})],
            stop_conditions,
            remove_conditions,
        }
    }
    pub fn walk(&mut self, path: &PathBuf, depth: u16) {
        if depth > self.depth {
            return;
        }
        let task_tx = self.task_tx.as_ref().unwrap().clone();
        for remove_condition in &self.remove_conditions {
            if remove_condition.is_removable(&path) {
                // println!("Remove: {:?}", path);
                task_tx
                    .send(AnalyzeTarget {
                        path: path.clone(),
                        size: get_size(path).unwrap_or(0),
                        depth,
                    })
                    .unwrap();
                return;
            }
        }
        drop(task_tx);

        for stop_condition in &self.general_stop_conditions {
            if stop_condition.stop(&path) {
                return;
            }
        }
        for stop_condition in &self.stop_conditions {
            if stop_condition.stop(&path) {
                println!("Stop: {:?}", path);
                return;
            }
        }
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            self.walk(&entry.path(), depth + 1);
        }
        if depth == 0 {
            println!("Drop tx");
        }
        return;
    }

    pub fn scan(&mut self) {
        self.walk(&self.path.clone(), 0);
        let x = self.task_tx.take(); // This takes the sender out of the Option and drops it
        drop(x.unwrap());
    }
}
