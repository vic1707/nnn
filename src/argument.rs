/* Modules */
mod associated_const;
mod default;
mod derive;
mod new_unchecked;
/* Crate imports */
use self::{
    associated_const::AssociatedConst, default::Default, derive::Derive,
    new_unchecked::NewUnchecked,
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
    consts: Vec<AssociatedConst>,
    derives: Vec<Derive>,
    default: Option<Default>,
    new_unchecked: Option<NewUnchecked>,
}

impl Arguments {
    pub(crate) fn get_impls(
        &self,
        type_name: &syn::Ident,
        inner_type: &syn::Type,
    ) -> Vec<gen::Implementation> {
        (self
            .consts
            .iter()
            .map(|cst| cst.gen_impl(type_name, inner_type)))
        .chain(
            self.derives
                .iter()
                .map(|der| der.gen_impl(type_name, inner_type)),
        )
        .chain(
            self.default
                .iter()
                .map(|def| def.gen_impl(type_name, inner_type)),
        )
        .chain(
            self.new_unchecked
                .iter()
                .map(|nu| nu.gen_impl(type_name, inner_type)),
        )
        .collect()
    }
}

impl From<Punctuated<Argument, Comma>> for Arguments {
    fn from(punctuated_args: Punctuated<Argument, Comma>) -> Self {
        let mut args = Self::default();
        for arg in punctuated_args {
            match arg {
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
    Consts(Punctuated<AssociatedConst, Comma>),
    Derive(Punctuated<Derive, Comma>),
    Default(Default),
    NewUnchecked(NewUnchecked),
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let arg = match ident.to_string().as_str() {
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
            // Branch will be removed
            _ => todo!("Unknown argument"),
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(arg)
    }
}
