mod abstract_factory;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse_macro_input;

use abstract_factory::{abstract_factory_function, AbstractFactoryFunction};

#[proc_macro]
pub fn abstract_factory(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as AbstractFactoryFunction);

    abstract_factory_function(&input).into()
}