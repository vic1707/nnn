/* Dependencies */
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub(crate) struct AssociatedConst {
    visibility: syn::Visibility,
    name: syn::Ident,
    value: syn::Expr,
}

impl Parse for AssociatedConst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = input.parse::<syn::Visibility>()?;
        let name = input.parse::<syn::Ident>()?;
        let _: syn::Token![=] = input.parse()?;
        let value = input.parse::<syn::Expr>()?;
        Ok(Self {
            visibility,
            name,
            value,
        })
    }
}
