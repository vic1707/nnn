/* Modules */
mod associated_const;
mod cfg;
mod default;
mod derive;
mod new_unchecked;
mod nnn_derive;
mod sanitizer;
mod validator;
/* Built-in imports */
extern crate alloc;
use alloc::{string::ToString as _, vec, vec::Vec};
/* Crate imports */
use self::{
    associated_const::AssociatedConst, cfg::Cfg, default::Default,
    derive::Derive, new_unchecked::NewUnchecked, nnn_derive::NNNDerive,
    sanitizer::Sanitizer, validator::Validator,
};
use crate::{
    codegen::{self, Gen as _},
    utils::syn_ext::SynParseBufferExt as _,
};
/* Dependencies */
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Default)]
pub(crate) struct Arguments {
    nnn_derives: Vec<NNNDerive>,
    consts: Vec<AssociatedConst>,
    derives: Vec<Derive>,
    default: Option<Default>,
    new_unchecked: Option<NewUnchecked>,
    sanitizers: Vec<Sanitizer>,
    validators: Vec<Validator>,
    cfgs: Vec<Cfg>,
    transparents: Vec<syn::Meta>,
}

impl Arguments {
    pub(crate) fn get_impls(
        &self,
        ctx: &crate::Context,
    ) -> Vec<codegen::Implementation> {
        (self.nnn_derives.iter().flat_map(|der| der.gen_impl(ctx)))
            .chain(self.cfgs.iter().flat_map(|cfg| cfg.gen_impl(ctx)))
            .chain(self.consts.iter().flat_map(|cst| cst.gen_impl(ctx)))
            .chain(self.derives.iter().flat_map(|der| der.gen_impl(ctx)))
            .chain(self.default.iter().flat_map(|def| def.gen_impl(ctx)))
            .chain(self.new_unchecked.iter().flat_map(|nu| nu.gen_impl(ctx)))
            .chain(self.sanitizers.iter().flat_map(|san| san.gen_impl(ctx)))
            .chain(self.validators.iter().flat_map(|val| val.gen_impl(ctx)))
            .chain(
                self.transparents
                    .iter()
                    .map(|meta| parse_quote! { #[#meta] })
                    .map(|attr| codegen::Implementation::Attribute(vec![attr])),
            )
            .collect()
    }

    pub(crate) fn get_tests(
        &self,
        ctx: &crate::Context,
    ) -> Vec<codegen::TestFn> {
        (self.nnn_derives.iter().flat_map(|der| der.gen_tests(ctx)))
            .chain(self.cfgs.iter().flat_map(|cfg| cfg.gen_tests(ctx)))
            .chain(self.consts.iter().flat_map(|cst| cst.gen_tests(ctx)))
            .chain(self.derives.iter().flat_map(|der| der.gen_tests(ctx)))
            .chain(self.default.iter().flat_map(|def| def.gen_tests(ctx)))
            .chain(self.new_unchecked.iter().flat_map(|nu| nu.gen_tests(ctx)))
            .chain(self.sanitizers.iter().flat_map(|san| san.gen_tests(ctx)))
            .chain(self.validators.iter().flat_map(|val| val.gen_tests(ctx)))
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
                Argument::Cfg(cfg) => args.cfgs.push(cfg),
                Argument::Consts(consts) => args.consts.extend(consts),
                Argument::Derive(derives) => args.derives.extend(derives),
                Argument::Default(default) => args.default = Some(default),
                Argument::NewUnchecked(nu) => args.new_unchecked = Some(nu),
                Argument::Sanitizers(sanitizers) => {
                    args.sanitizers.extend(sanitizers);
                },
                Argument::Validators(validators) => {
                    args.validators.extend(validators);
                },
                Argument::Transparent(metas) => {
                    args.transparents.extend(metas);
                },
            }
        }
        args
    }
}

#[allow(clippy::large_enum_variant)]
pub(crate) enum Argument {
    NNNDerive(Punctuated<NNNDerive, Comma>),
    Cfg(Cfg),
    Consts(Punctuated<AssociatedConst, Comma>),
    Derive(Punctuated<Derive, Comma>),
    Default(Default),
    NewUnchecked(NewUnchecked),
    Sanitizers(Punctuated<Sanitizer, Comma>),
    Validators(Punctuated<Validator, Comma>),
    Transparent(Punctuated<syn::Meta, Comma>),
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let arg = match ident.to_string().as_str() {
            "nnn_derive" => {
                Self::NNNDerive(input.parse_parenthesized::<NNNDerive>()?)
            },
            "cfg" => Self::Cfg(input.parse::<Cfg>()?),
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
            "sanitizers" => {
                Self::Sanitizers(input.parse_parenthesized::<Sanitizer>()?)
            },
            "validators" => {
                Self::Validators(input.parse_parenthesized::<Validator>()?)
            },
            "attrs" => {
                Self::Transparent(input.parse_parenthesized::<syn::Meta>()?)
            },
            _ => {
                return Err(syn::Error::new_spanned(ident, "Unknown argument."))
            },
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(arg)
    }
}
