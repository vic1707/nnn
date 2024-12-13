macro_rules! ts_new_type {
    (iter_of $i:ident in $arr:ident) => {
        $arr.iter().filter_map(|item| {
            if let Self::$i(ref el) = *item {
                Some(el)
            } else {
                None
            }
        })
    };
    ($i:ident) => {
        #[derive(Debug)]
        pub(crate) struct $i(proc_macro2::TokenStream);

        impl From<proc_macro2::TokenStream> for $i {
            fn from(ts: proc_macro2::TokenStream) -> Self {
                Self(ts)
            }
        }

        impl quote::ToTokens for $i {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                tokens.extend(self.0.clone());
            }
        }
    };
}

pub(crate) use ts_new_type;
