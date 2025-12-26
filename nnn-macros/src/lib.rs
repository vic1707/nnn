#![doc = include_str!("../README.md")]
#![no_std]
/* Modules */
mod argument;
mod codegen;
mod ctx;
mod utils;
/* Built-in imports */
extern crate alloc;
use alloc::collections::BTreeMap;
/* Crate imports */
use argument::{Argument, Arguments};
use ctx::Context;
/* Dependencies imports */
use quote::quote;
use syn::{parse::Parser as _, punctuated::Punctuated};
use utils::syn_ext::SynDataExt as _;

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

    let args = Arguments::from(
        Punctuated::<Argument, syn::Token![,]>::parse_terminated
            .parse(nnn_args)?,
    );

    let (type_name, inner_type, generics) = split_derive_input(input.clone())?;
    let ctx = Context::try_from((input, args))?;
    let (impl_generics, ty_generics, where_clause) =
        ctx.generics().split_for_impl();

    let tests = ctx.args().get_tests(&ctx);
    let impls = ctx.args().get_impls(&ctx);
    let (
        impl_blocks,
        bare_impls,
        macro_attrs,
        err_variants,
        validity_checks,
        err_display_arm,
        sanitization_steps,
        new_enums,
        custom_test_harness,
    ) = codegen::Implementation::separate_variants(&impls);

    let dedup_err_variants = err_variants
        .map(|variant| (variant.ident.clone(), variant))
        .collect::<BTreeMap<_, _>>()
        .into_values();

    let error_type = quote::format_ident!("{type_name}Error",);
    let mod_name = quote::format_ident!("__private_{type_name}",);

    Ok(quote! {
        #[doc(hidden)]
        #[allow(non_snake_case, reason = "Includes NNNType name which is probably CamelCase.")]
        #[allow(clippy::module_name_repetitions, reason = "Includes NNNType which is probably the name of the file.")]
        mod #mod_name {
            use super::*;

            #(#macro_attrs)*
            pub struct #type_name #generics (#inner_type) #where_clause;

            #[derive(Debug, Clone, PartialEq, Eq)]
            #[non_exhaustive]
            pub enum #error_type {
                #(#dedup_err_variants),*
            }

            impl ::core::error::Error for #error_type {}

            impl ::core::fmt::Display for #error_type {
                fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match *self {
                        #(#err_display_arm)*
                    }
                }
            }

            impl #impl_generics nnn::NNNewType for #type_name #ty_generics #where_clause {
                type Inner = #inner_type;
                type Error = #error_type;

                fn sanitize(mut value: Self::Inner) -> Self::Inner {
                    #(#sanitization_steps;)*
                    value
                }

                fn try_new(mut value: Self::Inner) -> Result<Self, Self::Error> {
                    value = Self::sanitize(value);
                    #(#validity_checks;)*
                    Ok(Self(value))
                }

                fn into_inner(self) -> Self::Inner {
                    self.0
                }
            }

            impl #impl_generics #type_name #ty_generics #where_clause {
                #(#bare_impls)*
            }

            #(#impl_blocks)*

            #(#new_enums)*

            #[cfg(test)]
            #custom_test_harness
            mod tests {
                use super::*;

                #(#tests)*
            }
        }

        #[allow(clippy::pub_use, reason = "pub use can happen if struct is meant to be public.")]
        #original_visibility use #mod_name::*;
    })
}

fn split_derive_input(
    input: syn::DeriveInput,
) -> Result<(syn::Ident, syn::Type, syn::Generics), syn::Error> {
    if let Some(attr) = input.attrs.first() {
        return Err(syn::Error::new_spanned(
            attr,
            "Attributes are not supported; pass additional parameters via `nnn` instead.",
        ));
    }

    let syn::DeriveInput {
        data,
        ident: type_name,
        generics,
        ..
    } = input;

    let syn::Data::Struct(data_struct) = data else {
        return Err(syn::Error::new(
            data.decl_span(),
            "nnn is only supported on structs.",
        ));
    };

    let syn::Fields::Unnamed(syn::FieldsUnnamed {
        unnamed: fields, ..
    }) = data_struct.fields
    else {
        return Err(syn::Error::new_spanned(
            data_struct.fields,
            "`nnn` can only be used on structs with unnamed fields.",
        ));
    };

    let mut fields_iter = fields.iter();
    let Some(inner_field) = fields_iter.next() else {
        return Err(syn::Error::new_spanned(
            fields,
            "Cannot use `nnn` on empty structs.",
        ));
    };

    if !matches!(inner_field.vis, syn::Visibility::Inherited) {
        return Err(syn::Error::new_spanned(
            &inner_field.vis,
            "You can only have a private field here.",
        ));
    }

    if let Some(extra_field) = fields_iter.next() {
        return Err(syn::Error::new_spanned(
            extra_field,
            "You cannot have more than one field.",
        ));
    }

    Ok((type_name, inner_field.ty.clone(), generics))
}
