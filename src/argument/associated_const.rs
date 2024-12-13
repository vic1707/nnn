/* Dependencies */
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
/* Crate imports */
use crate::gen;

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

impl gen::Gen for AssociatedConst {
    fn gen_impl(&self, _: &syn::Ident, _: &syn::Type) -> gen::Implementation {
        let visibility = &self.visibility;
        let const_name = &self.name;
        let value = &self.value;

        gen::Implementation::BareImpl(
            quote! {
                #visibility const #const_name: Self = Self(#value);
            }
            .into(),
        )
    }

    fn gen_tests(&self, type_name: &syn::Ident) -> proc_macro2::TokenStream {
        let const_name = &self.name;

        let err_msg = format!(
            "Type `{type_name}` has invalid value for associated const `{const_name}`.",
        );
        let test_name =
            format_ident!("const_{const_name}_should_have_valid_value");

        quote! {
            #[test]
            fn #test_name() {
                let inner_value = <#type_name>::#const_name.into_inner();
                <#type_name>::try_new(inner_value).expect(#err_msg);
            }
        }
    }
}
