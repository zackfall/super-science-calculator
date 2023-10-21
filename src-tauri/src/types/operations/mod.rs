pub mod advanced_operation;
pub mod basic_operation;

use self::advanced_operation::AdvancedOperations;
use self::basic_operation::BasicOperation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    AdvancedOperation(AdvancedOperations),
    BasicOperation(BasicOperation),
}
