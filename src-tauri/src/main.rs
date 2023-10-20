// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod types;

use commands::{operations, trigonometrics};
use types::{
    operation::{BasicOperation, Operation},
    sign::Sign,
};

use crate::types::operation::{AdvancedOperation, AdvancedOperations};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            operations::basic_operation,
            operations::advanced_operations,
            trigonometrics::sin,
            trigonometrics::cos,
            trigonometrics::tan,
            trigonometrics::cot,
            trigonometrics::sec,
            trigonometrics::csc
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
