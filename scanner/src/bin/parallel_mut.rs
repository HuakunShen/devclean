use fs_extra::dir::get_size;
use rayon::prelude::*;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::path::PathBuf;
use std::time::Duration;

struct Scanner {
    count: Arc<AtomicU64>,
}

impl Scanner {
    fn scan_parallel(&self, path: PathBuf) -> Vec<PathBuf> {
        self.count.fetch_add(1, Ordering::SeqCst);
        get_size(path.clone()).unwrap();
        let entries = fs::read_dir(path)
            .unwrap()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        entries
            .par_iter()
            .flat_map(|entry| {
                let path = entry.path();
                if path.is_dir() {
                    self.scan_parallel(path)
                } else {
                    vec![path]
                }
            })
            .collect()
    }
}

fn main() {
    let path = PathBuf::from("/Users/hacker/Dev/learn");
    let start = std::time::Instant::now();
    let scanner = Scanner {
        count: Arc::new(AtomicU64::new(0)),
    };
    let results = scanner.scan_parallel(path);
    println!("Scan Finished in {:?}", start.elapsed());
    println!("Found {} files", results.len());
    println!("Directories scanned: {}", scanner.count.load(Ordering::SeqCst));
}
