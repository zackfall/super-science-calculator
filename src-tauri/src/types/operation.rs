use super::sign::Sign;

pub enum Operation {
    LROperation(LROperation),
}

pub struct LROperation(f64, Sign, f64);
