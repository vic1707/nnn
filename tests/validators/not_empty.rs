/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(validators(not_empty))]
struct NonEmptyStr<'str>(&'str str);

#[rstest]
#[case("Hello")]
#[case("A")]
fn valid_not_empty_str(#[case] input: &str) {
    NonEmptyStr::try_new(input).unwrap();
}

#[rstest]
#[case("")]
fn invalid_not_empty_str(#[case] input: &str) {
    assert!(matches!(
        NonEmptyStr::try_new(input),
        Err(NonEmptyStrError::NotEmpty)
    ));
}
