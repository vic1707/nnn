/* Built-in imports */
use core::iter;
/* Crate imports */
use crate::codegen;
/* Dependencies */
use syn::parse_quote;

#[derive(Debug)]
pub(crate) struct NewUnchecked;

impl codegen::Gen for NewUnchecked {
    fn gen_impl(
        &self,
        _: &crate::Context,
    ) -> impl Iterator<Item = codegen::Implementation> {
        iter::once(codegen::Implementation::ImplItem(codegen::ImplItem::Fn(
            parse_quote! {
                #[inline]
                #[must_use]
                pub const unsafe fn new_unchecked(inner: <Self as nnn::NNNewType>::Inner) -> Self {
                    Self(inner)
                }
            },
        )))
    }
}
