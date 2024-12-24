/* Modules */
mod argument;
mod gen;
mod nnn_type;
mod utils;
/* Built-in imports */
use std::collections::HashSet;
/* Crate imports */
use argument::{Argument, Arguments};
use nnn_type::NNNType;
/* Dependencies imports */
use quote::{format_ident, quote};
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
    let (
        impl_blocks,
        bare_impls,
        macro_attrs,
        err_variants,
        validity_checks,
        err_display_arm,
        sanitization_steps,
    ) = gen::Implementation::separate_variants(&impls);

    let dedup_err_variants = err_variants.collect::<HashSet<_>>().into_iter();

    let mod_name = format_ident!("__private_{}", new_type.type_name());
    Ok(quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, reason = "Includes NNNType name which is probably CamelCase.")]
        mod #mod_name {
            use super::*;

            #(#macro_attrs)*
            #new_type

            #[derive(Debug)]
            pub enum #error_name {
                #(#dedup_err_variants),*
            }

            impl ::core::error::Error for #error_name {}

            impl ::core::fmt::Display for #error_name {
                fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match *self {
                        #(#err_display_arm)*
                    }
                }
            }

            impl #impl_generics #type_name #ty_generics #where_clause {
                #[inline]
                #[must_use]
                // TODO: Can it be const ?
                pub fn into_inner(self) -> #inner_type {
                    self.0
                }

                #[inline]
                #[must_use]
                pub fn try_new(mut value: #inner_type) -> Result<Self, #error_name> {
                    value = Self::sanitize(value);
                    #(#validity_checks)*
                    Ok(Self(value))
                }

                #[inline]
                fn sanitize(mut value: #inner_type) -> #inner_type {
                    #(#sanitization_steps)*
                    value
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

        #[allow(clippy::pub_use, reason = "pub use can happen if struct is meant to be public.")]
        #original_visibility use #mod_name::#type_name;
        #[allow(clippy::pub_use, reason = "pub use can happen if struct is meant to be public.")]
        #original_visibility use #mod_name::#error_name;
    })
}
