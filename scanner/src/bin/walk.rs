use fs_extra::dir::get_size;
use scanner::predicates::{
    languages::{node::NodeModulesPredicate, rust::RustTargetPredicate},
    stop::{HiddenDirStop, IsFileStop, Stop},
    Removable,
};
use std::{
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

#[derive(Debug)]
struct AnalyzeTarget {
    path: PathBuf,
    size: u64,
    depth: u16,
}

struct Scanner {
    depth: u16,
    path: PathBuf,
    task_tx: Option<Sender<AnalyzeTarget>>,
    general_stop_conditions: Vec<Box<dyn Stop>>,
    stop_conditions: Vec<Box<dyn Stop>>,
    remove_conditions: Vec<Box<dyn Removable>>,
}

impl Scanner {
    fn new(
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
    fn walk(&mut self, path: &PathBuf, depth: u16) {
        // let tx_clone = self.task_tx.clone();

        if depth > self.depth {
            return;
        }
        let task_tx = self.task_tx.as_mut().unwrap().clone();
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
        drop(task_tx);
        if depth == 0 {
            println!("Drop tx");
        }
        return;
    }

    fn scan(&mut self) {
        self.walk(&self.path.clone(), 0);
        println!("Drop Sender");
        let x = self.task_tx.take(); // This takes the sender out of the Option and drops it
        drop(x.unwrap());
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<AnalyzeTarget>(); // Add type annotation for T
    let rust_target_pred = Box::new(RustTargetPredicate {});
    let node_modules_pred = Box::new(NodeModulesPredicate {});
    let stop_conditions: Vec<Box<dyn Stop>> = vec![
        Box::new(HiddenDirStop {}),
        rust_target_pred.clone(),
        node_modules_pred.clone(),
    ];
    let remove_conditions: Vec<Box<dyn Removable>> = vec![rust_target_pred, node_modules_pred];
    let mut scanner = Scanner::new(
        PathBuf::from("/Users/hacker/Dev/research/winden"),
        5,
        tx.clone(),
        stop_conditions,
        remove_conditions,
    );
    scanner.scan();
    // drop(tx);
    while let Ok(target) = rx.recv() {
        println!("{:?}", target);
    }

    println!("Done");
}
