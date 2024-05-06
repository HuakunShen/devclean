use clap::{Parser, Subcommand};
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use humantime;
use indicatif::{ProgressBar, ProgressStyle};
use scanner::{
    results::{AnalyzeTarget, AnalyzeTargets},
    scanner::{get_dirty_git_repo_scanner, get_project_garbage_scanner},
};
use std::{io::Write, path::PathBuf, time::Duration};

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

    #[arg(
        short,
        long,
        help = "Clean all without Confirmation",
        default_value = "false"
    )]
    all: bool,
    // #[arg(short, long, help = "No Need to Confirm", default_value = "false")]
    // yes: bool,
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
    need_confirm: bool,
}
impl Cleaner {
    fn new(dry_run: bool, all: bool) -> Self {
        Cleaner {
            bytes_cleaned: 0,
            dry_run,
            need_confirm: !all,
        }
    }
    fn clean_all(&mut self, targets: &Vec<AnalyzeTarget>) -> Result<()> {
        let pb = ProgressBar::new(targets.len() as u64);
        // let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}").unwrap();
        // pb.set_style(spinner_style);
        for target in targets {
            if !self.dry_run {
                std::fs::remove_dir_all(&target.path)?;
            }
            self.bytes_cleaned += get_size(target.path.clone())? as u128;
            pb.inc(1);
            std::thread::sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    fn prompt_clean(&mut self, target: &AnalyzeTarget) -> Result<()> {
        let parent = target.path.parent().unwrap();
        let dir_name: String = target.path.file_name().unwrap().to_string_lossy().into();
        let size = get_size(target.path.clone())?;
        let last_modified = target.path.metadata()?.modified()?;
        let human_modified_ago = humantime::format_duration(std::time::Duration::from_secs(
            last_modified.elapsed()?.as_secs(),
        ));
        println!(
            "{}\n  └─ {} ({}) \t\t({} seconds ago)",
            parent.display(),
            dir_name,
            human_bytes(size as f64),
            human_modified_ago
        );
        if self.need_confirm {
            if self.dry_run {
                print!("(Dry Run)  ");
            }
            print!("Do you want to clean this directory? [y/N]:");
        }
        std::io::stdout().flush()?;
        let mut input = String::new();
        if self.need_confirm {
            std::io::stdin().read_line(&mut input)?;
        } else {
            input = "y".to_string();
        }
        if input.trim().to_lowercase() == "y" {
            if self.dry_run {
                print!("(Dry Run)  ");
            }
            println!("Cleaning {}", target.path.display());
            if !self.dry_run {
                std::fs::remove_dir_all(&target.path)?;
            }
            self.bytes_cleaned += size as u128;
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
            let mut scanner = get_dirty_git_repo_scanner(path.as_path(), depth, false);
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
            let mut removable_scanner =
                get_project_garbage_scanner(path.as_path(), args.depth, true);
            let mut cleaner = Cleaner::new(args.dry_run, args.all);
            let mut target_paths = removable_scanner.scan_recursive(&path, 0);
            target_paths.sort_by(|a, b| b.cmp(a));
            println!("{}", args.all);
            let default_selection = if args.all {
                // Select all by default
                vec![true; target_paths.len()]
            } else {
                // Select No by default
                vec![false; target_paths.len()]
            };
            println!("{:?}", default_selection);
            let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick the directories to clean")
                .items(&target_paths)
                .defaults(&default_selection[..])
                .interact()?;
            let mut to_clean = vec![];
            for select in selections {
                to_clean.push(target_paths[select].clone());
            }
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
