use crate::types::{operation::Operation, sign::Sign};

use super::format_number;

#[tauri::command]
pub fn basic_operation(operation: Operation) -> f64 {
    match operation {
        Operation::BasicOperation(operation) => match operation.get_sign() {
            Sign::Plus => format_number(operation.sum()),
            Sign::Minus => format_number(operation.substract()),
            Sign::Division => format_number(operation.divide()),
            Sign::Multiplication => format_number(operation.multiply()),
        },
    }
}
