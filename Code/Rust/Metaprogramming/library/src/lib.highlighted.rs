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
    // Parse the context tokens to a DeriveInput syntax tree
    let context = parse_macro_input!(tokens as DeriveInput);
    // Get the type name from the syntax tree
    let name = &context.ident;
    // Construct the output using the quote macro
    let output = quote! {
        impl GetType for |\#|name {
            fn get_type(&self) -> String {
                String::from(stringify!(|\#|name))
            }
        }
    };

    output.into() // Convert the output to a TokenStream
}

#[proc_macro_attribute]
pub fn |\functionh{reflect\_two}|(|\inputh{input: TokenStream}|, |\contexth{context: TokenStream}|) -> |\outputh{TokenStream}| {
    context
}
