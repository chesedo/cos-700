extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn |\colorbox{function}{reflect}|(|\colorbox{input}{input: TokenStream}|) -> |\colorbox{output}{TokenStream}| {
    input
}
