use super::{
    errors::{MathError, Result},
    sign::Sign,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    AdvancedOperation(AdvancedOperations),
    BasicOperation(BasicOperation),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicOperation(f64, Sign, f64);

impl BasicOperation {
    pub fn new(lhs: f64, sign: Sign, rhs: f64) -> Self {
        Self(lhs, sign, rhs)
    }

    pub fn sum(&self) -> f64 {
        self.0 + self.2
    }

    pub fn substract(&self) -> f64 {
        self.0 - self.2
    }

    pub fn multiply(&self) -> f64 {
        self.0 * self.2
    }

    pub fn divide(&self) -> f64 {
        self.0 / self.2
    }

    pub fn get_sign(&self) -> Sign {
        self.1.clone()
    }
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
            Some(index) => match self.base < 0.0 {
                true => Err(MathError::MinusBasedError),
                false => Ok(self.base.powf(1.0 / index as f64)),
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
            Some(index) => match self.base <= 0.0 {
                true => Err(MathError::LogBaseNotMinorThanZero),
                false => Ok(self.base.ln() / (index as f64).ln()),
            },
            None => Err(MathError::IncorrectExponent),
        }
    }

    pub fn ln(&self) -> Result<f64> {
        match self.base <= 0.0 {
            true => Err(MathError::MinusBasedError),
            false => Ok(self.base.ln()),
        }
    }

    pub fn factorial(&self) -> Result<f64> {
        match self.index {
            Some(index) => {
                let mut res = 1;
                for i in 1..=index {
                    res *= i;
                }
                Ok(res as f64)
            }
            _ => Err(MathError::InvalidFactorialBase),
        }
    }
}

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
