/* Built-in imports */
use core::iter;
/* Crate imports */
use crate::{
    gen,
    utils::{closure::WithFunction, syn_ext::SynParseBufferExt as _},
};
/* Dependencies */
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Debug)]
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
    With(WithFunction),
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
            Self::Trim => parse_quote! {{ value = value.trim().to_owned(); }},
            Self::Lowercase => {
                parse_quote! {{ value = value.to_lowercase().to_owned(); }}
            },
            Self::Uppercase => {
                parse_quote! {{ value = value.to_uppercase().to_owned(); }}
            },
            // Common
            Self::With(ref with) => match *with {
                WithFunction::Block(ref block) => parse_quote! { #block },
                WithFunction::Path(ref path) => {
                    parse_quote! {{ value = #path(value); }}
                },
                WithFunction::Closure(ref expr_closure) => {
                    parse_quote! {{ value = (#expr_closure)(value); }}
                },
            },
        }
    }
}

impl gen::Gen for Sanitizer {
    fn gen_impl(
        &self,
        _: &crate::NNNType,
    ) -> impl Iterator<Item = gen::Implementation> {
        iter::once(gen::Implementation::SanitizationStep(self.step()))
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
            "with" => Self::With(input.parse_equal()?),
            _ => {
                return Err(syn::Error::new_spanned(name, "Unknown sanitizer."))
            },
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(validator)
    }
}
