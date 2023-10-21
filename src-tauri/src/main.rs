// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod types;

use commands::operations;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            operations::basic_operation,
            operations::advanced_operations,
            operations::trigonometric_operations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
