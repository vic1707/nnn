/* Modules */
pub(crate) mod closure;
pub(crate) mod regex_input;
pub(crate) mod syn_ext;

pub(crate) fn capitalize(str: &str) -> String {
    let (head, tail) = str.split_at(1);
    format!("{}{}", head.to_uppercase(), tail)
}
