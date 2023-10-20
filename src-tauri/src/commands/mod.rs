pub mod operations;
pub mod trigonometrics;

pub fn format_number(num: f64) -> f64 {
    format!("{:.4}", num).parse::<f64>().unwrap()
}
