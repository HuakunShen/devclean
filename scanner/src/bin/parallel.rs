use fs_extra::dir::get_size;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

struct Scanner {
    count: u64,
    callbacks: Vec<Box<dyn Fn(PathBuf) -> Vec<PathBuf> + Send + Sync>>,
}

impl Scanner {
    fn scan_parallel(&self, path: PathBuf) -> Vec<PathBuf> {
        // self.count += 1;
        // println!("Scanning: {:?}", path);
        // std::thread::sleep(Duration::from_millis(10));
        get_size(path.clone()).unwrap();
        // let path_clone = path.clone();
        let entries = fs::read_dir(path)
            .unwrap()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        // We use `flat_map` here to concatenate all the results into a single vector
        entries
            .par_iter()
            .flat_map(|entry| {
                // get_size(path_clone).unwrap();
                let path = entry.path();
                if path.is_dir() {
                    // Recursively scan the directory in parallel
                    self.scan(path)
                } else {
                    vec![path]
                }
            })
            .collect()
    }

    fn scan(&self, path: PathBuf) -> Vec<PathBuf> {
        // std::thread::sleep(Duration::from_millis(10));
        get_size(path.clone()).unwrap();
        let mut results = vec![];
        let entries: Vec<_> = path.read_dir().unwrap().collect();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                results.extend(self.scan(path));
            } else {
                results.push(path);
            }
        }

        results
    }
}

fn main() {
    // let path = PathBuf::from("/Users/hacker/Dev/projects/");
    let path = PathBuf::from("/Users/hacker/Dev/learn");
    let start = std::time::Instant::now();
    let mut scanner = Scanner {
        callbacks: vec![],
        count: 0,
    };
    let results = scanner.scan_parallel(path);
    // let results = scanner.scan(path);
    println!("Scan Finished in {:?}", start.elapsed());
    println!("Found {} files", results.len());
    // Optionally, print results
}
