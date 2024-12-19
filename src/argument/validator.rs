/* Crate imports */
use crate::utils::regex_input::RegexInput;
/* Dependencies */
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Debug)]
pub(crate) enum Validator {
    // Containers
    Each(Punctuated<Validator, Comma>),
    MinLength(syn::Lit),
    Length(syn::Lit),
    MaxLength(syn::Lit),
    NotEmpty,
    // Numbers
    Min(syn::Lit),
    MinOrEq(syn::Lit),
    Exactly(syn::Lit),
    Max(syn::Lit),
    MaxOrEq(syn::Lit),
    // Floats
    Finite,
    NotNAN,
    // String
    Regex(RegexInput),
    // Commons
    // TODO: also takes in an error type
    // With
}

impl Parse for Validator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let validator = match name.to_string().as_str() {
            "each" => {
                let content;
                syn::parenthesized!(content in input);
                Self::Each(
                    content.parse_terminated(Self::parse, syn::Token![,])?,
                )
            },
            "min_length" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<syn::Lit>()?;
                Self::MinLength(value)
            },
            "length" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<syn::Lit>()?;
                Self::Length(value)
            },
            "max_length" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<syn::Lit>()?;
                Self::MaxLength(value)
            },
            "not_empty" => Self::NotEmpty,
            "min" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<syn::Lit>()?;
                Self::Min(value)
            },
            "min_or_eq" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<syn::Lit>()?;
                Self::MinOrEq(value)
            },
            "exactly" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<syn::Lit>()?;
                Self::Exactly(value)
            },
            "max" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<syn::Lit>()?;
                Self::Max(value)
            },
            "max_or_eq" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<syn::Lit>()?;
                Self::MaxOrEq(value)
            },
            "finite" => Self::Finite,
            "not_nan" => Self::NotNAN,
            "regex" => {
                input.parse::<syn::Token![=]>()?;
                let value = input.parse::<RegexInput>()?;
                Self::Regex(value)
            },
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
