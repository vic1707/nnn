/* Dependencies */
use syn::parse::{Parse, ParseStream};

pub(crate) enum RegexInput {
    StringLiteral(syn::LitStr),
    Path(syn::Path),
}

impl Parse for RegexInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let regex = if let Ok(lit_str) = input.parse::<syn::LitStr>() {
            #[cfg(feature = "regex_validation")]
            {
                extern crate alloc;
                // Compile time check for literal regex
                regex::Regex::new(&lit_str.value()).map_err(|err| {
                    syn::Error::new_spanned(
                        &lit_str,
                        ::alloc::format!("Incorrect Regex {err}"),
                    )
                })?;
            };
            Self::StringLiteral(lit_str)
        } else {
            Self::Path(input.parse::<syn::Path>()?)
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(regex)
    }
}
