use serde::{Deserialize, Serialize};

use crate::types::sign::Sign;

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicOperation(f64, Sign, f64);

impl BasicOperation {
    // pub fn new(lhs: f64, sign: Sign, rhs: f64) -> Self {
    //     Self(lhs, sign, rhs)
    // }

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
