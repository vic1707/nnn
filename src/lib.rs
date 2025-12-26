#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "macros")]
pub use nnn_macros::nnn;

pub trait NNNewType: Sized {
    type Inner;
    type Error;

    fn try_new(value: Self::Inner) -> Result<Self, Self::Error>;
    fn into_inner(self) -> Self::Inner;
    fn sanitize(value: Self::Inner) -> Self::Inner {
        value
    }
}
