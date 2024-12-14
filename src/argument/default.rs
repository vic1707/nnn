/* Crate imports */
use crate::gen;
/* Dependencies */
use quote::quote;

#[derive(Debug)]
pub(crate) enum Default {
    WithInnerDefault,
    WithValue(syn::Expr),
}

impl gen::Gen for Default {
    fn gen_impl(&self, new_type: &crate::NNNType) -> gen::Implementation {
        let inner_type = new_type.inner_type();
        let type_name = new_type.type_name();

        let default_value = match *self {
            Self::WithInnerDefault => quote! { #inner_type::default() },
            Self::WithValue(ref expr) => quote! { #expr },
        };

        gen::Implementation::ImplBlock(
            quote! {
                impl ::core::default::Default for #type_name {
                    fn default() -> Self {
                        #[doc = "Safety: Checked by automatically generated test."]
                        unsafe { Self::try_new(#default_value).unwrap_unchecked() }
                    }
                }
            }
            .into(),
        )
    }

    fn gen_tests(&self, new_type: &crate::NNNType) -> proc_macro2::TokenStream {
        let type_name = new_type.type_name();
        let err_msg = format!("Type `{type_name}` has invalid default value.",);

        quote! {
            #[test]
            fn should_have_valid_default_value() {
                let default_inner_value = #type_name::default().into_inner();
                #type_name::try_new(default_inner_value).expect(#err_msg);
            }
        }
    }
}
