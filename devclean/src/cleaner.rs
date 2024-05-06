use crate::results::AnalyzeTarget;
use color_eyre::Result;
use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use indicatif::ProgressBar;
use std::io::Write;

pub struct Cleaner {
    pub bytes_cleaned: u128,
    pub dry_run: bool,
    pub need_confirm: bool,
}
impl Cleaner {
    pub fn new(dry_run: bool, all: bool) -> Self {
        Cleaner {
            bytes_cleaned: 0,
            dry_run,
            need_confirm: !all,
        }
    }
    pub fn clean_all(&mut self, targets: &Vec<AnalyzeTarget>) -> Result<()> {
        let pb = ProgressBar::new(targets.len() as u64);
        for target in targets {
            if !self.dry_run {
                std::fs::remove_dir_all(&target.path)?;
            }
            let size = target
                .size
                .unwrap_or_else(|| get_size(target.path.clone()).unwrap_or(0));
            self.bytes_cleaned += size as u128;
            pb.inc(1);
        }
        Ok(())
    }

    pub fn prompt_clean(&mut self, target: &AnalyzeTarget) -> Result<()> {
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
