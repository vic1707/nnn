/* Built-in imports */
extern crate alloc;
use alloc::format;
use core::iter;
/* Crate imports */
use super::Validator;
use crate::{codegen, utils::syn_ext::SynPathExt as _};
/* Dependencies */
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
};

pub(crate) enum Derive {
    Eq(syn::Path),
    Ord(syn::Path),
    Deserialize(syn::Path),
    /// A derive that will be passed down transparently.
    Transparent(syn::Path),
}

impl Parse for Derive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let trait_path = syn::Path::parse(input)?;
        match trait_path.item_name()?.as_str() {
            // Special cases
            "Eq" => Ok(Self::Eq(trait_path)),
            "Ord" => Ok(Self::Ord(trait_path)),
            "Deserialize" => Ok(Self::Deserialize(trait_path)),
            // Forbidden derives
            derive_more @ ("Add" | "AddAssign" | "Constructor" | "Div"
            | "DivAssign" | "From" | "FromStr" | "Mul"
            | "MulAssign" | "Neg" | "Rem" | "RemAssign"
            | "Shl" | "ShlAssign" | "Shr" | "ShrAssign"
            | "Sub" | "SubAssign" | "Sum") => Err(syn::Error::new_spanned(
                trait_path,
                format!(
                    "Deriving `{derive_more}` results in a possible bypass of the validators and sanitizers and is therefore forbidden."
                ),
            )),
            "Default" => Err(syn::Error::new_spanned(
                trait_path,
                "To derive the `Default` trait, use the `default` or `default = ..` argument.",
            )),
            _ => Ok(Self::Transparent(trait_path)),
        }
    }
}

impl codegen::Gen for Derive {
    fn gen_impl(
        &self,
        ctx: &crate::Context,
    ) -> impl Iterator<Item = codegen::Implementation> {
        let is_nan_excluded_for_floats = ctx
            .args()
            .validators
            .iter()
            .any(Validator::excludes_float_nan);

        iter::once(match *self {
            Self::Eq(ref path) => {
                if is_nan_excluded_for_floats {
                    let type_name = ctx.type_name();
                    let (impl_generics, ty_generics, where_clause) =
                        ctx.generics().split_for_impl();
                    codegen::Implementation::ItemImpl(parse_quote! {
                        impl #impl_generics #path for #type_name #ty_generics #where_clause {}
                    })
                } else {
                    codegen::Implementation::Attribute(
                        parse_quote! { #[derive(#path)] },
                    )
                }
            },
            Self::Ord(ref path) => {
                if is_nan_excluded_for_floats {
                    let type_name = ctx.type_name();
                    let (impl_generics, ty_generics, where_clause) =
                        ctx.generics().split_for_impl();
                    let panic_msg = format!(
                        "{type_name}::cmp() panicked, because partial_cmp() returned None. Could it be that you're using unsafe {type_name}::new_unchecked() ?"
                    );
                    codegen::Implementation::ItemImpl(parse_quote! {
                        #[expect(clippy::derive_ord_xor_partial_ord, reason = "Manual impl when involving floats.")]
                        impl #impl_generics #path for #type_name #ty_generics #where_clause {
                            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                                self.partial_cmp(other)
                                    .unwrap_or_else(|| panic!(#panic_msg))
                            }
                        }
                    })
                } else {
                    codegen::Implementation::Attribute(
                        parse_quote! { #[derive(#path)] },
                    )
                }
            },
            Self::Deserialize(ref path) => {
                codegen::Implementation::Attribute(parse_quote! {
                    #[derive(#path)]
                    #[serde(try_from = "<Self as nnn::NNNewType>::Inner")]
                })
            },
            Self::Transparent(ref path) => codegen::Implementation::Attribute(
                parse_quote! { #[derive(#path)] },
            ),
        })
    }
}
