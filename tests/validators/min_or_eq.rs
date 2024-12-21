#![allow(clippy::float_cmp, reason = "_")]
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

// Integer min_or_eq
#[nnn(validators(min_or_eq = 42_i32))]
struct MinOrEqInt(i32);

#[rstest]
#[case(42_i32)]
#[case(43_i32)]
fn valid_min_or_eq_signed_int(#[case] input: i32) {
    MinOrEqInt::try_new(input).unwrap();
}

#[rstest]
#[case(41_i32)]
fn invalid_min_or_eq_signed_int(#[case] input: i32) {
    assert!(matches!(
        MinOrEqInt::try_new(input),
        Err(MinOrEqIntError::MinOrEq)
    ));
}

// Float min_or_eq
#[nnn(validators(min_or_eq = 3.00_f64))]
struct MinOrEqFloat(f64);

#[rstest]
#[case(3.00_f64)]
#[case(3.01_f64)]
fn valid_min_or_eq_float(#[case] input: f64) {
    MinOrEqFloat::try_new(input).unwrap();
}

#[rstest]
#[case(2.99_f64)]
fn invalid_min_or_eq_float(#[case] input: f64) {
    assert!(matches!(
        MinOrEqFloat::try_new(input),
        Err(MinOrEqFloatError::MinOrEq)
    ));
}
