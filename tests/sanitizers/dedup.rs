/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(sanitizers(dedup))]
struct SanitizedDedupedVec(Vec<i32>);

#[rstest]
#[case(vec![3_i32, 3_i32, 2_i32, 1_i32, 1_i32, 2_i32], vec![3_i32, 2_i32, 1_i32, 2_i32])] // dedeup only removes consecutives duplicates
#[case(vec![1_i32, 1_i32, 1_i32], vec![1_i32])]
fn sanitize_dedup(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
    let sanitized = SanitizedDedupedVec::try_new(input).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}
