/* Modules */
mod impl_item;
mod test_fn;
/* Re-exports */
pub(crate) use self::{impl_item::ImplItem, test_fn::TestFn};

pub(crate) trait Gen {
    fn gen_tests(&self, _: &crate::NNNType) -> Option<TestFn> {
        None
    }

    fn gen_impl(
        &self,
        new_type: &crate::NNNType,
    ) -> impl Iterator<Item = Implementation>;
}

#[derive(Debug)]
pub(crate) enum Implementation {
    /// an impl block
    ItemImpl(syn::ItemImpl),
    /// an item within an impl block
    ImplItem(ImplItem),
    /// A macro attribute for the generated [`crate::NNNType`]
    Attribute(syn::Attribute),
}

impl Implementation {
    pub(crate) fn separate_variants(
        impls: &[Self],
    ) -> (
        impl Iterator<Item = &syn::ItemImpl>,
        impl Iterator<Item = &ImplItem>,
        impl Iterator<Item = &syn::Attribute>,
    ) {
        let impl_blocks = impls.iter().filter_map(|item| match *item {
            Self::ItemImpl(ref el) => Some(el),
            Self::ImplItem(_) | Self::Attribute(_) => None,
        });
        let impl_items = impls.iter().filter_map(|item| match *item {
            Self::ImplItem(ref el) => Some(el),
            Self::ItemImpl(_) | Self::Attribute(_) => None,
        });
        let proc_macro_attrs = impls.iter().filter_map(|item| match *item {
            Self::Attribute(ref el) => Some(el),
            Self::ItemImpl(_) | Self::ImplItem(_) => None,
        });

        (impl_blocks, impl_items, proc_macro_attrs)
    }
}
