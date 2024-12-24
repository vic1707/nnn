/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(sanitizers(sort))]
struct SanitizedSortedVec(Vec<i32>);

#[rstest]
#[case(vec![3_i32, 1_i32, 2_i32, 3_i32, 2_i32, 1_i32], vec![1_i32, 1_i32, 2_i32, 2_i32, 3_i32, 3_i32])]
#[case(vec![5_i32, 3_i32, 4_i32], vec![3_i32, 4_i32, 5_i32])]
fn sanitize_sort(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
    let sanitized = SanitizedSortedVec::try_new(input).unwrap();
    assert_eq!(sanitized.into_inner(), expected);
}
