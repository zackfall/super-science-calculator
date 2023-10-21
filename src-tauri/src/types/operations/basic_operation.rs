use serde::{Deserialize, Serialize};

use crate::types::{
    errors::{MathError, Result},
    sign::Sign,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicOperation(f64, Sign, f64);

impl BasicOperation {
    // pub fn new(lhs: f64, sign: Sign, rhs: f64) -> Self {
    //     Self(lhs, sign, rhs)
    // }

    pub fn sum(&self) -> Result<f64> {
        Ok(self.0 + self.2)
    }

    pub fn substract(&self) -> Result<f64> {
        Ok(self.0 - self.2)
    }

    pub fn multiply(&self) -> Result<f64> {
        Ok(self.0 * self.2)
    }

    pub fn divide(&self) -> Result<f64> {
        if self.2 == 0.0 {
            return Err(MathError::DivisionByZero);
        }
        Ok(self.0 / self.2)
    }

    pub fn get_sign(&self) -> Sign {
        self.1.clone()
    }
}
