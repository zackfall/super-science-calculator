use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, MathError>;

#[derive(Debug, Serialize, Deserialize, Clone, Error)]
pub enum MathError {
    #[error("MathError: Not a basic math operation")]
    NotABasicOperation,
    #[error("MathError: Not an advanced math operation")]
    NotAnAdvancedOperation,
    #[error("MathError: Incorrect exponent")]
    IncorrectExponent,
    #[error("MathError: Root based must not be minor than 0")]
    MinusBasedError,
    #[error("MathError: Log base must not be equal or minor than 0")]
    LogBaseNotMinorThanZero,
    #[error("MathError: Factorial Base must be a positive integer")]
    InvalidFactorialBase,
}
