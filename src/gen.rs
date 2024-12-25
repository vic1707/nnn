/* Modules */
mod impl_item;
mod test_fn;
/* Dependencies */
use syn::{parse_quote, punctuated::Punctuated, token::Comma};
/* Re-exports */
pub(crate) use self::{impl_item::ImplItem, test_fn::TestFn};

pub(crate) trait Gen {
    fn gen_tests(&self, _: &crate::NNNType) -> impl Iterator<Item = TestFn> {
        [].into_iter()
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
    Attribute(Vec<syn::Attribute>),

    ErrorVariant(Punctuated<syn::Variant, Comma>),
    ValidityCheck(syn::Block),
    ErrorDisplayArm(Vec<syn::Arm>),

    SanitizationStep(syn::Block),
}

impl Implementation {
    pub(crate) fn make_conditional(&mut self, condition: &syn::Expr) {
        let cfg_attr: syn::Attribute = parse_quote! { #[cfg(#condition)] };

        match *self {
            Self::ItemImpl(ref mut item_impl) => item_impl.attrs.push(cfg_attr),
            Self::ImplItem(ref mut item_impl) => {
                item_impl.attrs_mut().push(cfg_attr);
            },
            Self::Attribute(ref mut attrs) => {
                for attr in attrs.iter_mut() {
                    let inner = &attr.meta;
                    *attr = parse_quote! { #[cfg_attr(#condition, #inner)]};
                }
            },
            Self::ErrorVariant(ref mut punctuated) => {
                punctuated
                    .iter_mut()
                    .for_each(|variant| variant.attrs.push(cfg_attr.clone()));
            },
            Self::ValidityCheck(ref mut block)
            | Self::SanitizationStep(ref mut block) => {
                *block = parse_quote! {{
                    #cfg_attr
                    #block
                }};
            },
            Self::ErrorDisplayArm(ref mut arms) => {
                arms.iter_mut()
                    .for_each(|arm| arm.attrs.push(cfg_attr.clone()));
            },
        }
    }

    #[expect(clippy::wildcard_enum_match_arm, reason = "Specific extractions.")]
    pub(crate) fn separate_variants(
        impls: &[Self],
    ) -> (
        impl Iterator<Item = &syn::ItemImpl>,
        impl Iterator<Item = &ImplItem>,
        impl Iterator<Item = &syn::Attribute>,
        impl Iterator<Item = &syn::Variant>,
        impl Iterator<Item = &syn::Block>,
        impl Iterator<Item = &syn::Arm>,
        impl Iterator<Item = &syn::Block>,
    ) {
        let impl_blocks = impls.iter().filter_map(|item| match *item {
            Self::ItemImpl(ref el) => Some(el),
            _ => None,
        });
        let impl_items = impls.iter().filter_map(|item| match *item {
            Self::ImplItem(ref el) => Some(el),
            _ => None,
        });
        let proc_macro_attrs = impls
            .iter()
            .filter_map(|item| match *item {
                Self::Attribute(ref el) => Some(el),
                _ => None,
            })
            .flatten();

        let err_variants = impls
            .iter()
            .filter_map(|item| match *item {
                Self::ErrorVariant(ref el) => Some(el),
                _ => None,
            })
            .flatten();
        let validity_checks = impls.iter().filter_map(|item| match *item {
            Self::ValidityCheck(ref el) => Some(el),
            _ => None,
        });
        let err_display_arm = impls
            .iter()
            .filter_map(|item| match *item {
                Self::ErrorDisplayArm(ref el) => Some(el),
                _ => None,
            })
            .flatten();

        let sanitization_steps = impls.iter().filter_map(|item| match *item {
            Self::SanitizationStep(ref el) => Some(el),
            _ => None,
        });

        (
            impl_blocks,
            impl_items,
            proc_macro_attrs,
            err_variants,
            validity_checks,
            err_display_arm,
            sanitization_steps,
        )
    }
}
