use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sign {
    Plus,
    Minus,
    Division,
    Multiplication,
}
