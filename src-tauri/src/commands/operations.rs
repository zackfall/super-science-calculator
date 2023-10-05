use super::format_number;

#[tauri::command]
pub fn sum(x: f64, y: f64) -> f64 {
    format_number(x + y)
}

#[tauri::command]
pub fn substract(x: f64, y: f64) -> f64 {
    format_number(x - y)
}

#[tauri::command]
pub fn multiply(x: f64, y: f64) -> f64 {
    format_number(x * y)
}

#[tauri::command]
pub fn divide(numerator: f64, denominator: f64) -> f64 {
    format_number(numerator / denominator)
}
