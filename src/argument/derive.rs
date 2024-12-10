/* Dependencies */
use quote::ToTokens as _;
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub(crate) enum Derive {
    /// A derive that will be passed down transparently.
    Transparent(syn::Path),
}

impl Derive {
    const fn trait_name(&self) -> &syn::Path {
        match *self {
            Self::Transparent(ref path) => path,
        }
    }
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
            _ => Ok(Self::Transparent(trait_path)),
        }
    }
}
