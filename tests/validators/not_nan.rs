/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(validators(not_nan))]
struct NonNanFloat(f64);

#[rstest]
#[case(f64::NEG_INFINITY)]
#[case(f64::MIN)]
#[case(-10.0_f64)]
#[case(-0.0_f64)]
#[case(0.0_f64)]
#[case(10.0_f64)]
#[case(f64::MAX)]
#[case(f64::INFINITY)]
fn valid_not_nan_test(#[case] input: f64) {
    NonNanFloat::try_new(input).unwrap();
}

#[rstest]
#[case(f64::NAN)]
fn invalid_not_nan_test(#[case] input: f64) {
    assert!(matches!(
        NonNanFloat::try_new(input),
        Err(NonNanFloatError::NotNAN)
    ));
}
