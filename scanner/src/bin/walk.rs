use std::{path::PathBuf, sync::mpsc};

use scanner::{
    predicates::{
        languages::{node::NodeModulesPredicate, rust::RustTargetPredicate},
        stop::{HiddenDirStop, IsFileStop},
        Removable, Reportable, Stop,
    },
    scanner::{AnalyzeTarget, Scanner},
};

fn main() {
    let (tx, rx) = mpsc::channel::<AnalyzeTarget>(); // Add type annotation for T
    let rust_target_pred = Box::new(RustTargetPredicate {});
    let node_modules_pred = Box::new(NodeModulesPredicate {});
    let stop_conditions: Vec<Box<dyn Stop>> = vec![
        Box::new(IsFileStop {}),
        Box::new(HiddenDirStop {}),
        // rust_target_pred.clone(),
        // node_modules_pred.clone(),
    ];
    let report_conditions: Vec<Box<dyn Reportable>> = vec![rust_target_pred, node_modules_pred];
    let mut scanner = Scanner::new(
        // PathBuf::from("/Users/hacker/Dev/projects/magic-wormhole.rs"),
        PathBuf::from("/Users/hacker/Dev/research/winden"),
        5,
        tx,
        stop_conditions,
        report_conditions,
    );
    scanner.scan();
    while let Ok(target) = rx.recv() {
        println!("{:#?}", target);
    }
}
