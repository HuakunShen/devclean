use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let pb = ProgressBar::new(1);
    let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}").unwrap();
    pb.set_style(spinner_style);
    for i in 0..100 {
        pb.set_message(format!("{i}:"));
        // pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    pb.finish_with_message("Scan Finished...");
}
