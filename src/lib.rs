#![expect(
    clippy::needless_pass_by_value,
    clippy::todo,
    unused_variables,
    reason = "WIP"
)]

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
    if let Some(attr) = input.attrs.first() {
        return Err(syn::Error::new_spanned(
            attr,
            "Attributes are not supported; pass additional parameters via `nnn` instead.",
        ));
    }

    todo!();
}
