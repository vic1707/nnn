/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(max_length_or_eq = 5))]
struct MaxLengthOrEqStr<'str>(&'str str);

#[rstest]
#[case("")]
#[case("abcde")]
fn valid_max_length_or_eq_str(#[case] input: &str) {
    MaxLengthOrEqStr::try_new(input).unwrap();
}

#[rstest]
#[case("abcdef")]
#[case("abcdefgh")]
fn invalid_max_length_or_eq_str(#[case] input: &str) {
    assert!(matches!(
        MaxLengthOrEqStr::try_new(input),
        Err(MaxLengthOrEqStrError::MaxLengthOrEq)
    ));
}
