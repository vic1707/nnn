/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(sanitizers(uppercase))]
struct UppercasedString(String);

#[rstest]
#[case("HeLLo WoRLd", "HELLO WORLD")]
#[case("mixed_case", "MIXED_CASE")]
fn sanitize_uppercase(#[case] input: &str, #[case] expected: &str) {
    let sanitized = UppercasedString::try_new(input.to_owned()).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}
