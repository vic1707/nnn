/* Crate imports */
use nnn::nnn;
/* Dependencies */
use rstest::rstest;

#[nnn(validators(each(finite, min = 0.0_f64)))]
struct FiniteFloatsVec(Vec<f64>);

#[rstest]
#[case(vec![])]
#[case(vec![0.1_f64, 1.5_f64, 2.7_f64])]
fn valid_each_vec(#[case] input: Vec<f64>) {
    FiniteFloatsVec::try_new(input).unwrap();
}

#[rstest]
#[case(vec![-0.1_f64, 1.5_f64, 2.7_f64])]
#[case(vec![0.1_f64, f64::INFINITY, 2.7_f64])]
fn invalid_each_vec(#[case] input: Vec<f64>) {
    assert!(matches!(
        FiniteFloatsVec::try_new(input),
        Err(FiniteFloatsVecError::Each(_, _))
    ));
}


#[nnn(validators(not_empty, each(not_empty, each(finite))))]
struct VecFiniteFloatVec(Vec<Vec<f64>>);

#[rstest]
#[case(vec![vec![2.7_f64]])]
#[case(vec![vec![0.1_f64, 1.5_f64], vec![2.7_f64, 3.8_f64, 3.0_f64]])]
fn valid_vec_of_vecs_of_finite_floats(#[case] input: Vec<Vec<f64>>) {
    VecFiniteFloatVec::try_new(input).unwrap();
}

#[rstest]
#[case(vec![vec![]])]
#[case(vec![vec![-0.1_f64, f64::INFINITY]])]
#[case(vec![vec![-0.1_f64], vec![]])]
fn invalid_vec_of_finite_floats(#[case] input: Vec<Vec<f64>>) {
    assert!(matches!(
        VecFiniteFloatVec::try_new(input),
        Err(VecFiniteFloatVecError::Each(_, _))
    ));
}
