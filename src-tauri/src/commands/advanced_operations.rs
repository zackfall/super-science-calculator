use super::format_number;

#[tauri::command]
pub fn expon(base: f64, exp: u32) -> f64 {
    format_number(base.powi(exp as i32))
}

#[tauri::command]
pub fn sqrt(base: f64) -> f64 {
    format_number(base.sqrt())
}

#[tauri::command]
pub fn root(base: f64, index: u32) -> f64 {
    format_number(base.powf(1.0 / index as f64))
}

#[tauri::command]
pub fn log(num: f64) -> f64 {
    format_number(num.ln() / 10f64.ln())
}

#[tauri::command]
pub fn log_b(num: f64, base: f64) -> f64 {
    format_number(num.ln() / base.ln())
}

#[tauri::command]
pub fn ln(num: f64) -> f64 {
    format_number(num.ln())
}

#[tauri::command]
pub fn factorial(num: u32) -> f64 {
    let mut res = 1;
    for i in 1..=num {
        res *= i;
    }

    format_number(res as f64)
}
