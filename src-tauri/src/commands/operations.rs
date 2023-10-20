use crate::types::errors::{MathError, Result};
use crate::types::operation::AdvancedOperations;
use crate::types::{operation::Operation, sign::Sign};

use super::format_number;

#[tauri::command]
pub fn basic_operation(operation: Operation) -> Result<f64> {
    match operation {
        Operation::BasicOperation(operation) => match operation.get_sign() {
            Sign::Plus => Ok(format_number(operation.sum())),
            Sign::Minus => Ok(format_number(operation.substract())),
            Sign::Division => Ok(format_number(operation.divide())),
            Sign::Multiplication => Ok(format_number(operation.multiply())),
        },
        _ => Err(MathError::NotABasicOperation),
    }
}

#[tauri::command]
pub fn advanced_operations(operation: Operation) -> Result<f64> {
    match operation {
        Operation::AdvancedOperation(operation) => match operation {
            AdvancedOperations::Expon(operation) => operation.expon(),
            AdvancedOperations::Sqrt(operation) => operation.sqrt(),
            AdvancedOperations::Root(operation) => operation.root(),
            AdvancedOperations::Log(operation) => operation.log(),
            AdvancedOperations::LogB(operation) => operation.log_b(),
            AdvancedOperations::Ln(operation) => operation.ln(),
            AdvancedOperations::Factorial(operation) => operation.factorial(),
        },
        _ => Err(MathError::NotAnAdvancedOperation),
    }
}
