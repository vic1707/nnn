/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

// Integer max (not inclusive)
#[nnn(validators(max = 42_i32))]
struct MaxInt(i32);

#[rstest]
#[case(41_i32)]
fn valid_max_signed_int(#[case] input: i32) {
    MaxInt::try_new(input).unwrap();
}

#[rstest]
#[case(42_i32)]
#[case(43_i32)]
fn invalid_max_signed_int(#[case] input: i32) {
    assert!(matches!(
        MaxInt::try_new(input),
        Err(MaxIntError::Max)
    ));
}

// Float max (not inclusive)
#[nnn(validators(max = 3.00_f64))]
struct MaxFloat(f64);

#[rstest]
#[case(2.99_f64)]
fn valid_max_float(#[case] input: f64) {
    MaxFloat::try_new(input).unwrap();
}

#[rstest]
#[case(3.00_f64)]
#[case(3.01_f64)]
fn invalid_max_float(#[case] input: f64) {
    assert!(matches!(
        MaxFloat::try_new(input),
        Err(MaxFloatError::Max)
    ));
}
