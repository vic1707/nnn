/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(negative))]
struct NegativeInt(i32);

#[rstest]
#[case(-42_i32)]
fn valid_negative_int(#[case] input: i32) {
    NegativeInt::try_new(input).unwrap();
}

#[rstest]
#[case(0_i32)] // 0 is not negative
#[case(1_i32)]
fn invalid_negative_int(#[case] input: i32) {
    assert!(matches!(
        NegativeInt::try_new(input),
        Err(NegativeIntError::Negative)
    ));
}

#[nnn(validators(negative))]
struct NegativeFloat(f64);

#[rstest]
#[case(-0.0_f64)]
#[case(-3.0_f64)]
fn valid_negative_float(#[case] input: f64) {
    NegativeFloat::try_new(input).unwrap();
}

#[rstest]
#[case(0.0_f64)]
#[case(3.0_f64)]
fn invalid_negative_float(#[case] input: f64) {
    assert!(matches!(
        NegativeFloat::try_new(input),
        Err(NegativeFloatError::Negative)
    ));
}
