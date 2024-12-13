/* Crate imports */
use crate::gen;
/* Dependencies */
use quote::quote;

#[derive(Debug)]
pub(crate) struct NewUnchecked;

impl gen::Gen for NewUnchecked {
    fn gen_impl(
        &self,
        _: &syn::Ident,
        inner_type: &syn::Type,
    ) -> gen::Implementation {
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

    fn gen_tests(&self, _: &syn::Ident) -> proc_macro2::TokenStream {
        quote! {}
    }
}
