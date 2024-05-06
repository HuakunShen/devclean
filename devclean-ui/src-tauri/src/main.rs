// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use devclean::{results::AnalyzeTarget, scanner::get_project_garbage_scanner};
use std::path::PathBuf;
use tauri::Runtime;

#[tauri::command]
async fn scan<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    path: PathBuf,
    depth: u16,
) -> Result<Vec<AnalyzeTarget>, String> {
    let scanner = get_project_garbage_scanner(depth, false);
    let mut target_paths = scanner.scan_parallel(&path, 0);
    target_paths.sort_by(|a, b| b.cmp(a));
    Ok(target_paths)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
