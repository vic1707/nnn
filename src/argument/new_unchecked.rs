/* Crate imports */
use crate::gen;
/* Dependencies */
use quote::quote;

#[derive(Debug)]
pub(crate) struct NewUnchecked;

impl gen::Gen for NewUnchecked {
    fn gen_impl(&self, new_type: &crate::NNNType) -> gen::Implementation {
        let inner_type = new_type.inner_type();

        gen::Implementation::BareImpl(
            quote! {
                #[inline]
                #[must_use]
                pub const unsafe fn new_unchecked(inner: #inner_type) -> Self {
                    Self(inner)
                }
            }
            .into(),
        )
    }

    fn gen_tests(&self, _: &crate::NNNType) -> proc_macro2::TokenStream {
        quote! {}
    }
}
