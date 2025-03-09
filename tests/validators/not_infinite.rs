/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(validators(not_infinite))]
struct NonInfiniteFloat(f64);

#[rstest]
#[case(f64::MIN)]
#[case(-10.0_f64)]
#[case(-0.0_f64)]
#[case(f64::NAN)]
#[case(0.0_f64)]
#[case(10.0_f64)]
#[case(f64::MAX)]
fn valid_not_nan_test(#[case] input: f64) {
    NonInfiniteFloat::try_new(input).unwrap();
}

#[rstest]
#[case(f64::INFINITY)]
#[case(f64::NEG_INFINITY)]
fn invalid_not_nan_test(#[case] input: f64) {
    assert!(matches!(
        NonInfiniteFloat::try_new(input),
        Err(NonInfiniteFloatError::NotInfinite)
    ));
}
