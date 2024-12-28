/* Crate imports */
use crate::{
    gen,
    utils::{
        closure::CustomFunction,
        regex_input::RegexInput,
        syn_ext::{SynParseBufferExt as _, SynPathExt as _},
    },
};
/* Dependencies */
use quote::{format_ident, ToTokens as _};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Debug)]
pub(crate) enum Validator {
    // Containers
    NotEmpty,
    Each(Punctuated<Self, Comma>),
    MinLength(syn::Expr),
    MinLengthOrEq(syn::Expr),
    Length(syn::Expr),
    MaxLength(syn::Expr),
    MaxLengthOrEq(syn::Expr),
    // Numerics
    Min(syn::Expr),
    MinOrEq(syn::Expr),
    Max(syn::Expr),
    MaxOrEq(syn::Expr),
    Positive,
    Negative,
    // Float specifics
    Finite,
    NotInfinite,
    NotNAN,
    // String specifics
    Regex(RegexInput),
    // Commons
    Exactly(syn::Expr),
    Custom {
        check: CustomFunction,
        error: syn::Path,
        /// derived from error path
        error_name: syn::Ident,
    },
    Predicate {
        check: CustomFunction,
        // if optional and defaults to `Predicate`.
        variant: syn::Ident,
    },
}

impl gen::Gen for Validator {
    fn gen_impl(
        &self,
        new_type: &crate::NNNType,
    ) -> impl Iterator<Item = gen::Implementation> {
        [
            gen::Implementation::ErrorVariant(self.variant()),
            gen::Implementation::ErrorDisplayArm(self.display_arm(new_type)),
            gen::Implementation::ValidityCheck(self.check(new_type)),
        ]
        .into_iter()
    }

    fn gen_tests(
        &self,
        _: &crate::NNNType,
    ) -> impl Iterator<Item = gen::TestFn> {
        if let Self::Regex(ref regex) = *self {
            let check: syn::Stmt = match *regex {
                RegexInput::Path(ref path) => {
                    parse_quote! { ::regex::Regex::new(#path.as_str()).unwrap(); }
                },
                RegexInput::StringLiteral(ref lit) => {
                    parse_quote! { ::regex::Regex::new(&#lit).unwrap(); }
                },
            };

            vec![parse_quote! {
                #[test]
                fn regex_validator_should_be_valid() {
                    #check
                }
            }]
        } else {
            vec![]
        }.into_iter()
    }
}

#[expect(clippy::too_many_lines, reason = "Lots of validators.")]
impl Validator {
    pub(crate) fn excludes_float_nan(&self) -> bool {
        #[expect(clippy::wildcard_enum_match_arm, reason = "_")]
        // Note: Self::Cfg will always be false. See the readme.
        match *self {
            Self::Each(ref steps) => steps.iter().any(Self::excludes_float_nan),
            Self::Finite | Self::NotNAN => true,
            _ => false,
        }
    }

    pub(crate) fn variant(&self) -> Punctuated<syn::Variant, Comma> {
        match *self {
            // Containers
            Self::NotEmpty => parse_quote! { NotEmpty },
            Self::Each(ref steps) => {
                let steps_variants = steps.iter().map(Self::variant);
                parse_quote! {
                    Each(usize, Box<Self>),
                    #(#steps_variants),*
                }
            },
            Self::MinLength(_) => parse_quote! { MinLength },
            Self::MinLengthOrEq(_) => parse_quote! { MinLengthOrEq },
            Self::Length(_) => parse_quote! { Length },
            Self::MaxLength(_) => parse_quote! { MaxLength },
            Self::MaxLengthOrEq(_) => parse_quote! { MaxLengthOrEq },
            // Numerics
            Self::Min(_) => parse_quote! { Min },
            Self::MinOrEq(_) => parse_quote! { MinOrEq },
            Self::Max(_) => parse_quote! { Max },
            Self::MaxOrEq(_) => parse_quote! { MaxOrEq },
            Self::Positive => parse_quote! { Positive },
            Self::Negative => parse_quote! { Negative },
            // Float specifics
            Self::Finite => parse_quote! { Finite },
            Self::NotInfinite => parse_quote! { NotInfinite },
            Self::NotNAN => parse_quote! { NotNAN },
            // String specifics
            Self::Regex(_) => parse_quote! { Regex },
            // Commons
            Self::Exactly(_) => parse_quote! { Exactly },
            Self::Custom {
                ref error,
                ref error_name,
                ..
            } => {
                parse_quote! { #error_name(#error) }
            },
            Self::Predicate { ref variant, .. } => {
                parse_quote! { #variant }
            },
        }
    }

    pub(crate) fn check(&self, new_type: &crate::NNNType) -> syn::Block {
        let error_type = new_type.error_name();
        match *self {
            // Containers
            Self::NotEmpty => {
                parse_quote! {{ if value.is_empty() { return Err(#error_type::NotEmpty) } }}
            },
            Self::Each(ref checks) => {
                let inner_branches =
                    checks.iter().map(|val| val.check(new_type));
                parse_quote! {{
                    // TODO: is .cloned() really the solution ?
                    value.iter().cloned().enumerate().try_for_each(
                        |(idx, value)| {
                            // Used to avoid short circuits from `return` statements in branches
                            let check = || {
                                #(#inner_branches;)*
                                Ok(())
                            };
                            check().map_err(|err| #error_type::Each(idx, Box::new(err)))
                        }
                    )?
                }}
            },
            Self::MinLength(ref val) => {
                parse_quote! {{ if !(value.len() > #val) { return Err(#error_type::MinLength) } }}
            },
            Self::MinLengthOrEq(ref val) => {
                parse_quote! {{ if !(value.len() >= #val) { return Err(#error_type::MinLengthOrEq) } }}
            },
            Self::Length(ref val) => {
                parse_quote! {{ if value.len() != #val { return Err(#error_type::Length) } }}
            },
            Self::MaxLength(ref val) => {
                parse_quote! {{ if !(value.len() < #val) { return Err(#error_type::MaxLength) } }}
            },
            Self::MaxLengthOrEq(ref val) => {
                parse_quote! {{ if !(value.len() <= #val) { return Err(#error_type::MaxLengthOrEq) } }}
            },
            // Numerics
            Self::Min(ref val) => {
                parse_quote! {{ if !(value > #val) { return Err(#error_type::Min) } }}
            },
            Self::MinOrEq(ref val) => {
                parse_quote! {{ if !(value >= #val) { return Err(#error_type::MinOrEq) } }}
            },
            Self::Max(ref val) => {
                parse_quote! {{ if !(value < #val) { return Err(#error_type::Max) } }}
            },
            Self::MaxOrEq(ref val) => {
                parse_quote! {{ if !(value <= #val) { return Err(#error_type::MaxOrEq) } }}
            },
            Self::Positive => {
                parse_quote! {{
                    #[allow(deprecated, reason = "Allows transparency between signed numbers and floats.")]
                    if ! value.is_positive() { return Err(#error_type::Positive) }
                }}
            },
            Self::Negative => {
                parse_quote! {{
                    #[allow(deprecated, reason = "Allows transparency between signed numbers and floats.")]
                    if ! value.is_negative() { return Err(#error_type::Negative) }
                }}
            },
            // Float specifics
            Self::Finite => {
                parse_quote! {{ if ! value.is_finite() { return Err(#error_type::Finite) } }}
            },
            Self::NotInfinite => {
                parse_quote! {{ if value.is_infinite() { return Err(#error_type::NotInfinite) } }}
            },
            Self::NotNAN => {
                parse_quote! {{ if value.is_nan() { return Err(#error_type::NotNAN) } }}
            },
            // String specifics
            Self::Regex(ref regex) => {
                let condition: syn::Block = match *regex {
                    RegexInput::Path(ref path) => {
                        parse_quote! {{ ! #path.is_match(&value) }}
                    },
                    RegexInput::StringLiteral(ref lit) => {
                        let err = format!("Invalid Regex`{}`.", lit.value());
                        parse_quote! {{
                            static REGEX_TO_MATCH: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| ::regex::Regex::new(&#lit).expect(#err));
                            ! REGEX_TO_MATCH.is_match(&value)
                        }}
                    },
                };
                parse_quote! {{
                    if #condition { return Err(#error_type::Regex) }
                }}
            },
            // Commons
            Self::Exactly(ref val) => {
                parse_quote! {{
                    #[allow(clippy::float_cmp, reason = "Allows transparency between signed numbers and floats.")]
                    if value != #val { return Err(#error_type::Exactly) }
                }}
            },
            Self::Custom {
                ref check,
                ref error_name,
                ..
            } => match *check {
                CustomFunction::Block(ref block) => parse_quote! {{
                    if let Err(err) = #block { return Err(#error_type::#error_name(err)) }
                }},
                CustomFunction::Path(ref func) => parse_quote! {{
                    if let Err(err) = #func(&value) { return Err(#error_type::#error_name(err)) }
                }},
                CustomFunction::Closure(ref expr_closure) => parse_quote! {{
                    if let Err(err) = (#expr_closure)(&value) { return Err(#error_type::#error_name(err)) }
                }},
            },
            Self::Predicate {
                ref check,
                ref variant,
                ..
            } => match *check {
                CustomFunction::Block(ref block) => parse_quote! {{
                    if !#block { return Err(#error_type::#variant) }
                }},
                CustomFunction::Path(ref func) => parse_quote! {{
                    if !#func(&value) { return Err(#error_type::#variant) }
                }},
                CustomFunction::Closure(ref expr_closure) => {
                    parse_quote! {{
                        if !(#expr_closure)(&value) { return Err(#error_type::#variant) }
                    }}
                },
            },
        }
    }

    pub(crate) fn display_arm(
        &self,
        new_type: &crate::NNNType,
    ) -> Vec<syn::Arm> {
        let type_name = new_type.type_name();
        match *self {
            // Containers
            Self::NotEmpty => {
                let msg = format!("[{type_name}] Value should not empty.");
                parse_quote! { Self::NotEmpty => write!(fmt, #msg), }
            },
            Self::Each(ref steps) => {
                let steps_fmt =
                    steps.iter().flat_map(|step| step.display_arm(new_type));
                parse_quote! {
                    Self::Each(ref idx, ref inner_err) => write!(fmt, "[{}] Error: '{inner_err}', at index {idx}.", stringify!(#type_name)),
                    #(#steps_fmt)*
                }
            },
            Self::MinLength(ref val) => {
                let msg = format!(
                    "[{type_name}] Length should be greater than {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::MinLength => write!(fmt, #msg), }
            },
            Self::MinLengthOrEq(ref val) => {
                let msg = format!(
                    "[{type_name}] Length should be greater or equal to {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::MinLengthOrEq => write!(fmt, #msg), }
            },
            Self::Length(ref val) => {
                let msg = format!(
                    "[{type_name}] Length should be exactly {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::Length => write!(fmt, #msg), }
            },
            Self::MaxLength(ref val) => {
                let msg = format!(
                    "[{type_name}] Length should be lesser than {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::MaxLength => write!(fmt, #msg), }
            },
            Self::MaxLengthOrEq(ref val) => {
                let msg = format!(
                    "[{type_name}] Length should be lesser or equal to {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::MaxLengthOrEq => write!(fmt, #msg), }
            },
            // Numerics
            Self::Min(ref val) => {
                let msg = format!(
                    "[{type_name}] Value should be greater than {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::Min => write!(fmt, #msg), }
            },
            Self::MinOrEq(ref val) => {
                let msg = format!(
                    "[{type_name}] Value should be greater or equal to {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::MinOrEq => write!(fmt, #msg), }
            },
            Self::Max(ref val) => {
                let msg = format!(
                    "[{type_name}] Value should be lesser than {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::Max => write!(fmt, #msg), }
            },
            Self::MaxOrEq(ref val) => {
                let msg = format!(
                    "[{type_name}] Value should be lesser or equal to {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::MaxOrEq => write!(fmt, #msg), }
            },
            Self::Positive => {
                let msg = format!("[{type_name}] Value should be positive.");
                parse_quote! { Self::Positive => write!(fmt, #msg), }
            },
            Self::Negative => {
                let msg = format!("[{type_name}] Value should be negative.");
                parse_quote! { Self::Negative => write!(fmt, #msg), }
            },
            // Float specifics
            Self::Finite => {
                let msg = format!(
                    "[{type_name}] Value should not be NAN nor infinite."
                );
                parse_quote! { Self::Finite => write!(fmt, #msg), }
            },
            Self::NotInfinite => {
                let msg =
                    format!("[{type_name}] Value should not be infinite.");
                parse_quote! { Self::NotInfinite => write!(fmt, #msg), }
            },
            Self::NotNAN => {
                let msg = format!("[{type_name}] Value should not be NAN.");
                parse_quote! { Self::NotNAN => write!(fmt, #msg), }
            },
            // String specifics
            Self::Regex(ref regex) => {
                let regex_expression_display = match *regex {
                    RegexInput::StringLiteral(ref lit) => {
                        quote::quote! { #lit }
                    },
                    RegexInput::Path(ref path) => {
                        quote::quote! { #path.as_str() }
                    },
                };
                parse_quote! {
                    Self::Regex => write!(fmt, "[{}] Value should match `{}`.", stringify!(#type_name), #regex_expression_display),
                }
            },
            // Commons
            Self::Exactly(ref val) => {
                let msg = format!(
                    "[{type_name}] Value should be exactly {}.",
                    val.to_token_stream()
                );
                parse_quote! { Self::Exactly => write!(fmt, #msg), }
            },
            Self::Custom { ref error_name, .. } => {
                let msg =
                    format!("[{type_name}] Value should be exactly {{}}.");
                parse_quote! { Self::#error_name(ref inner_err) => write!(fmt, #msg, inner_err), }
            },
            Self::Predicate { ref variant, .. } => {
                let msg = format!("[{type_name}] {variant} violated.");
                parse_quote! { Self::#variant => write!(fmt, #msg), }
            },
        }
    }
}

impl Parse for Validator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let validator = match name.to_string().as_str() {
            // Containers
            "not_empty" => Self::NotEmpty,
            "each" => Self::Each(input.parse_parenthesized::<Self>()?),
            "min_length" => Self::MinLength(input.parse_equal()?),
            "min_length_or_eq" => Self::MinLengthOrEq(input.parse_equal()?),
            "length" => Self::Length(input.parse_equal()?),
            "max_length" => Self::MaxLength(input.parse_equal()?),
            "max_length_or_eq" => Self::MaxLengthOrEq(input.parse_equal()?),
            // Numerics
            "min" => Self::Min(input.parse_equal()?),
            "min_or_eq" => Self::MinOrEq(input.parse_equal()?),
            "exactly" => Self::Exactly(input.parse_equal()?),
            "max" => Self::Max(input.parse_equal()?),
            "max_or_eq" => Self::MaxOrEq(input.parse_equal()?),
            "positive" => Self::Positive,
            "negative" => Self::Negative,
            // Float specifics
            "finite" => Self::Finite,
            "not_infinite" => Self::NotInfinite,
            "not_nan" => Self::NotNAN,
            // String specifics
            "regex" => Self::Regex(input.parse_equal()?),
            "custom" => {
                let content;
                syn::parenthesized!(content in input);

                content.require_ident("with")?;
                let check = content.parse_equal()?;

                content.parse::<syn::Token![,]>()?;

                content.require_ident("error")?;
                let error = content.parse_equal::<syn::Path>()?;
                let error_name = error.as_ident();

                Self::Custom {
                    check,
                    error,
                    error_name,
                }
            },
            "predicate" => {
                let content;
                syn::parenthesized!(content in input);

                content.require_ident("with")?;
                let check = content.parse_equal()?;

                let variant = if content.parse::<syn::Token![,]>().is_ok() {
                    content.require_ident("error_name")?;
                    content.parse_equal::<syn::Ident>()?
                } else {
                    format_ident!("Predicate")
                };

                Self::Predicate { check, variant }
            },
            _ => {
                return Err(syn::Error::new_spanned(name, "Unknown validator."))
            },
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(validator)
    }
}
