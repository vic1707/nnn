/* Built-in imports */
extern crate alloc;
use alloc::string::ToString as _;
use core::iter;
/* Crate imports */
use crate::{
    codegen,
    utils::{closure::CustomFunction, syn_ext::SynParseBufferExt as _},
};
/* Dependencies */
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    token::Comma,
};

pub(crate) enum Sanitizer {
    // Containers
    Each(Punctuated<Self, Comma>),
    Sort,
    Dedup,
    // Strings
    Trim,
    Lowercase,
    Uppercase,
    // Commons
    Custom(CustomFunction),
}

impl Sanitizer {
    fn step(&self) -> syn::Block {
        match *self {
            // Containers
            Self::Each(ref steps) => {
                let inner_steps = steps.iter().map(Self::step);
                parse_quote! {{
                    value = value.into_iter().map(|mut value| {
                        #(#inner_steps;)*
                        value
                    }).collect();
                }}
            },
            Self::Sort => parse_quote! {{ value.sort(); }},
            Self::Dedup => parse_quote! {{ value.dedup(); }},
            // Strings
            Self::Trim => parse_quote! {{ value = value.trim().into(); }},
            Self::Lowercase => parse_quote! {{ value = value.to_lowercase(); }},
            Self::Uppercase => parse_quote! {{ value = value.to_uppercase(); }},
            // Common
            Self::Custom(ref custom) => match *custom {
                CustomFunction::Block(ref block) => parse_quote! { #block },
                CustomFunction::Path(ref path) => {
                    parse_quote! {{ value = #path(value); }}
                },
                CustomFunction::Closure(ref expr_closure) => {
                    parse_quote! {{ value = (#expr_closure)(value); }}
                },
            },
        }
    }
}

impl codegen::Gen for Sanitizer {
    fn gen_impl(
        &self,
        _: &crate::Context,
    ) -> impl Iterator<Item = codegen::Implementation> {
        iter::once(codegen::Implementation::SanitizationStep(self.step()))
    }
}

impl Parse for Sanitizer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let validator = match name.to_string().as_str() {
            // Containers
            "each" => Self::Each(input.parse_parenthesized::<Self>()?),
            "sort" => Self::Sort,
            "dedup" => Self::Dedup,
            // Strings
            "trim" => Self::Trim,
            "lowercase" => Self::Lowercase,
            "uppercase" => Self::Uppercase,
            // Common
            "custom" => Self::Custom(input.parse_equal()?),
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Unknown sanitizer.",
                ));
            },
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(validator)
    }
}
