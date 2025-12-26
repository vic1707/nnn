/* Built-in imports */
extern crate alloc;
use alloc::{
    format,
    string::{String, ToString},
};
/* Crate imports */
use crate::utils;
/* Dependencies */
use quote::ToTokens as _;
use syn::{
    PathSegment,
    parse::{Parse, ParseBuffer},
    punctuated::Punctuated,
    token::Comma,
};

pub(crate) trait SynDataExt {
    fn decl_span(&self) -> proc_macro2::Span;
}

impl SynDataExt for syn::Data {
    fn decl_span(&self) -> proc_macro2::Span {
        match *self {
            Self::Struct(ref data_struct) => data_struct.struct_token.span,
            Self::Enum(ref data_enum) => data_enum.enum_token.span,
            Self::Union(ref data_union) => data_union.union_token.span,
        }
    }
}

pub(crate) trait SynParseBufferExt {
    fn parse_equal<T: Parse>(&self) -> syn::Result<T>;
    fn parse_assign<T: Parse>(&self) -> syn::Result<(syn::Ident, T)>;
    fn parse_parenthesized<T: Parse>(
        &self,
    ) -> syn::Result<Punctuated<T, Comma>>;
    fn require_ident(&self, name: &str) -> syn::Result<syn::Ident>;
}

impl SynParseBufferExt for ParseBuffer<'_> {
    fn parse_equal<T: Parse>(&self) -> syn::Result<T> {
        self.parse::<syn::Token![=]>()?;
        T::parse(self)
    }

    fn parse_assign<T: Parse>(&self) -> syn::Result<(syn::Ident, T)> {
        let name = self.parse::<syn::Ident>()?;
        let value = self.parse_equal::<T>()?;
        Ok((name, value))
    }

    fn parse_parenthesized<T: Parse>(
        &self,
    ) -> syn::Result<Punctuated<T, Comma>> {
        let content;
        syn::parenthesized!(content in self);
        content.parse_terminated(T::parse, syn::Token![,])
    }

    fn require_ident(&self, name: &str) -> syn::Result<syn::Ident> {
        let ident = self.parse::<syn::Ident>()?;
        if ident != name {
            return Err(syn::Error::new_spanned(
                ident,
                format!("Expected ident to be `{name}`."),
            ));
        }
        Ok(ident)
    }
}

pub(crate) trait SynPathExt {
    fn as_ident(&self) -> syn::Ident;
    fn trait_segment(&self) -> syn::Result<&PathSegment>;
    fn item_name(&self) -> syn::Result<String>;
}

impl SynPathExt for syn::Path {
    /// turns `std::io::Error` into `StdIoError`
    fn as_ident(&self) -> syn::Ident {
        quote::format_ident!(
            "{}",
            self.to_token_stream()
                .to_string()
                .to_ascii_lowercase()
                .replace(':', "")
                .split_ascii_whitespace()
                .map(utils::capitalize)
                .collect::<String>()
        )
    }

    fn trait_segment(&self) -> syn::Result<&PathSegment> {
        self.segments.last().ok_or_else(|| {
            syn::Error::new_spanned(self, "Trait doesn't have a name ??")
        })
    }

    fn item_name(&self) -> syn::Result<String> {
        self.trait_segment()
            .map(|seg| &seg.ident)
            .map(ToString::to_string)
    }
}
