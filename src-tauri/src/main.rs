// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod types;

use commands::{advanced_operations, operations};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            operations::divide,
            operations::multiply,
            operations::substract,
            operations::sum,
            advanced_operations::expon,
            advanced_operations::factorial,
            advanced_operations::ln,
            advanced_operations::log,
            advanced_operations::log_b,
            advanced_operations::root,
            advanced_operations::sqrt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
