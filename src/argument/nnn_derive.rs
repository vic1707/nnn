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
/// Derives provided by the crate.
/// Most of them are also available via crates like `derive_more`.
/// Providing them so users aren't required to install other crates for trivial derives.
pub(crate) enum NNNDerive {
    Into,
    TryFrom,
}

impl Parse for NNNDerive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let trait_path = syn::Path::parse(input)?;
        match trait_path
            .segments
            .last()
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    &trait_path,
                    "Trait doesn't have a name ??",
                )
            })?
            .to_token_stream()
            .to_string()
            .as_str()
        {
            "Into" => Ok(Self::Into),
            "TryFrom" => Ok(Self::TryFrom),
            _ => Err(syn::Error::new_spanned(
                trait_path,
                "Unknown `nnn_derive`.",
            )),
        }
    }
}

impl gen::Gen for NNNDerive {
    fn gen_impl(
        &self,
        nnn_type: &crate::NNNType,
    ) -> impl Iterator<Item = gen::Implementation> {
        let type_name = nnn_type.type_name();
        let inner_type = nnn_type.inner_type();
        let error_name = nnn_type.error_name();
        let (impl_generics, ty_generics, where_clause) =
            nnn_type.generics().split_for_impl();

        iter::once(gen::Implementation::ItemImpl(match *self {
            Self::Into => parse_quote! {
                impl #impl_generics ::core::convert::Into<#inner_type> for #type_name #ty_generics #where_clause {
                    fn into(self) -> #inner_type {
                        self.0
                    }
                }
            },
            Self::TryFrom => parse_quote! {
                impl #impl_generics ::core::convert::TryFrom<#inner_type> for #type_name #ty_generics #where_clause {
                    type Error = #error_name;
                    fn try_from(value: #inner_type) -> Result<Self, Self::Error> {
                        Self::try_new(value)
                    }
                }
            },
        }))
    }
}
