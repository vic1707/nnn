#![allow(clippy::float_cmp, reason = "_")]
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(positive))]
struct PositiveInt(i32);

#[rstest]
#[case(42_i32)]
fn valid_positive_int(#[case] input: i32) {
    PositiveInt::try_new(input).unwrap();
}

#[rstest]
#[case(0_i32)]  // Zero is not positive
#[case(-1_i32)]
fn invalid_positive_int(#[case] input: i32) {
    assert!(matches!(
        PositiveInt::try_new(input),
        Err(PositiveIntError::Positive)
    ));
}

#[nnn(validators(positive))]
struct PositiveFloat(f64);

#[rstest]
#[case(0.0_f64)]
#[case(3.0_f64)]
fn valid_positive_float(#[case] input: f64) {
    PositiveFloat::try_new(input).unwrap();
}

#[rstest]
#[case(-0.0_f64)]
#[case(-3.0_f64)]
fn invalid_positive_float(#[case] input: f64) {
    assert!(matches!(
        PositiveFloat::try_new(input),
        Err(PositiveFloatError::Positive)
    ));
}
