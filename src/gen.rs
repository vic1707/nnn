/* Modules */
mod gen_utils;
/* Crate imports */
use self::gen_utils::ts_new_type;

pub(crate) trait Gen {
    fn gen_tests(&self, new_type: &crate::NNNType) -> proc_macro2::TokenStream;
    fn gen_impl(&self, new_type: &crate::NNNType) -> Implementation;
}

#[derive(Debug)]
pub(crate) enum Implementation {
    ImplBlock(ImplBlock),
    MacroAttribute(MacroAttribute),
    BareImpl(BareImpl),
}

ts_new_type!(ImplBlock);
ts_new_type!(MacroAttribute);
ts_new_type!(BareImpl);

impl Implementation {
    pub(crate) fn separate_variants(
        impls: &[Self],
    ) -> (
        impl Iterator<Item = &ImplBlock>,
        impl Iterator<Item = &MacroAttribute>,
        impl Iterator<Item = &BareImpl>,
    ) {
        let impl_blocks = ts_new_type!(iter_of ImplBlock in impls);
        let proc_macro_attrs = ts_new_type!(iter_of MacroAttribute in impls);
        let bare_impls = ts_new_type!(iter_of BareImpl in impls);

        (impl_blocks, proc_macro_attrs, bare_impls)
    }
}
