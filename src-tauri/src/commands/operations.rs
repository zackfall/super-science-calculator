use crate::types::errors::{MathError, Result};
use crate::types::operations::advanced_operation::AdvancedOperations;
use crate::types::operations::trigonometrics::TrigonometricOperations;
use crate::types::{operations::Operation, sign::Sign};

use super::format_number;

#[tauri::command]
pub fn basic_operation(operation: Operation) -> Result<f64> {
    match operation {
        Operation::BasicOperation(operation) => match operation.get_sign() {
            Sign::Plus => Ok(format_number(operation.sum().unwrap())),
            Sign::Minus => Ok(format_number(operation.substract().unwrap())),
            Sign::Division => match operation.divide() {
                Ok(result) => Ok(format_number(result)),
                Err(error) => Err(error),
            },
            Sign::Multiplication => Ok(format_number(operation.multiply().unwrap())),
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

#[tauri::command]
pub fn trigonometric_operations(operation: Operation) -> Result<f64> {
    match operation {
        Operation::TrigonometricOperation(operation) => match operation {
            TrigonometricOperations::Sin(operation) => Ok(operation.sin()),
            TrigonometricOperations::Cos(operation) => Ok(operation.cos()),
            TrigonometricOperations::Tan(operation) => Ok(operation.tan()),
            TrigonometricOperations::Cot(operation) => Ok(operation.cot()),
            TrigonometricOperations::Sec(operation) => Ok(operation.sec()),
            TrigonometricOperations::Csc(operation) => Ok(operation.csc()),
        },
        _ => Err(MathError::NotATrigonometricFunction),
    }
}
