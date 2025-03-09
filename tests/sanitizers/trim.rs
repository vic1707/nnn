/* Crate imports */
use nnn::{nnn, NNNewType as _};
/* Dependencies */
use rstest::rstest;

#[nnn(sanitizers(trim))]
struct SanitizedString(String);

#[rstest]
#[case("  hello world  ", "hello world")]
#[case("\n\t trim this!  ", "trim this!")]
fn sanitize_trim(#[case] input: &str, #[case] expected: &str) {
    let sanitized = SanitizedString::try_new(input.to_owned()).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}
