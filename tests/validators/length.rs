#![allow(clippy::float_cmp, reason = "_")]
/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(length = 5))]
struct ExactLengthStr<'str>(&'str str);

#[rstest]
#[case("abcde")]
fn valid_exact_length_str(#[case] input: &str) {
    ExactLengthStr::try_new(input).unwrap();
}

#[rstest]
#[case("")]
#[case("abc")]
#[case("abcdef")]
fn invalid_exact_length_str(#[case] input: &str) {
    assert!(matches!(
        ExactLengthStr::try_new(input),
        Err(ExactLengthStrError::Length)
    ));
}
