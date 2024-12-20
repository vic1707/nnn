/* Modules */
mod associated_const;
mod default;
mod derive;
mod new_unchecked;
mod nnn_derive;
mod validator;
/* Crate imports */
use self::{
    associated_const::AssociatedConst, default::Default, derive::Derive,
    new_unchecked::NewUnchecked, nnn_derive::NNNDerive, validator::Validator,
};
use crate::{
    gen::{self, Gen as _},
    utils::syn_ext::SynParseBufferExt as _,
};
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
    validators: Vec<Validator>,
}

impl Arguments {
    pub(crate) fn get_impls(
        &self,
        new_type: &crate::NNNType,
    ) -> Vec<gen::Implementation> {
        (self
            .nnn_derives
            .iter()
            .flat_map(|der| der.gen_impl(new_type)))
        .chain(self.consts.iter().flat_map(|cst| cst.gen_impl(new_type)))
        .chain(self.derives.iter().flat_map(|der| der.gen_impl(new_type)))
        .chain(self.default.iter().flat_map(|def| def.gen_impl(new_type)))
        .chain(
            self.new_unchecked
                .iter()
                .flat_map(|nu| nu.gen_impl(new_type)),
        )
        .collect()
    }

    pub(crate) fn get_tests(
        &self,
        new_type: &crate::NNNType,
    ) -> Vec<gen::TestFn> {
        (self.nnn_derives.iter().map(|der| der.gen_tests(new_type)))
            .chain(self.consts.iter().map(|cst| cst.gen_tests(new_type)))
            .chain(self.derives.iter().map(|der| der.gen_tests(new_type)))
            .chain(self.default.iter().map(|def| def.gen_tests(new_type)))
            .chain(self.new_unchecked.iter().map(|nu| nu.gen_tests(new_type)))
            .flatten()
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
                Argument::Validators(validators) => {
                    args.validators.extend(validators);
                },
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
    Validators(Punctuated<Validator, Comma>),
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let arg = match ident.to_string().as_str() {
            "nnn_derive" => {
                Self::NNNDerive(input.parse_parenthesized::<NNNDerive>()?)
            },
            "consts" => {
                Self::Consts(input.parse_parenthesized::<AssociatedConst>()?)
            },
            "derive" => Self::Derive(input.parse_parenthesized::<Derive>()?),
            "default" => {
                Self::Default(match input.parse_equal::<syn::Expr>() {
                    Ok(expr) => Default::WithValue(expr),
                    Err(_err) => Default::WithInnerDefault,
                })
            },
            "new_unchecked" => Self::NewUnchecked(NewUnchecked),
            "validators" => {
                Self::Validators(input.parse_parenthesized::<Validator>()?)
            },
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
