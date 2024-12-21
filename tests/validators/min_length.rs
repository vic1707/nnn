#![allow(clippy::float_cmp, reason = "_")]
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(min_length = 3))]
struct MinLengthStr<'str>(&'str str);

#[rstest]
#[case("abcde")]
fn valid_min_length_str(#[case] input: &str) {
    MinLengthStr::try_new(input).unwrap();
}

#[rstest]
#[case("")]
#[case("abc")]
fn invalid_min_length_str(#[case] input: &str) {
    assert!(matches!(
        MinLengthStr::try_new(input),
        Err(MinLengthStrError::MinLength)
    ));
}
