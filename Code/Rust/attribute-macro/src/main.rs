extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn |\colorbox{function}{reflect\_two}|(|\colorbox{input}{attr: TokenStream}|, |\colorbox{context}{item: TokenStream}|) -> |\colorbox{output}{TokenStream}| {
    item
}

