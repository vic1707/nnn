/* Built-in imports */
extern crate alloc;
use alloc::vec;
/* Crate imports */
use crate::{codegen, utils::syn_ext::SynPathExt as _};
/* Dependencies */
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
};

#[derive(Debug)]
/// Derives provided by the crate.
/// Most of them are also available via crates like `derive_more`.
/// Providing them so users aren't required to install other crates for trivial derives.
pub(crate) enum NNNDerive {
    Into(Option<syn::AngleBracketedGenericArguments>),
    From(Option<syn::AngleBracketedGenericArguments>),
    TryFrom,
    Borrow,
    FromStr,
    IntoIterator,
}

impl NNNDerive {
    fn default_target(
        &self,
        ctx: &crate::Context,
    ) -> syn::AngleBracketedGenericArguments {
        let type_name = ctx.type_name();
        match *self {
            Self::Into(_) => parse_quote! { <<Self as nnn::NNNewType>::Inner> },
            Self::From(_) => {
                parse_quote! { <<#type_name as nnn::NNNewType>::Inner> }
            },
            _ => unreachable!(),
        }
    }
}

impl Parse for NNNDerive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let trait_path = syn::Path::parse(input)?;
        match trait_path.item_name()?.as_str() {
            "Into" => {
                let targets = extract_generics_targets(&trait_path)?;
                Ok(Self::Into(targets))
            },
            "From" => {
                let targets = extract_generics_targets(&trait_path)?;
                Ok(Self::From(targets))
            },
            "TryFrom" => {
                assert_no_generics_params(&trait_path)?;
                Ok(Self::TryFrom)
            },
            "Borrow" => {
                assert_no_generics_params(&trait_path)?;
                Ok(Self::Borrow)
            },
            "FromStr" => {
                assert_no_generics_params(&trait_path)?;
                Ok(Self::FromStr)
            },
            "IntoIterator" => {
                assert_no_generics_params(&trait_path)?;
                Ok(Self::IntoIterator)
            },
            _ => Err(syn::Error::new_spanned(
                trait_path,
                "Unknown `nnn_derive`.",
            )),
        }
    }
}

impl codegen::Gen for NNNDerive {
    fn gen_impl(
        &self,
        ctx: &crate::Context,
    ) -> impl Iterator<Item = codegen::Implementation> {
        let type_name = ctx.type_name();
        let (impl_generics, ty_generics, where_clause) =
            ctx.generics().split_for_impl();

        let impls = match *self {
            Self::Into(ref targets) => {
                targets.clone().unwrap_or(self.default_target(ctx))
                    .args
                    .iter()
                    .map(|target|
                        codegen::Implementation::ItemImpl(parse_quote! {
                            impl #impl_generics ::core::convert::Into<#target> for #type_name #ty_generics #where_clause {
                                fn into(self) -> #target {
                                    self.0.into()
                                }
                            }
                        }))
                    .collect()
            },
            Self::From(ref targets) => {
                targets.clone().unwrap_or(self.default_target(ctx))
                    .args
                    .iter()
                    .map(|target|
                        codegen::Implementation::ItemImpl(parse_quote! {
                            impl #impl_generics ::core::convert::From<#type_name #ty_generics> for #target #where_clause {
                                fn from(value: #type_name #ty_generics) -> #target {
                                    value.0.into()
                                }
                            }
                        }))
                    .collect()
            },
            // TODO: String can do str, Vec can do slices?
            Self::Borrow => {
                vec![codegen::Implementation::ItemImpl(parse_quote! {
                    impl #impl_generics ::core::borrow::Borrow<<Self as nnn::NNNewType>::Inner> for #type_name #ty_generics #where_clause {
                        fn borrow(&self) -> &<Self as nnn::NNNewType>::Inner {
                            &self.0
                        }
                    }
                })]
            },
            Self::TryFrom => {
                vec![codegen::Implementation::ItemImpl(parse_quote! {
                    impl #impl_generics ::core::convert::TryFrom<<Self as nnn::NNNewType>::Inner> for #type_name #ty_generics #where_clause {
                        type Error = <Self as nnn::NNNewType>::Error;
                        fn try_from(value: <Self as nnn::NNNewType>::Inner) -> Result<Self, Self::Error> {
                            <Self as nnn::NNNewType>::try_new(value)
                        }
                    }
                })]
            },
            Self::FromStr => {
                let parse_err_name = format_ident!("{type_name}ParseError");
                vec![
                    codegen::Implementation::Enum(parse_quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        #[non_exhaustive]
                        pub enum #impl_generics #parse_err_name #where_clause {
                            InnerParse(<<#type_name as nnn::NNNewType>::Inner as ::core::str::FromStr>::Err),
                            Validation(<#type_name as nnn::NNNewType>::Error),
                        }
                    }),
                    codegen::Implementation::ItemImpl(parse_quote! {
                        impl #impl_generics ::core::fmt::Display for #parse_err_name #ty_generics #where_clause {
                            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                match *self {
                                    #parse_err_name::InnerParse(ref err) => {
                                        write!(fmt, "Failed to parse {}'s inner: {err:?}.", stringify!(#type_name))
                                    },
                                    #parse_err_name::Validation(ref err) => {
                                        write!(fmt, "Failed to validate parsed: {err}.")
                                    },
                                }
                            }
                        }
                    }),
                    codegen::Implementation::ItemImpl(parse_quote! {
                        impl #impl_generics ::core::str::FromStr for #type_name #ty_generics #where_clause {
                            type Err = #parse_err_name;

                            fn from_str(input: &str) -> ::core::result::Result<Self, Self::Err> {
                                <Self as nnn::NNNewType>::try_new(
                                    input.parse().map_err(#parse_err_name::InnerParse)?
                                ).map_err(#parse_err_name::Validation)
                            }
                        }
                    }),
                ]
            },
            Self::IntoIterator => {
                let lifetime: syn::GenericParam = parse_quote! { '__iter_ref };
                let generics_with_lifetime = {
                    let mut generics = ctx.generics().clone();
                    generics.params.push(lifetime.clone());
                    generics
                };

                vec![
                    codegen::Implementation::ItemImpl(parse_quote! {
                        impl #impl_generics ::core::iter::IntoIterator for #type_name #ty_generics #where_clause {
                            type Item = <<Self as nnn::NNNewType>::Inner as ::core::iter::IntoIterator>::Item;
                            type IntoIter = <<Self as nnn::NNNewType>::Inner as ::core::iter::IntoIterator>::IntoIter;

                            fn into_iter(self) -> Self::IntoIter {
                                self.0.into_iter()
                            }
                        }
                    }),
                    codegen::Implementation::ItemImpl(parse_quote! {
                        impl #generics_with_lifetime ::core::iter::IntoIterator for &#lifetime #type_name #ty_generics #where_clause {
                            type Item = <&#lifetime <#type_name #ty_generics as nnn::NNNewType>::Inner as ::core::iter::IntoIterator>::Item;
                            type IntoIter = <&#lifetime <#type_name #ty_generics as nnn::NNNewType>::Inner as ::core::iter::IntoIterator>::IntoIter;

                            fn into_iter(self) -> Self::IntoIter {
                                self.0.iter()
                            }
                        }
                    }),
                ]
            },
        };

        impls.into_iter()
    }
}

fn extract_generics_targets(
    trait_path: &syn::Path,
) -> syn::Result<Option<syn::AngleBracketedGenericArguments>> {
    match trait_path.trait_segment().cloned()?.arguments {
        // if no arguments were given to the trait e.g: Into instead of Into<Target>
        // we insert the new-type's inner type as the target
        syn::PathArguments::None => Ok(None),
        syn::PathArguments::AngleBracketed(args) if args.args.is_empty() => Err(syn::Error::new_spanned(
            args,
            "Please provide generics arguments, or omit the '<>' for the default derive.",
        )),
        syn::PathArguments::AngleBracketed(args) => Ok(Some(args)),
        syn::PathArguments::Parenthesized(args) => {
            Err(syn::Error::new_spanned(
                args,
                "Trait aren't allowed to take parenthesized generics arguments.",
            ))
        },
    }
}

fn assert_no_generics_params(trait_path: &syn::Path) -> syn::Result<()> {
    let args = trait_path.trait_segment().cloned()?.arguments;
    match args {
        syn::PathArguments::None => Ok(()),
        _ => Err(syn::Error::new_spanned(
            args,
            "Trait isn't allowed to take generics arguments.",
        )),
    }
}
