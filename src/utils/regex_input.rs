/* Dependencies */
use quote::{quote, ToTokens as _};
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub(crate) enum RegexInput {
    StringLiteral(syn::LitStr),
    Path(syn::Path),
}

impl Parse for RegexInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let regex = if let Ok(lit_str) = input.parse::<syn::LitStr>() {
            /* TODO: Enable this properly
            // Compile time check for literal regex
            regex::Regex::new(&lit_str.value()).map_err(|err| {
                syn::Error::new_spanned(
                    &lit_str,
                    format!("Incorrect Regex {err}"),
                )
            })?;
            */
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

// TODO: is moche
impl RegexInput {
    pub(crate) fn in_code_access_to_str(&self) -> proc_macro2::TokenStream {
        match *self {
            Self::StringLiteral(ref lit_str) => lit_str.to_token_stream(),
            Self::Path(ref path) => quote! { #path.as_str() },
        }
    }

    pub(crate) fn decl(
        &self,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        match *self {
            Self::StringLiteral(ref lit_str) => {
                let err_message =
                    format!("'{}' is an invalid Regex.", lit_str.value());
                (
                    quote! { ::std::sync::LazyLock<::regex::Regex> },
                    quote! { ::std::sync::LazyLock::new(|| ::regex::Regex::new(&#lit_str).expect(#err_message)) },
                )
            },
            Self::Path(ref path) => {
                (quote! { ::regex::Regex }, quote! { #path })
            },
        }
    }
}
