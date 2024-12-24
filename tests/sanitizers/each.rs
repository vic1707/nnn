/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(sanitizers(each(trim, lowercase)))]
struct SanitizedVec(Vec<String>);

#[rstest]
#[case(vec!["  HELLO ", " WoRLD  "], vec!["hello", "world"])]
fn sanitize_each_trim_lowercase(
    #[case] input: Vec<&str>,
    #[case] expected: Vec<&str>,
) {
    let input_as_strings = input.into_iter().map(String::from).collect();
    let sanitized = SanitizedVec::try_new(input_as_strings).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}
