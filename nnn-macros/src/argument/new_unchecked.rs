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
        _: &crate::Context,
    ) -> impl Iterator<Item = gen::Implementation> {
        iter::once(gen::Implementation::ImplItem(gen::ImplItem::Fn(
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
