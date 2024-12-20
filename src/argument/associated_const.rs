/* Built-in imports */
use core::iter;
/* Crate imports */
use crate::{gen, utils::syn_ext::SynParseBufferExt as _};
/* Dependencies */
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
};

#[derive(Debug)]
pub(crate) struct AssociatedConst {
    visibility: syn::Visibility,
    name: syn::Ident,
    value: syn::Expr,
}

impl Parse for AssociatedConst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = input.parse::<syn::Visibility>()?;
        let (name, value) = input.parse_assign::<syn::Expr>()?;
        Ok(Self {
            visibility,
            name,
            value,
        })
    }
}

impl gen::Gen for AssociatedConst {
    fn gen_impl(
        &self,
        _: &crate::NNNType,
    ) -> impl Iterator<Item = gen::Implementation> {
        let visibility = &self.visibility;
        let const_name = &self.name;
        let value = &self.value;

        iter::once(gen::Implementation::ImplItem(gen::ImplItem::Const(
            parse_quote! {
                #visibility const #const_name: Self = Self(#value);
            },
        )))
    }

    fn gen_tests(&self, new_type: &crate::NNNType) -> Option<gen::TestFn> {
        let const_name = &self.name;
        let type_name = new_type.type_name();

        let err_msg = format!(
            "Type `{type_name}` has invalid value for associated const `{const_name}`.",
        );
        let test_name =
            format_ident!("const_{const_name}_should_have_valid_value");

        Some(parse_quote! {
            #[test]
            fn #test_name() {
                let inner_value = <#type_name>::#const_name.into_inner();
                <#type_name>::try_new(inner_value).expect(#err_msg);
            }
        })
    }
}
