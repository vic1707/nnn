/* Modules */
mod impl_item;
mod test_fn;
/* Dependencies */
use syn::{punctuated::Punctuated, token::Comma};
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

    ErrorVariant(Punctuated<syn::Variant, Comma>),
    ValidityCheck(syn::Block),
    ErrorDisplayArm(Punctuated<syn::Arm, Comma>),
}

impl Implementation {
    #[expect(clippy::type_complexity, reason = "LGTM")]
    #[expect(clippy::wildcard_enum_match_arm, reason = "Specific extractions.")]
    pub(crate) fn separate_variants(
        impls: &[Self],
    ) -> (
        impl Iterator<Item = &syn::ItemImpl>,
        impl Iterator<Item = &ImplItem>,
        impl Iterator<Item = &syn::Attribute>,
        impl Iterator<Item = &Punctuated<syn::Variant, Comma>>,
        impl Iterator<Item = &syn::Block>,
        impl Iterator<Item = &Punctuated<syn::Arm, Comma>>,
    ) {
        let impl_blocks = impls.iter().filter_map(|item| match *item {
            Self::ItemImpl(ref el) => Some(el),
            _ => None,
        });
        let impl_items = impls.iter().filter_map(|item| match *item {
            Self::ImplItem(ref el) => Some(el),
            _ => None,
        });
        let proc_macro_attrs = impls.iter().filter_map(|item| match *item {
            Self::Attribute(ref el) => Some(el),
            _ => None,
        });

        let err_variants = impls.iter().filter_map(|item| match *item {
            Self::ErrorVariant(ref el) => Some(el),
            _ => None,
        });
        let validity_checks = impls.iter().filter_map(|item| match *item {
            Self::ValidityCheck(ref el) => Some(el),
            _ => None,
        });
        let err_display_arm = impls.iter().filter_map(|item| match *item {
            Self::ErrorDisplayArm(ref el) => Some(el),
            _ => None,
        });

        (
            impl_blocks,
            impl_items,
            proc_macro_attrs,
            err_variants,
            validity_checks,
            err_display_arm,
        )
    }
}
