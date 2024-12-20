/* Built-in imports */
use core::iter;
/* Crate imports */
use crate::gen;
/* Dependencies */
use quote::ToTokens as _;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
};

#[derive(Debug)]
pub(crate) enum Derive {
    Deserialize(syn::Path),
    /// A derive that will be passed down transparently.
    Transparent(syn::Path),
}

impl Parse for Derive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let trait_path = syn::Path::parse(input)?;
        match trait_path
            .segments
            .last()
            .ok_or_else(|| syn::Error::new_spanned(&trait_path, "Trait doesn't have a name ??"))?
            .to_token_stream()
            .to_string()
            .as_str()
        {
            // Available via crates like `derive_more`.
            "From" => Err(syn::Error::new_spanned(
                trait_path,
                "Deriving `From` results in a possible bypass of the validators and sanitizers and is therefore forbidden."
            )),
            "Default" => Err(syn::Error::new_spanned(
                trait_path,
                "To derive the `Default` trait, use the `default` or `default = ..` argument."
            )),
            "Deserialize" => Ok(Self::Deserialize(trait_path)),
            _ => Ok(Self::Transparent(trait_path)),
        }
    }
}

impl gen::Gen for Derive {
    fn gen_impl(
        &self,
        new_type: &crate::NNNType,
    ) -> impl Iterator<Item = gen::Implementation> {
        iter::once(gen::Implementation::Attribute(match *self {
            Self::Deserialize(ref path) => {
                let inner_type =
                    new_type.inner_type().to_token_stream().to_string();
                parse_quote! {
                    #[derive(#path)]
                    #[serde(try_from = #inner_type)]
                }
            },
            Self::Transparent(ref path) => parse_quote! { #[derive(#path)] },
        }))
    }
}
