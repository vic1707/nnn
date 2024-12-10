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
