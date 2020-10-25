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
    let context = parse_macro_input!(tokens as DeriveInput);
    let name = &context.ident;

    let output = quote! {
        impl GetType for #name {
            fn get_type(&self) -> String {
                String::from(stringify!(#name))
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn reflect_two(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
