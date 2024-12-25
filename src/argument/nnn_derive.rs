/* Crate imports */
use crate::gen;
/* Dependencies */
use quote::{format_ident, ToTokens as _};
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
    From,
    TryFrom,
    Borrow,
    BorrowMut,
    FromStr,
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
            "From" => Ok(Self::From),
            "TryFrom" => Ok(Self::TryFrom),
            "Borrow" => Ok(Self::Borrow),
            "BorrowMut" => Ok(Self::BorrowMut),
            "FromStr" => Ok(Self::FromStr),
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
        let mod_name = nnn_type.mod_name();
        let type_name = nnn_type.type_name();
        let inner_type = nnn_type.inner_type();
        let error_name = nnn_type.error_name();
        let (impl_generics, ty_generics, where_clause) =
            nnn_type.generics().split_for_impl();

        let impls = match *self {
            Self::Into => vec![gen::Implementation::ItemImpl(parse_quote! {
                impl #impl_generics ::core::convert::Into<#inner_type> for #type_name #ty_generics #where_clause {
                    fn into(self) -> #inner_type {
                        self.0
                    }
                }
            })],
            Self::From => vec![gen::Implementation::ItemImpl(parse_quote! {
                impl #impl_generics ::core::convert::From<#type_name #ty_generics> for #inner_type #where_clause {
                    fn from(value: #type_name #ty_generics) -> #inner_type {
                        value.0
                    }
                }
            })],
            // TODO: String can do str, Vec can do slices?
            Self::Borrow => vec![gen::Implementation::ItemImpl(parse_quote! {
                impl #impl_generics ::core::borrow::Borrow<#inner_type> for #type_name #ty_generics #where_clause {
                    fn borrow(&self) -> &#inner_type {
                        &self.0
                    }
                }
            })],
            Self::BorrowMut => {
                vec![gen::Implementation::ItemImpl(parse_quote! {
                    impl #impl_generics ::core::borrow::BorrowMut<#inner_type> for #type_name #ty_generics #where_clause {
                        fn borrow_mut(&mut self) -> &mut #inner_type {
                            &mut self.0
                        }
                    }
                })]
            },
            Self::TryFrom => {
                vec![gen::Implementation::ItemImpl(parse_quote! {
                    impl #impl_generics ::core::convert::TryFrom<#inner_type> for #type_name #ty_generics #where_clause {
                        type Error = #error_name;
                        fn try_from(value: #inner_type) -> Result<Self, Self::Error> {
                            Self::try_new(value)
                        }
                    }
                })]
            },
            Self::FromStr => {
                let parse_err_name = format_ident!("{type_name}ParseError");
                vec![
                    gen::Implementation::Enum(parse_quote! {
                        #[derive(Debug)]
                        pub enum #impl_generics #parse_err_name #where_clause {
                            InnerParse(<#inner_type as ::core::str::FromStr>::Err),
                            Validation(#error_name),
                        }
                    }),
                    gen::Implementation::Export(
                        parse_quote! { use #mod_name::#parse_err_name; },
                    ),
                    gen::Implementation::ItemImpl(parse_quote! {
                        impl #impl_generics ::core::fmt::Display for #parse_err_name #ty_generics #where_clause {
                            fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                match *self {
                                    #parse_err_name::InnerParse(ref err) => {
                                        write!(fmt, "Failed to parse {}: {err:?}.", stringify!(#inner_type))
                                    },
                                    #parse_err_name::Validation(ref err) => {
                                        write!(fmt, "Failed to validate parsed: {err}.")
                                    },
                                }
                            }
                        }
                    }),
                    gen::Implementation::ItemImpl(parse_quote! {
                        impl #impl_generics ::core::str::FromStr for #type_name #ty_generics #where_clause {
                            type Err = #parse_err_name;

                            fn from_str(input: &str) -> ::core::result::Result<Self, Self::Err> {
                                let parsed = <#inner_type>::from_str(input).map_err(#parse_err_name::InnerParse)?;
                                Self::try_new(parsed).map_err(#parse_err_name::Validation)
                            }
                        }
                    }),
                ]
            },
        };

        impls.into_iter()
    }
}
