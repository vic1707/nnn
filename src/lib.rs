#![expect(clippy::needless_pass_by_value, unused_variables, reason = "WIP")]
/* Dependencies imports */
use quote::quote;
use syn::{token::Pub, Visibility};

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
        // we make a backup for later use statement
        vis: original_visibility,
        ident: ref type_name,
        ..
    } = input;
    // Override visibility to public since the struct is in a private module
    input.vis = Visibility::Public(Pub::default());

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
