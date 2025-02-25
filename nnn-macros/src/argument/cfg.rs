/* Crate imports */
use super::{Argument, Arguments};
use crate::codegen;
/* Dependencies */
use syn::parse::{Parse, ParseStream};

pub(crate) struct Cfg {
    condition: syn::Expr,
    args: Arguments,
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let condition = content.parse()?;
        content.parse::<syn::Token![,]>()?;
        let args = Arguments::from(
            content.parse_terminated(Argument::parse, syn::Token![,])?,
        );

        Ok(Self { condition, args })
    }
}

impl codegen::Gen for Cfg {
    fn gen_impl(
        &self,
        ctx: &crate::Context,
    ) -> impl Iterator<Item = codegen::Implementation> {
        self.args.get_impls(ctx).into_iter().map(|mut r#impl| {
            r#impl.make_conditional(&self.condition);
            r#impl
        })
    }

    fn gen_tests(
        &self,
        ctx: &crate::Context,
    ) -> impl Iterator<Item = codegen::TestFn> {
        self.args.get_tests(ctx).into_iter().map(|mut test| {
            test.make_conditional(&self.condition);
            test
        })
    }
}
