/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(finite))]
struct FiniteFloat(f64);

#[rstest]
#[case(f64::MIN)]
#[case(-10.0_f64)]
#[case(-0.0_f64)]
#[case(0.0_f64)]
#[case(10.0_f64)]
#[case(f64::MAX)]
fn valid_finite_float(#[case] input: f64) {
    FiniteFloat::try_new(input).unwrap();
}

#[rstest]
#[case(f64::NEG_INFINITY)]
#[case(f64::NAN)]
#[case(f64::INFINITY)]
fn invalid_finite_float(#[case] input: f64) {
    assert!(matches!(
        FiniteFloat::try_new(input),
        Err(FiniteFloatError::Finite)
    ));
}
