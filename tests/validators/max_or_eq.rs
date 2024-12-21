/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

// Integer max_or_eq
#[nnn(validators(max_or_eq = 42_i32))]
struct MaxOrEqInt(i32);

#[rstest]
#[case(42_i32)]
#[case(41_i32)]
fn valid_max_or_eq_signed_int(#[case] input: i32) {
    MaxOrEqInt::try_new(input).unwrap();
}

#[rstest]
#[case(43_i32)]
fn invalid_max_or_eq_signed_int(#[case] input: i32) {
    assert!(matches!(
        MaxOrEqInt::try_new(input),
        Err(MaxOrEqIntError::MaxOrEq)
    ));
}

// Float max_or_eq
#[nnn(validators(max_or_eq = 3.00_f64))]
struct MaxOrEqFloat(f64);

#[rstest]
#[case(3.00_f64)]
#[case(2.99_f64)]
fn valid_max_or_eq_float(#[case] input: f64) {
    MaxOrEqFloat::try_new(input).unwrap();
}

#[rstest]
#[case(3.01_f64)]
fn invalid_max_or_eq_float(#[case] input: f64) {
    assert!(matches!(
        MaxOrEqFloat::try_new(input),
        Err(MaxOrEqFloatError::MaxOrEq)
    ));
}
