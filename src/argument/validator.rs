/* Crate imports */
use crate::utils::{regex_input::RegexInput, syn_ext::SynParseBufferExt as _};
/* Dependencies */
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Debug)]
pub(crate) enum Validator {
    // Containers
    NotEmpty,
    Each(Punctuated<Validator, Comma>),
    MinLength(syn::Lit),
    Length(syn::Lit),
    MaxLength(syn::Lit),
    // Numerics
    Min(syn::Lit),
    MinOrEq(syn::Lit),
    Exactly(syn::Lit),
    Max(syn::Lit),
    MaxOrEq(syn::Lit),
    // Float specifics
    Finite,
    NotNAN,
    // String specifics
    Regex(RegexInput),
    // Commons
    // TODO: also takes in an error type
    // With
}

impl Parse for Validator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let validator = match name.to_string().as_str() {
            // Containers
            "not_empty" => Self::NotEmpty,
            "each" => Self::Each(input.parse_parenthesized::<Self>()?),
            "min_length" => Self::MinLength(input.parse_equal::<syn::Lit>()?),
            "length" => Self::Length(input.parse_equal::<syn::Lit>()?),
            "max_length" => Self::MaxLength(input.parse_equal::<syn::Lit>()?),
            // Numerics
            "min" => Self::Min(input.parse_equal::<syn::Lit>()?),
            "min_or_eq" => Self::MinOrEq(input.parse_equal::<syn::Lit>()?),
            "exactly" => Self::Exactly(input.parse_equal::<syn::Lit>()?),
            "max" => Self::Max(input.parse_equal::<syn::Lit>()?),
            "max_or_eq" => Self::MaxOrEq(input.parse_equal::<syn::Lit>()?),
            // Float specifics
            "finite" => Self::Finite,
            "not_nan" => Self::NotNAN,
            // String specifics
            "regex" => Self::Regex(input.parse_equal::<RegexInput>()?),
            _ => {
                return Err(syn::Error::new_spanned(name, "Unknown validator."))
            },
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(validator)
    }
}
