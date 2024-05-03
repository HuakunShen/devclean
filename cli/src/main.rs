use clap::{Parser, Subcommand};
use color_eyre::Result;
use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use scanner::{
    results::{AnalyzeTarget, AnalyzeTargets},
    scanner::{get_dirty_git_repo_scanner, get_project_garbage_scanner},
};
use std::{io::Write, path::PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, help = "Depth Limit", default_value = "10")]
    depth: u16,

    path: Option<PathBuf>,

    #[arg(long, help = "Dry Run", default_value = "false")]
    dry_run: bool,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// Adds files to myapp
    FindDirtyGit {
        path: Option<PathBuf>,
        #[arg(short, long, help = "Depth Limit", default_value = "10")]
        depth: u16,
    },
}

struct Cleaner {
    bytes_cleaned: u128,
    dry_run: bool,
}
impl Cleaner {
    fn new(dry_run: bool) -> Self {
        Cleaner {
            bytes_cleaned: 0,
            dry_run,
        }
    }
    fn prompt_clean(&mut self, target: &AnalyzeTarget) -> Result<()> {
        let parent = target.path.parent().unwrap();
        let dir_name: String = target.path.file_name().unwrap().to_string_lossy().into();
        let size = get_size(target.path.clone())?;
        self.bytes_cleaned += size as u128;
        println!(
            "{}\n  └─ {} ({})",
            parent.display(),
            dir_name,
            human_bytes(size as f64)
        );
        if self.dry_run {
            print!("(Dry Run)  ");
        }
        print!("Do you want to clean this directory? [y/N]:");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() == "y" {
            if self.dry_run {
                print!("(Dry Run)  ");
            }
            println!("Cleaning {}", target.path.display());
            if !self.dry_run {
                std::fs::remove_dir_all(&target.path)?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Some(Commands::FindDirtyGit { path, depth }) => {
            let path = path.unwrap_or_else(|| PathBuf::from("."));
            // turn p into absolute path
            let path = std::fs::canonicalize(path)?;
            let mut scanner = get_dirty_git_repo_scanner(path.as_path(), depth);
            scanner.scan();
            let mut targets = vec![];
            while let Ok(target) = scanner.task_rx.recv() {
                targets.push(target);
            }
            AnalyzeTargets(targets).to_table().printstd();
        }
        None => {
            let path = args.path.unwrap_or_else(|| PathBuf::from("."));
            let path = std::fs::canonicalize(path)?;
            let mut removable_scanner = get_project_garbage_scanner(path.as_path(), args.depth);
            removable_scanner.scan();
            let mut targets = Vec::new();
            let mut cleaner = Cleaner::new(args.dry_run);
            while let Ok(target) = removable_scanner.task_rx.recv() {
                cleaner.prompt_clean(&target)?;
                targets.push(target);
            }
            println!(
                "Total Bytes Cleaned: {}",
                human_bytes(cleaner.bytes_cleaned as f64)
            );
            // AnalyzeTargets(targets).to_table().printstd();
        }
    }

    Ok(())
}
