/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

// Integer min
#[nnn(validators(min = 42_i32))]
struct MinInt(i32);

#[rstest]
#[case(43_i32)]
fn valid_min_signed_int(#[case] input: i32) {
    MinInt::try_new(input).unwrap();
}

#[rstest]
#[case(42_i32)]
#[case(41_i32)]
fn invalid_min_signed_int(#[case] input: i32) {
    assert!(matches!(
        MinInt::try_new(input),
        Err(MinIntError::Min)
    ));
}

// Float min
#[nnn(validators(min = 3.00_f64))]
struct MinFloat(f64);

#[rstest]
#[case(3.01_f64)]
fn valid_min_float(#[case] input: f64) {
    MinFloat::try_new(input).unwrap();
}

#[rstest]
#[case(3.00_f64)]
#[case(2.99_f64)]
fn invalid_min_float(#[case] input: f64) {
    assert!(matches!(
        MinFloat::try_new(input),
        Err(MinFloatError::Min)
    ));
}
