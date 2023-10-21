use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TrigonometricOperations {
    Sin(TrigonometricOperation),
    Cos(TrigonometricOperation),
    Tan(TrigonometricOperation),
    Cot(TrigonometricOperation),
    Sec(TrigonometricOperation),
    Csc(TrigonometricOperation),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrigonometricOperation(f64);

impl TrigonometricOperation {
    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    pub fn tan(&self) -> f64 {
        self.0.tan()
    }

    pub fn cot(&self) -> f64 {
        1.0 / self.0.tan()
    }

    pub fn sec(&self) -> f64 {
        1.0 / self.0.cos()
    }

    pub fn csc(&self) -> f64 {
        1.0 / self.0.sin()
    }
}
