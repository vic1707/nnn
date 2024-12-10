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
