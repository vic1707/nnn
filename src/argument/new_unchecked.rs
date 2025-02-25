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
        new_type: &crate::NNNType,
    ) -> impl Iterator<Item = codegen::Implementation> {
        let inner_type = new_type.inner_type();

        iter::once(codegen::Implementation::ImplItem(codegen::ImplItem::Fn(
            parse_quote! {
                #[inline]
                #[must_use]
                pub const unsafe fn new_unchecked(inner: #inner_type) -> Self {
                    Self(inner)
                }
            },
        )))
    }
}
