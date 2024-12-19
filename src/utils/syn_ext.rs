/* Dependencies */
use syn::{
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
}
