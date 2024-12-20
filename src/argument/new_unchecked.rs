/* Built-in imports */
use core::iter;
/* Crate imports */
use crate::gen;
/* Dependencies */
use syn::parse_quote;

#[derive(Debug)]
pub(crate) struct NewUnchecked;

impl gen::Gen for NewUnchecked {
    fn gen_impl(
        &self,
        new_type: &crate::NNNType,
    ) -> impl Iterator<Item = gen::Implementation> {
        let inner_type = new_type.inner_type();

        iter::once(gen::Implementation::ImplItem(gen::ImplItem::Fn(
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
