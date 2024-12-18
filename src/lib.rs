/* Modules */
mod argument;
mod gen;
mod nnn_type;
mod utils;
/* Crate imports */
use argument::{Argument, Arguments};
use nnn_type::NNNType;
/* Dependencies imports */
use quote::quote;
use syn::{parse::Parser as _, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn nnn(
    nnn_args: proc_macro::TokenStream,
    type_definition: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    expand(nnn_args, type_definition)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn expand(
    nnn_args: proc_macro::TokenStream,
    type_definition: proc_macro::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let input: syn::DeriveInput = syn::parse(type_definition)?;
    let original_visibility = input.vis.clone();

    let new_type = NNNType::try_from(input)?;
    let type_name = new_type.type_name();
    let inner_type = new_type.inner_type();
    let error_name = new_type.error_name();
    let (impl_generics, ty_generics, where_clause) =
        new_type.generics().split_for_impl();

    let args = Arguments::from(
        Punctuated::<Argument, syn::Token![,]>::parse_terminated
            .parse(nnn_args)?,
    );

    let tests = args.get_tests(&new_type);
    let impls = args.get_impls(&new_type);
    let (impl_blocks, macro_attrs, bare_impls) =
        gen::Implementation::separate_variants(&impls);

    Ok(quote! {
        #[doc(hidden)]
        mod __private {
            use super::*;

            #(#macro_attrs)*
            #new_type

            #[derive(Debug)]
            pub enum #error_name {}

            impl #impl_generics #type_name #ty_generics #where_clause {
                #[inline]
                #[must_use]
                pub const fn into_inner(self) -> #inner_type {
                    self.0
                }
                #[inline]
                #[must_use]
                pub fn try_new(value: #inner_type) -> Result<Self, #error_name> {
                    Ok(Self(value))
                }

                #(#bare_impls)*
            }

            #(#impl_blocks)*

            #[cfg(test)]
            mod tests {
                use super::*;

                #(#tests)*
            }
        }

        #[allow(clippy::pub_use, reason = "_")]
        #original_visibility use __private::#type_name;
        #[allow(clippy::pub_use, reason = "_")]
        #original_visibility use __private::#error_name;
    })
}
