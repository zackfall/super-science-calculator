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

#[cfg(test)]
mod tests {
    use crate::types::operations::basic_operation::BasicOperation;

    use super::*;

    #[test]
    fn test_basic_operation() {
        let sum = BasicOperation::new(1.0, Sign::Plus, 2.0);
        let sum_result = basic_operation(Operation::BasicOperation(sum));
        let sub = BasicOperation::new(6.0, Sign::Minus, 2.0);
        let sub_result = basic_operation(Operation::BasicOperation(sub));
        let mul = BasicOperation::new(3.0, Sign::Multiplication, 2.0);
        let mul_result = basic_operation(Operation::BasicOperation(mul));
        let div = BasicOperation::new(10.0, Sign::Division, 2.0);
        let div_result = basic_operation(Operation::BasicOperation(div));
        assert_eq!(sum_result.unwrap(), 3.0);
        assert_eq!(sub_result.unwrap(), 4.0);
        assert_eq!(mul_result.unwrap(), 6.0);
        assert_eq!(div_result.unwrap(), 5.0);
    }
}
