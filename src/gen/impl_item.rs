#[derive(Debug)]
/// Basically [`syn::ImplItem`] without the `Verbatim` variant.
pub(crate) enum ImplItem {
    Const(syn::ImplItemConst),
    Fn(syn::ImplItemFn),
    Macro(syn::ImplItemMacro),
    Type(syn::ImplItemType),
}

impl quote::ToTokens for ImplItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            Self::Const(ref r#const) => r#const.to_tokens(tokens),
            Self::Fn(ref r#fn) => r#fn.to_tokens(tokens),
            Self::Macro(ref r#macro) => r#macro.to_tokens(tokens),
            Self::Type(ref r#type) => r#type.to_tokens(tokens),
        }
    }
}

impl From<syn::ImplItemConst> for ImplItem {
    fn from(value: syn::ImplItemConst) -> Self {
        Self::Const(value)
    }
}

impl From<syn::ImplItemFn> for ImplItem {
    fn from(value: syn::ImplItemFn) -> Self {
        Self::Fn(value)
    }
}

impl From<syn::ImplItemMacro> for ImplItem {
    fn from(value: syn::ImplItemMacro) -> Self {
        Self::Macro(value)
    }
}

impl From<syn::ImplItemType> for ImplItem {
    fn from(value: syn::ImplItemType) -> Self {
        Self::Type(value)
    }
}