use scanner::scanner::get_project_garbage_scanner;
use std::path::PathBuf;

fn main() {
    let p = PathBuf::from("/Users/hacker/Dev/projects/magic-wormhole.rs");
    let p = PathBuf::from("/Users/hacker/Dev/research/winden");

    let mut scanner = get_project_garbage_scanner(p.as_path(), 5);
    scanner.scan();
    while let Ok(target) = scanner.task_rx.recv() {
        println!("{:#?}", target);
    }
}
