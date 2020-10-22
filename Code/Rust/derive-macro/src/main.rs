extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DerivedInput};

#[proc_macro_derive(|\colorbox{function}{GetType}|)]
pub fn derive_answer_fn(|\colorbox{context}{tokens: TokenStream}|) -> |\colorbox{output}{TokenStream}| {
    let context = parse_macro_input!(tokens as DerivedInput);
    let name = context.ident;

    let output = quote! {
        impl GetType for |\#|name {
            fn get_type(&self) -> str {
                stringify!(|\#|name)
            }
        }
    }

    output.into()
}

|\#|(derive(|\colorbox{function}{GetType}|))
|\colorbox{context}{struct SomeStruct;}|
|\colorbox{output}{impl GetType for SomeStruct \{ ... \}}|