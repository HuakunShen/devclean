use clap::{Parser, Subcommand};
use color_eyre::Result;
use devclean::Cleaner;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use human_bytes::human_bytes;
use scanner::{
    results::AnalyzeTargets,
    scanner::{get_dirty_git_repo_scanner, get_project_garbage_scanner},
};
use std::path::PathBuf;

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

    #[arg(short, long, help = "Select All by Default", default_value = "false")]
    all: bool,

    #[arg(short, long, help = "No Need to Confirm", default_value = "false")]
    yes: bool,

    #[arg(long, help = "Display Relative Path", default_value = "true")]
    relative: bool,
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    match args.command {
        Some(Commands::FindDirtyGit { path, depth }) => {
            let path = path.unwrap_or_else(|| PathBuf::from("."));
            let path = std::fs::canonicalize(path)?;
            let scanner = get_dirty_git_repo_scanner(depth, true);
            let found = scanner.scan_parallel(&path, 0);
            AnalyzeTargets(found).to_table().printstd();
        }
        None => {
            let mut path = args.path.unwrap_or_else(|| PathBuf::from("."));
            if !args.relative {
                path = std::fs::canonicalize(path)?;
            }
            let removable_scanner = get_project_garbage_scanner(args.depth, true);
            let mut cleaner = Cleaner::new(args.dry_run, args.all);
            let start = std::time::Instant::now();
            let mut target_paths = removable_scanner.scan_parallel(&path, 0);
            println!("Scan Finished in {:?}", start.elapsed());
            target_paths.sort_by(|a, b| b.cmp(a));
            let to_clean = if args.yes {
                target_paths.clone()
            } else {
                let default_selection = if args.all {
                    // Select all by default
                    vec![true; target_paths.len()]
                } else {
                    // Select No by default
                    vec![false; target_paths.len()]
                };
                let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pick the directories to clean")
                    .items(&target_paths)
                    .defaults(&default_selection[..])
                    .interact()?;
                let mut to_clean = vec![];
                for select in selections {
                    to_clean.push(target_paths[select].clone());
                }
                to_clean
            };
            cleaner.clean_all(&to_clean)?;
            println!(
                "Total Bytes Cleaned: {}",
                human_bytes(cleaner.bytes_cleaned as f64)
            );
            AnalyzeTargets(to_clean).to_table().printstd();
        }
    }
    Ok(())
}
