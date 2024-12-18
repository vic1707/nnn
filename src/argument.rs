/* Modules */
mod associated_const;
mod default;
mod derive;
mod new_unchecked;
mod nnn_derive;
/* Crate imports */
use self::{
    associated_const::AssociatedConst, default::Default, derive::Derive,
    new_unchecked::NewUnchecked, nnn_derive::NNNDerive,
};
use crate::gen::{self, Gen as _};
/* Dependencies */
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Debug, Default)]
pub(crate) struct Arguments {
    nnn_derives: Vec<NNNDerive>,
    consts: Vec<AssociatedConst>,
    derives: Vec<Derive>,
    default: Option<Default>,
    new_unchecked: Option<NewUnchecked>,
}

impl Arguments {
    pub(crate) fn get_impls(
        &self,
        new_type: &crate::NNNType,
    ) -> Vec<gen::Implementation> {
        (self.nnn_derives.iter().map(|der| der.gen_impl(new_type)))
            .chain(self.consts.iter().map(|cst| cst.gen_impl(new_type)))
            .chain(self.derives.iter().map(|der| der.gen_impl(new_type)))
            .chain(self.default.iter().map(|def| def.gen_impl(new_type)))
            .chain(self.new_unchecked.iter().map(|nu| nu.gen_impl(new_type)))
            .collect()
    }

    pub(crate) fn get_tests(
        &self,
        new_type: &crate::NNNType,
    ) -> Vec<proc_macro2::TokenStream> {
        (self.nnn_derives.iter().map(|der| der.gen_tests(new_type)))
            .chain(self.consts.iter().map(|cst| cst.gen_tests(new_type)))
            .chain(self.derives.iter().map(|der| der.gen_tests(new_type)))
            .chain(self.default.iter().map(|def| def.gen_tests(new_type)))
            .chain(self.new_unchecked.iter().map(|nu| nu.gen_tests(new_type)))
            .collect()
    }
}

impl From<Punctuated<Argument, Comma>> for Arguments {
    fn from(punctuated_args: Punctuated<Argument, Comma>) -> Self {
        let mut args = Self::default();
        for arg in punctuated_args {
            match arg {
                Argument::NNNDerive(derives) => {
                    args.nnn_derives.extend(derives);
                },
                Argument::Consts(consts) => args.consts.extend(consts),
                Argument::Derive(derives) => args.derives.extend(derives),
                Argument::Default(default) => args.default = Some(default),
                Argument::NewUnchecked(nu) => args.new_unchecked = Some(nu),
            }
        }
        args
    }
}

#[derive(Debug)]
pub(crate) enum Argument {
    NNNDerive(Punctuated<NNNDerive, Comma>),
    Consts(Punctuated<AssociatedConst, Comma>),
    Derive(Punctuated<Derive, Comma>),
    Default(Default),
    NewUnchecked(NewUnchecked),
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let arg = match ident.to_string().as_str() {
            "nnn_derive" => {
                let content;
                syn::parenthesized!(content in input);
                Self::NNNDerive(
                    content
                        .parse_terminated(NNNDerive::parse, syn::Token![,])?,
                )
            },
            "consts" => {
                let content;
                syn::parenthesized!(content in input);
                Self::Consts(
                    content.parse_terminated(
                        AssociatedConst::parse,
                        syn::Token![,],
                    )?,
                )
            },
            "derive" => {
                let content;
                syn::parenthesized!(content in input);
                Self::Derive(
                    content.parse_terminated(Derive::parse, syn::Token![,])?,
                )
            },
            "default" => {
                if input.peek(syn::Token![=]) {
                    let _: syn::Token![=] = input.parse()?;
                    Self::Default(Default::WithValue(
                        input.parse::<syn::Expr>()?,
                    ))
                } else {
                    Self::Default(Default::WithInnerDefault)
                }
            },
            "new_unchecked" => Self::NewUnchecked(NewUnchecked),
            // TODO: remove branch
            _ => {
                return Err(syn::Error::new_spanned(ident, "Unknon argument."))
            },
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(arg)
    }
}
