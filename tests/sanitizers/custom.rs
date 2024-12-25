/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(sanitizers(custom = |value: String| value.to_uppercase() ))]
struct SanitizedWithClosureString(String);

#[rstest]
#[case("HeLLo WoRLd", "HELLO WORLD")]
#[case("mixed_case", "MIXED_CASE")]
fn sanitize_uppercase_with_closure(
    #[case] input: &str,
    #[case] expected: &str,
) {
    let sanitized =
        SanitizedWithClosureString::try_new(input.to_owned()).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "For now we don't pass by reference"
)]
fn sanitizer(value: String) -> String {
    value.to_uppercase()
}
#[nnn(sanitizers(custom = sanitizer))]
struct SanitizedWithFnPathString(String);

#[rstest]
#[case("HeLLo WoRLd", "HELLO WORLD")]
#[case("mixed_case", "MIXED_CASE")]
fn sanitize_uppercase_with_fn_path(
    #[case] input: &str,
    #[case] expected: &str,
) {
    let sanitized =
        SanitizedWithFnPathString::try_new(input.to_owned()).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}

#[nnn(sanitizers(custom = { value = value.to_uppercase(); } ))]
struct SanitizedWithBlockString(String);

#[rstest]
#[case("HeLLo WoRLd", "HELLO WORLD")]
#[case("mixed_case", "MIXED_CASE")]
fn sanitize_uppercase_with_block(#[case] input: &str, #[case] expected: &str) {
    let sanitized =
        SanitizedWithBlockString::try_new(input.to_owned()).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}
