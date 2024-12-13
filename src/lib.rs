#![expect(clippy::needless_pass_by_value, unused_variables, reason = "WIP")]
/* Modules */
mod utils;
/* Crate imports */
use utils::syn_ext::SynDataExt as _;
/* Dependencies imports */
use quote::quote;
use syn::{token::Pub, Fields, Visibility};

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
        ..
    } = input;
    // Override visibility to public since the struct is in a private module
    input.vis = Visibility::Public(Pub::default());

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
    match (fields.next(), fields.next()) {
        // Exactly one private field
        (Some(field), None) if matches!(field.vis, Visibility::Inherited) => (),
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
    }

    Ok(quote! {
        #[doc(hidden)]
        mod __private {
            use super::*;

            #input

            #[cfg(test)]
            mod tests {
                use super::*;
            }
        }

        #[allow(clippy::pub_use, reason = "_")]
        #original_visibility use __private::#type_name;
    })
}
