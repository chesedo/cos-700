extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn |\functionh{reflect}|(|\inputh{input: TokenStream}|) -> |\outputh{TokenStream}| {
    input
}

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(|\functionh{GetType}|)]
pub fn derive_answer_fn(|\contexth{tokens: TokenStream}|) -> |\outputh{TokenStream}| {
    let context = parse_macro_input!(tokens as DeriveInput);
    let name = &context.ident;

    let output = quote! {
        impl GetType for |\#|name {
            fn get_type(&self) -> String {
                String::from(stringify!(|\#|name))
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn |\functionh{reflect\_two}|(|\inputh{attr: TokenStream}|, |\contexth{item: TokenStream}|) -> |\outputh{TokenStream}| {
    item
}
