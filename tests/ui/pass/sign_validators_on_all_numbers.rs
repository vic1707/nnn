#![expect(unused_imports, reason = "_")]

macro_rules! number_wrapper {
    ($($ty:ty),*) => {
        paste::paste! {
            $(
                mod [< signs _ $ty >] {
                    use nnn::nnn;

                    #[nnn(validators(positive, negative))]
                    struct NNN($ty);
                }
            )*
        }
    }
}

number_wrapper!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);

fn main() {}
