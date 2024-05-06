// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use devclean::{results::AnalyzeTarget, scanner::get_project_garbage_scanner};
use std::path::PathBuf;
use tauri::Manager;

#[tauri::command]
async fn scan(path: PathBuf, depth: u16) -> Result<Vec<AnalyzeTarget>, String> {
    let scanner = get_project_garbage_scanner(depth, false);
    let mut target_paths = scanner.scan_parallel(&path, 0);
    target_paths.sort_by(|a, b| b.cmp(a));
    Ok(target_paths)
}

#[tauri::command]
async fn delete_dir(path: PathBuf) -> Result<(), String> {
    std::fs::remove_dir_all(&path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn path_exists(path: PathBuf) -> Result<bool, String> {
    Ok(path.exists())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan, delete_dir, path_exists])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
