use super::format_number;

#[tauri::command]
pub fn sin(num: f64) -> f64 {
    format_number(num.sin())
}

#[tauri::command]
pub fn cos(num: f64) -> f64 {
    format_number(num.cos())
}

#[tauri::command]
pub fn tan(num: f64) -> f64 {
    format_number(num.tan())
}

#[tauri::command]
pub fn cot(num: f64) -> f64 {
    format_number(1.0 / num.tan())
}

#[tauri::command]
pub fn sec(num: f64) -> f64 {
    format_number(1.0 / num.cos())
}

#[tauri::command]
pub fn csc(num: f64) -> f64 {
    format_number(1.0 / num.sin())
}
