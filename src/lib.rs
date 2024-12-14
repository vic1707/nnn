#![expect(clippy::todo, reason = "WIP")]
#![expect(
    clippy::print_stderr,
    clippy::use_debug,
    reason = "Usefull for debuging"
)]
/* Modules */
mod argument;
mod gen;
mod utils;
/* Crate imports */
use argument::{Argument, Arguments};
use utils::syn_ext::SynDataExt as _;
/* Dependencies imports */
use quote::{format_ident, quote};
use syn::{
    parse::Parser as _, punctuated::Punctuated, token::Pub, Fields, Visibility,
};

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
    let mut input: syn::DeriveInput = syn::parse(type_definition)?;
    if let Some(attr) = input.attrs.first() {
        return Err(syn::Error::new_spanned(
            attr,
            "Attributes are not supported; pass additional parameters via `nnn` instead.",
        ));
    }

    let syn::DeriveInput {
        ref data,
        // we make a backup for later use statement
        vis: original_visibility,
        ident: ref type_name,
        ref generics,
        ..
    } = input;
    // Override visibility to public since the struct is in a private module
    input.vis = Visibility::Public(Pub::default());

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let syn::Data::Struct(ref data_struct) = *data else {
        return Err(syn::Error::new(
            input.data.decl_span(),
            "nnn is only supported on structs.",
        ));
    };

    if !matches!(data_struct.fields, Fields::Unnamed(_)) {
        return Err(syn::Error::new_spanned(
            &data_struct.fields,
            "nnn can only be used on structs with unnamed fields.",
        ));
    }
    let mut fields = data_struct.fields.iter();
    let inner_type = match (fields.next(), fields.next()) {
        // Exactly one private field
        (Some(field), None) if matches!(field.vis, Visibility::Inherited) => {
            &field.ty
        },
        (Some(field), None) => {
            return Err(syn::Error::new_spanned(
                &field.vis,
                "nnn can only be used on structs with one unnamed private field.",
            ));
        },
        (_, Some(extra_field)) => {
            return Err(syn::Error::new_spanned(
                extra_field,
                "nnn can only be used on structs with exactly one field.",
            ));
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "nnn can only be used on structs with exactly one field.",
            ));
        },
    };

    let args = Arguments::from(
        Punctuated::<Argument, syn::Token![,]>::parse_terminated
            .parse(nnn_args)?,
    );
    eprintln!("ATTRIBUTES PARSED: {args:#?}");

    let tests = args.get_tests(type_name);
    let impls = args.get_impls(type_name, inner_type);
    let (impl_blocks, macro_attrs, bare_impls) =
        gen::Implementation::separate_variants(&impls);

    let error_name = format_ident!("{type_name}Error");
    Ok(quote! {
        #[doc(hidden)]
        mod __private {
            use super::*;

            #(#macro_attrs)*
            #input

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
