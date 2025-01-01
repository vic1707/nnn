/* Crate imports */
use nnn::nnn;
use crate::utils::sign_tests;

sign_tests!(positive,
    // i_ suite
    i8, valids = [42_i8], invalids = [0_i8, -1_i8],
    i16, valids = [42_i16], invalids = [0_i16, -1_i16],
    i32, valids = [42_i32], invalids = [0_i32, -1_i32],
    i64, valids = [42_i64], invalids = [0_i64, -1_i64],
    i128, valids = [42_i128], invalids = [0_i128, -1_i128],
    isize, valids = [42_isize], invalids = [0_isize, -1_isize],
    // f_ suite
    f32, valids = [3.0_f32], invalids = [0.0_f32, -0.0_f32, -3.0_f32, f32::NAN],
    f64, valids = [3.0_f64], invalids = [0.0_f64, -0.0_f64, -3.0_f64, f64::NAN]
);
