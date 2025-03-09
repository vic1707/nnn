/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(sanitizers(lowercase))]
struct LowercasedString(String);

#[rstest]
#[case("HeLLo WoRLd", "hello world")]
#[case("MIXED_CASE", "mixed_case")]
fn sanitize_lowercase(#[case] input: &str, #[case] expected: &str) {
    let sanitized = LowercasedString::try_new(input.to_owned()).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}
