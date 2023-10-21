use super_science_calculator::{
    commands::format_number,
    types::{errors::MathError, operations::advanced_operation::AdvancedOperation},
};

#[test]
fn test_exponentiation() {
    let exp1 = AdvancedOperation {
        base: 3.0,
        index: Some(2),
    };
    let exp1_result = exp1.expon();
    let exp2 = AdvancedOperation {
        base: 9.0,
        index: Some(2),
    };
    let exp2_result = exp2.expon();
    let exp3 = AdvancedOperation {
        base: 7.0,
        index: Some(2),
    };
    let exp3_result = exp3.expon();
    let exp4 = AdvancedOperation {
        base: 9.0,
        index: None,
    };
    let exp4_result = exp4.expon();
    assert_eq!(exp1_result.unwrap(), 9.0);
    assert_eq!(exp2_result.unwrap(), 81.0);
    assert_eq!(exp3_result.unwrap(), 49.0);
    assert_eq!(exp4_result, Err(MathError::IncorrectExponent));
}

#[test]
fn test_sqrt() {
    let sqrt1 = AdvancedOperation {
        base: 9.0,
        index: None,
    };
    let sqrt1_result = sqrt1.sqrt();
    let sqrt2 = AdvancedOperation {
        base: 4.0,
        index: None,
    };
    let sqrt2_result = sqrt2.sqrt();
    let sqrt3 = AdvancedOperation {
        base: -4.0,
        index: None,
    };
    let sqrt3_result = sqrt3.sqrt();
    assert_eq!(sqrt1_result.unwrap(), 3.0);
    assert_eq!(sqrt2_result.unwrap(), 2.0);
    assert_eq!(sqrt3_result, Err(MathError::MinusBasedError));
}

#[test]
fn test_root() {
    let root1 = AdvancedOperation {
        base: 9.0,
        index: Some(2),
    };
    let root1_result = root1.root();
    let root2 = AdvancedOperation {
        base: 4.0,
        index: Some(2),
    };
    let root2_result = root2.root();
    let root3 = AdvancedOperation {
        base: -4.0,
        index: Some(2),
    };
    let root3_result = root3.root();
    let root4 = AdvancedOperation {
        base: -4.0,
        index: None,
    };
    let root4_result = root4.root();
    let root5 = AdvancedOperation {
        base: 4.0,
        index: Some(0),
    };
    let root5_result = root5.root();
    assert_eq!(root1_result.unwrap(), 3.0);
    assert_eq!(root2_result.unwrap(), 2.0);
    assert_eq!(root3_result, Err(MathError::MinusBasedError));
    assert_eq!(root4_result, Err(MathError::IncorrectExponent));
    assert_eq!(root5_result, Err(MathError::IncorrectExponent));
}

#[test]
fn test_log() {
    let log1 = AdvancedOperation {
        base: 9.0,
        index: None,
    };
    let log1_result = log1.log();
    let log2 = AdvancedOperation {
        base: 4.0,
        index: None,
    };
    let log2_result = log2.log();
    let log3 = AdvancedOperation {
        base: -10.0,
        index: None,
    };
    let log3_result = log3.log();
    assert_eq!(format_number(log1_result.unwrap()), 0.9542);
    assert_eq!(format_number(log2_result.unwrap()), 0.6021);
    assert_eq!(log3_result, Err(MathError::LogBaseNotMinorThanZero));
}

#[test]
fn test_logb() {
    let log1 = AdvancedOperation {
        base: 10f64,
        index: Some(4),
    };
    let log1_result = log1.log_b();
    let log2 = AdvancedOperation {
        base: 77f64,
        index: Some(8),
    };
    let log2_result = log2.log_b();
    let log3 = AdvancedOperation {
        base: 0f64,
        index: Some(4),
    };
    let log3_result = log3.log_b();
    let log4 = AdvancedOperation {
        base: 10f64,
        index: Some(0),
    };
    let log4_result = log4.log_b();
    assert_eq!(format_number(log1_result.unwrap()), 1.661);
    assert_eq!(format_number(log2_result.unwrap()), 2.0889);
    assert_eq!(log3_result, Err(MathError::LogBaseNotMinorThanZero));
    assert_eq!(log4_result, Err(MathError::IncorrectExponent));
}
