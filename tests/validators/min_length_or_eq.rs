#![allow(clippy::float_cmp, reason = "_")]
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(min_length_or_eq = 3))]
struct MinLengthOrEqStr<'str>(&'str str);

#[rstest]
#[case("abc")]
#[case("abcd")]
fn valid_min_length_or_eq_str(#[case] input: &str) {
    MinLengthOrEqStr::try_new(input).unwrap();
}

#[rstest]
#[case("")]
#[case("ab")]
fn invalid_min_length_or_eq_str(#[case] input: &str) {
    assert!(matches!(
        MinLengthOrEqStr::try_new(input),
        Err(MinLengthOrEqStrError::MinLengthOrEq)
    ));
}
