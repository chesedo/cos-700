extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn reflect(input: TokenStream) -> TokenStream {
    input
}

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GetType)]
pub fn derive_answer_fn(tokens: TokenStream) -> TokenStream {
    // Parse the context tokens to a DeriveInput syntax tree
    let context = parse_macro_input!(tokens as DeriveInput);
    // Get the type name from the syntax tree
    let name = &context.ident;
    // Construct the output using the quote macro
    let output = quote! {
        impl GetType for #name {
            fn get_type(&self) -> String {
                String::from(stringify!(#name))
            }
        }
    };

    output.into() // Convert the output to a TokenStream
}

#[proc_macro_attribute]
pub fn reflect_two(input: TokenStream, context: TokenStream) -> TokenStream {
    context
}
