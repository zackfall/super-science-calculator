use serde::{Deserialize, Serialize};

use crate::types::errors::{MathError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub enum AdvancedOperations {
    Expon(AdvancedOperation),
    Sqrt(AdvancedOperation),
    Root(AdvancedOperation),
    Log(AdvancedOperation),
    LogB(AdvancedOperation),
    Ln(AdvancedOperation),
    Factorial(AdvancedOperation),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvancedOperation {
    pub base: f64,
    pub index: Option<u32>,
}

impl AdvancedOperation {
    pub fn expon(&self) -> Result<f64> {
        match self.index {
            Some(index) => Ok(self.base.powf(index as f64)),
            None => Err(MathError::IncorrectExponent),
        }
    }

    pub fn sqrt(&self) -> Result<f64> {
        match self.base < 0.0 {
            true => Err(MathError::MinusBasedError),
            false => Ok(self.base.sqrt()),
        }
    }

    pub fn root(&self) -> Result<f64> {
        match self.index {
            Some(index) => match index == 0 {
                true => Err(MathError::IncorrectExponent),
                false => match self.base < 0.0 {
                    true => Err(MathError::MinusBasedError),
                    false => Ok(self.base.powf(1.0 / index as f64)),
                },
            },
            None => Err(MathError::IncorrectExponent),
        }
    }

    pub fn log(&self) -> Result<f64> {
        match self.base <= 0.0 {
            true => Err(MathError::LogBaseNotMinorThanZero),
            false => Ok(self.base.ln() / 10f64.ln()),
        }
    }

    pub fn log_b(&self) -> Result<f64> {
        match self.index {
            Some(index) => match index == 0 {
                true => Err(MathError::IncorrectExponent),
                false => match self.base <= 0.0 {
                    true => Err(MathError::LogBaseNotMinorThanZero),
                    false => Ok(self.base.ln() / (index as f64).ln()),
                },
            },
            None => Err(MathError::IncorrectExponent),
        }
    }

    pub fn ln(&self) -> Result<f64> {
        match self.base <= 0.0 {
            true => Err(MathError::LogBaseNotMinorThanZero),
            false => Ok(self.base.ln()),
        }
    }

    pub fn factorial(&self) -> Result<f64> {
        if self.base < 0.0 {
            return Err(MathError::MinusBasedError);
        }
        let mut res = 1;
        for i in 1..=self.base as i32 {
            res *= i;
        }
        Ok(res as f64)
    }
}
