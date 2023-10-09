use super::sign::Sign;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    BasicOperation(BasicOperation),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicOperation(f64, Sign, f64);

impl BasicOperation {
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
