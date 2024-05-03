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

fn ask_clean(target: &AnalyzeTarget) -> Result<()> {
    let parent = target.path.parent().unwrap();
    let dir_name: String = target.path.file_name().unwrap().to_string_lossy().into();
    let size = get_size(target.path.clone())?;
    println!(
        "{}\n\t└─ {} ({})",
        parent.display(),
        dir_name,
        human_bytes(size as f64)
    );
    print!("Do you want to clean this directory? [y/N]:");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.trim().to_lowercase() == "y" {
        println!("Cleaning {}", target.path.display());
        std::fs::remove_dir_all(&target.path)?;
    }
    Ok(())
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
            while let Ok(target) = removable_scanner.task_rx.recv() {
                ask_clean(&target);
                targets.push(target);
            }
            // AnalyzeTargets(targets).to_table().printstd();
        }
    }

    Ok(())
}
