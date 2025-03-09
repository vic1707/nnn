
/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(validators(max_length = 5))]
struct MaxStr<'str>(&'str str);

#[rstest]
#[case("")]
#[case("abcd")]
fn valid_max_str(#[case] input: &str) {
    MaxStr::try_new(input).unwrap();
}

#[rstest]
#[case("abcde")]
#[case("abcdefgh")]
fn invalid_max_str(#[case] input: &str) {
    assert!(matches!(
        MaxStr::try_new(input),
        Err(MaxStrError::MaxLength)
    ));
}
