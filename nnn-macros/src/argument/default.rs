/* Built-in imports */
extern crate alloc;
use alloc::format;
use core::iter;
/* Crate imports */
use crate::gen;
/* Dependencies */
use quote::quote;
use syn::parse_quote;

pub(crate) enum Default {
    WithInnerDefault,
    WithValue(syn::Expr),
}

impl gen::Gen for Default {
    fn gen_impl(
        &self,
        ctx: &crate::Context,
    ) -> impl Iterator<Item = gen::Implementation> {
        let type_name = ctx.type_name();
        let (impl_generics, ty_generics, where_clause) =
            ctx.generics().split_for_impl();

        let default_value = match *self {
            Self::WithInnerDefault => {
                quote! { <Self as nnn::NNNewType>::Error::default() }
            },
            Self::WithValue(ref expr) => quote! { #expr },
        };

        iter::once(gen::Implementation::ItemImpl(parse_quote! {
            impl #impl_generics ::core::default::Default for #type_name #ty_generics #where_clause {
                fn default() -> Self {
                    #[doc = "Is checked by automatically generated test."]
                    Self(#default_value)
                }
            }
        }))
    }

    fn gen_tests(
        &self,
        ctx: &crate::Context,
    ) -> impl Iterator<Item = gen::TestFn> {
        let type_name = ctx.type_name();
        let err_msg = format!("Type `{type_name}` has invalid default value.",);

        iter::once(parse_quote! {
            #[test]
            fn should_have_valid_default_value() {
                let default_inner = #type_name::default().into_inner();
                let rebuilt_default = #type_name::try_new(default_inner).expect(#err_msg);
                assert_eq!(
                    default_inner,
                    rebuilt_default.into_inner(),
                    "Default and rebuilt default are different, maybe you didn't sanitize your default?"
                );
            }
        })
    }
}
