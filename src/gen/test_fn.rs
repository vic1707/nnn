/* Dependencies */
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub(crate) struct TestFn(syn::ItemFn);

impl Parse for TestFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_fn: syn::ItemFn = input.parse()?;

        if !item_fn
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("test"))
        {
            return Err(syn::Error::new_spanned(
                item_fn,
                "Missing #[test] attribute on test function.",
            ));
        }

        Ok(Self(item_fn))
    }
}

impl quote::ToTokens for TestFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
