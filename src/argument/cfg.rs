/* Crate imports */
use super::{Argument, Arguments};
use crate::gen;
/* Dependencies */
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
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

impl gen::Gen for Cfg {
    fn gen_impl(
        &self,
        new_type: &crate::NNNType,
    ) -> impl Iterator<Item = gen::Implementation> {
        self.args.get_impls(new_type).into_iter().map(|mut r#impl| {
            r#impl.make_conditional(&self.condition);
            r#impl
        })
    }

    fn gen_tests(
        &self,
        new_type: &crate::NNNType,
    ) -> impl Iterator<Item = gen::TestFn> {
        self.args.get_tests(new_type).into_iter().map(|mut test| {
            test.make_conditional(&self.condition);
            test
        })
    }
}
