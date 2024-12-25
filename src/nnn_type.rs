/* Crate imports */
use crate::{argument::Arguments, utils::syn_ext::SynDataExt as _};
/* Dependencies */
use quote::{format_ident, quote};

#[derive(Debug)]
pub(crate) struct NNNType {
    inner_field: syn::Field,
    generics: syn::Generics,
    type_name: syn::Ident,
    arguments: Arguments,
}

impl NNNType {
    pub(crate) fn error_name(&self) -> syn::Ident {
        format_ident!("{}Error", self.type_name())
    }

    pub(crate) fn mod_name(&self) -> syn::Ident {
        format_ident!("__private_{}", self.type_name())
    }

    pub(crate) const fn inner_type(&self) -> &syn::Type {
        &self.inner_field.ty
    }

    pub(crate) const fn generics(&self) -> &syn::Generics {
        &self.generics
    }

    pub(crate) const fn type_name(&self) -> &syn::Ident {
        &self.type_name
    }

    pub(crate) const fn args(&self) -> &Arguments {
        &self.arguments
    }
}

impl quote::ToTokens for NNNType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            ref generics,
            ref inner_field,
            ref type_name,
            ..
        } = *self;
        let where_clause = &generics.where_clause;

        tokens.extend(quote! {
            pub struct #type_name #generics (#inner_field) #where_clause;
        });
    }
}

impl TryFrom<(syn::DeriveInput, Arguments)> for NNNType {
    type Error = syn::Error;

    fn try_from((input, arguments): (syn::DeriveInput, Arguments)) -> Result<Self, Self::Error> {
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

        if inner_field.vis != syn::Visibility::Inherited {
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

        Ok(Self {
            generics,
            type_name,
            inner_field: inner_field.clone(),
            arguments,
        })
    }
}
