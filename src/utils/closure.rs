/* Dependencies */
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
/// Either
/// with = `path::to::fn`
/// with = |input: Type| { ...; return .. }
/// with = { instructions; }
pub(crate) enum WithFunction {
    /// Path to the function to run of signature
    /// Fn(mut Inner) -> Inner
    /// mut being optional
    Path(syn::Path),
    /// Closure of type
    /// Fn(mut Inner) -> Inner
    /// mut being optional
    Closure(syn::ExprClosure),
    /// Depends on `value` being the input value to sanitize.
    /// Block can be `{ value.sort(); }` or `{ value = value....; }`
    Block(syn::Block),
}

impl Parse for WithFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let closure = if let Ok(path) = input.parse::<syn::Path>() {
            Self::Path(path)
        } else if let Ok(closure) = input.parse::<syn::ExprClosure>() {
            Self::Closure(closure)
        } else if let Ok(block) = input.parse::<syn::Block>() {
            Self::Block(block)
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Invalid `with` argument input.",
            ));
        };

        if !input.peek(syn::Token![,]) && !input.is_empty() {
            return Err(input.error("Unexpected token(s)."));
        }

        Ok(closure)
    }
}
